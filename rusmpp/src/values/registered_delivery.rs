crate::create! {
    @[repr = u8]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct RegisteredDelivery {
        mc_delivery_receipt: MCDeliveryReceipt,
        sme_originated_acknowledgement: SmeOriginatedAcknowledgement,
        intermediate_notification: IntermediateNotification,
        other: u8,
    }
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

    /// Request all delivery receipts, acknowledgements and notifications
    pub fn request_all() -> Self {
        Self::new(
            MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure,
            SmeOriginatedAcknowledgement::BothDeliveryAndUserAcknowledgmentRequested,
            IntermediateNotification::IntermediateNotificationRequested,
            0,
        )
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

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum MCDeliveryReceipt {
        #[default]
        NoMcDeliveryReceiptRequested = 0b00000000,
        McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure = 0b00000001,
        McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure = 0b00000010,
        McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess = 0b00000011,
        Other(u8),
    }
}

impl From<u8> for MCDeliveryReceipt {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => MCDeliveryReceipt::NoMcDeliveryReceiptRequested,
            0b00000001 => MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure,
            0b00000010 => MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure,
            0b00000011 => MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess,
            value => MCDeliveryReceipt::Other(value),
        }
    }
}

impl From<MCDeliveryReceipt> for u8 {
    fn from(value: MCDeliveryReceipt) -> Self {
        match value {
            MCDeliveryReceipt::NoMcDeliveryReceiptRequested => 0b00000000,
            MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccessOrFailure => 0b00000001,
            MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsFailure => 0b00000010,
            MCDeliveryReceipt::McDeliveryReceiptRequestedWhereFinalDeliveryOutcomeIsSuccess => 0b00000011,
            MCDeliveryReceipt::Other(value) => value,
        }
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum SmeOriginatedAcknowledgement {
        #[default]
        NoReceiptSmeAcknowledgementRequested = 0b00000000,
        SmeDeliveryAcknowledgementRequested = 0b00000100,
        SmeUserAcknowledgementRequested = 0b00001000,
        BothDeliveryAndUserAcknowledgmentRequested = 0b00001100,
        Other(u8),
    }
}

impl From<u8> for SmeOriginatedAcknowledgement {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => SmeOriginatedAcknowledgement::NoReceiptSmeAcknowledgementRequested,
            0b00000100 => SmeOriginatedAcknowledgement::SmeDeliveryAcknowledgementRequested,
            0b00001000 => SmeOriginatedAcknowledgement::SmeUserAcknowledgementRequested,
            0b00001100 => SmeOriginatedAcknowledgement::BothDeliveryAndUserAcknowledgmentRequested,
            value => SmeOriginatedAcknowledgement::Other(value),
        }
    }
}

impl From<SmeOriginatedAcknowledgement> for u8 {
    fn from(value: SmeOriginatedAcknowledgement) -> Self {
        match value {
            SmeOriginatedAcknowledgement::NoReceiptSmeAcknowledgementRequested => 0b00000000,
            SmeOriginatedAcknowledgement::SmeDeliveryAcknowledgementRequested => 0b00000100,
            SmeOriginatedAcknowledgement::SmeUserAcknowledgementRequested => 0b00001000,
            SmeOriginatedAcknowledgement::BothDeliveryAndUserAcknowledgmentRequested => 0b00001100,
            SmeOriginatedAcknowledgement::Other(value) => value,
        }
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum IntermediateNotification {
        #[default]
        NoIntermediaryNotificationRequested = 0b00000000,
        IntermediateNotificationRequested = 0b00010000,
        Other(u8),
    }
}

impl From<u8> for IntermediateNotification {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => IntermediateNotification::NoIntermediaryNotificationRequested,
            0b00010000 => IntermediateNotification::IntermediateNotificationRequested,
            value => IntermediateNotification::Other(value),
        }
    }
}

impl From<IntermediateNotification> for u8 {
    fn from(value: IntermediateNotification) -> Self {
        match value {
            IntermediateNotification::NoIntermediaryNotificationRequested => 0b00000000,
            IntermediateNotification::IntermediateNotificationRequested => 0b00010000,
            IntermediateNotification::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<RegisteredDelivery>();
        crate::tests::encode_decode_test_instances::<MCDeliveryReceipt>();
        crate::tests::encode_decode_test_instances::<SmeOriginatedAcknowledgement>();
        crate::tests::encode_decode_test_instances::<IntermediateNotification>();
    }
}
