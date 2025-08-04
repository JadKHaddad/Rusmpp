use std::{
    collections::{BTreeMap, VecDeque},
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use crate::{Action, Event, Request, Timer, UnregisteredRequest, error::Error};

use futures::Sink;
use futures::Stream;
use pin_project_lite::pin_project;
use rusmpp::{Command, CommandId, CommandStatus, Pdu, codec::CommandCodec};
use tokio::{
    io::{AsyncRead, AsyncWrite},
    sync::{
        mpsc::{self, UnboundedSender},
        oneshot, watch,
    },
};
use tokio_stream::wrappers::UnboundedReceiverStream;
use tokio_util::codec::Framed;

const CONN: &str = "rusmppc::connection";
const TIMER: &str = "rusmppc::connection::timer";

#[derive(Debug)]
enum State {
    Active,
    /// The user sent a close request.
    Closing,
    Errored,
}

// Make sure to drop the Connection after completion to prevent clients from queueing more actions.
// This way if the Connection was closed and the Connection is not in an active state but not dropped,
// clients will still be able to send actions and will not get an immediate error that the channel is closed.
// Actions will not be queued and the client would wait forever until the Connection is dropped.
// We rely on this mechanism to work, to report correct and predictable errors.
pin_project! {
    #[derive(Debug)]
    pub struct Connection<S> {
        state: State,
        sequence_number: u32,
        requests: VecDeque<Request>,
        // This is a request that has been written to the sink using start_send, but not yet flushed.
        pending_request: Option<Request>,
        responses: BTreeMap<u32, oneshot::Sender<Command>>,
        enquire_link_interval: Duration,
        last_enquire_link_sequence_number: Option<u32>,
        enquire_link_response_timeout: Duration,
        events: UnboundedSender<Event>,
        // Used to let the client wait for the connection to be closed
        _watch: watch::Receiver<()>,
        #[pin]
        enquire_link_timer: Timer,
        #[pin]
        enquire_link_response_timer: Timer,
        #[pin]
        framed: Framed<S, CommandCodec>,
        #[pin]
        actions: UnboundedReceiverStream<Action>,
    }
}

impl<S: AsyncRead + AsyncWrite> Connection<S> {
    pub fn new(
        stream: S,
        max_command_length: usize,
        enquire_link_interval: Duration,
        enquire_link_response_timeout: Duration,
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
                state: State::Active,
                sequence_number: 2,
                requests: VecDeque::new(),
                pending_request: None,
                responses: BTreeMap::new(),
                enquire_link_interval,
                last_enquire_link_sequence_number: None,
                enquire_link_response_timeout,
                enquire_link_timer: Timer::active(enquire_link_interval),
                enquire_link_response_timer: Timer::inactive(),
                _watch: watch_rx,
                events: events_tx,
                framed: Framed::new(
                    stream,
                    CommandCodec::new().with_max_length(max_command_length),
                ),
                actions: UnboundedReceiverStream::new(actions_rx),
            },
            watch_tx,
            actions_tx,
            UnboundedReceiverStream::new(events_rx),
        )
    }

    fn insert_response(
        self: Pin<&mut Self>,
        sequence_number: u32,
        response: oneshot::Sender<Command>,
    ) {
        self.project().responses.insert(sequence_number, response);
    }

    fn remove_response(
        self: Pin<&mut Self>,
        sequence_number: u32,
    ) -> Option<oneshot::Sender<Command>> {
        self.project().responses.remove(&sequence_number)
    }

    fn requests_push_back(self: Pin<&mut Self>, request: Request) {
        self.project().requests.push_back(request);
    }

    fn requests_push_front(self: Pin<&mut Self>, request: Request) {
        self.project().requests.push_front(request);
    }

    fn requests_pop_front(self: Pin<&mut Self>) -> Option<Request> {
        self.project().requests.pop_front()
    }

    fn set_pending_request(self: Pin<&mut Self>, request: Request) {
        *self.project().pending_request = Some(request);
    }

    fn take_pending_request(self: Pin<&mut Self>) -> Option<Request> {
        self.project().pending_request.take()
    }

    fn set_state(self: Pin<&mut Self>, state: State) {
        *self.project().state = state;
    }

    fn deactivate_enquire_link_timer(self: Pin<&mut Self>) {
        self.project().enquire_link_timer.deactivate();

        tracing::trace!(target: TIMER, "Deactivated enquire_link_timer");
    }

    fn activate_enquire_link_timer(self: Pin<&mut Self>) {
        let delay = self.as_ref().enquire_link_interval;

        self.project().enquire_link_timer.activate(delay);

        tracing::trace!(target: TIMER, ?delay, "Activated enquire_link_timer");
    }

    fn set_last_enquire_link_sequence_number(self: Pin<&mut Self>, sequence_number: u32) {
        *self.project().last_enquire_link_sequence_number = Some(sequence_number);
    }

    fn unset_last_enquire_link_sequence_number(self: Pin<&mut Self>) {
        *self.project().last_enquire_link_sequence_number = None;
    }

    fn deactivate_enquire_link_response_timer(self: Pin<&mut Self>) {
        self.project().enquire_link_response_timer.deactivate();

        tracing::trace!(target: TIMER, "Deactivated enquire_link_response_timer");
    }

    fn activate_enquire_link_response_timer(self: Pin<&mut Self>) {
        let delay = self.as_ref().enquire_link_response_timeout;

        self.project().enquire_link_response_timer.activate(delay);

        tracing::trace!(target: TIMER, ?delay, "Activated enquire_link_response_timer");
    }

    /// [`Self::sequence_number`] is incremented by 2 after each call.
    ///
    /// The clients also hold an atomic sequence number, which is incremented by 2 for each request, starting from 1.
    ///
    /// This is done to ensure that commands sent by the connection [`EnquireLink`](Pdu::EnquireLink) are differentiated from the commands sent by the client,
    /// without the use of atomic operations in the connection.
    fn sequence_number_fetch_and_increment(self: Pin<&mut Self>) -> u32 {
        let sequence_number = self.sequence_number;

        *(self.project().sequence_number) += 2;

        sequence_number
    }
}

