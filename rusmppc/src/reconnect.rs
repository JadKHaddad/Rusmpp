use std::{pin::Pin, time::Duration};

use futures::StreamExt;
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{mpsc::UnboundedSender, watch},
};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{Action, Connection, Event, error::Error};

pub type ConnectFunction<S> = Box<
    dyn Fn() -> Pin<
            Box<
                dyn Future<
                        Output = Result<
                            (
                                Connection<S>,
                                UnboundedSender<Action>,
                                UnboundedReceiverStream<Event>,
                            ),
                            Error,
                        >,
                    > + Send,
            >,
        > + Send,
>;

pub struct ReconnectingConnection<S> {
    connect: ConnectFunction<S>,
    events: UnboundedSender<Event>,
    actions: UnboundedReceiverStream<Action>,
    _watch: watch::Receiver<()>,
}

impl<S: AsyncRead + AsyncWrite + Send + Sync + 'static> ReconnectingConnection<S> {
    pub fn new(
        connect: ConnectFunction<S>,
        events: UnboundedSender<Event>,
        actions: UnboundedReceiverStream<Action>,
        _watch: watch::Receiver<()>,
    ) -> Self {
        Self {
            connect,
            events,
            actions,
            _watch,
        }
    }

    pub async fn run(
        self,
        connected: (
            Connection<S>,
            UnboundedSender<Action>,
            UnboundedReceiverStream<Event>,
        ),
    ) {
        let connect = self.connect;
        let events = self.events;
        let mut actions = self.actions;

        let (connection, mut connection_actions, mut connection_events) = connected;

        tokio::spawn(connection);

        'outer: loop {
            'inner: loop {
                tokio::select! {
                    event = connection_events.next() => {
                        match event {
                            None => {
                                tracing::warn!("Disconnected");

                                break 'inner;
                            }
                            Some(event) => {
                                let _ = events.send(event);
                            }
                        }
                    }
                    action = actions.next() => {
                        match action {
                            None => {
                                tracing::warn!("Client dropped");

                                break 'outer;
                            }
                            Some(action) => {
                                // If this fails, the connection might be terminating.
                                let _ = connection_actions.send(action);
                            }
                        }
                    }
                }
            }

            tokio::time::sleep(Duration::from_secs(5)).await;

            tracing::debug!("Reconnecting");

            let (connection, new_connection_actions, new_connection_events) =
                connect().await.unwrap();

            connection_actions = new_connection_actions;
            connection_events = new_connection_events;

            tokio::spawn(connection);
        }
    }
}
