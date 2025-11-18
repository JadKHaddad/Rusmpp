use std::{convert::Infallible, pin::pin, time::Duration};

use futures::{
    Stream, StreamExt, TryFutureExt,
    channel::{
        mpsc::{UnboundedSender, unbounded},
        oneshot,
    },
};

use tokio::io::{AsyncRead, AsyncWrite};
use tracing::Instrument;
use tryhard::backoff_strategies::*;

use crate::{Client, ConnectionBuilder, Event};

struct GenericFactory;

impl GenericFactory {
    fn tuple<Item, Error, F, Fut, ConnectionFuture, O>(
        f: F,
        on_connect: O,
        restart: bool,
    ) -> (GenericHandle<Item>, impl Future<Output = ()>)
    where
        Item: Clone,
        F: Fn() -> Fut,
        Fut: Future<Output = Result<(Item, ConnectionFuture), Error>>,
        // A future, when resolved, `connect` must be called again.
        ConnectionFuture: Future<Output = ()>,
        O: OnConnect<Item = Item>,
    {
        let (tx, mut rx) = unbounded::<Request<Item>>();

        let future = async move {
            let mut id: usize = 0;

            'restart: loop {
                'outer: loop {
                    id += 1;

                    tracing::info!(%id, "Connecting");

                    match f().await {
                        Ok((item, connection)) => {
                            tracing::info!("Connected");

                            let connection =
                                connection.instrument(tracing::info_span!("connection", %id));

                            let mut connection = pin!(connection);

                            let item = tokio::select! {
                                _ = &mut connection => {
                                    tracing::warn!(%id, "Disconnected");

                                    continue 'outer;
                                },
                                item  = on_connect.on_connect(item) => {
                                    match item {
                                        Ok(item) => item,
                                        Err(_) => {
                                            tracing::error!(%id, "On connect error");

                                            continue 'outer;
                                        }
                                    }
                                }
                            };

                            'inner: loop {
                                tokio::select! {
                                    request = rx.next() => {
                                        match request {
                                            None => {
                                                tracing::info!(%id, "Closed");

                                                connection.await;

                                                tracing::info!(%id, "Stopping");

                                                break 'outer;
                                            }
                                            Some(request) => {
                                                if request.tx.send(item.clone()).is_err() {
                                                    tracing::info!(%id, "Closed");

                                                    connection.await;

                                                    tracing::info!(%id, "Stopping");

                                                    break 'outer;
                                                }
                                            }
                                        }
                                    },
                                    _ = &mut connection => {
                                        tracing::warn!(%id, "Disconnected");

                                        break 'inner;
                                    }
                                }
                            }
                        }
                        Err(_) => {
                            tracing::error!(%id, "Connect error");

                            break 'outer;
                        }
                    }
                }

                if !restart {
                    break 'restart;
                }

                tracing::info!(%id, "Restarting connection");
            }
        };

        (GenericHandle::new(tx), future)
    }
}

pub trait OnConnect {
    type Item;
    type Error;

    fn on_connect(
        &self,
        item: Self::Item,
    ) -> impl Future<Output = Result<Self::Item, Self::Error>> + Send;
}

impl OnConnect for () {
    type Item = Client;
    type Error = Infallible;

    async fn on_connect(&self, item: Self::Item) -> Result<Self::Item, Self::Error> {
        Ok(item)
    }
}

#[derive(Debug)]
pub struct OnConnectFn<F, Fut, Item, Error> {
    f: F,
    _phantom: std::marker::PhantomData<(Fut, Item, Error)>,
}

