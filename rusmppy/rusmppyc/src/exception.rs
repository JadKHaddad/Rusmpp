use humantime::format_duration;
use pyo3::{create_exception, exceptions::PyException, PyErr};

create_exception!(
    exceptions,
    RusmppycException,
    PyException,
    "Base exception for Rusmppyc errors"
);

create_exception!(
    exceptions,
    DnsException,
    RusmppycException,
    "DNS resolution failed"
);

create_exception!(
    exceptions,
    ConnectException,
    RusmppycException,
    "Connection to `SMPP` server failed"
);

create_exception!(
    exceptions,
    ConnectionClosedException,
    RusmppycException,
    "The connection to the `SMPP` server is closed"
);

create_exception!(
    exceptions,
    IoException,
    RusmppycException,
    "IO error occurred"
);

create_exception!(
    exceptions,
    ResponseTimeoutException,
    RusmppycException,
    "The `SMPP` operation timed out"
);

create_exception!(
    exceptions,
    UnexpectedResponseException,
    RusmppycException,
    "The `SMPP` operation failed with an error response from the server"
);

create_exception!(
    exceptions,
    UnsupportedInterfaceVersionException,
    RusmppycException,
    "The client used an interface version that is not supported by the library"
);

create_exception!(
    exceptions,
    PduException,
    RusmppycException,
    "The user created a invalid `SMPP` PDU."
);

/// Errors that can occur while calling Rusmppyc functions.
///
/// These errors are not send through the events stream, but are raised directly when calling the functions.
#[derive(Debug, Clone)]
pub enum Exception {
    /// DNS resolution failed.
    Dns(String),
    /// Connection to `SMPP` server failed.
    Connect(String),
    /// IO error occurred.
    Io(String),
    /// The connection to the `SMPP` server is closed.
    ConnectionClosed(),
    /// The `SMPP` operation timed out.
    ///
    /// The server did not respond to the request within the specified timeout.
    // This happen when the response timer expires.
    // e.g. We send a bind request and the server doesn't respond.
    ResponseTimeout {
        sequence_number: u32,
        timeout: String,
    },
    /// The `SMPP` operation failed with an error response from the server.
    ///
    /// Error responses are responses with the status code other than EsmeRok.
    UnexpectedResponse { response: String },
    /// The client used an interface version that is not supported by the library.
    UnsupportedInterfaceVersion {
        version: crate::generated::InterfaceVersion,
        supported_version: crate::generated::InterfaceVersion,
    },
    /// The user created a invalid `SMPP` PDU.
    Pdu { field: String, error: String },
    /// Other error type.
    ///
    /// Rusmppc error type is non-exhaustive and contains all errors returned by the library including the ones returned by the events stream.
    /// This error should not be returned by this library and if so it should be considered a bug.
    Other(String),
}

impl From<rusmppc::error::Error> for Exception {
    fn from(error: rusmppc::error::Error) -> Self {
        match error {
            rusmppc::error::Error::Dns(error) => Exception::Dns(error.to_string()),
            rusmppc::error::Error::Connect(error) => Exception::Connect(error.to_string()),
            rusmppc::error::Error::Io(error) => Exception::Io(error.to_string()),
            rusmppc::error::Error::ConnectionClosed => Exception::ConnectionClosed(),
            rusmppc::error::Error::ResponseTimeout {
                sequence_number,
                timeout,
            } => Exception::ResponseTimeout {
                sequence_number,
                timeout: format_duration(timeout).to_string(),
            },
            rusmppc::error::Error::UnexpectedResponse { response } => {
                Exception::UnexpectedResponse {
                    response: format!("{response:?}"),
                }
            }
            rusmppc::error::Error::UnsupportedInterfaceVersion {
                version,
                supported_version,
            } => Exception::UnsupportedInterfaceVersion {
                version: version.into(),
                supported_version: supported_version.into(),
            },
            _ => Exception::Other(error.to_string()),
        }
    }
}

impl From<Exception> for PyErr {
    fn from(error: Exception) -> Self {
        match error {
            Exception::Dns(error) => DnsException::new_err(error),
            Exception::Connect(error) => ConnectException::new_err(error),
            Exception::Io(error) => IoException::new_err(error),
            Exception::ConnectionClosed() => {
                ConnectionClosedException::new_err("Connection closed")
            }
            Exception::ResponseTimeout {
                sequence_number,
                timeout,
            } => ResponseTimeoutException::new_err(format!(
                "Response timeout. Sequence number: {sequence_number}, Timeout: {timeout}",
            )),
            Exception::UnexpectedResponse { response } => {
                UnexpectedResponseException::new_err(response)
            }
            Exception::UnsupportedInterfaceVersion {
                version,
                supported_version,
            } => UnsupportedInterfaceVersionException::new_err(format!(
                "Unsupported interface version. Version: {version:?}, Supported version: {supported_version:?}",
            )),
            Exception::Pdu { field, error } => {
                PduException::new_err(format!("Invalid PDU. Field: {field}, Error: {error}"))
            }
            Exception::Other(error) => RusmppycException::new_err(error),
        }
    }
}

pub trait PduExceptionExt<T> {
    fn map_pdu_err(self, field: &'static str) -> Result<T, Exception>;
}

impl<T, E: std::error::Error> PduExceptionExt<T> for Result<T, E> {
    fn map_pdu_err(self, field: &'static str) -> Result<T, Exception> {
        self.map_err(|error| Exception::Pdu {
            field: field.to_string(),
            error: error.to_string(),
        })
    }
}
