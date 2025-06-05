//! Automatically reconnecting connection.

use std::time::Duration;

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
    Action, Connection, Event,
    error::Error,
    reconnect::{ReconnectingEvent, connector::ConnectType, error::ReconnectingError},
};

use super::connector::Connector;

const TARGET: &str = "rusmppc::connection::reconnect";

pub(crate) struct ReconnectingConnection<S, F> {
    connector: Connector<S, F>,
    events: UnboundedSender<ReconnectingEvent>,
    actions: UnboundedReceiverStream<Action>,
    _watch: watch::Receiver<()>,
    delay: Duration,
    max_retries: usize,
}

impl<S, F, Fut> ReconnectingConnection<S, F>
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
{
    pub fn new(
        connected: (
            Connection<S>,
            watch::Sender<()>,
            UnboundedSender<Action>,
            UnboundedReceiverStream<Event>,
        ),
        connect: F,
        delay: Duration,
        max_retries: usize,
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
                events: events_tx,
                actions: UnboundedReceiverStream::new(actions_rx),
                _watch: watch_rx,
                delay,
                max_retries,
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
        let mut max_retries = self.max_retries;
        let mut close = false;

        'outer: loop {
            match connector.connect().await {
                Err(err) => {
                    tracing::error!(target: TARGET, ?err, "Failed to connect");

                    let _ = events.send(ReconnectingEvent::Connection(Event::error(err)));
                }
                Ok((connect_type, (connection, _, connection_actions, mut connection_events))) => {
                    if matches!(connect_type, ConnectType::Reconnect) {
                        let _ = events.send(ReconnectingEvent::Reconnected);
                    }

                    max_retries = self.max_retries;

                    tokio::pin!(connection);

                    'inner: loop {
                        tokio::select! {
                            _ = &mut connection => {
                                tracing::warn!(target: TARGET, "Disconnected");

                                let _ = events.send(ReconnectingEvent::Disconnected);

                                break 'inner;
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

            if max_retries == 0 {
                tracing::error!(target: TARGET, max_retries=%self.max_retries, "Max retries exceeded");

                let _ = events.send(ReconnectingEvent::error(
                    ReconnectingError::max_retries_exceeded(self.max_retries),
                ));

                break 'outer;
            }

            max_retries -= 1;

            tracing::debug!(target: TARGET, delay=?self.delay, "Reconnecting after delay");

            tokio::time::sleep(self.delay).await;

            tracing::debug!(target: TARGET, current_retry=%(self.max_retries - max_retries), max_retries=%self.max_retries, "Reconnecting");
        }
    }
}
