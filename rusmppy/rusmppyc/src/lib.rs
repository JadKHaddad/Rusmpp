use pyo3::{
    pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyResult,
};
use pyo3_stub_gen::define_stub_info_gatherer;

use crate::generated::BindTransceiverResp;

mod client;
mod event;
mod generated;
mod io;

#[pymodule]
fn rusmppyc(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<client::Client>()?;
    m.add_class::<event::Event>()?;
    m.add_class::<event::Events>()?;
    m.add_class::<BindTransceiverResp>()?; // TODO: must bind all generated structs and enums

    Ok(())
}

define_stub_info_gatherer!(stub_info);
