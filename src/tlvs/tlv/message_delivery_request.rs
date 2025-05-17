use crate::{
    types::{COctetString, OctetString},
    values::*,
};

crate::create_tlv_value! {
    #[non_exhaustive]
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum MessageDeliveryRequestTlvValue {
        CallbackNum(OctetString<4, 19>),
        CallbackNumAtag(OctetString<0, 65>),
        CallbackNumPresInd(CallbackNumPresInd),
        DestAddrNpCountry(OctetString<1, 5>),
        DestAddrNpInformation(OctetString<0, 10>),
        DestAddrNpResolution(DestAddrNpResolution),
        DestAddrSubunit(AddrSubunit),
        DestNetworkId(COctetString<7, 66>),
        DestNodeId(OctetString<6, 6>),
        DestSubaddress(Subaddress),
        DestPort(u16),
        DpfResult(DpfResult),
        ItsReplyType(ItsReplyType),
        ItsSessionInfo(ItsSessionInfo),
        LanguageIndicator(LanguageIndicator),
        MessagePayload(MessagePayload),
        MessageState(MessageState),
        NetworkErrorCode(NetworkErrorCode),
        PayloadType(PayloadType),
        PrivacyIndicator(PrivacyIndicator),
        ReceiptedMessageId(COctetString<1, 65>),
        SarMsgRefNum(u16),
        SarSegmentSeqnum(u8),
        SarTotalSegments(u8),
        SourceAddrSubunit(AddrSubunit),
        SourceNetworkId(COctetString<7, 66>),
        SourceNodeId(OctetString<6, 6>),
        SourcePort(u16),
        SourceSubaddress(Subaddress),
        UserMessageReference(UserMessageReference),
        UserResponseCode(u8),
        UssdServiceOp(UssdServiceOp),
    }
}
