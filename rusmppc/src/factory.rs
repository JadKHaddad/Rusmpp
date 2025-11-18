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

use crate::{Client, ConnectionBuilder, Event};

#[non_exhaustive]
pub struct GenericFactory;

impl GenericFactory {
    pub fn tuple<Item, Error, F, Fut, DisconnectedFut>(
        f: F,
    ) -> (GenericHandle<Item>, impl Future<Output = ()>)
    where
        Item: Clone,
        F: Fn() -> Fut,
        Fut: Future<Output = Result<(Item, DisconnectedFut), Error>>,
        // A future, when resolved, `connect` must be called again.
        DisconnectedFut: Future<Output = ()>,
    {
        let (tx, mut rx) = unbounded::<Request<Item>>();

        let future = async move {
            'outer: loop {
                tracing::info!("Connecting");

                match f().await {
                    Ok((item, disconnected)) => {
                        tracing::info!("Connected");

                        let mut disconnected = pin!(disconnected);

                        'inner: loop {
                            tokio::select! {
                                request = rx.next() => {
                                    match request {
                                        None => {
                                            tracing::info!("Factory closed, stopping");
                                            break 'outer;
                                        }
                                        Some(request) => {
                                            request.tx.send(item.clone()).ok();
                                        }
                                    }
                                },
                                _ = &mut disconnected => {
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

    pub fn spawn<Item, Error, F, Fut, DisconnectedFut>(f: F) -> GenericHandle<Item>
    where
        Item: Clone + Send + 'static,
        Error: Send + 'static,
        F: Fn() -> Fut + Send + 'static,
        Fut: Future<Output = Result<(Item, DisconnectedFut), Error>> + Send + 'static,
        // A future, when resolved, `connect` must be called again.
        DisconnectedFut: Future<Output = ()> + Send + 'static,
    {
        let (handle, future) = Self::tuple(f);

        tokio::spawn(future);

        handle
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

#[derive(Debug)]
pub struct FactoryBuilder<F> {
    builder: ConnectionBuilder,
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
            on_connect: |client: Client| {
                Box::pin(async { Ok(client) })
                    as Pin<Box<dyn Future<Output = Result<Client, crate::error::Error>> + Send>>
            },
        }
    }
}

impl<F> FactoryBuilder<F> {
    pub fn no_spawn(self) -> NoSpawnFactoryBuilder<F> {
        NoSpawnFactoryBuilder { builder: self }
    }

    pub fn on_connect<K>(self, on_connect: K) -> FactoryBuilder<K> {
        FactoryBuilder {
            builder: self.builder,
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

#[derive(Debug)]
pub struct NoSpawnFactoryBuilder<F> {
    builder: FactoryBuilder<F>,
}

impl<F> NoSpawnFactoryBuilder<F> {
    async fn connect_once<Fun, Fut>(
        url: impl AsRef<str>,
        tx: UnboundedSender<Event>,
        builder: ConnectionBuilder,
        on_connect: Fun,
    ) -> Result<(Client, impl Future<Output = ()>), crate::error::Error>
    where
        Fun: Fn(Client) -> Fut,
        Fut: Future<Output = Result<Client, crate::error::Error>> + 'static,
    {
        let (client, events, future) = builder.no_spawn().connect(url).await?;

        // FIX: because we call on_connect here we cant poll the future in the Factory
        // In the factory we should the connection and the disconnect (events forward).
        // The factory should call on_connect
        tokio::spawn(future);

        let client = on_connect(client).await?;

        let disconnected = events.map(Ok).forward(tx).unwrap_or_else(|_| ());

        Ok((client, disconnected))
    }

    async fn connect_retry<Fun, Fut>(
        url: impl AsRef<str> + Clone,
        tx: UnboundedSender<Event>,
        builder: ConnectionBuilder,
        on_connect: Fun,
    ) -> Result<(Client, impl Future<Output = ()>), crate::error::Error>
    where
        Fun: Fn(Client) -> Fut + Clone,
        Fut: Future<Output = Result<Client, crate::error::Error>> + 'static,
    {
        tryhard::retry_fn(move || {
            Self::connect_once(url.clone(), tx.clone(), builder.clone(), on_connect.clone())
        })
        .retries(10)
        .exponential_backoff(Duration::from_millis(500))
        .max_delay(Duration::from_secs(10))
        .await
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

        let (handle, future) = GenericFactory::tuple(move || {
            Self::connect_retry(
                url.clone(),
                tx.clone(),
                self.builder.builder.clone(),
                self.builder.on_connect.clone(),
            )
        });

        (ClientHandle::new(handle), rx, future)
    }
}
