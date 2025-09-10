#![allow(clippy::too_many_arguments)]

// XXX: Current limitations:
// - Sending requests without waiting for a response is not supported. In rusmppc:
//   client.no_wait().submit_sm(...)
// - Sending requests with a custom timeout or without a timeout is not supported. In rusmppc:
//   client.response_timeout(...).submit_sm(...)
//   client.no_response_timeout().submit_sm(...)

use std::{str::FromStr, time::Duration};

use futures::StreamExt;
use pyo3::{pyclass, pymethods, types::PyType, Bound, PyAny, PyObject, PyResult, Python};
use pyo3_async_runtimes::tokio::future_into_py;
use rusmpp::{
    pdus::{BindReceiver, BindTransceiver, BindTransmitter, DeliverSmResp, SubmitSm},
    tlvs::MessageSubmissionRequestTlvValue,
    types::{AnyOctetString, COctetString, EmptyOrFullCOctetString, OctetString},
    values::{MessagePayload, ServiceType},
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

    #[pyo3(signature=(system_id = String::new(),
        password = String::new(),
        system_type = String::new(),
        interface_version = crate::generated::InterfaceVersion::Smpp5_0(),
        addr_ton = crate::generated::Ton::Unknown(),
        addr_npi = crate::generated::Npi::Unknown(),
        address_range = String::new(),
        status=crate::generated::CommandStatus::EsmeRok()))]
    fn bind_transmitter<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
        system_type: String,
        interface_version: crate::generated::InterfaceVersion,
        addr_ton: crate::generated::Ton,
        addr_npi: crate::generated::Npi,
        address_range: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let pdu = BindTransmitter::builder()
            .system_id(COctetString::from_str(&system_id).map_pdu_err("system_id")?)
            .password(COctetString::from_str(&password).map_pdu_err("password")?)
            .system_type(COctetString::from_str(&system_type).map_pdu_err("system_type")?)
            .interface_version(interface_version.into())
            .addr_ton(addr_ton.into())
            .addr_npi(addr_npi.into())
            .address_range(COctetString::from_str(&address_range).map_pdu_err("address_range")?)
            .build();

        tracing::debug!(?pdu, "Built Pdu");

        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .bind_transmitter(pdu)
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::BindTransmitterResp::from(response))
        })
    }

    #[pyo3(signature=(system_id = String::new(),
        password = String::new(),
        system_type = String::new(),
        interface_version = crate::generated::InterfaceVersion::Smpp5_0(),
        addr_ton = crate::generated::Ton::Unknown(),
        addr_npi = crate::generated::Npi::Unknown(),
        address_range = String::new(),
        status=crate::generated::CommandStatus::EsmeRok()))]
    fn bind_receiver<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
        system_type: String,
        interface_version: crate::generated::InterfaceVersion,
        addr_ton: crate::generated::Ton,
        addr_npi: crate::generated::Npi,
        address_range: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let pdu = BindReceiver::builder()
            .system_id(COctetString::from_str(&system_id).map_pdu_err("system_id")?)
            .password(COctetString::from_str(&password).map_pdu_err("password")?)
            .system_type(COctetString::from_str(&system_type).map_pdu_err("system_type")?)
            .interface_version(interface_version.into())
            .addr_ton(addr_ton.into())
            .addr_npi(addr_npi.into())
            .address_range(COctetString::from_str(&address_range).map_pdu_err("address_range")?)
            .build();

        tracing::debug!(?pdu, "Built Pdu");

        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .bind_receiver(pdu)
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::BindReceiverResp::from(response))
        })
    }

    #[pyo3(signature=(system_id = String::new(),
        password = String::new(),
        system_type = String::new(),
        interface_version = crate::generated::InterfaceVersion::Smpp5_0(),
        addr_ton = crate::generated::Ton::Unknown(),
        addr_npi = crate::generated::Npi::Unknown(),
        address_range = String::new(),
        status=crate::generated::CommandStatus::EsmeRok()))]
    fn bind_transceiver<'p>(
        &self,
        py: Python<'p>,
        system_id: String,
        password: String,
        system_type: String,
        interface_version: crate::generated::InterfaceVersion,
        addr_ton: crate::generated::Ton,
        addr_npi: crate::generated::Npi,
        address_range: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let pdu = BindTransceiver::builder()
            .system_id(COctetString::from_str(&system_id).map_pdu_err("system_id")?)
            .password(COctetString::from_str(&password).map_pdu_err("password")?)
            .system_type(COctetString::from_str(&system_type).map_pdu_err("system_type")?)
            .interface_version(interface_version.into())
            .addr_ton(addr_ton.into())
            .addr_npi(addr_npi.into())
            .address_range(COctetString::from_str(&address_range).map_pdu_err("address_range")?)
            .build();

        tracing::debug!(?pdu, "Built Pdu");

        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .bind_transceiver(pdu)
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::BindTransceiverResp::from(response))
        })
    }

    // XXX: `ServiceType`, `EsmClass`, `PriorityFlag`, `RegisteredDelivery` and `ReplaceIfPresentFlag`
    // are represented as u8 and then converted to the Rusmpp appropriate type: Structs that are repr(u8): (u8 values wrapped in helper structs).
    // Helper functions like `RegisteredDelivery::request_all()` are not available in the Python API.

    // XXX: Message submission request TLVs are not supported.
    #[pyo3(signature=(service_type=String::new(),
        source_addr_ton=crate::generated::Ton::Unknown(),
        source_addr_npi=crate::generated::Npi::Unknown(),
        source_addr=String::new(),
        dest_addr_ton=crate::generated::Ton::Unknown(),
        dest_addr_npi=crate::generated::Npi::Unknown(),
        destination_addr=String::new(),
        esm_class=u8::default(),
        protocol_id=u8::default(),
        priority_flag=u8::default(),
        schedule_delivery_time=String::new(),
        validity_period=String::new(),
        registered_delivery=u8::default(),
        replace_if_present_flag=u8::default(),
        data_coding=crate::generated::DataCoding::McSpecific(),
        sm_default_msg_id=u8::default(),
        short_message=Vec::new(),
        message_payload=None,
        status=crate::generated::CommandStatus::EsmeRok()))]
    fn submit_sm<'p>(
        &self,
        py: Python<'p>,
        service_type: String,
        source_addr_ton: crate::generated::Ton,
        source_addr_npi: crate::generated::Npi,
        source_addr: String,
        dest_addr_ton: crate::generated::Ton,
        dest_addr_npi: crate::generated::Npi,
        destination_addr: String,
        esm_class: u8,
        protocol_id: u8,
        priority_flag: u8,
        schedule_delivery_time: String,
        validity_period: String,
        registered_delivery: u8,
        replace_if_present_flag: u8,
        data_coding: crate::generated::DataCoding,
        sm_default_msg_id: u8,
        short_message: Vec<u8>,
        message_payload: Option<Vec<u8>>,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let builder = SubmitSm::builder()
            .service_type(ServiceType::new(
                COctetString::from_str(&service_type).map_pdu_err("service_type")?,
            ))
            .source_addr_ton(source_addr_ton.into())
            .source_addr_npi(source_addr_npi.into())
            .source_addr(COctetString::from_str(&source_addr).map_pdu_err("source_addr")?)
            .dest_addr_ton(dest_addr_ton.into())
            .dest_addr_npi(dest_addr_npi.into())
            .destination_addr(
                COctetString::from_str(&destination_addr).map_pdu_err("destination_addr")?,
            )
            .esm_class(esm_class.into())
            .protocol_id(protocol_id)
            .priority_flag(priority_flag.into())
            .schedule_delivery_time(
                EmptyOrFullCOctetString::from_str(&schedule_delivery_time)
                    .map_pdu_err("schedule_delivery_time")?,
            )
            .validity_period(
                EmptyOrFullCOctetString::from_str(&validity_period)
                    .map_pdu_err("validity_period")?,
            )
            .registered_delivery(registered_delivery.into())
            .replace_if_present_flag(replace_if_present_flag.into())
            .data_coding(data_coding.into())
            .sm_default_msg_id(sm_default_msg_id)
            .short_message(OctetString::new(short_message).map_pdu_err("short_message")?);

        let builder = match message_payload {
            Some(payload) => builder.push_tlv(MessageSubmissionRequestTlvValue::MessagePayload(
                MessagePayload::new(AnyOctetString::new(payload)),
            )),
            None => builder,
        };

        let pdu = builder.build();

        tracing::debug!(?pdu, "Built Pdu");

        let client = self.clone();

        future_into_py(py, async move {
            let response = client
                .inner
                .status(status.into())
                .submit_sm(pdu)
                .await
                .map_err(Exception::from)?;

            Ok(crate::generated::SubmitSmResp::from(response))
        })
    }

    #[pyo3(signature=(sequence_number, message_id=String::new(), status=crate::generated::CommandStatus::EsmeRok()))]
    fn deliver_sm_resp<'p>(
        &self,
        py: Python<'p>,
        sequence_number: u32,
        message_id: String,
        status: crate::generated::CommandStatus,
    ) -> PyResult<Bound<'p, PyAny>> {
        let pdu = DeliverSmResp::builder()
            .message_id(COctetString::from_str(&message_id).map_pdu_err("message_id")?)
            .build();

        tracing::debug!(?pdu, "Built Pdu");

        let client = self.clone();

        future_into_py(py, async move {
            client
                .inner
                .status(status.into())
                .deliver_sm_resp(sequence_number, pdu)
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

    fn is_closed(&self) -> PyResult<bool> {
        Ok(self.inner.is_closed())
    }

    fn is_active(&self) -> PyResult<bool> {
        Ok(self.inner.is_active())
    }
}