impl<S: AsyncRead + AsyncWrite> Future for Connection<S> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !matches!(self.state, State::Active | State::Closing) {
            return Poll::Ready(());
        }

        'main: loop {
            if matches!(self.state, State::Active) {
                match self.as_mut().project().enquire_link_response_timer.poll(cx) {
                    Poll::Ready(()) => {
                        tracing::error!(target: TIMER, "EnquireLinkResp timeout");

                        self.as_mut().set_state(State::Errored);

                        let timeout = self.enquire_link_response_timeout;

                        let _ = self
                            .as_mut()
                            .events
                            .send(Event::error(Error::EnquireLinkTimeout { timeout }));

                        return Poll::Ready(());
                    }
                    Poll::Pending => {}
                }

                match self.as_mut().project().enquire_link_timer.poll(cx) {
                    Poll::Ready(()) => {
                        let sequence_number = self.as_mut().sequence_number_fetch_and_increment();

                        tracing::trace!(target: TIMER, sequence_number, "EnquireLink");

                        let command = Command::builder()
                            .status(CommandStatus::EsmeRok)
                            .sequence_number(sequence_number)
                            .pdu(Pdu::EnquireLink);

                        let (request, _) = UnregisteredRequest::new(command);

                        self.as_mut()
                            .requests_push_front(Request::Unregistered(request));

                        self.as_mut()
                            .set_last_enquire_link_sequence_number(sequence_number);
                        self.as_mut().deactivate_enquire_link_timer();
                        self.as_mut().activate_enquire_link_response_timer();

                        // Poll the enquire_link_response_timer again to register the waker
                        let _ = self.as_mut().project().enquire_link_response_timer.poll(cx);
                    }
                    Poll::Pending => {}
                }
            }

            if matches!(self.state, State::Active | State::Closing) {
                let mut i: u8 = 0;

                'actions: loop {
                    i += 1;

                    if i > 5 {
                        break 'actions;
                    }

                    match self.as_mut().project().actions.poll_next(cx) {
                        Poll::Ready(Some(action)) => match action {
                            Action::Ping => {
                                // If we get here,
                                // this means that the connection is still active (did not close the actions channel) and can receive actions from the client.
                                // The client relies on the Action::Ping to be sent successfully to the connection, to determine if the connection is still active,
                                // using the `Client::is_active` method.
                            }
                            Action::PendingResponses(pending_responses) => {
                                let pending =
                                    self.as_mut().project().responses.keys().copied().collect();

                                let _ = pending_responses.ack.send(Ok(pending));
                            }
                            Action::Request(request) => {
                                tracing::trace!(target: CONN,
                                    sequence_number=request.command().sequence_number(),
                                    status=?request.command().status(),
                                    id=?request.command().id(),
                                    "Received request"
                                );

                                self.as_mut().requests_push_back(request);
                            }
                            Action::Remove(sequence_number) => {
                                tracing::trace!(target: CONN, sequence_number, "Received remove response");

                                self.as_mut().remove_response(sequence_number);
                            }
                            Action::Close(request) => {
                                tracing::trace!(target: CONN, "Received close");

                                self.as_mut().set_state(State::Closing);

                                self.as_mut().project().actions.close();

                                let _ = request.ack.send(());

                                continue 'main;
                            }
                        },
                        Poll::Ready(None) => {
                            if matches!(self.state, State::Closing) {
                                // We closed the channel to prevent more actions

                                break 'actions;
                            }

                            tracing::trace!(target: CONN, "Client dropped");

                            self.as_mut().set_state(State::Errored);

                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            break 'actions;
                        }
                    }
                }

                let mut i: u8 = 0;

                'sink: loop {
                    i += 1;

                    if i > 5 {
                        break 'sink;
                    }

                    match self.as_mut().take_pending_request() {
                        Some(request) => {
                            let sequence_number = request.command().sequence_number();
                            let status = request.command().status();
                            let id = request.command().id();

                            tracing::trace!(target: CONN, sequence_number, ?status, ?id, "Sending command");

                            match Sink::<Command>::poll_flush(self.as_mut().project().framed, cx) {
                                Poll::Ready(Ok(_)) => {
                                    tracing::debug!(target: CONN, sequence_number, ?status, ?id, "Sent command");

                                    match request {
                                        Request::Registered(request) => {
                                            tracing::debug!(target: CONN, sequence_number, ?status, ?id, "Registered");

                                            let _ = request.ack.send(Ok(()));

                                            self.as_mut()
                                                .insert_response(sequence_number, request.response);
                                        }
                                        Request::Unregistered(request) => {
                                            let _ = request.ack.send(Ok(()));
                                        }
                                    }

                                    continue 'sink;
                                }
                                Poll::Ready(Err(err)) => {
                                    tracing::error!(target: CONN, ?err);

                                    self.as_mut().set_state(State::Errored);

                                    match request.send_ack(Err(Error::from(err))) {
                                        Ok(()) => {
                                            return Poll::Ready(());
                                        }
                                        Err(Err(err)) => {
                                            // Client not waiting

                                            let _ = self.as_mut().events.send(Event::error(err));

                                            return Poll::Ready(());
                                        }
                                        Err(Ok(_)) => {
                                            unreachable!()
                                        }
                                    }
                                }
                                Poll::Pending => {
                                    self.as_mut().set_pending_request(request);

                                    break 'sink;
                                }
                            }
                        }
                        None => {
                            tracing::trace!(target: CONN, "No pending request");
                        }
                    }

                    match self.as_mut().requests_pop_front() {
                        Some(request) => {
                            match Sink::<Command>::poll_ready(self.as_mut().project().framed, cx) {
                                Poll::Ready(Ok(())) => {
                                    let sequence_number = request.command().sequence_number();
                                    let status = request.command().status();
                                    let id = request.command().id();

                                    tracing::trace!(target: CONN, sequence_number, ?status, ?id, "Writing command");

                                    if let Err(err) =
                                        self.as_mut().project().framed.start_send(request.command())
                                    {
                                        tracing::error!(target: CONN, sequence_number, ?status, ?id, ?err);

                                        self.as_mut().set_state(State::Errored);

                                        match request.send_ack(Err(Error::from(err))) {
                                            Ok(()) => {
                                                return Poll::Ready(());
                                            }
                                            Err(Err(err)) => {
                                                let _ =
                                                    self.as_mut().events.send(Event::error(err));

                                                return Poll::Ready(());
                                            }
                                            Err(Ok(_)) => {
                                                unreachable!()
                                            }
                                        }
                                    }

                                    // Start send was ok, we encoded the command now we set the request as a pending request.

                                    self.as_mut().set_pending_request(request);

                                    continue 'sink;
                                }
                                Poll::Ready(Err(err)) => {
                                    tracing::error!(target: CONN, ?err);

                                    self.as_mut().set_state(State::Errored);

                                    match request.send_ack(Err(Error::from(err))) {
                                        Ok(()) => {
                                            return Poll::Ready(());
                                        }
                                        Err(Err(err)) => {
                                            // Client not waiting

                                            let _ = self.as_mut().events.send(Event::error(err));

                                            return Poll::Ready(());
                                        }
                                        Err(Ok(_)) => {
                                            unreachable!()
                                        }
                                    }
                                }
                                Poll::Pending => {
                                    self.as_mut().requests_push_front(request);

                                    break 'sink;
                                }
                            }
                        }
                        None => {
                            tracing::trace!(target: CONN, "No requests in queue");

                            if matches!(self.state, State::Closing) {
                                tracing::debug!(target: CONN, "Closed");

                                self.as_mut().set_state(State::Errored);

                                return Poll::Ready(());
                            }

                            break 'sink;
                        }
                    }
                }
            }

            if matches!(self.state, State::Active) {
                let mut i: u8 = 0;

                'stream: loop {
                    i += 1;

                    if i > 5 {
                        break 'stream;
                    }

                    match self.as_mut().project().framed.poll_next(cx) {
                        Poll::Ready(Some(Ok(command))) => {
                            let sequence_number = command.sequence_number();
                            let status = command.status();
                            let id = command.id();

                            tracing::debug!(target: CONN, sequence_number, ?status, ?id, "Received command");

                            if let CommandId::EnquireLink = command.id() {
                                let response = Command::builder()
                                    .status(CommandStatus::EsmeRok)
                                    .sequence_number(command.sequence_number())
                                    .pdu(Pdu::EnquireLinkResp);

                                let (request, _) = UnregisteredRequest::new(response);

                                self.as_mut()
                                    .requests_push_front(Request::Unregistered(request));

                                continue 'main;
                            }

                            if let CommandId::EnquireLinkResp = command.id() {
                                if let Some(last_sequence_number) =
                                    self.last_enquire_link_sequence_number
                                {
                                    if let CommandStatus::EsmeRok = command.status() {
                                        if last_sequence_number == sequence_number {
                                            self.as_mut().unset_last_enquire_link_sequence_number();
                                            self.as_mut().deactivate_enquire_link_response_timer();
                                            self.as_mut().activate_enquire_link_timer();

                                            // Poll the enquire_link_timer again to register the waker
                                            let _ =
                                                self.as_mut().project().enquire_link_timer.poll(cx);

                                            continue 'stream;
                                        }
                                    }
                                }
                            }

                            if id.is_response() {
                                match self.as_mut().remove_response(sequence_number) {
                                    Some(response) => {
                                        tracing::trace!(target: CONN, sequence_number, ?status, ?id, "Found response");

                                        match response.send(command) {
                                            Ok(()) => {
                                                // Sent, do nothing
                                            }
                                            Err(command) => {
                                                // Client not waiting, return the command as an incoming event instead

                                                tracing::trace!(target: CONN, sequence_number, ?status, ?id, "Client not waiting");

                                                let _ = self
                                                    .as_mut()
                                                    .events
                                                    .send(Event::incoming(command));
                                            }
                                        }
                                    }
                                    None => {
                                        tracing::trace!(target: CONN, sequence_number, ?status, ?id, "No response found");

                                        // The client might have cancelled the request or it timed out.
                                        // In this case we just send the command as an incoming event.
                                        let _ = self.as_mut().events.send(Event::incoming(command));
                                    }
                                }

                                continue 'stream;
                            }

                            // Command is an operation from the server.
                            let _ = self.as_mut().events.send(Event::incoming(command));
                        }
                        Poll::Ready(Some(Err(err))) => {
                            tracing::error!(target: CONN, ?err);

                            self.as_mut().set_state(State::Errored);

                            let _ = self.as_mut().events.send(Event::error(Error::from(err)));

                            return Poll::Ready(());
                        }
                        Poll::Ready(None) => {
                            // TODO: send event TCP Connection closed by server error
                            tracing::debug!(target: CONN, "Connection closed by the server");

                            self.as_mut().set_state(State::Errored);

                            return Poll::Ready(());
                        }
                        Poll::Pending => {
                            return Poll::Pending;
                        }
                    }
                }
            }
        }
    }
}
