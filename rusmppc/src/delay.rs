use std::time::Duration;

pub trait Delay: Default {
    type Future: Future<Output = ()>;

    fn delay(&self, duration: Duration) -> Self::Future;
}

#[derive(Debug, Default)]
#[non_exhaustive]
pub struct TokioDelay;

impl TokioDelay {
    pub const fn new() -> Self {
        Self
    }
}

impl Delay for TokioDelay {
    type Future = tokio::time::Sleep;

    fn delay(&self, duration: Duration) -> Self::Future {
        tokio::time::sleep(duration)
    }
}
