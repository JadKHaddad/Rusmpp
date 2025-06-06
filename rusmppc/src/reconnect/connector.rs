use tokio::sync::{mpsc::UnboundedSender, watch};
use tokio_stream::wrappers::UnboundedReceiverStream;

use crate::{Action, Connection, Event, error::Error};

pub enum ConnectType {
    Connect,
    Reconnect,
}

#[allow(clippy::type_complexity)]
pub struct Connector<S, F> {
    connected: Option<(
        Connection<S>,
        watch::Sender<()>,
        UnboundedSender<Action>,
        UnboundedReceiverStream<Event>,
    )>,
    connect: F,
}

impl<S, F, Fut> Connector<S, F>
where
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
    ) -> Self {
        Self {
            connected: Some(connected),
            connect,
        }
    }

    pub async fn connect(
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
