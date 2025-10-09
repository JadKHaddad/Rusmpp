use rusmpp_macros::TlvValue;

use crate::{
    CommandStatus,
    tlvs::{
        TlvTag,
        borrowed::{Tlv, TlvValue},
    },
    values::borrowed::*,
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum BroadcastResponseTlvValue<'a> {
    BroadcastErrorStatus(CommandStatus),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier<'a>),
}
