use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        owned::{Tlv, TlvValue},
    },
    types::owned::OctetString,
    values::{
        broadcast_area_identifier::owned::BroadcastAreaIdentifier,
        broadcast_area_success::BroadcastAreaSuccess, message_state::MessageState,
        user_message_reference::UserMessageReference,
    },
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum QueryBroadcastResponseTlvValue {
    MessageState(MessageState),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    BroadcastAreaSuccess(BroadcastAreaSuccess),
    BroadcastEndTime(OctetString<0, 17>),
    UserMessageReference(UserMessageReference),
}
