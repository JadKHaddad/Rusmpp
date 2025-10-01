use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    types::borrowed::COctetString,
    values::{
        delivery_failure_reason::DeliveryFailureReason, dpf_result::DpfResult,
        network_error_code::NetworkErrorCode,
    },
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum MessageSubmissionResponseTlvValue<'a> {
    AdditionalStatusInfoText(COctetString<'a, 1, 256>),
    DeliveryFailureReason(DeliveryFailureReason),
    DpfResult(DpfResult),
    NetworkErrorCode(NetworkErrorCode),
}
