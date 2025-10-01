use rusmpp_macros::TlvValue;

use crate::{
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    types::borrowed::OctetString,
    values::{
        broadcast_area_identifier::borrowed::BroadcastAreaIdentifier,
        broadcast_area_success::BroadcastAreaSuccess, message_state::MessageState,
        user_message_reference::UserMessageReference,
    },
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum QueryBroadcastResponseTlvValue<'a> {
    MessageState(MessageState),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier<'a>),
    BroadcastAreaSuccess(BroadcastAreaSuccess),
    BroadcastEndTime(OctetString<'a, 0, 17>),
    UserMessageReference(UserMessageReference),
}
