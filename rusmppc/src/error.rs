use std::time::Duration;

use rusmpp::{
    Command,
    codec::tokio::{DecodeError, EncodeError},
    session::SessionState,
};

#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to connect to the server: {0}")]
    Connect(#[source] std::io::Error),
    #[error("Io error: {0}")]
    Io(#[source] std::io::Error),
    #[error("Connection closed")]
    ConnectionClosed,
    #[error("Invalid session state: {session_state:?}")]
    InvalidSessionState { session_state: SessionState },
    #[error("Protocol encode error: {0}")]
    Encode(#[source] EncodeError),
    #[error("Protocol decode error: {0}")]
    Decode(#[source] DecodeError),
    #[error("Server did not respond to enquire link: {timeout:?}")]
    EnquireLinkTimeout { timeout: Duration },
    #[error("Enquire link response invalid: {response:?}")]
    EnquireLinkFailed { response: Box<Command> },
    // This happen when the response timer expires.
    // e.g. We send a bind request and the server doesn't respond.
    #[error("Request timed out")]
    Timeout,
    // This happen when we get any other status code than esmeRok.
    #[error("Unexpected response from the server: response: {response:?}")]
    UnexpectedResponse { response: Box<Command> },
}

impl Error {
    pub(crate) fn unexpected_response(response: impl Into<Box<Command>>) -> Self {
        Self::UnexpectedResponse {
            response: response.into(),
        }
    }
}

impl From<DecodeError> for Error {
    fn from(value: DecodeError) -> Self {
        match value {
            DecodeError::Io(error) => Error::Io(error),
            error => Error::Decode(error),
        }
    }
}

impl From<EncodeError> for Error {
    fn from(value: EncodeError) -> Self {
        match value {
            EncodeError::Io(error) => Error::Io(error),
            error => Error::Encode(error),
        }
    }
}
