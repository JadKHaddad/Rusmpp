use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        owned::{Tlv, TlvValue},
    },
    types::owned::COctetString,
    values::*,
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum MessageDeliveryResponseTlvValue {
    AdditionalStatusInfoText(COctetString<1, 256>),
    DeliveryFailureReason(DeliveryFailureReason),
    NetworkErrorCode(NetworkErrorCode),
}
