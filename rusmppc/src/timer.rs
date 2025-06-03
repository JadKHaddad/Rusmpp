use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};

use pin_project_lite::pin_project;

pin_project! {
    #[derive(Debug)]
    pub struct Timer {
        #[pin]
        state: Option<tokio::time::Sleep>,
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::inactive()
    }
}

impl Timer {
    pub const fn inactive() -> Self {
        Self { state: None }
    }

    pub fn active(duration: Duration) -> Self {
        Self::inactive().activated(duration)
    }

    pub fn activated(mut self, duration: Duration) -> Timer {
        self.state = Some(tokio::time::sleep(duration));
        self
    }

    pub fn deactivate(self: Pin<&mut Self>) {
        self.project().state.set(None);
    }

    pub fn activate(self: Pin<&mut Self>, duration: Duration) {
        self.project().state.set(Some(tokio::time::sleep(duration)));
    }
}

impl Future for Timer {
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        self.project()
            .state
            .as_pin_mut()
            .map(|sleep| sleep.poll(cx))
            .unwrap_or(Poll::Pending)
    }
}
