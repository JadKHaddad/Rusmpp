use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use pyo3::{pyclass, pymethods, types::PyType, Bound, PyAny, PyObject, PyResult, Python};
use pyo3_async_runtimes::tokio::future_into_py;
use rusmpp::{
    pdus::{BindReceiver, BindTransceiver, BindTransmitter, DeliverSmResp},
    types::COctetString,
};
use rusmppc::ConnectionBuilder;

use crate::{
    error::{Error, PduErrorExt},
    event::{Event, Events},
    io::IO,
};

#[pyclass]
#[derive(Clone)]
pub struct Client {
    inner: rusmppc::Client,
}

#[pymethods]
impl Client {
    #[classmethod]
    #[pyo3(signature=(host, enquire_link_interval=5, enquire_link_response_timeout=2, response_timeout=2))]
    fn connect<'p>(
        _cls: &'p Bound<'p, PyType>,
        py: Python<'p>,
        host: String,
        enquire_link_interval: u64,
        enquire_link_response_timeout: u64,
        response_timeout: u64,
    ) -> PyResult<Bound<'p, PyAny>> {
        future_into_py(py, async move {
            let (client, events) = ConnectionBuilder::new()
                .enquire_link_interval(Duration::from_secs(enquire_link_interval))
                .enquire_link_response_timeout(Duration::from_secs(enquire_link_response_timeout))
                .response_timeout(Duration::from_secs(response_timeout))
                .connect(host)
                .await
                .map_err(Error::from)?;

            let events = Box::pin(events.map(Event::from));

            Ok((Client { inner: client }, Events::new(events)))
        })
    }

    #[classmethod]
    #[pyo3(signature=(read, write, enquire_link_interval=5, enquire_link_response_timeout=2, response_timeout=2))]
    fn connected<'p>(
        _cls: &'p Bound<'p, PyType>,
        py: Python<'p>,
        read: PyObject,
        write: PyObject,
        enquire_link_interval: u64,
        enquire_link_response_timeout: u64,
        response_timeout: u64,
    ) -> PyResult<Bound<'p, PyAny>> {
        future_into_py(py, async move {
            let read_write = (read, write).into_tokio_async_read_and_write();

            let (client, events, connection) = ConnectionBuilder::new()
                .enquire_link_interval(Duration::from_secs(enquire_link_interval))
                .enquire_link_response_timeout(Duration::from_secs(enquire_link_response_timeout))
                .response_timeout(Duration::from_secs(response_timeout))
                .no_spawn()
                .connected(read_write);

            // the read and write are python-futures, we spawn them with current locals
            let task_locals = Python::with_gil(pyo3_async_runtimes::tokio::get_current_locals)?;
            tokio::spawn(pyo3_async_runtimes::tokio::scope(task_locals, connection));

            let events = Box::pin(events.map(Event::from));

            Ok((Client { inner: client }, Events::new(events)))
        })
    }

    #[pyo3(signature=(system_id, password))]
    fn bind_transmitter<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .bind_transmitter(
                    BindTransmitter::builder()
                        .system_id(COctetString::from_str(&system_id).map_pdu_err("system_id")?)
                        .password(COctetString::from_str(&password).map_pdu_err("password")?)
                        .build(),
                )
                .await
                .map_err(Error::from)?;

            Ok(crate::generated::BindTransmitterResp::from(response))
        })
    }

    #[pyo3(signature=(system_id, password))]
    fn bind_receiver<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .bind_receiver(
                    BindReceiver::builder()
                        .system_id(COctetString::from_str(&system_id).map_pdu_err("system_id")?)
                        .password(COctetString::from_str(&password).map_pdu_err("password")?)
                        .build(),
                )
                .await
                .map_err(Error::from)?;

            Ok(crate::generated::BindReceiverResp::from(response))
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
                        .system_id(COctetString::from_str(&system_id).map_pdu_err("system_id")?)
                        .password(COctetString::from_str(&password).map_pdu_err("password")?)
                        .build(),
                )
                .await
                .map_err(Error::from)?;

            Ok(crate::generated::BindTransceiverResp::from(response))
        })
    }

    #[pyo3(signature=(sequence_number, message_id))]
    fn deliver_sm_resp<'p>(
        &self,
        py: Python<'p>,
        sequence_number: u32,
        message_id: String,
        // TODO: we add here the status, and custom timeouts
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .deliver_sm_resp(
                    sequence_number,
                    DeliverSmResp::builder()
                        .message_id(COctetString::from_str(&message_id).map_pdu_err("message_id")?)
                        .build(),
                )
                .await
                .map_err(Error::from)?;

            Ok(())
        })
    }

    fn unbind<'p>(&self, py: Python<'p>) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client.inner.unbind().await.map_err(Error::from)?;

            Ok(())
        })
    }

    fn unbind_resp<'p>(&self, py: Python<'p>, sequence_number: u32) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .unbind_resp(sequence_number)
                .await
                .map_err(Error::from)?;

            Ok(())
        })
    }

    fn generic_nack<'p>(&self, py: Python<'p>, sequence_number: u32) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .generic_nack(sequence_number)
                .await
                .map_err(Error::from)?;

            Ok(())
        })
    }

    fn close<'p>(&self, py: Python<'p>) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client.inner.close().await.map_err(Error::from)?;

            Ok(())
        })
    }

    fn closed<'p>(&self, py: Python<'p>) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client.inner.closed().await;

            Ok(())
        })
    }
}
