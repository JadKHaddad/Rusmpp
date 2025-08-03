use humantime::format_duration;
use pyo3::{
    create_exception,
    exceptions::{PyException, PyRuntimeError},
    pyclass, pymethods, PyErr,
};
use pyo3_stub_gen_derive::{gen_stub_pyclass_complex_enum, gen_stub_pymethods};

create_exception!(exceptions, RusmppycException, PyException);
create_exception!(exceptions, DnsResolverError, RusmppycException);
create_exception!(exceptions, ConnectionError, RusmppycException);
create_exception!(exceptions, IOError, RusmppycException);
create_exception!(exceptions, EncodeError, RusmppycException);
create_exception!(exceptions, DecodeError, RusmppycException);
create_exception!(exceptions, EnquireLinkTimeoutError, RusmppycException);
create_exception!(exceptions, ResponseTimeoutError, RusmppycException);
create_exception!(exceptions, UnexpectedResponseError, RusmppycException);
create_exception!(
    exceptions,
    UnsupportedInterfaceVersionError,
    RusmppycException
);
create_exception!(exceptions, PduError, RusmppycException);
create_exception!(exceptions, OtherError, RusmppycException);

#[pyclass]
#[gen_stub_pyclass_complex_enum]
#[derive(Debug, Clone)]
pub enum Error {
    Dns(String),
    Connect(String),
    Io(String),
    ConnectionClosed(),
    Encode(String),
    Decode(String),
    EnquireLinkTimeout {
        timeout: String,
    },
    ResponseTimeout {
        sequence_number: u32,
        timeout: String,
    },
    UnexpectedResponse {
        response: String,
    },
    UnsupportedInterfaceVersion {
        version: crate::generated::InterfaceVersion,
        supported_version: crate::generated::InterfaceVersion,
    },
    /// The user created a invalid `SMPP` PDU.
    Pdu {
        field: String,
        error: String,
    },
    Python(String),
    /// Other error type.
    ///
    /// [`rusmppc::error::Error`] is not exhaustive.
    Other(String),
}

impl From<rusmppc::error::Error> for Error {
    fn from(error: rusmppc::error::Error) -> Self {
        match error {
            rusmppc::error::Error::Dns(error) => Error::Dns(error.to_string()),
            rusmppc::error::Error::Connect(error) => Error::Connect(error.to_string()),
            rusmppc::error::Error::Io(error) => Error::Io(error.to_string()),
            rusmppc::error::Error::ConnectionClosed => Error::ConnectionClosed(),
            rusmppc::error::Error::Encode(error) => Error::Encode(error.to_string()),
            rusmppc::error::Error::Decode(error) => Error::Decode(error.to_string()),
            rusmppc::error::Error::EnquireLinkTimeout { timeout } => Error::EnquireLinkTimeout {
                timeout: format_duration(timeout).to_string(),
            },
            rusmppc::error::Error::ResponseTimeout {
                sequence_number,
                timeout,
            } => Error::ResponseTimeout {
                sequence_number,
                timeout: format_duration(timeout).to_string(),
            },
            rusmppc::error::Error::UnexpectedResponse { response } => Error::UnexpectedResponse {
                response: format!("{response:?}"),
            },
            rusmppc::error::Error::UnsupportedInterfaceVersion {
                version,
                supported_version,
            } => Error::UnsupportedInterfaceVersion {
                version: version.into(),
                supported_version: supported_version.into(),
            },
            _ => Error::Other(error.to_string()),
        }
    }
}

impl From<Error> for PyErr {
    fn from(error: Error) -> Self {
        match error {
            Error::Dns(error) => DnsResolverError::new_err(error),
            Error::Connect(error) => ConnectionError::new_err(error),
            Error::Io(error) => IOError::new_err(error),
            Error::ConnectionClosed() => ConnectionError::new_err("Connection closed"),
            Error::Encode(error) => EncodeError::new_err(error),
            Error::Decode(error) => DecodeError::new_err(error),
            Error::EnquireLinkTimeout { timeout } => EnquireLinkTimeoutError::new_err(timeout),
            Error::ResponseTimeout {
                sequence_number,
                timeout,
            } => ResponseTimeoutError::new_err(format!(
                "Sequence number: {sequence_number}, Timeout: {timeout}"
            )),
            Error::UnexpectedResponse { response } => UnexpectedResponseError::new_err(response),
            Error::UnsupportedInterfaceVersion {
                version,
                supported_version,
            } => UnsupportedInterfaceVersionError::new_err(format!(
                "Version: {version:?}, Supported version: {supported_version:?}",
            )),
            Error::Pdu { field, error } => {
                PduError::new_err(format!("Field: {field}, Error: {error}"))
            }
            Error::Other(error) => OtherError::new_err(error),
            Error::Python(error) => PyRuntimeError::new_err(error),
        }
    }
}

#[pymethods]
#[gen_stub_pymethods]
impl Error {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}

pub trait PduErrorExt<T> {
    fn map_pdu_err(self, field: &'static str) -> Result<T, Error>;
}

impl<T, E: std::error::Error> PduErrorExt<T> for Result<T, E> {
    fn map_pdu_err(self, field: &'static str) -> Result<T, Error> {
        self.map_err(|error| Error::Pdu {
            field: field.to_string(),
            error: error.to_string(),
        })
    }
}
