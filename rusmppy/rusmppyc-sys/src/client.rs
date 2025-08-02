use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use pyo3::{
    exceptions::{PyIOError, PyValueError},
    pyclass, pymethods,
    types::PyType,
    Bound, PyAny, PyErr, PyResult, Python,
};
use pyo3_async_runtimes::tokio::future_into_py;
use pyo3_stub_gen_derive::{gen_stub_pyclass, gen_stub_pymethods};
use rusmpp::{pdus::BindTransceiver, types::COctetString};
use rusmppc::ConnectionBuilder;

use crate::event::{Event, Events};

#[pyclass]
#[gen_stub_pyclass]
#[derive(Clone)]
pub struct Client {
    inner: rusmppc::Client,
}

#[pymethods]
#[gen_stub_pymethods]
impl Client {
    #[classmethod]
    #[pyo3(signature=(host, enquire_link_interval=5, response_timeout=2))]
    fn connect<'p>(
        cls: &'p Bound<'p, PyType>,
        py: Python<'p>,
        host: String,
        enquire_link_interval: u64,
        response_timeout: u64,
    ) -> PyResult<Bound<'p, PyAny>> {
        future_into_py(py, async move {
            let (client, events) = ConnectionBuilder::new()
                .enquire_link_interval(Duration::from_secs(enquire_link_interval))
                .response_timeout(Duration::from_secs(response_timeout))
                .connect(host)
                .await
                .map_err(|err| PyErr::new::<PyIOError, _>(format!("Connection failed: {err}")))?;

            let events = Box::pin(events.map(move |event| Event::from(event)));

            Ok((Client { inner: client }, Events::new(events)))
        })
    }

    #[pyo3(signature=(system_id, password))]
    fn bind_transceiver<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();
        future_into_py(py, async move {
            let response = client
                .inner
                .bind_transceiver(
                    BindTransceiver::builder()
                        .system_id(COctetString::from_str(&system_id).map_err(|err| {
                            PyErr::new::<PyValueError, _>(format!("Invalid system_id: {err}"))
                        })?)
                        .password(COctetString::from_str(&password).map_err(|err| {
                            PyErr::new::<PyValueError, _>(format!("Invalid password: {err}"))
                        })?)
                        .build(),
                )
                .await
                .map_err(|err| PyErr::new::<PyIOError, _>(format!("Bind failed: {err}")))?;

            Ok(crate::generated::BindTransceiverResp::from(response))
        })
    }
}
