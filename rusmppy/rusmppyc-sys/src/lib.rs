use pyo3::{
    pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyResult,
};
use pyo3_stub_gen::define_stub_info_gatherer;

mod client;
mod event;
mod generated;

/// Raw Python bindings for Rusmppc.
#[pymodule]
fn rusmppyc_sys(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;
    m.add_class::<client::Client>()?;
    m.add_class::<event::Event>()?;
    m.add_class::<event::Events>()?;

    Ok(())
}

define_stub_info_gatherer!(stub_info);
