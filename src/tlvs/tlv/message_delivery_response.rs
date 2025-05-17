use crate::{types::COctetString, values::*};

crate::create_tlv_value! {
    #[non_exhaustive]
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum MessageDeliveryResponseTlvValue {
        AdditionalStatusInfoText(COctetString<1, 256>),
        DeliveryFailureReason(DeliveryFailureReason),
        NetworkErrorCode(NetworkErrorCode),
    }
}
