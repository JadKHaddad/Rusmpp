use std::{
    pin::Pin,
    task::{Context, Poll},
};

use futures::{Sink, Stream};
use rusmpp::{Command, decode::DecodeError, tokio_codec::EncodeError};

#[mockall::automock]
trait Framed {
    fn poll_next_pin<'a>(
        self: Pin<&mut Self>,
        cx: &mut Context<'a>,
    ) -> Poll<Option<Result<Command, DecodeError>>>;

    fn poll_ready_pin<'a>(
        self: Pin<&mut Self>,
        cx: &mut Context<'a>,
    ) -> Poll<Result<(), EncodeError>>;

    fn start_send_pin(self: Pin<&mut Self>, item: Command) -> Result<(), EncodeError>;

    fn poll_flush_pin<'a>(
        self: Pin<&mut Self>,
        cx: &mut Context<'a>,
    ) -> Poll<Result<(), EncodeError>>;

    fn poll_close_pin<'a>(
        self: Pin<&mut Self>,
        cx: &mut Context<'a>,
    ) -> Poll<Result<(), EncodeError>>;
}

impl Stream for MockFramed {
    type Item = Result<Command, DecodeError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.poll_next_pin(cx)
    }
}

impl Sink<Command> for MockFramed {
    type Error = EncodeError;

    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll_ready_pin(cx)
    }

    fn start_send(self: Pin<&mut Self>, item: Command) -> Result<(), Self::Error> {
        self.start_send_pin(item)
    }

    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll_flush_pin(cx)
    }

    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.poll_close_pin(cx)
    }
}

impl MockFramed {
    pub fn poll_ready_always_ready_ok(mut self) -> MockFramed {
        self.expect_poll_ready_pin()
            .returning(|_cx| Poll::Ready(Ok(())));
        self
    }

    pub fn poll_start_send_always_ok(mut self) -> MockFramed {
        self.expect_start_send_pin().returning(|_item| Ok(()));
        self
    }

    pub fn poll_flush_always_ready_ok(mut self) -> MockFramed {
        self.expect_poll_flush_pin()
            .returning(|_cx| Poll::Ready(Ok(())));
        self
    }

    pub fn poll_close_always_ready_ok(mut self) -> MockFramed {
        self.expect_poll_close_pin()
            .returning(|_cx| Poll::Ready(Ok(())));
        self
    }

    pub fn sink_always_ready_ok(self) -> MockFramed {
        self.poll_ready_always_ready_ok()
            .poll_start_send_always_ok()
            .poll_flush_always_ready_ok()
            .poll_close_always_ready_ok()
    }
}

#[test]
fn test_sink_always_ready_ok() {
    let mut mock_framed = MockFramed::new().sink_always_ready_ok();

    let waker = futures::task::noop_waker();
    let mut cx = Context::from_waker(&waker);

    let mut pinned = Pin::new(&mut mock_framed);

    for _ in 0..5 {
        let result = pinned.as_mut().poll_ready(&mut cx);
        assert!(matches!(result, Poll::Ready(Ok(()))));

        let result = pinned.as_mut().start_send(Command::default());
        assert!(matches!(result, Ok(())));

        let result = pinned.as_mut().poll_flush(&mut cx);
        assert!(matches!(result, Poll::Ready(Ok(()))));

        let result = pinned.as_mut().poll_close(&mut cx);
        assert!(matches!(result, Poll::Ready(Ok(()))));
    }
}
