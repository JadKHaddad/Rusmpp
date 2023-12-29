use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

use crate::{
    io::{length::IoLength, read::AsyncIoReadWithLength},
    pdus::{
        tlvs::{tlv::TLV, tlv_tag::TLVTag},
        types::{
            data_coding::DataCoding, esm_class::EsmClass, npi::Npi, priority_flag::PriorityFlag,
            registered_delivery::RegisteredDelivery, replace_if_present_flag::ReplaceIfPresentFlag,
            service_type::ServiceType, ton::Ton,
        },
    },
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        octet_string::OctetString,
    },
};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoLength, RusmppIoWrite, RusmppIoRead,
)]
pub struct SSm {
    serivce_type: ServiceType,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<1, 21>,
    dest_addr_ton: Ton,
    dest_addr_npi: Npi,
    destination_addr: COctetString<1, 21>,
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
    sm_default_msg_id: u8,
    sm_length: u8,
    #[rusmpp_io_read(length=(sm_length))]
    short_message: OctetString<0, 255>,
}

impl SSm {
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
        protocol_id: u8,
        priority_flag: PriorityFlag,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        registered_delivery: RegisteredDelivery,
        replace_if_present_flag: ReplaceIfPresentFlag,
        data_coding: DataCoding,
        sm_default_msg_id: u8,
        short_message: OctetString<0, 255>,
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

    pub(crate) fn check_for_message_payload_and_update(self, tlvs: &[TLV]) -> Self {
        let message_payload_exists = tlvs
            .iter()
            .any(|v| matches!(v.tag(), TLVTag::MessagePayload));

        let short_message = if message_payload_exists {
            OctetString::empty()
        } else {
            self.short_message
        };

        let sm_length = short_message.length() as u8;

        Self {
            short_message,
            sm_length,
            ..self
        }
    }

    pub fn service_type(&self) -> &ServiceType {
        &self.serivce_type
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

    pub fn dest_addr_ton(&self) -> Ton {
        self.dest_addr_ton
    }

    pub fn dest_addr_npi(&self) -> Npi {
        self.dest_addr_npi
    }

    pub fn destination_addr(&self) -> &COctetString<1, 21> {
        &self.destination_addr
    }

    pub fn esm_class(&self) -> EsmClass {
        self.esm_class
    }

    pub fn protocol_id(&self) -> u8 {
        self.protocol_id
    }

    pub fn priority_flag(&self) -> PriorityFlag {
        self.priority_flag
    }

    pub fn schedule_delivery_time(&self) -> &EmptyOrFullCOctetString<17> {
        &self.schedule_delivery_time
    }

    pub fn validity_period(&self) -> &EmptyOrFullCOctetString<17> {
        &self.validity_period
    }

    pub fn registered_delivery(&self) -> RegisteredDelivery {
        self.registered_delivery
    }

    pub fn replace_if_present_flag(&self) -> ReplaceIfPresentFlag {
        self.replace_if_present_flag
    }

    pub fn data_coding(&self) -> DataCoding {
        self.data_coding
    }

    pub fn sm_default_msg_id(&self) -> u8 {
        self.sm_default_msg_id
    }

    pub fn sm_length(self) -> u8 {
        self.sm_length
    }

    pub fn short_message(&self) -> &OctetString<0, 255> {
        &self.short_message
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
        u8,
        PriorityFlag,
        EmptyOrFullCOctetString<17>,
        EmptyOrFullCOctetString<17>,
        RegisteredDelivery,
        ReplaceIfPresentFlag,
        DataCoding,
        u8,
        OctetString<0, 255>,
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
            self.protocol_id,
            self.priority_flag,
            self.schedule_delivery_time,
            self.validity_period,
            self.registered_delivery,
            self.replace_if_present_flag,
            self.data_coding,
            self.sm_default_msg_id,
            self.short_message,
        )
    }
}

// #[async_trait::async_trait]
// impl AsyncIoRead for SSm {
//     async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
//         let serivce_type = ServiceType::async_io_read(buf).await?;
//         let source_addr_ton = Ton::async_io_read(buf).await?;
//         let source_addr_npi = Npi::async_io_read(buf).await?;
//         let source_addr = COctetString::async_io_read(buf).await?;
//         let dest_addr_ton = Ton::async_io_read(buf).await?;
//         let dest_addr_npi = Npi::async_io_read(buf).await?;
//         let destination_addr = COctetString::async_io_read(buf).await?;
//         let esm_class = EsmClass::async_io_read(buf).await?;
//         let protocol_id = u8::async_io_read(buf).await?;
//         let priority_flag = PriorityFlag::async_io_read(buf).await?;
//         let schedule_delivery_time = EmptyOrFullCOctetString::async_io_read(buf).await?;
//         let validity_period = EmptyOrFullCOctetString::async_io_read(buf).await?;
//         let registered_delivery = RegisteredDelivery::async_io_read(buf).await?;
//         let replace_if_present_flag = ReplaceIfPresentFlag::async_io_read(buf).await?;
//         let data_coding = DataCoding::async_io_read(buf).await?;
//         let sm_default_msg_id = u8::async_io_read(buf).await?;
//         let sm_length = u8::async_io_read(buf).await?;
//         let short_message = OctetString::async_io_read(buf, sm_length as usize).await?;

//         Ok(Self {
//             serivce_type,
//             source_addr_ton,
//             source_addr_npi,
//             source_addr,
//             dest_addr_ton,
//             dest_addr_npi,
//             destination_addr,
//             esm_class,
//             protocol_id,
//             priority_flag,
//             schedule_delivery_time,
//             validity_period,
//             registered_delivery,
//             replace_if_present_flag,
//             data_coding,
//             sm_default_msg_id,
//             sm_length,
//             short_message,
//         })
//     }
// }
