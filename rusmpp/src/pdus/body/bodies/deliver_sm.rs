use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::tlvs::tlv::{MessageDeliveryRequestTLV, TLV},
};

use super::sm::Sm;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct DeliverSm {
    sm: Sm,
    tlvs: Vec<TLV>,
}

impl DeliverSm {
    #[allow(clippy::too_many_arguments)]
    pub fn new(sm: Sm, tlvs: Vec<MessageDeliveryRequestTLV>) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        let sm = Sm::check_for_message_payload_and_update(sm, &tlvs);

        Self { sm, tlvs }
    }

    pub fn sm(&self) -> &Sm {
        &self.sm
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }
}

impl IoLength for DeliverSm {
    fn length(&self) -> usize {
        self.sm.length() + self.tlvs.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for DeliverSm {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.sm.async_io_write(buf).await?;
        self.tlvs.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for DeliverSm {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let sm = Sm::async_io_read(buf).await?;

        let tlvs_expected_length = length.saturating_sub(sm.length());

        let tlvs = Vec::<TLV>::async_io_read(buf, tlvs_expected_length).await?;

        Ok(Self { sm, tlvs })
    }
}
