use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::types::{
        data_coding::DataCoding, esm_class::EsmClass, npi::Npi, priority_flag::PriorityFlag,
        registered_delivery::RegisteredDelivery, replace_if_present_flag::ReplaceIfPresentFlag,
        service_type::ServiceType, ton::Ton,
    },
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        greater_than_u8::GreaterThanU8, octet_string::OctetString,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubmitSm {
    serivce_type: ServiceType,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<21>,
    dest_addr_ton: Ton,
    dest_addr_npi: Npi,
    destination_addr: COctetString<21>,
    esm_class: EsmClass,
    protocol_id: u8,
    priority_flag: PriorityFlag,
    schedule_delivery_time: EmptyOrFullCOctetString<17>,
    validity_period: EmptyOrFullCOctetString<17>,
    registered_delivery: RegisteredDelivery,
    replace_if_present_flag: ReplaceIfPresentFlag,
    data_coding: DataCoding,
    /// The sm_default_msg_id parameter specifies the MC index of a pre-defined (‘canned’)
    /// message.
    sm_default_msg_id: GreaterThanU8<0>,
    sm_length: u8,
    short_message: OctetString<255>,
    // TODO: message_submission_tlvs: Vec<MessageSubmittionTLV>,
}

impl SubmitSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        serivce_type: ServiceType,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<21>,
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<21>,
        esm_class: EsmClass,
        protocol_id: u8,
        priority_flag: PriorityFlag,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        registered_delivery: RegisteredDelivery,
        replace_if_present_flag: ReplaceIfPresentFlag,
        data_coding: DataCoding,
        sm_default_msg_id: GreaterThanU8<0>,
        short_message: OctetString<255>,
    ) -> Self {
        let sm_length = short_message.length() as u8;

        Self {
            serivce_type,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            esm_class,
            protocol_id,
            priority_flag,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            replace_if_present_flag,
            data_coding,
            sm_default_msg_id,
            sm_length,
            short_message,
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

    pub fn source_addr(&self) -> &COctetString<21> {
        &self.source_addr
    }

    pub fn dest_addr_ton(&self) -> &Ton {
        &self.dest_addr_ton
    }

    pub fn dest_addr_npi(&self) -> &Npi {
        &self.dest_addr_npi
    }

    pub fn destination_addr(&self) -> &COctetString<21> {
        &self.destination_addr
    }

    pub fn esm_class(&self) -> &EsmClass {
        &self.esm_class
    }

    pub fn protocol_id(&self) -> u8 {
        self.protocol_id
    }

    pub fn priority_flag(&self) -> &PriorityFlag {
        &self.priority_flag
    }

    pub fn schedule_delivery_time(&self) -> &EmptyOrFullCOctetString<17> {
        &self.schedule_delivery_time
    }

    pub fn validity_period(&self) -> &EmptyOrFullCOctetString<17> {
        &self.validity_period
    }

    pub fn registered_delivery(&self) -> &RegisteredDelivery {
        &self.registered_delivery
    }

    pub fn replace_if_present_flag(&self) -> &ReplaceIfPresentFlag {
        &self.replace_if_present_flag
    }

    pub fn data_coding(&self) -> &DataCoding {
        &self.data_coding
    }

    pub fn sm_default_msg_id(&self) -> GreaterThanU8<0> {
        self.sm_default_msg_id
    }

    pub fn sm_length(self) -> u8 {
        self.sm_length
    }

    pub fn short_message(&self) -> &OctetString<255> {
        &self.short_message
    }
}

impl IoLength for SubmitSm {
    fn length(&self) -> usize {
        self.serivce_type.length()
            + self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
            + self.dest_addr_ton.length()
            + self.dest_addr_npi.length()
            + self.destination_addr.length()
            + self.esm_class.length()
            + self.protocol_id.length()
            + self.priority_flag.length()
            + self.schedule_delivery_time.length()
            + self.validity_period.length()
            + self.registered_delivery.length()
            + self.replace_if_present_flag.length()
            + self.data_coding.length()
            + self.sm_default_msg_id.length()
            + self.sm_length.length()
            + self.short_message.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for SubmitSm {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.serivce_type.async_io_write(buf).await?;
        self.source_addr_ton.async_io_write(buf).await?;
        self.source_addr_npi.async_io_write(buf).await?;
        self.source_addr.async_io_write(buf).await?;
        self.dest_addr_ton.async_io_write(buf).await?;
        self.dest_addr_npi.async_io_write(buf).await?;
        self.destination_addr.async_io_write(buf).await?;
        self.esm_class.async_io_write(buf).await?;
        self.protocol_id.async_io_write(buf).await?;
        self.priority_flag.async_io_write(buf).await?;
        self.schedule_delivery_time.async_io_write(buf).await?;
        self.validity_period.async_io_write(buf).await?;
        self.registered_delivery.async_io_write(buf).await?;
        self.replace_if_present_flag.async_io_write(buf).await?;
        self.data_coding.async_io_write(buf).await?;
        self.sm_default_msg_id.async_io_write(buf).await?;
        self.sm_length.async_io_write(buf).await?;
        self.short_message.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for SubmitSm {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let sm_length = u8::async_io_read(buf).await?;

        Ok(Self {
            serivce_type: ServiceType::async_io_read(buf).await?,
            source_addr_ton: Ton::async_io_read(buf).await?,
            source_addr_npi: Npi::async_io_read(buf).await?,
            source_addr: COctetString::async_io_read(buf).await?,
            dest_addr_ton: Ton::async_io_read(buf).await?,
            dest_addr_npi: Npi::async_io_read(buf).await?,
            destination_addr: COctetString::async_io_read(buf).await?,
            esm_class: EsmClass::async_io_read(buf).await?,
            protocol_id: u8::async_io_read(buf).await?,
            priority_flag: PriorityFlag::async_io_read(buf).await?,
            schedule_delivery_time: EmptyOrFullCOctetString::async_io_read(buf).await?,
            validity_period: EmptyOrFullCOctetString::async_io_read(buf).await?,
            registered_delivery: RegisteredDelivery::async_io_read(buf).await?,
            replace_if_present_flag: ReplaceIfPresentFlag::async_io_read(buf).await?,
            data_coding: DataCoding::async_io_read(buf).await?,
            sm_default_msg_id: GreaterThanU8::async_io_read(buf).await?,
            sm_length,
            short_message: OctetString::async_io_read(buf, sm_length as usize).await?,
        })
    }
}