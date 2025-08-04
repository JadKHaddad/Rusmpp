//! `SMPP` client error type.

use std::time::Duration;

use rusmpp::{
    Command,
    codec::tokio::{DecodeError, EncodeError},
    values::InterfaceVersion,
};

/// Errors that can occur during `SMPP` operations.
#[non_exhaustive]
#[derive(Debug, thiserror::Error)]
// TODO: add TCP Connection closed by server error
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
    #[error("Connection closed")] // TODO: Rename to smpp connection is closed!
    ConnectionClosed,
    /// Protocol encode error.
    #[error("Protocol encode error: {0}")]
    Encode(#[source] EncodeError),
    /// Protocol decode error.
    #[error("Protocol decode error: {0}")]
    Decode(#[source] DecodeError),
    /// The `SMPP` server did not respond to the [`EnquireLink`](rusmpp::Pdu::EnquireLink) request within the specified timeout.
    #[error("Server did not respond to enquire link: timeout: {timeout:?}")]
    EnquireLinkTimeout {
        /// The timeout duration.
        timeout: Duration,
    },
    /// The `SMPP` operation timed out.
    ///
    /// The server did not respond to the request within the specified timeout.
    // This happen when the response timer expires.
    // e.g. We send a bind request and the server doesn't respond.
    #[error("Response timed out: sequence number: {sequence_number}, timeout: {timeout:?}")]
    ResponseTimeout {
        /// The sequence number of the request that timed out.
        sequence_number: u32,
        /// The timeout duration.
        timeout: Duration,
    },
    /// The `SMPP` operation failed with an error response from the server.
    ///
    /// Error responses are responses with the status code other than [`EsmeRok`](rusmpp::CommandStatus::EsmeRok).
    #[error("Unexpected response from the server: response: {response:?}")]
    UnexpectedResponse {
        /// The response that was received from the server.
        response: Box<Command>,
    },
    /// The client used an interface version that is not supported by the library.
    #[error("Unsupported interface version: {version:?}, supported version: {supported_version:?}")]
    UnsupportedInterfaceVersion {
        /// The requested interface version.
        version: InterfaceVersion,
        /// The version that is supported by the library.
        supported_version: InterfaceVersion,
    },
}

impl Error {
    pub(crate) fn unexpected_response(response: impl Into<Box<Command>>) -> Self {
        Self::UnexpectedResponse {
            response: response.into(),
        }
    }

    pub(crate) const fn unsupported_interface_version(version: InterfaceVersion) -> Self {
        Self::UnsupportedInterfaceVersion {
            version,
            supported_version: InterfaceVersion::Smpp5_0,
        }
    }

    pub(crate) const fn response_timeout(sequence_number: u32, timeout: Duration) -> Self {
        Self::ResponseTimeout {
            sequence_number,
            timeout,
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