impl<F, Fut, Item, Error> Clone for OnConnectFn<F, Fut, Item, Error>
where
    F: Clone,
{
    fn clone(&self) -> Self {
        Self {
            f: self.f.clone(),
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F, Fut, Item, Error> OnConnectFn<F, Fut, Item, Error> {
    fn new(f: F) -> Self {
        Self {
            f,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<F, Fut, Item, Error> OnConnect for OnConnectFn<F, Fut, Item, Error>
where
    F: Fn(Item) -> Fut + Send + Sync,
    Fut: Future<Output = Result<Item, Error>> + Send,
{
    type Item = Item;
    type Error = Error;

    fn on_connect(
        &self,
        item: Self::Item,
    ) -> impl Future<Output = Result<Self::Item, Self::Error>> + Send {
        (self.f)(item)
    }
}

#[derive(Clone)]
struct GenericHandle<T> {
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
#[derive(Debug, Clone, Copy)]
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

// TODO: we need on_retry (when retry future retries), on_restart (when factory restarts), etc. callbacks.
#[derive(Debug, Clone)]
pub struct ReconnectBuilder<C> {
    builder: ConnectionBuilder,
    restart: bool,
    back_off: BackOff,
    max_delay: Option<Duration>,
    max_retries: u32,
    on_connect: C,
}

impl ReconnectBuilder<()> {
    #[allow(clippy::type_complexity)]
    pub(crate) fn new(builder: ConnectionBuilder) -> ReconnectBuilder<()> {
        ReconnectBuilder {
            builder,
            restart: false,
            back_off: BackOff::None,
            max_delay: None,
            max_retries: 10,
            on_connect: (),
        }
    }
}

// XXX: Do not provide a `custom_backoff` like retry. If so, we would need a lot of generics.
impl<C> ReconnectBuilder<C> {
    pub fn no_spawn(self) -> NoSpawnReconnectBuilder<C> {
        NoSpawnReconnectBuilder { builder: self }
    }

    pub fn restart(mut self) -> Self {
        self.restart = true;
        self
    }

    pub fn no_restart(mut self) -> Self {
        self.restart = false;
        self
    }

    pub fn with_restart(mut self, restart: bool) -> Self {
        self.restart = restart;
        self
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

    pub fn on_connect<K, Fut, Error>(
        self,
        on_connect: K,
    ) -> ReconnectBuilder<OnConnectFn<K, Fut, Client, Error>> {
        ReconnectBuilder {
            builder: self.builder,
            restart: self.restart,
            back_off: self.back_off,
            max_delay: self.max_delay,
            max_retries: self.max_retries,
            on_connect: OnConnectFn::new(on_connect),
        }
    }

    pub fn no_on_connect(self) -> ReconnectBuilder<()> {
        ReconnectBuilder {
            builder: self.builder,
            restart: self.restart,
            back_off: self.back_off,
            max_delay: self.max_delay,
            max_retries: self.max_retries,
            on_connect: (),
        }
    }

    pub fn connect(
        self,
        url: impl AsRef<str> + Clone + Send + 'static,
    ) -> (ClientHandle, impl Stream<Item = Event> + Unpin + 'static)
    where
        C: OnConnect<Item = Client> + Clone + Send + 'static,
    {
        let (handle, events, future) = self.no_spawn().connect(url);

        tokio::spawn(future);

        (handle, events)
    }

    pub fn connect_with<F, Fut, E, S>(
        self,
        connect: F,
    ) -> (ClientHandle, impl Stream<Item = Event> + Unpin + 'static)
    where
        F: FnOnce() -> Fut + Clone + Send + 'static,
        Fut: Future<Output = Result<S, E>> + Send + 'static,
        S: AsyncRead + AsyncWrite + Send + 'static,
        E: Into<std::io::Error> + Send + 'static,
        C: OnConnect<Item = Client> + Clone + Send + 'static,
    {
        let (handle, events, future) = self.no_spawn().connect_with(connect);

        tokio::spawn(future);

        (handle, events)
    }
}

#[derive(Debug, Clone)]
pub struct NoSpawnReconnectBuilder<C> {
    builder: ReconnectBuilder<C>,
}

impl<C> NoSpawnReconnectBuilder<C> {
    async fn connect_once<F, Fut, S, Conn>(
        connect: F,
        tx: UnboundedSender<Event>,
    ) -> Result<(Client, impl Future<Output = ()>), crate::error::Error>
    where
        F: FnOnce() -> Fut,
        Fut: Future<Output = Result<(Client, S, Conn), crate::error::Error>>,
        S: Stream<Item = Event> + Unpin + 'static,
        Conn: Future<Output = ()> + Send + 'static,
    {
        let (client, events, future) = connect().await?;

        let future = async move {
            tokio::select! {
                _ = future => {},
                _ = events.map(Ok).forward(tx).unwrap_or_else(|_| ()) => {}
            }
        };

        Ok((client, future))
    }

    async fn connect_retry<F, Fut, S, Conn>(
        connect: F,
        max_retries: u32,
        back_off: BackOff,
        max_delay: Option<Duration>,
        tx: UnboundedSender<Event>,
    ) -> Result<(Client, impl Future<Output = ()>), crate::error::Error>
    where
        F: FnOnce() -> Fut + Clone,
        Fut: Future<Output = Result<(Client, S, Conn), crate::error::Error>>,
        S: Stream<Item = Event> + Unpin + 'static,
        Conn: Future<Output = ()> + Send + 'static,
        C: OnConnect<Item = Client> + Clone + Send + 'static,
    {
        let mut fut = tryhard::retry_fn(move || Self::connect_once(connect.clone(), tx.clone()))
            .retries(max_retries)
            .custom_backoff(back_off);

        if let Some(delay) = max_delay {
            fut = fut.max_delay(delay)
        };

        fut.await
    }

    fn connected<F, Fut, S, Conn>(
        self,
        connect: F,
    ) -> (
        ClientHandle,
        impl Stream<Item = Event> + Unpin + 'static,
        impl Future<Output = ()> + 'static,
    )
    where
        F: FnOnce() -> Fut + Clone + 'static,
        Fut: Future<Output = Result<(Client, S, Conn), crate::error::Error>> + 'static,
        S: Stream<Item = Event> + Unpin + 'static,
        Conn: Future<Output = ()> + Send + 'static,
        C: OnConnect<Item = Client> + Clone + Send + 'static,
    {
        let (tx, rx) = unbounded::<Event>();

        let max_retries = self.builder.max_retries;
        let back_off = self.builder.back_off;
        let max_delay = self.builder.max_delay;
        let restart = self.builder.restart;

        let on_connect = self.builder.on_connect;

        let (handle, future) = GenericFactory::tuple(
            move || {
                Self::connect_retry(
                    connect.clone(),
                    max_retries,
                    back_off,
                    max_delay,
                    tx.clone(),
                )
            },
            on_connect,
            restart,
        );

        (ClientHandle::new(handle), rx, future)
    }

    pub fn connect(
        self,
        url: impl AsRef<str> + Clone + 'static,
    ) -> (
        ClientHandle,
        impl Stream<Item = Event> + Unpin + 'static,
        impl Future<Output = ()> + 'static,
    )
    where
        C: OnConnect<Item = Client> + Clone + Send + 'static,
    {
        let this = self.clone();
        let connect = move || this.builder.builder.no_spawn().connect(url);

        self.connected(connect)
    }

    pub fn connect_with<F, Fut, E, S>(
        self,
        connect: F,
    ) -> (
        ClientHandle,
        impl Stream<Item = Event> + Unpin + 'static,
        impl Future<Output = ()> + 'static,
    )
    where
        F: FnOnce() -> Fut + Clone + Send + 'static,
        Fut: Future<Output = Result<S, E>> + Send + 'static,
        S: AsyncRead + AsyncWrite + Send + 'static,
        E: Into<std::io::Error> + Send + 'static,
        C: OnConnect<Item = Client> + Clone + Send + 'static,
    {
        let this = self.clone();

        let connect = move || async move {
            let stream = connect()
                .await
                .map_err(Into::into)
                .map_err(crate::error::Error::Connect)?;

            let connected = this.builder.builder.no_spawn().connected(stream);

            Ok(connected)
        };

        self.connected(connect)
    }
}
