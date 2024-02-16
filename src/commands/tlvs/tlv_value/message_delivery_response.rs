use crate::{
    commands::types::{
        delivery_failure_reason::DeliveryFailureReason, network_error_code::NetworkErrorCode,
    },
    types::c_octet_string::COctetString,
};

use super::TLVValue;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum MessageDeliveryResponseTLVValue {
    AdditionalStatusInfoText(COctetString<1, 256>),
    DeliveryFailureReason(DeliveryFailureReason),
    NetworkErrorCode(NetworkErrorCode),
}

impl From<MessageDeliveryResponseTLVValue> for TLVValue {
    fn from(value: MessageDeliveryResponseTLVValue) -> Self {
        match value {
            MessageDeliveryResponseTLVValue::AdditionalStatusInfoText(v) => {
                TLVValue::AdditionalStatusInfoText(v)
            }
            MessageDeliveryResponseTLVValue::DeliveryFailureReason(v) => {
                TLVValue::DeliveryFailureReason(v)
            }
            MessageDeliveryResponseTLVValue::NetworkErrorCode(v) => TLVValue::NetworkErrorCode(v),
        }
    }
}
