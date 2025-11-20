use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

pub mod framed {
    use futures::{Sink, Stream};
    use rusmpp::{Command, tokio_codec::DecodeError, tokio_codec::EncodeError};

    use super::*;

    #[mockall::automock]
    pub trait Framed {
        fn poll_next_pin<'a>(
            self: Pin<&mut Self>,
            cx: &mut Context<'a>,
        ) -> Poll<Option<Result<Command, DecodeError>>>;

        fn poll_ready_pin<'a>(
            self: Pin<&mut Self>,
            cx: &mut Context<'a>,
        ) -> Poll<Result<(), EncodeError>>;

        fn start_send_pin(self: Pin<&mut Self>, item: &Command) -> Result<(), EncodeError>;

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

    impl Sink<&Command> for MockFramed {
        type Error = EncodeError;

        fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
            self.poll_ready_pin(cx)
        }

        fn start_send(self: Pin<&mut Self>, item: &Command) -> Result<(), Self::Error> {
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

            let result = pinned.as_mut().start_send(&Command::default());
            assert!(matches!(result, Ok(())));

            let result = pinned.as_mut().poll_flush(&mut cx);
            assert!(matches!(result, Poll::Ready(Ok(()))));

            let result = pinned.as_mut().poll_close(&mut cx);
            assert!(matches!(result, Poll::Ready(Ok(()))));
        }
    }
}

pub mod delay {
    use super::*;

    #[mockall::automock]
    pub trait Delay {
        fn delay_(&self, duration: Duration) -> MockDelayFuture;
    }

    impl crate::delay::Delay for MockDelay {
        type Future = MockDelayFuture;

        fn delay(&self, duration: Duration) -> Self::Future {
            <Self as Delay>::delay_(self, duration)
        }
    }

    impl MockDelay {
        /// Each second in the duration will correspond to one poll before completion.
        pub fn delay_after_seconds(mut self) -> MockDelay {
            self.expect_delay_()
                .returning(move |duration| MockDelayFuture::new(duration.as_secs()));
            self
        }
    }

    pub struct MockDelayFuture {
        complete: bool,
        /// Number of polls before completion.
        after: u64,
    }

    impl MockDelayFuture {
        pub const fn new(after: u64) -> Self {
            Self {
                complete: false,
                after,
            }
        }
    }

    impl Future for MockDelayFuture {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
            if self.complete {
                panic!("polled after completion");
            }

            if self.after == 0 {
                self.complete = true;

                Poll::Ready(())
            } else {
                self.after -= 1;

                cx.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    #[test]
    fn test_delay_always_after() {
        use crate::delay::Delay;

        let mock_delay = MockDelay::new().delay_after_seconds();

        let mut delay_future = mock_delay.delay(Duration::from_secs(3));

        let waker = futures::task::noop_waker();
        let mut cx = Context::from_waker(&waker);
        let mut pinned = Pin::new(&mut delay_future);

        for i in 0..5 {
            let result = pinned.as_mut().poll(&mut cx);

            if i < 3 {
                assert!(matches!(result, Poll::Pending));
            } else {
                assert!(matches!(result, Poll::Ready(())));
                break;
            }
        }
    }
}
