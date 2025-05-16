use crate::{
    commands::types::{
        delivery_failure_reason::DeliveryFailureReason, dpf_result::DpfResult,
        network_error_code::NetworkErrorCode,
    },
    types::COctetString,
};

crate::create_tlv_value! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum MessageSubmissionResponseTlvValue {
        AdditionalStatusInfoText(COctetString<1, 256>),
        DeliveryFailureReason(DeliveryFailureReason),
        DpfResult(DpfResult),
        NetworkErrorCode(NetworkErrorCode),
    }
}
