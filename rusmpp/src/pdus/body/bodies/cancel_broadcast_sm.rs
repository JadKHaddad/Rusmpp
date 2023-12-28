use rusmpp_macros::RusmppIo;

use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
    },
    pdus::{
        tlvs::tlv::{CancelBroadcastTLV, TLV},
        types::{npi::Npi, service_type::ServiceType, ton::Ton},
    },
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct CancelBroadcastSm {
    serivce_type: ServiceType,
    message_id: COctetString<1, 65>,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<1, 21>,
    tlvs: Vec<TLV>,
}

impl CancelBroadcastSm {
    pub fn new(
        serivce_type: ServiceType,
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        tlvs: Vec<CancelBroadcastTLV>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect();

        Self {
            serivce_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            tlvs,
        }
    }

    pub fn service_type(&self) -> &ServiceType {
        &self.serivce_type
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn source_addr_ton(&self) -> Ton {
        self.source_addr_ton
    }

    pub fn source_addr_npi(&self) -> Npi {
        self.source_addr_npi
    }

    pub fn source_addr(&self) -> &COctetString<1, 21> {
        &self.source_addr
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn into_parts(
        self,
    ) -> (
        ServiceType,
        COctetString<1, 65>,
        Ton,
        Npi,
        COctetString<1, 21>,
        Vec<TLV>,
    ) {
        (
            self.serivce_type,
            self.message_id,
            self.source_addr_ton,
            self.source_addr_npi,
            self.source_addr,
            self.tlvs,
        )
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for CancelBroadcastSm {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let serivce_type = ServiceType::async_io_read(buf).await?;
        let message_id = COctetString::async_io_read(buf).await?;
        let source_addr_ton = Ton::async_io_read(buf).await?;
        let source_addr_npi = Npi::async_io_read(buf).await?;
        let source_addr = COctetString::async_io_read(buf).await?;

        let tlvs_expected_len = length
            .saturating_sub(serivce_type.length())
            .saturating_sub(message_id.length())
            .saturating_sub(source_addr_ton.length())
            .saturating_sub(source_addr_npi.length())
            .saturating_sub(source_addr.length());

        let tlvs = Vec::<TLV>::async_io_read(buf, tlvs_expected_len).await?;

        Ok(Self {
            serivce_type,
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            tlvs,
        })
    }
}
