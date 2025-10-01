use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    types::borrowed::{COctetString, OctetString},
    values::{
        addr_subunit::AddrSubunit, callback_num_pres_ind::CallbackNumPresInd,
        dest_addr_np_resolution::DestAddrNpResolution, dpf_result::DpfResult,
        its_reply_type::ItsReplyType, its_session_info::ItsSessionInfo,
        language_indicator::LanguageIndicator, message_payload::borrowed::MessagePayload,
        message_state::MessageState, network_error_code::NetworkErrorCode,
        payload_type::PayloadType, privacy_indicator::PrivacyIndicator,
        sub_address::borrowed::Subaddress, user_message_reference::UserMessageReference,
        ussd_service_op::UssdServiceOp,
    },
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum MessageDeliveryRequestTlvValue<'a> {
    CallbackNum(OctetString<'a, 4, 19>),
    CallbackNumAtag(OctetString<'a, 0, 65>),
    CallbackNumPresInd(CallbackNumPresInd),
    DestAddrNpCountry(OctetString<'a, 1, 5>),
    DestAddrNpInformation(OctetString<'a, 0, 10>),
    DestAddrNpResolution(DestAddrNpResolution),
    DestAddrSubunit(AddrSubunit),
    DestNetworkId(COctetString<'a, 7, 66>),
    DestNodeId(OctetString<'a, 6, 6>),
    DestSubaddress(Subaddress<'a>),
    DestPort(u16),
    DpfResult(DpfResult),
    ItsReplyType(ItsReplyType),
    ItsSessionInfo(ItsSessionInfo),
    LanguageIndicator(LanguageIndicator),
    MessagePayload(MessagePayload<'a>),
    MessageState(MessageState),
    NetworkErrorCode(NetworkErrorCode),
    PayloadType(PayloadType),
    PrivacyIndicator(PrivacyIndicator),
    ReceiptedMessageId(COctetString<'a, 1, 65>),
    SarMsgRefNum(u16),
    SarSegmentSeqnum(u8),
    SarTotalSegments(u8),
    SourceAddrSubunit(AddrSubunit),
    SourceNetworkId(COctetString<'a, 7, 66>),
    SourceNodeId(OctetString<'a, 6, 6>),
    SourcePort(u16),
    SourceSubaddress(Subaddress<'a>),
    UserMessageReference(UserMessageReference),
    UserResponseCode(u8),
    UssdServiceOp(UssdServiceOp),
}
