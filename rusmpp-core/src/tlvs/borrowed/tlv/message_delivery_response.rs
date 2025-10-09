use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    types::borrowed::COctetString,
    values::*,
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum MessageDeliveryResponseTlvValue<'a> {
    AdditionalStatusInfoText(COctetString<'a, 1, 256>),
    DeliveryFailureReason(DeliveryFailureReason),
    NetworkErrorCode(NetworkErrorCode),
}
