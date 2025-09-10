use pyo3::{
    pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyResult, Python,
};
use pyo3_stub_gen::define_stub_info_gatherer;

use crate::generated::{add_classes, Command, CommandId, CommandStatus};

mod client;
mod error;
mod event;
mod exception;
mod generated;
mod generated_impl;
mod io;

#[pymodule]
fn rusmppyc(py: Python, m: &Bound<'_, PyModule>) -> PyResult<()> {
    pyo3_log::init();

    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    m.add_class::<client::Client>()?;
    m.add_class::<event::Event>()?;
    m.add_class::<event::Events>()?;
    m.add_class::<error::Error>()?;

    m.add_class::<Command>()?;
    m.add_class::<CommandId>()?;
    m.add_class::<CommandStatus>()?;
    // add all auto generated structs and enums
    add_classes(m)?;

    use exception::*;
    m.add("RusmppycException", py.get_type::<RusmppycException>())?;
    m.add("DnsException", py.get_type::<DnsException>())?;
    m.add("ConnectException", py.get_type::<ConnectException>())?;
    m.add(
        "ConnectionClosedException",
        py.get_type::<ConnectionClosedException>(),
    )?;
    m.add("IoException", py.get_type::<IoException>())?;
    m.add(
        "ResponseTimeoutException",
        py.get_type::<ResponseTimeoutException>(),
    )?;
    m.add(
        "UnexpectedResponseException",
        py.get_type::<UnexpectedResponseException>(),
    )?;
    m.add(
        "UnsupportedInterfaceVersionException",
        py.get_type::<UnsupportedInterfaceVersionException>(),
    )?;
    m.add("PduException", py.get_type::<PduException>())?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
