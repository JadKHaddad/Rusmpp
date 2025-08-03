use pyo3::{
    pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyResult, Python,
};
use pyo3_stub_gen::define_stub_info_gatherer;

use crate::generated::{BindReceiverResp, BindTransceiverResp, BindTransmitterResp, Command};

mod client;
mod error;
mod event;
mod generated;
mod io;

#[pymodule]
fn rusmppyc(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_class::<client::Client>()?;
    m.add_class::<event::Event>()?;
    m.add_class::<event::Events>()?;
    m.add_class::<error::Error>()?;

    // TODO: must bind all generated structs and enums
    m.add_class::<Command>()?;
    m.add_class::<BindTransmitterResp>()?;
    m.add_class::<BindReceiverResp>()?;
    m.add_class::<BindTransceiverResp>()?;

    use error::*;
    m.add("RusmppycException", py.get_type::<RusmppycException>())?;
    m.add("DnsResolverError", py.get_type::<DnsResolverError>())?;
    m.add("ConnectionError", py.get_type::<ConnectionError>())?;
    m.add("IOError", py.get_type::<IOError>())?;
    m.add("EncodeError", py.get_type::<EncodeError>())?;
    m.add("DecodeError", py.get_type::<DecodeError>())?;
    m.add(
        "EnquireLinkTimeoutError",
        py.get_type::<EnquireLinkTimeoutError>(),
    )?;
    m.add(
        "ResponseTimeoutError",
        py.get_type::<ResponseTimeoutError>(),
    )?;
    m.add(
        "UnexpectedResponseError",
        py.get_type::<UnexpectedResponseError>(),
    )?;
    m.add(
        "UnsupportedInterfaceVersionError",
        py.get_type::<UnsupportedInterfaceVersionError>(),
    )?;
    m.add("PduError", py.get_type::<PduError>())?;
    m.add("OtherError", py.get_type::<OtherError>())?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
