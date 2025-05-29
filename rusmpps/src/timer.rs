use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use pin_project_lite::pin_project;

pin_project! {
    pub struct Timer {
        #[pin]
        state: Option<tokio::time::Sleep>,
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

impl Timer {
    pub fn new() -> Self {
        Self { state: None }
    }

    pub fn activated(mut self, duration: Duration) -> Timer {
        self.state = Some(tokio::time::sleep(duration));
        self
    }

    pub fn activate(self: Pin<&mut Self>, duration: Duration) {
        let mut this = self.project();

        this.state.set(Some(tokio::time::sleep(duration)));
    }

    pub fn disable(self: Pin<&mut Self>) {
        let mut this = self.project();

        this.state.set(None);
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let state = self.as_mut().project().state;

        match state.as_pin_mut() {
            Some(sleep) => sleep.poll(cx),
            None => Poll::Pending,
        }
    }
}
