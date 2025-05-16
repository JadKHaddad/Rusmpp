use crate::{
    commands::types::{
        delivery_failure_reason::DeliveryFailureReason, network_error_code::NetworkErrorCode,
    },
    types::COctetString,
};

crate::create_tlv_value! {
    #[non_exhaustive]
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum MessageDeliveryResponseTlvValue {
        AdditionalStatusInfoText(COctetString<1, 256>),
        DeliveryFailureReason(DeliveryFailureReason),
        NetworkErrorCode(NetworkErrorCode),
    }
}
