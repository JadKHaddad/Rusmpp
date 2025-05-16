use crate::commands::types::{
    broadcast_area_identifier::BroadcastAreaIdentifier, command_status::CommandStatus,
};

crate::create_tlv_value! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum BroadcastResponseTlvValue {
        BroadcastErrorStatus(CommandStatus),
        BroadcastAreaIdentifier(BroadcastAreaIdentifier),
    }
}
