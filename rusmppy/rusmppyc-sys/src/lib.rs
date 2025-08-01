use pyo3::{
    pymodule,
    types::{PyModule, PyModuleMethods},
    Bound, PyResult,
};

mod generated;

/// Raw Python bindings for Rusmppc.
#[pymodule]
fn rusmppyc_sys(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add("__version__", env!("CARGO_PKG_VERSION"))?;

    Ok(())
}
