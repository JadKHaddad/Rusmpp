use std::{pin::Pin, time::Duration};

use futures::StreamExt;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{
        mpsc::{self, UnboundedSender},
        watch,
    },
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{Action, Connection, Event, error::Error};

const TARGET: &str = "rusmppc::connection::reconnect";

// The output of the future matches the constructor of the `Connection` type.
pub type ConnectFunction<S> = Box<
    dyn Fn() -> Pin<
            Box<
                dyn Future<
                        Output = Result<
                            (
                                Connection<S>,
                                watch::Sender<()>,
                                UnboundedSender<Action>,
                                UnboundedReceiverStream<Event>,
                            ),
                            Error,
                        >,
                    > + Send,
            >,
        > + Send,
>;

enum ConnectType {
    Connect,
    Reconnect,
}

#[allow(clippy::type_complexity)]
struct Connector<S> {
    connected: Option<(
        Connection<S>,
        watch::Sender<()>,
        UnboundedSender<Action>,
        UnboundedReceiverStream<Event>,
    )>,
    connect: ConnectFunction<S>,
}

impl<S> Connector<S> {
    fn new(
        connected: (
            Connection<S>,
            watch::Sender<()>,
            UnboundedSender<Action>,
            UnboundedReceiverStream<Event>,
        ),
        connect: ConnectFunction<S>,
    ) -> Self {
        Self {
            connected: Some(connected),
            connect,
        }
    }

    async fn connect(
        &mut self,
    ) -> Result<
        (
            ConnectType,
            (
                Connection<S>,
                watch::Sender<()>,
                UnboundedSender<Action>,
                UnboundedReceiverStream<Event>,
            ),
        ),
        Error,
    > {
        match self.connected.take() {
            Some(connected) => Ok((ConnectType::Connect, connected)),
            None => Ok((ConnectType::Reconnect, (self.connect)().await?)),
        }
    }
}

pub struct ReconnectingConnection<S> {
    connector: Connector<S>,
    events: UnboundedSender<Event>,
    actions: UnboundedReceiverStream<Action>,
    _watch: watch::Receiver<()>,
    delay: Duration,
    max_retries: usize,
}

impl<S: AsyncRead + AsyncWrite + Send + Sync + 'static> ReconnectingConnection<S> {
    pub fn new(
        connected: (
            Connection<S>,
            watch::Sender<()>,
            UnboundedSender<Action>,
            UnboundedReceiverStream<Event>,
        ),
        connect: ConnectFunction<S>,
        delay: Duration,
        max_retries: usize,
    ) -> (
        Self,
        watch::Sender<()>,
        UnboundedSender<Action>,
        UnboundedReceiverStream<Event>,
    ) {
        let (events_tx, events_rx) = mpsc::unbounded_channel::<Event>();
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

                    let _ = events.send(Event::error(err));
                }
                Ok((connect_type, (connection, _, connection_actions, mut connection_events))) => {
                    if matches!(connect_type, ConnectType::Reconnect) {
                        let _ = events.send(Event::Reconnected);
                    }

                    max_retries = self.max_retries;

                    tokio::pin!(connection);

                    'inner: loop {
                        tokio::select! {
                            _ = &mut connection => {
                                tracing::warn!(target: TARGET, "Disconnected");

                                let _ = events.send(Event::Disconnected);

                                break 'inner;
                            }
                            event = connection_events.next() => {
                                // If event is None, the connection is closed.
                                // But we check for this already in the `connection` select branch.

                                if let Some(event) = event {
                                    let _ = events.send(event);
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

                let _ = events.send(Event::error(Error::max_retries_exceeded(self.max_retries)));

                break 'outer;
            }

            max_retries -= 1;

            tracing::debug!(target: TARGET, delay=?self.delay, "Reconnecting after delay");

            tokio::time::sleep(self.delay).await;

            tracing::debug!(target: TARGET, current_retry=%(self.max_retries - max_retries), max_retries=%self.max_retries, "Reconnecting");
        }
    }
}
