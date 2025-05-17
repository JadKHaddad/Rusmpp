use crate::{CommandStatus, values::BroadcastAreaIdentifier};

crate::create_tlv_value! {
    #[non_exhaustive]
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum BroadcastResponseTlvValue {
        BroadcastErrorStatus(CommandStatus),
        BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    }
}
