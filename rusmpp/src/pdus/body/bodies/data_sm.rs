use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::{
        tlvs::tlv::{MessageSubmissionRequestTLV, TLV},
        types::{
            data_coding::DataCoding, esm_class::EsmClass, npi::Npi,
            registered_delivery::RegisteredDelivery, service_type::ServiceType, ton::Ton,
        },
    },
    types::c_octet_string::COctetString,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DataSm {
    serivce_type: ServiceType,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<1, 21>,
    dest_addr_ton: Ton,
    dest_addr_npi: Npi,
    destination_addr: COctetString<1, 21>,
    esm_class: EsmClass,
    registered_delivery: RegisteredDelivery,
    data_coding: DataCoding,
    tlvs: Vec<TLV>,
}

impl DataSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        serivce_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
        esm_class: EsmClass,
        registered_delivery: RegisteredDelivery,
        data_coding: DataCoding,
        tlvs: Vec<MessageSubmissionRequestTLV>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        Self {
            serivce_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            esm_class,
            registered_delivery,
            data_coding,
            tlvs,
        }
    }

    pub fn service_type(&self) -> &ServiceType {
        &self.serivce_type
    }

    pub fn source_addr_ton(&self) -> &Ton {
        &self.source_addr_ton
    }

    pub fn source_addr_npi(&self) -> &Npi {
        &self.source_addr_npi
    }

    pub fn source_addr(&self) -> &COctetString<1, 21> {
        &self.source_addr
    }

    pub fn dest_addr_ton(&self) -> &Ton {
        &self.dest_addr_ton
    }

    pub fn dest_addr_npi(&self) -> &Npi {
        &self.dest_addr_npi
    }

    pub fn destination_addr(&self) -> &COctetString<1, 21> {
        &self.destination_addr
    }

    pub fn esm_class(&self) -> &EsmClass {
        &self.esm_class
    }

    pub fn registered_delivery(&self) -> &RegisteredDelivery {
        &self.registered_delivery
    }

    pub fn data_coding(&self) -> &DataCoding {
        &self.data_coding
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    #[allow(clippy::type_complexity)]
    pub fn into_parts(
        self,
    ) -> (
        ServiceType,
        Ton,
        Npi,
        COctetString<1, 21>,
        Ton,
        Npi,
        COctetString<1, 21>,
        EsmClass,
        RegisteredDelivery,
        DataCoding,
        Vec<TLV>,
    ) {
        (
            self.serivce_type,
            self.source_addr_ton,
            self.source_addr_npi,
            self.source_addr,
            self.dest_addr_ton,
            self.dest_addr_npi,
            self.destination_addr,
            self.esm_class,
            self.registered_delivery,
            self.data_coding,
            self.tlvs,
        )
    }
}

impl IoLength for DataSm {
    fn length(&self) -> usize {
        self.serivce_type.length()
            + self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
            + self.dest_addr_ton.length()
            + self.dest_addr_npi.length()
            + self.destination_addr.length()
            + self.esm_class.length()
            + self.registered_delivery.length()
            + self.data_coding.length()
            + self.tlvs.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for DataSm {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.serivce_type.async_io_write(buf).await?;
        self.source_addr_ton.async_io_write(buf).await?;
        self.source_addr_npi.async_io_write(buf).await?;
        self.source_addr.async_io_write(buf).await?;
        self.dest_addr_ton.async_io_write(buf).await?;
        self.dest_addr_npi.async_io_write(buf).await?;
        self.destination_addr.async_io_write(buf).await?;
        self.esm_class.async_io_write(buf).await?;
        self.registered_delivery.async_io_write(buf).await?;
        self.data_coding.async_io_write(buf).await?;
        self.tlvs.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for DataSm {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let serivce_type = ServiceType::async_io_read(buf).await?;
        let source_addr_ton = Ton::async_io_read(buf).await?;
        let source_addr_npi = Npi::async_io_read(buf).await?;
        let source_addr = COctetString::<1, 21>::async_io_read(buf).await?;
        let dest_addr_ton = Ton::async_io_read(buf).await?;
        let dest_addr_npi = Npi::async_io_read(buf).await?;
        let destination_addr = COctetString::<1, 21>::async_io_read(buf).await?;
        let esm_class = EsmClass::async_io_read(buf).await?;
        let registered_delivery = RegisteredDelivery::async_io_read(buf).await?;
        let data_coding = DataCoding::async_io_read(buf).await?;

        let tlvs_expected_length = length
            .saturating_sub(serivce_type.length())
            .saturating_sub(source_addr_ton.length())
            .saturating_sub(source_addr_npi.length())
            .saturating_sub(source_addr.length())
            .saturating_sub(dest_addr_ton.length())
            .saturating_sub(dest_addr_npi.length())
            .saturating_sub(destination_addr.length())
            .saturating_sub(esm_class.length())
            .saturating_sub(registered_delivery.length())
            .saturating_sub(data_coding.length());

        let tlvs = Vec::<TLV>::async_io_read(buf, tlvs_expected_length).await?;

        Ok(Self {
            serivce_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            esm_class,
            registered_delivery,
            data_coding,
            tlvs,
        })
    }
}