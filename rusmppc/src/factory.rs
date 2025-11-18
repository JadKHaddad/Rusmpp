use std::{pin::pin, time::Duration};

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

    pub async fn get(&self) -> Option<T> {
        let (request, response) = Request::new();

        self.tx.unbounded_send(request).ok()?;

        response.await.ok()
    }
}

#[derive(Clone)]
pub struct ClientHandle {
    handle: GenericHandle<Client>,
}

impl ClientHandle {
    const fn new(handle: GenericHandle<Client>) -> Self {
        Self { handle }
    }

    pub async fn get(&self) -> Option<Client> {
        self.handle.get().await
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
pub struct FactoryBuilder {
    connection_builder: ConnectionBuilder,
}

impl FactoryBuilder {
    pub fn no_spawn(self) -> NoSpawnFactoryBuilder {
        NoSpawnFactoryBuilder { builder: self }
    }

    pub async fn connect(
        self,
        url: impl AsRef<str> + Clone + Send + 'static,
    ) -> (ClientHandle, impl Stream<Item = Event> + Unpin + 'static) {
        let (handle, events, future) = self.no_spawn().connect(url).await;

        tokio::spawn(future);

        (handle, events)
    }
}

#[derive(Debug)]
pub struct NoSpawnFactoryBuilder {
    builder: FactoryBuilder,
}

impl NoSpawnFactoryBuilder {
    async fn connect_once(
        url: impl AsRef<str>,
        tx: UnboundedSender<Event>,
        builder: ConnectionBuilder,
    ) -> Result<(Client, impl Future<Output = ()>), crate::error::Error> {
        let (client, events, future) = builder.no_spawn().connect(url).await?;

        /// TODO: add connection id to the span
        tokio::spawn(future);

        let disconnected = events.map(Ok).forward(tx).unwrap_or_else(|_| ());

        Ok((client, disconnected))
    }

    async fn connect_retry(
        url: impl AsRef<str> + Clone,
        tx: UnboundedSender<Event>,
        builder: ConnectionBuilder,
    ) -> Result<(Client, impl Future<Output = ()>), crate::error::Error> {
        // TODO: the connect_once spawns. how do we get the future out of it to spawn it?
        tryhard::retry_fn(move || Self::connect_once(url.clone(), tx.clone(), builder.clone()))
            .retries(10)
            .exponential_backoff(Duration::from_millis(500))
            .max_delay(Duration::from_secs(10))
            .await
    }

    pub async fn connect(
        self,
        url: impl AsRef<str> + Clone + 'static,
    ) -> (
        ClientHandle,
        impl Stream<Item = Event> + Unpin + 'static,
        impl Future<Output = ()> + 'static,
    ) {
        let (tx, rx) = unbounded::<Event>();

        let (handle, future) = GenericFactory::tuple(move || {
            Self::connect_retry(
                url.clone(),
                tx.clone(),
                self.builder.connection_builder.clone(),
            )
        });

        (ClientHandle::new(handle), rx, future)
    }
}
