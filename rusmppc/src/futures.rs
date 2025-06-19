use std::{
    pin::Pin,
    task::{Context, Poll},
};

use tokio::sync::mpsc::UnboundedSender;

use crate::Action;

pin_project_lite::pin_project! {
    /// The [`RequestFutureGuard`] is used to wrap a pending request future and remove its corresponding sequence number
    /// from the pending responses if the future got dropped.
    pub struct RequestFutureGuard<'a, F> {
        done: bool,
        sequence_number: u32,
        actions: &'a UnboundedSender<Action>,
        #[pin]
        fut: F,
    }

    impl<F> PinnedDrop for RequestFutureGuard<'_, F> {
        fn drop(this: Pin<&mut Self>) {
            let this = this.project();

            if !*this.done {
                let _ = this.actions
                    .send(Action::Remove(*this.sequence_number));
            }
        }
    }
}

impl<'a, F> RequestFutureGuard<'a, F> {
    pub fn new(actions: &'a UnboundedSender<Action>, sequence_number: u32, fut: F) -> Self {
        Self {
            done: false,
            sequence_number,
            actions,
            fut,
        }
    }
}

impl<'a, F: Future> Future for RequestFutureGuard<'a, F> {
    type Output = F::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        match this.fut.poll(cx) {
            Poll::Ready(result) => {
                // Mark as done to prevent removing the sequence number on drop
                *this.done = true;

                Poll::Ready(result)
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
