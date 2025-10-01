use rusmpp_macros::TlvValue;

use crate::{
    CommandStatus,
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    values::broadcast_area_identifier::borrowed::BroadcastAreaIdentifier,
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum BroadcastResponseTlvValue<'a> {
    BroadcastErrorStatus(CommandStatus),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier<'a>),
}
