use crate::{commands::types::broadcast_area_identifier::BroadcastAreaIdentifier, CommandStatus};

crate::create_tlv_value! {
    #[non_exhaustive]
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum BroadcastResponseTlvValue {
        BroadcastErrorStatus(CommandStatus),
        BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    }
}
