use std::{
    pin::{Pin, pin},
    time::Duration,
};

use futures::{
    Stream, StreamExt, TryFutureExt,
    channel::{
        mpsc::{UnboundedSender, unbounded},
        oneshot,
    },
};
use tracing::Instrument;
use tryhard::backoff_strategies::*;

use crate::{Client, ConnectionBuilder, Event};

#[non_exhaustive]
pub struct GenericFactory;

impl GenericFactory {
    pub fn tuple<Item, Error, F, Fut, ConnectionFuture, OnConnectF, OnConnectFut, OnConnectError>(
        f: F,
        on_connect: OnConnectF,
    ) -> (GenericHandle<Item>, impl Future<Output = ()>)
    where
        Item: Clone,
        F: Fn() -> Fut,
        Fut: Future<Output = Result<(Item, ConnectionFuture), Error>>,
        // A future, when resolved, `connect` must be called again.
        ConnectionFuture: Future<Output = ()>,
        OnConnectF: Fn(Item) -> OnConnectFut,
        OnConnectFut: Future<Output = Result<Item, OnConnectError>>,
    {
        let (tx, mut rx) = unbounded::<Request<Item>>();

        let future = async move {
            let mut id: usize = 0;

            'outer: loop {
                id += 1;

                tracing::info!("Connecting");

                match f().await {
                    Ok((item, connection)) => {
                        tracing::info!("Connected");

                        let connection =
                            connection.instrument(tracing::info_span!("connection", %id));

                        let mut connection = pin!(connection);

                        let item = tokio::select! {
                            _ = &mut connection => {
                                tracing::warn!("Disconnected");

                                continue;
                            },
                            item  = on_connect(item) => {
                                match item {
                                    Ok(item) => item,
                                    Err(_) => {
                                        tracing::error!("On connect failed");

                                        continue;
                                    }
                                }
                            }
                        };

                        'inner: loop {
                            tokio::select! {
                                request = rx.next() => {
                                    match request {
                                        None => {
                                            tracing::info!("Factory closed");

                                            connection.await;

                                            tracing::info!("Stopping");

                                            break 'outer;
                                        }
                                        Some(request) => {
                                            if request.tx.send(item.clone()).is_err() {
                                                tracing::info!("Factory closed");

                                                connection.await;

                                                tracing::info!("Stopping");

                                                break 'outer;
                                            }
                                        }
                                    }
                                },
                                _ = &mut connection => {
                                    tracing::warn!("Disconnected");

                                    break 'inner;
                                }
                            }
                        }
                    }
                    Err(_) => {
                        tracing::error!("Connect error");

                        break;
                    }
                }
            }
        };

        (GenericHandle::new(tx), future)
    }
}

#[derive(Clone)]
pub struct GenericHandle<T> {
    tx: UnboundedSender<Request<T>>,
}

impl<T> GenericHandle<T> {
    const fn new(tx: UnboundedSender<Request<T>>) -> Self {
        Self { tx }
    }

    async fn get(&self) -> Option<T> {
        let (request, response) = Request::new();

        self.tx.unbounded_send(request).ok()?;

        response.await.ok()
    }

    fn close(&self) {
        self.tx.close_channel();
    }
}

#[derive(Clone)]
pub struct ClientHandle {
    handle: GenericHandle<Client>,
}

impl std::fmt::Debug for ClientHandle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ClientHandle").finish()
    }
}

impl ClientHandle {
    const fn new(handle: GenericHandle<Client>) -> Self {
        Self { handle }
    }

    pub async fn get(&self) -> Option<Client> {
        self.handle.get().await
    }

    pub fn close(&self) {
        self.handle.close()
    }
}

struct Request<T> {
    tx: oneshot::Sender<T>,
}

impl<T> Request<T> {
    fn new() -> (Self, oneshot::Receiver<T>) {
        let (tx, rx) = oneshot::channel();

        (Self { tx }, rx)
    }
}

/// XXX: Do not expose.
#[derive(Debug, Clone)]
enum BackOff {
    None,
    Exponential(ExponentialBackoff),
    Fixed(FixedBackoff),
    Linear(LinearBackoff),
}

impl<'a, E> BackoffStrategy<'a, E> for BackOff {
    type Output = Duration;

    fn delay(&mut self, attempt: u32, error: &'a E) -> Duration {
        match self {
            BackOff::None => NoBackoff.delay(attempt, error),
            BackOff::Exponential(backoff) => backoff.delay(attempt, error),
            BackOff::Fixed(backoff) => backoff.delay(attempt, error),
            BackOff::Linear(backoff) => backoff.delay(attempt, error),
        }
    }
}

#[derive(Debug, Clone)]
pub struct FactoryBuilder<F> {
    builder: ConnectionBuilder,
    back_off: BackOff,
    max_delay: Option<Duration>,
    max_retries: u32,
    on_connect: F,
}

