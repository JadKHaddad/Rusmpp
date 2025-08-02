use pyo3::{pyclass, pymethods};
use pyo3_stub_gen_derive::{gen_stub_pyclass_complex_enum, gen_stub_pymethods};

#[pyclass]
#[gen_stub_pyclass_complex_enum]
#[derive(Debug, Clone)]
pub enum Error {
    // TODO
    IO(String),
}

impl From<rusmppc::error::Error> for Error {
    fn from(error: rusmppc::error::Error) -> Self {
        Error::IO(format!("SMPP error: {error}"))
    }
}

#[pymethods]
#[gen_stub_pymethods]
impl Error {
    fn __repr__(&self) -> String {
        format!("{self:?}")
    }
}
