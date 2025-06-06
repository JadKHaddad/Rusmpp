use std::time::Duration;

use futures::{Stream, future::Ready};
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    Client, Connection, ConnectionBuilder,
    error::Error,
    reconnect::{
        connection::ReconnectingConnection, error::ReconnectingError, event::ReconnectingEvent,
    },
};

#[derive(Debug)]
pub struct ReconnectingConnectionBuilder<F, OnF> {
    builder: ConnectionBuilder,
    connect: F,
    on_connect: OnF,
    delay: Duration,
    max_retries: Option<usize>,
}

impl<S, F, Fut, OnF, OnFut> ReconnectingConnectionBuilder<F, OnF>
where
    S: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static,
    F: Fn() -> Fut + Send + Clone + 'static,
    Fut: Future<Output = Result<S, std::io::Error>> + Send,
    OnF: Fn(Client) -> OnFut + Send + 'static,
    OnFut: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>>
        + Send
        + 'static,
{
    pub async fn connect(
        self,
    ) -> Result<
        (
            Client,
            impl Stream<Item = ReconnectingEvent> + Unpin + 'static,
        ),
        ReconnectingError,
    > {
        let stream = (self.connect)().await.map_err(Error::Connect)?;

        let connected = Connection::new(
            stream,
            self.builder.max_command_length,
            self.builder.enquire_link_interval,
            self.builder.enquire_link_response_timeout,
        );

        let (reconnecting_connection, watch, actions, events) = ReconnectingConnection::new(
            connected,
            move || {
                let connect = self.connect.clone();

                async move {
                    let stream = connect().await.map_err(Error::Connect)?;

                    Ok::<_, Error>(Connection::new(
                        stream,
                        self.builder.max_command_length,
                        self.builder.enquire_link_interval,
                        self.builder.enquire_link_response_timeout,
                    ))
                }
            },
            self.on_connect,
            self.delay,
            self.max_retries,
            self.builder.response_timeout,
            self.builder.check_interface_version,
        );

        let client = Client::new(
            actions,
            self.builder.response_timeout,
            self.builder.check_interface_version,
            watch,
        );

        tokio::spawn(reconnecting_connection.run());

        Ok((client, events))
    }
}

impl<F, OnF> ReconnectingConnectionBuilder<F, OnF> {
    pub(crate) const fn new(builder: ConnectionBuilder, connect: F, on_connect: OnF) -> Self {
        Self {
            builder,
            connect,
            on_connect,
            delay: Duration::from_secs(5),
            max_retries: None,
        }
    }

    pub const fn delay(mut self, delay: Duration) -> Self {
        self.delay = delay;
        self
    }

    pub const fn max_retries(mut self, max_retries: usize) -> Self {
        self.max_retries = Some(max_retries);
        self
    }

    pub fn on_connect<OnF2, OnFut2>(
        self,
        on_connect: OnF2,
    ) -> ReconnectingConnectionBuilder<F, OnF2>
    where
        OnF2: Fn(Client) -> OnFut2,
        OnFut2: Future<Output = Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>>,
    {
        ReconnectingConnectionBuilder {
            builder: self.builder,
            connect: self.connect,
            on_connect,
            delay: self.delay,
            max_retries: self.max_retries,
        }
    }
}

impl ConnectionBuilder {
    pub fn reconnect_with<S, F, Fut>(
        self,
        connect: F,
    ) -> ReconnectingConnectionBuilder<
        F,
        fn(Client) -> Ready<Result<(), Box<dyn std::error::Error + Send + Sync + 'static>>>,
    >
    where
        S: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static,
        F: Fn() -> Fut + Send + Clone + 'static,
        Fut: Future<Output = Result<S, std::io::Error>> + Send,
    {
        ReconnectingConnectionBuilder::new(self, connect, |_| futures::future::ready(Ok(())))
    }
}
