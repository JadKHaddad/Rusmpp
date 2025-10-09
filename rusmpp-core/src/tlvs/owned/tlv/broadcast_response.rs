use rusmpp_macros::TlvValue;

use crate::{
    CommandStatus,
    tlvs::{
        TlvTag,
        owned::{Tlv, TlvValue},
    },
    values::owned::*,
};

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, TlvValue)]
pub enum BroadcastResponseTlvValue {
    BroadcastErrorStatus(CommandStatus),
    BroadcastAreaIdentifier(BroadcastAreaIdentifier),
}
