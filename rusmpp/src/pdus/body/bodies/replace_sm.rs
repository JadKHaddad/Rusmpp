use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::{
        tlvs::{
            tlv::{MessageReplacementTLV, TLV},
            tlv_tag::TLVTag,
        },
        types::{npi::Npi, registered_delivery::RegisteredDelivery, ton::Ton},
    },
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        octet_string::OctetString,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ReplaceSm {
    message_id: COctetString<1, 65>,
    source_addr_ton: Ton,
    source_addr_npi: Npi,
    source_addr: COctetString<1, 21>,
    schedule_delivery_time: EmptyOrFullCOctetString<17>,
    validity_period: EmptyOrFullCOctetString<17>,
    registered_delivery: RegisteredDelivery,
    sm_default_msg_id: u8,
    sm_length: u8,
    short_message: OctetString<0, 255>,
    tlvs: Vec<TLV>,
}

impl ReplaceSm {
    #[allow(clippy::too_many_arguments)]
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        message_id: COctetString<1, 65>,
        source_addr_ton: Ton,
        source_addr_npi: Npi,
        source_addr: COctetString<1, 21>,
        schedule_delivery_time: EmptyOrFullCOctetString<17>,
        validity_period: EmptyOrFullCOctetString<17>,
        registered_delivery: RegisteredDelivery,
        sm_default_msg_id: u8,
        short_message: OctetString<0, 255>,
        tlvs: Vec<MessageReplacementTLV>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect();

        let replace_sm = Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            sm_default_msg_id,
            sm_length: 0,
            short_message,
            tlvs,
        };

        Self::check_for_message_payload_and_update(replace_sm)
    }

    pub fn check_for_message_payload_and_update(self) -> Self {
        let message_payload_exists = self
            .tlvs
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
}

impl IoLength for ReplaceSm {
    fn length(&self) -> usize {
        self.message_id.length()
            + self.source_addr_ton.length()
            + self.source_addr_npi.length()
            + self.source_addr.length()
            + self.schedule_delivery_time.length()
            + self.validity_period.length()
            + self.registered_delivery.length()
            + self.sm_default_msg_id.length()
            + self.sm_length.length()
            + self.short_message.length()
            + self.tlvs.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for ReplaceSm {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.message_id.async_io_write(buf).await?;
        self.source_addr_ton.async_io_write(buf).await?;
        self.source_addr_npi.async_io_write(buf).await?;
        self.source_addr.async_io_write(buf).await?;
        self.schedule_delivery_time.async_io_write(buf).await?;
        self.validity_period.async_io_write(buf).await?;
        self.registered_delivery.async_io_write(buf).await?;
        self.sm_default_msg_id.async_io_write(buf).await?;
        self.sm_length.async_io_write(buf).await?;
        self.short_message.async_io_write(buf).await?;
        self.tlvs.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for ReplaceSm {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let message_id = COctetString::<1, 65>::async_io_read(buf).await?;
        let source_addr_ton = Ton::async_io_read(buf).await?;
        let source_addr_npi = Npi::async_io_read(buf).await?;
        let source_addr = COctetString::<1, 21>::async_io_read(buf).await?;
        let schedule_delivery_time = EmptyOrFullCOctetString::<17>::async_io_read(buf).await?;
        let validity_period = EmptyOrFullCOctetString::<17>::async_io_read(buf).await?;
        let registered_delivery = RegisteredDelivery::async_io_read(buf).await?;
        let sm_default_msg_id = u8::async_io_read(buf).await?;
        let sm_length = u8::async_io_read(buf).await?;
        let short_message = OctetString::async_io_read(buf, sm_length as usize).await?;

        let tlvs_expected_length = length
            .saturating_sub(message_id.length())
            .saturating_sub(source_addr_ton.length())
            .saturating_sub(source_addr_npi.length())
            .saturating_sub(source_addr.length())
            .saturating_sub(schedule_delivery_time.length())
            .saturating_sub(validity_period.length())
            .saturating_sub(registered_delivery.length())
            .saturating_sub(sm_default_msg_id.length())
            .saturating_sub(sm_length.length())
            .saturating_sub(short_message.length());

        let tlvs = Vec::<TLV>::async_io_read(buf, tlvs_expected_length).await?;

        Ok(Self {
            message_id,
            source_addr_ton,
            source_addr_npi,
            source_addr,
            schedule_delivery_time,
            validity_period,
            registered_delivery,
            sm_default_msg_id,
            sm_length,
            short_message,
            tlvs,
        })
    }
}
