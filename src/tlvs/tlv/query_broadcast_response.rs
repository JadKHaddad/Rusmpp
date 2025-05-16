use crate::{
    commands::types::{
        BroadcastAreaIdentifier, BroadcastAreaSuccess, MessageState, UserMessageReference,
    },
    types::OctetString,
};

crate::create_tlv_value! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum QueryBroadcastResponseTlvValue {
        MessageState(MessageState),
        BroadcastAreaIdentifier(BroadcastAreaIdentifier),
        BroadcastAreaSuccess(BroadcastAreaSuccess),
        BroadcastEndTime(OctetString<0, 17>),
        UserMessageReference(UserMessageReference),
    }
}
