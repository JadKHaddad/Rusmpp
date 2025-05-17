use crate::{
    types::OctetString,
    values::{BroadcastAreaIdentifier, BroadcastAreaSuccess, MessageState, UserMessageReference},
};

crate::create_tlv_value! {
    #[non_exhaustive]
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum QueryBroadcastResponseTlvValue {
        MessageState(MessageState),
        BroadcastAreaIdentifier(BroadcastAreaIdentifier),
        BroadcastAreaSuccess(BroadcastAreaSuccess),
        BroadcastEndTime(OctetString<0, 17>),
        UserMessageReference(UserMessageReference),
    }
}
