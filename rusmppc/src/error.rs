//! `SMPP` client error type.

use std::time::Duration;

use rusmpp::{
    Command,
    codec::tokio::{DecodeError, EncodeError},
    session::SessionState,
};

/// Errors that can occur during `SMPP` operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// DNS resolution failed.
    #[error("DNS resolution failed: {0}")]
    Dns(#[source] std::io::Error),
    /// Connection to `SMPP` server failed.
    #[error("Failed to connect to the server: {0}")]
    Connect(#[source] std::io::Error),
    /// IO error occurred.
    #[error("Io error: {0}")]
    Io(#[source] std::io::Error),
    /// The connection to the `SMPP` server is closed.
    #[error("Connection closed")]
    ConnectionClosed,
    /// Invalid session state.
    ///
    /// This error occurs when an operation is attempted that is not valid for the current session state.    
    /// E.g. trying to send [`SubmitSm`](rusmpp::Pdu::SubmitSm) in a [`BoundRx`](rusmpp::session::SessionState::BoundRx) state.
    #[error("Invalid session state: {session_state:?}")]
    InvalidSessionState { session_state: SessionState },
    /// Protocol encode error.
    #[error("Protocol encode error: {0}")]
    Encode(#[source] EncodeError),
    /// Protocol decode error.
    #[error("Protocol decode error: {0}")]
    Decode(#[source] DecodeError),
    /// The `SMPP` server did not respond to the [`EnquireLink`](rusmpp::Pdu::EnquireLink) request within the specified timeout.
    #[error("Server did not respond to enquire link: {timeout:?}")]
    EnquireLinkTimeout { timeout: Duration },
    /// The [`EnquireLink`](rusmpp::Pdu::EnquireLink) operation failed with an invalid response from the server.
    #[error("Enquire link response invalid: {response:?}")]
    EnquireLinkFailed { response: Box<Command> },
    /// The `SMPP` operation timed out.
    ///
    /// The server did not respond to the request within the specified timeout.
    // This happen when the response timer expires.
    // e.g. We send a bind request and the server doesn't respond.
    #[error("Request timed out")]
    Timeout,
    /// The `SMPP` operation failed with an error response from the server.
    ///
    /// Error responses are responses with the status code other than [`EsmeRok`](rusmpp::CommandStatus::EsmeRok).
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
