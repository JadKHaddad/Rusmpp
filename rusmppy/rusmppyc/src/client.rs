#![allow(clippy::too_many_arguments)]

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
    event::{Event, Events},
    exception::{Exception, PduExceptionExt},
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
    #[pyo3(signature=(host, 
        enquire_link_interval=5000, 
        enquire_link_response_timeout=2000, 
        response_timeout=2000, 
        max_command_length=4096, 
        disable_interface_version_check=false))]
    fn connect<'p>(
        _cls: &'p Bound<'p, PyType>,
        py: Python<'p>,
        host: String,
        enquire_link_interval: u64,
        enquire_link_response_timeout: u64,
        response_timeout: Option<u64>,
        max_command_length: usize,
        disable_interface_version_check: bool,
    ) -> PyResult<Bound<'p, PyAny>> {
        future_into_py(py, async move {
            let mut builder = ConnectionBuilder::new()
                .max_command_length(max_command_length)
                .enquire_link_interval(Duration::from_millis(enquire_link_interval))
                .enquire_link_response_timeout(Duration::from_millis(
                    enquire_link_response_timeout,
                ));

            builder = match response_timeout {
                Some(timeout) => builder.response_timeout(Duration::from_millis(timeout)),
                None => builder.no_response_timeout(),
            };

            builder = match disable_interface_version_check {
                true => builder.disable_interface_version_check(),
                false => builder,
            };

            let (client, events) = builder.connect(host).await.map_err(Exception::from)?;

            let events = Box::pin(events.map(Event::from));

            Ok((Client { inner: client }, Events::new(events)))
        })
    }

    #[classmethod]
    #[pyo3(signature=(read, 
        write, 
        enquire_link_interval=5000, 
        enquire_link_response_timeout=2000, 
        response_timeout=2000, 
        max_command_length=4096,
        disable_interface_version_check=false))]
    fn connected<'p>(
        _cls: &'p Bound<'p, PyType>,
        py: Python<'p>,
        read: PyObject,
        write: PyObject,
        enquire_link_interval: u64,
        enquire_link_response_timeout: u64,
        response_timeout: Option<u64>,
        max_command_length: usize,
        disable_interface_version_check: bool,
    ) -> PyResult<Bound<'p, PyAny>> {
        future_into_py(py, async move {
            let read_write = (read, write).into_tokio_async_read_and_write();

            let mut builder = ConnectionBuilder::new()
                .max_command_length(max_command_length)
                .enquire_link_interval(Duration::from_millis(enquire_link_interval))
                .enquire_link_response_timeout(Duration::from_millis(
                    enquire_link_response_timeout,
                ));

            builder = match response_timeout {
                Some(timeout) => builder.response_timeout(Duration::from_millis(timeout)),
                None => builder.no_response_timeout(),
            };

            builder = match disable_interface_version_check {
                true => builder.disable_interface_version_check(),
                false => builder,
            };


            let (client, events, connection) = builder.no_spawn().connected(read_write);

            // the read and write are python-futures, we spawn them with current locals
            let task_locals = Python::with_gil(pyo3_async_runtimes::tokio::get_current_locals)?;
            tokio::spawn(pyo3_async_runtimes::tokio::scope(task_locals, connection));

            let events = Box::pin(events.map(Event::from));

            Ok((Client { inner: client }, Events::new(events)))
        })
    }

    #[pyo3(signature=(system_id, password, status=crate::generated::CommandStatus::EsmeRok()))]
    fn bind_transmitter<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .bind_transmitter(
                    BindTransmitter::builder()
                        .system_id(COctetString::from_str(&system_id).map_pdu_err("system_id")?)
                        .password(COctetString::from_str(&password).map_pdu_err("password")?)
                        .build(),
                )
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::BindTransmitterResp::from(response))
        })
    }

    #[pyo3(signature=(system_id, password, status=crate::generated::CommandStatus::EsmeRok()))]
    fn bind_receiver<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .bind_receiver(
                    BindReceiver::builder()
                        .system_id(COctetString::from_str(&system_id).map_pdu_err("system_id")?)
                        .password(COctetString::from_str(&password).map_pdu_err("password")?)
                        .build(),
                )
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::BindReceiverResp::from(response))
        })
    }

    #[pyo3(signature=(system_id, password, status=crate::generated::CommandStatus::EsmeRok()))]
    fn bind_transceiver<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .bind_transceiver(
                    BindTransceiver::builder()
                        .system_id(COctetString::from_str(&system_id).map_pdu_err("system_id")?)
                        .password(COctetString::from_str(&password).map_pdu_err("password")?)
                        .build(),
                )
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::BindTransceiverResp::from(response))
        })
    }

    #[pyo3(signature=(sequence_number, message_id=String::new(), status=crate::generated::CommandStatus::EsmeRok()))]
    fn deliver_sm_resp<'p>(
        &self,
        py: Python<'p>,
        sequence_number: u32,
        message_id: String,
        // TODO: we add here the status, and custom timeouts
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .status(status.into())
                .deliver_sm_resp(
                    sequence_number,
                    DeliverSmResp::builder()
                        .message_id(COctetString::from_str(&message_id).map_pdu_err("message_id")?)
                        .build(),
                )
                .await
                .map_err(Exception::from)?;

            Ok(())
        })
    }

    #[pyo3(signature=(status=crate::generated::CommandStatus::EsmeRok()))]
    fn unbind<'p>(
        &self,
        py: Python<'p>,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .status(status.into())
                .unbind()
                .await
                .map_err(Exception::from)?;

            Ok(())
        })
    }

    #[pyo3(signature=(sequence_number, status=crate::generated::CommandStatus::EsmeRok()))]
    fn unbind_resp<'p>(
        &self,
        py: Python<'p>,
        sequence_number: u32,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .status(status.into())
                .unbind_resp(sequence_number)
                .await
                .map_err(Exception::from)?;

            Ok(())
        })
    }

    #[pyo3(signature=(sequence_number, status=crate::generated::CommandStatus::EsmeRok()))]
    fn generic_nack<'p>(
        &self,
        py: Python<'p>,
        sequence_number: u32,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .status(status.into())
                .generic_nack(sequence_number)
                .await
                .map_err(Exception::from)?;

            Ok(())
        })
    }

    fn close<'p>(&self, py: Python<'p>) -> PyResult<Bound<'p, PyAny>> {
        let client = self.clone();

        future_into_py(py, async move {
            client.inner.close().await.map_err(Exception::from)?;

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
