use crate::error::Error;

#[derive(Debug, thiserror::Error)]
pub enum ReconnectingError {
    #[error(transparent)]
    Connection(Error),
    /// The maximum number of retries for reconnecting to the server has been exceeded.
    #[error("The maximum number of reconnect retries exceeded: {max_retries}")]
    MaxRetriesExceeded {
        /// The maximum number of retries.
        max_retries: usize,
    },
    #[error("Error while executing on_connect callback")]
    OnConnectError(Box<dyn std::error::Error + Send + Sync + 'static>),
}

impl ReconnectingError {
    pub(crate) const fn max_retries_exceeded(max_retries: usize) -> Self {
        Self::MaxRetriesExceeded { max_retries }
    }
}

impl From<Error> for ReconnectingError {
    fn from(value: Error) -> Self {
        Self::Connection(value)
    }
}
