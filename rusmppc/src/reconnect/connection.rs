//! Automatically reconnecting connection.

use std::{ops::ControlFlow, time::Duration};

use futures::StreamExt;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{
        mpsc::{self, UnboundedSender},
        watch,
    },
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{
    Action, Client, Connection, Event,
    error::Error,
    reconnect::{connector::ConnectType, error::ReconnectingError, event::ReconnectingEvent},
};

use super::connector::Connector;

const TARGET: &str = "rusmppc::connection::reconnect";

pub(crate) struct ReconnectingConnection<S, F, OnF> {
    connector: Connector<S, F>,
    on_connect: OnF,
    events: UnboundedSender<ReconnectingEvent>,
    actions: UnboundedReceiverStream<Action>,
    _watch: watch::Receiver<()>,
    delay: Duration,
    max_retries: Option<usize>,
    /// Used in the on_connect callback.
    response_timeout: Option<Duration>,
    /// Used in the on_connect callback.
    check_interface_version: bool,
}

impl<S, F, Fut, OnF, OnFut> ReconnectingConnection<S, F, OnF>
where
    S: AsyncRead + AsyncWrite + Send + Sync + 'static,
    F: Fn() -> Fut,
    Fut: Future<
        Output = Result<
            (
                Connection<S>,
                watch::Sender<()>,
                UnboundedSender<Action>,
                UnboundedReceiverStream<Event>,
            ),
            Error,
        >,
    >,
    OnF: Fn(Client) -> OnFut,
    OnFut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>>,
{
    pub fn new(
        connected: (
            Connection<S>,
            watch::Sender<()>,
            UnboundedSender<Action>,
            UnboundedReceiverStream<Event>,
        ),
        connect: F,
        on_connect: OnF,
        delay: Duration,
        max_retries: Option<usize>,
        response_timeout: Option<Duration>,
        check_interface_version: bool,
    ) -> (
        Self,
        watch::Sender<()>,
        UnboundedSender<Action>,
        UnboundedReceiverStream<ReconnectingEvent>,
    ) {
        let (events_tx, events_rx) = mpsc::unbounded_channel::<ReconnectingEvent>();
        let (actions_tx, actions_rx) = mpsc::unbounded_channel::<Action>();
        let (watch_tx, watch_rx) = watch::channel(());

        (
            Self {
                connector: Connector::new(connected, connect),
                on_connect,
                events: events_tx,
                actions: UnboundedReceiverStream::new(actions_rx),
                _watch: watch_rx,
                delay,
                max_retries,
                response_timeout,
                check_interface_version,
            },
            watch_tx,
            actions_tx,
            UnboundedReceiverStream::new(events_rx),
        )
    }

    pub async fn run(self) {
        let events = self.events;
        let mut actions = self.actions;
        let mut connector = self.connector;
        let on_connect = self.on_connect;
        let mut retries = 0;
        let mut close = false;

        'outer: loop {
            let connect = connector.connect();

            tokio::pin!(connect);

            let result = 'connecting: loop {
                tokio::select! {
                    result = &mut connect => {
                        break 'connecting result;
                    }
                    action = actions.next() => {
                        if let ControlFlow::Break(()) = action_while_reconnecting(action) {
                            break 'outer
                        }
                    }
                }
            };

            match result {
                Err(err) => {
                    tracing::error!(target: TARGET, ?err, "Failed to connect");

                    let _ = events.send(ReconnectingEvent::Connection(Event::error(err)));
                }

                Ok((
                    connect_type,
                    (connection, _watch, connection_actions, mut connection_events),
                )) => {
                    tracing::debug!(target: TARGET, "Connected");

                    tokio::pin!(connection);

                    // on_connect takes ownership of the client, so we don't have to drop it to prevent holding a reference to actions channel.
                    let client = Client::new(
                        connection_actions.clone(),
                        self.response_timeout,
                        self.check_interface_version,
                        _watch,
                    );

                    tracing::trace!(target: TARGET, "Executing on_connect callback");

                    tokio::select! {
                        _ = &mut connection => {
                            tracing::warn!(target: TARGET, "Disconnected"); // Wtf, How unlucky is this?

                            let _ = events.send(ReconnectingEvent::Disconnected);

                            continue 'outer;
                        },
                        result = on_connect(client) => {
                            if let Err(err) = result {
                                tracing::error!(target: TARGET, ?err, "Failed to execute on_connect callback");

                                let _ = events.send(ReconnectingEvent::Error(ReconnectingError::OnConnectError(err)));

                                continue 'outer;
                            }
                        }
                        action = actions.next() => {
                            if let ControlFlow::Break(()) = action_while_reconnecting(action) {
                                break 'outer
                            }
                        }
                    }

                    if matches!(connect_type, ConnectType::Reconnect) {
                        let _ = events.send(ReconnectingEvent::Reconnected);
                    }

                    retries = 0;

                    'connected: loop {
                        tokio::select! {
                            _ = &mut connection => {
                                tracing::warn!(target: TARGET, "Disconnected");

                                let _ = events.send(ReconnectingEvent::Disconnected);

                                break 'connected;
                            }
                            event = connection_events.next() => {
                                // If event is None, the connection is closed.
                                // But we check for this already in the `connection` select branch.

                                if let Some(event) = event {
                                    let _ = events.send(ReconnectingEvent::from(event));
                                }
                            }
                            action = actions.next() => {
                                match action {
                                    None => {
                                        tracing::debug!(target: TARGET, "Client dropped");

                                        break 'outer;
                                    }
                                    Some(action) => {
                                        close = matches!(action, Action::Close(_));

                                        // If this fails, the connection might be terminating.
                                        let _ = connection_actions.send(action);
                                    }
                                }
                            }
                        }
                    }
                }
            }

            if close {
                break 'outer;
            }

            if let Some(max_retries) = self.max_retries {
                if retries >= max_retries {
                    tracing::error!(target: TARGET, tries=%retries, max_retries=%max_retries, "Max retries exceeded");

                    let _ = events.send(ReconnectingEvent::error(
                        ReconnectingError::max_retries_exceeded(max_retries),
                    ));

                    break 'outer;
                }
            }

            retries += 1;

            tracing::debug!(target: TARGET, delay=?self.delay, "Reconnecting after delay");

            let sleep = tokio::time::sleep(self.delay);
            tokio::pin!(sleep);

            'delay: loop {
                tokio::select! {
                    _ = &mut sleep => {
                        break 'delay;
                    }
                    action = actions.next() => {
                        if let ControlFlow::Break(()) = action_while_reconnecting(action) {
                            break 'outer
                        }
                    }
                }
            }

            tracing::debug!(target: TARGET, current_retry=retries, max_retries=?self.max_retries, "Reconnecting");
        }
    }
}

fn action_while_reconnecting(action: Option<Action>) -> ControlFlow<()> {
    match action {
        None => {
            tracing::debug!(target: TARGET, "Client dropped");

            ControlFlow::Break(())
        }
        Some(action) => match action {
            Action::Request(request) => {
                let _ = request.send_ack(Err(Error::ConnectionClosed));

                ControlFlow::Continue(())
            }
            Action::Remove(_) => {
                // Don't care, it was not there any ways

                ControlFlow::Continue(())
            }
            Action::Close(request) => {
                let _ = request.ack.send(());

                ControlFlow::Break(())
            }
            Action::PendingResponses(request) => {
                let _ = request.ack.send(Err(Error::ConnectionClosed));

                ControlFlow::Continue(())
            }
        },
    }
}
