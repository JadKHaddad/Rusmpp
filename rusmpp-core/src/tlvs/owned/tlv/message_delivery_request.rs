use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        owned::{Tlv, TlvValue},
    },
    types::owned::{COctetString, OctetString},
    values::{
        addr_subunit::AddrSubunit, callback_num_pres_ind::CallbackNumPresInd,
        dest_addr_np_resolution::DestAddrNpResolution, dpf_result::DpfResult,
        its_reply_type::ItsReplyType, its_session_info::ItsSessionInfo,
        language_indicator::LanguageIndicator, message_payload::owned::MessagePayload,
        message_state::MessageState, network_error_code::NetworkErrorCode,
        payload_type::PayloadType, privacy_indicator::PrivacyIndicator,
        sub_address::owned::Subaddress, user_message_reference::UserMessageReference,
        ussd_service_op::UssdServiceOp,
    },
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
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
