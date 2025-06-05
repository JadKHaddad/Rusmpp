use std::time::Duration;

use futures::Stream;
use tokio::io::{AsyncRead, AsyncWrite};

use crate::{
    Client, Connection, ConnectionBuilder,
    error::Error,
    reconnect::{ReconnectingEvent, connection::ReconnectingConnection},
};

#[derive(Debug)]
pub struct ReconnectingConnectionBuilder<F> {
    builder: ConnectionBuilder,
    connect: F,
    delay: Duration,
    max_retries: Option<usize>,
}

impl<S, F, Fut> ReconnectingConnectionBuilder<F>
where
    S: AsyncRead + AsyncWrite + Send + Sync + Unpin + 'static,
    F: Fn() -> Fut + Send + Clone + 'static,
    Fut: Future<Output = Result<S, std::io::Error>> + Send,
{
    pub async fn connect(
        self,
    ) -> Result<
        (
            Client,
            impl Stream<Item = ReconnectingEvent> + Unpin + 'static,
        ),
        Error,
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
            self.delay,
            self.max_retries,
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

impl<F> ReconnectingConnectionBuilder<F> {
    pub(crate) const fn new(builder: ConnectionBuilder, connect: F) -> Self {
        Self {
            builder,
            connect,
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
}
