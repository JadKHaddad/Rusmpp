use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        owned::{Tlv, TlvValue},
    },
    types::owned::COctetString,
    values::{
        delivery_failure_reason::DeliveryFailureReason, network_error_code::NetworkErrorCode,
    },
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum MessageDeliveryResponseTlvValue {
    AdditionalStatusInfoText(COctetString< 1, 256>),
    DeliveryFailureReason(DeliveryFailureReason),
    NetworkErrorCode(NetworkErrorCode),
}
