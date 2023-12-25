use num_enum::{FromPrimitive, IntoPrimitive};

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub struct RegisteredDelivery {
    mc_delivery_receipt: MCDeliveryReceipt,
    sme_originated_acknowledgement: SmeOriginatedAcknowledgement,
    intermediate_notification: IntermediateNotification,
    other: u8,
}

impl RegisteredDelivery {
    pub fn new(
        mc_delivery_receipt: MCDeliveryReceipt,
        sme_originated_acknowledgement: SmeOriginatedAcknowledgement,
        intermediate_notification: IntermediateNotification,
        other: u8,
    ) -> Self {
        // remove first 5 bits from other
        let other = other & 0b00011111;

        Self {
            mc_delivery_receipt,
            sme_originated_acknowledgement,
            intermediate_notification,
            other,
        }
    }

    pub fn mc_delivery_receipt(&self) -> MCDeliveryReceipt {
        self.mc_delivery_receipt
    }

    pub fn sme_originated_acknowledgement(&self) -> SmeOriginatedAcknowledgement {
        self.sme_originated_acknowledgement
    }

    pub fn intermediate_notification(&self) -> IntermediateNotification {
        self.intermediate_notification
    }

    pub fn other(&self) -> u8 {
        self.other
    }
}

impl From<u8> for RegisteredDelivery {
    fn from(value: u8) -> Self {
        let mc_delivery_receipt = MCDeliveryReceipt::from(value & 0b00000011);
        let sme_originated_acknowledgement = SmeOriginatedAcknowledgement::from(value & 0b00001100);
        let intermediate_notification = IntermediateNotification::from(value & 0b00010000);
        let other = value & 0b11100000;

        Self {
            mc_delivery_receipt,
            sme_originated_acknowledgement,
            intermediate_notification,
            other,
        }
    }
}

impl From<RegisteredDelivery> for u8 {
    fn from(value: RegisteredDelivery) -> Self {
        u8::from(value.mc_delivery_receipt)
            | u8::from(value.sme_originated_acknowledgement)
            | u8::from(value.intermediate_notification)
            | value.other
    }
}

impl IoLength for RegisteredDelivery {
    fn length(&self) -> usize {
        u8::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for RegisteredDelivery {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u8::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for RegisteredDelivery {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u8::async_io_read(buf).await.map(Self::from)
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum MCDeliveryReceipt {
    NoMcDeliveryReceiptRequested = 0b00000000,
    McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure = 0b00000001,
    McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure = 0b00000010,
    McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess = 0b00000011,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for MCDeliveryReceipt {
    fn default() -> Self {
        MCDeliveryReceipt::NoMcDeliveryReceiptRequested
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum SmeOriginatedAcknowledgement {
    NoReceiptSmeAcknowledgementRequested = 0b00000000,
    SmeDeliveryAcknowledgementRequested = 0b00000100,
    SmeUserAcknowledgementRequested = 0b00001000,
    BothDeliveryAndUserAcknowledgmentRequested = 0b00001100,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for SmeOriginatedAcknowledgement {
    fn default() -> Self {
        SmeOriginatedAcknowledgement::NoReceiptSmeAcknowledgementRequested
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum IntermediateNotification {
    NoIntermediaryNotificationRequested = 0b00000000,
    IntermediateNotificationRequested = 0b00010000,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for IntermediateNotification {
    fn default() -> Self {
        IntermediateNotification::NoIntermediaryNotificationRequested
    }
}
