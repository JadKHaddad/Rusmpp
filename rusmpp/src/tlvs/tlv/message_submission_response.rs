use crate::{
    types::COctetString,
    values::{DeliveryFailureReason, DpfResult, NetworkErrorCode},
};

crate::create_tlv_value! {
    #[non_exhaustive]
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum MessageSubmissionResponseTlvValue {
        AdditionalStatusInfoText(COctetString<1, 256>),
        DeliveryFailureReason(DeliveryFailureReason),
        DpfResult(DpfResult),
        NetworkErrorCode(NetworkErrorCode),
    }
}