// TODO: on_connect should be able to return any error
impl FactoryBuilder<()> {
    #[allow(clippy::type_complexity)]
    pub(crate) fn new(
        builder: ConnectionBuilder,
    ) -> FactoryBuilder<
        impl Fn(Client) -> Pin<Box<dyn Future<Output = Result<Client, crate::error::Error>> + Send>>
        + Clone,
    > {
        FactoryBuilder {
            builder,
            back_off: BackOff::None,
            max_delay: None,
            max_retries: 10,
            on_connect: |client: Client| {
                Box::pin(async { Ok(client) })
                    as Pin<Box<dyn Future<Output = Result<Client, crate::error::Error>> + Send>>
            },
        }
    }
}

// XXX: Do not provide a `custom_backoff` like retry. If so, we would need a lot of generics.
impl<F> FactoryBuilder<F> {
    pub fn no_spawn(self) -> NoSpawnFactoryBuilder<F> {
        NoSpawnFactoryBuilder { builder: self }
    }

    pub fn max_delay(mut self, delay: Duration) -> Self {
        self.max_delay = Some(delay);
        self
    }

    pub fn no_max_delay(mut self) -> Self {
        self.max_delay = None;
        self
    }

    pub fn with_max_delay(mut self, delay: Option<Duration>) -> Self {
        self.max_delay = delay;
        self
    }

    pub fn no_backoff(mut self) -> Self {
        self.back_off = BackOff::None;
        self
    }

    pub fn exponential_backoff(mut self, initial_delay: Duration) -> Self {
        self.back_off = BackOff::Exponential(ExponentialBackoff::new(initial_delay));
        self
    }

    pub fn fixed_backoff(mut self, delay: Duration) -> Self {
        self.back_off = BackOff::Fixed(FixedBackoff::new(delay));
        self
    }

    pub fn linear_backoff(mut self, delay: Duration) -> Self {
        self.back_off = BackOff::Linear(LinearBackoff::new(delay));
        self
    }

    pub fn max_retries(mut self, retries: u32) -> Self {
        self.max_retries = retries;
        self
    }

    pub fn on_connect<K>(self, on_connect: K) -> FactoryBuilder<K> {
        FactoryBuilder {
            builder: self.builder,
            back_off: self.back_off,
            max_delay: self.max_delay,
            max_retries: self.max_retries,
            on_connect,
        }
    }

    pub fn connect<Fut>(
        self,
        url: impl AsRef<str> + Clone + Send + 'static,
    ) -> (ClientHandle, impl Stream<Item = Event> + Unpin + 'static)
    where
        F: Fn(Client) -> Fut + Clone + Send + 'static,
        Fut: Future<Output = Result<Client, crate::error::Error>> + Send + 'static,
    {
        let (handle, events, future) = self.no_spawn().connect(url);

        tokio::spawn(future);

        (handle, events)
    }
}

#[derive(Debug, Clone)]
pub struct NoSpawnFactoryBuilder<F> {
    builder: FactoryBuilder<F>,
}

impl<F> NoSpawnFactoryBuilder<F> {
    async fn connect_once(
        url: impl AsRef<str>,
        tx: UnboundedSender<Event>,
        builder: ConnectionBuilder,
    ) -> Result<(Client, impl Future<Output = ()>), crate::error::Error> {
        let (client, events, future) = builder.no_spawn().connect(url).await?;

        let future = async move {
            tokio::select! {
                _ = future => {},
                _ = events.map(Ok).forward(tx).unwrap_or_else(|_| ()) => {}
            }
        };

        Ok((client, future))
    }

    async fn connect_retry<Fut>(
        self,
        url: impl AsRef<str> + Clone,
        tx: UnboundedSender<Event>,
    ) -> Result<(Client, impl Future<Output = ()>), crate::error::Error>
    where
        F: Fn(Client) -> Fut + Clone,
        Fut: Future<Output = Result<Client, crate::error::Error>> + 'static,
    {
        let mut fut = tryhard::retry_fn(move || {
            Self::connect_once(url.clone(), tx.clone(), self.builder.builder.clone())
        })
        .retries(self.builder.max_retries)
        .custom_backoff(self.builder.back_off);

        if let Some(delay) = self.builder.max_delay {
            fut = fut.max_delay(delay)
        };

        fut.await
    }

    pub fn connect<Fut>(
        self,
        url: impl AsRef<str> + Clone + 'static,
    ) -> (
        ClientHandle,
        impl Stream<Item = Event> + Unpin + 'static,
        impl Future<Output = ()> + 'static,
    )
    where
        F: Fn(Client) -> Fut + Clone + 'static,
        Fut: Future<Output = Result<Client, crate::error::Error>> + 'static,
    {
        let (tx, rx) = unbounded::<Event>();

        let on_connect = self.builder.on_connect.clone();

        let (handle, future) = GenericFactory::tuple(
            move || self.clone().connect_retry(url.clone(), tx.clone()),
            on_connect,
        );

        (ClientHandle::new(handle), rx, future)
    }
}
