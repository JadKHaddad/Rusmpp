use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::RusmppIoU32;

/// The command_id identifies the SMPP operation e.g. submit_sm, bind_transmitter etc. The
/// command_id is encoded as a 4-octet integer value.
///
/// Command_ids for request PDUs are allocated from a range of numbers; 0x00000000 to
/// 0x000001FF.
///
/// Command_ids for response PDUs are allocated from a range of numbers; 0x80000000 to
/// 0x800001FF.
///
/// The relationship between the command_id for a request PDU and its associated response
/// PDU is that bit 31 is cleared for the request and set for the response. For example,
/// replace_sm has a command_id = 0x00000007 and its’ response PDU replace_sm_resp has
/// a command_id = 0x80000007.
#[repr(u32)]
#[derive(
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    RusmppIoU32,
)]
pub enum CommandId {
    BindReceiver = 0x00000001,
    BindTransmitter = 0x00000002,
    QuerySm = 0x00000003,
    SubmitSm = 0x00000004,
    DeliverSm = 0x00000005,
    Unbind = 0x00000006,
    ReplaceSm = 0x00000007,
    CancelSm = 0x00000008,
    BindTransceiver = 0x00000009,
    Outbind = 0x0000000B,
    EnquireLink = 0x00000015,
    SubmitMulti = 0x00000021,
    AlertNotification = 0x00000102,
    DataSm = 0x00000103,
    BroadcastSm = 0x00000111,
    QueryBroadcastSm = 0x00000112,
    CancelBroadcastSm = 0x00000113,
    GenericNack = 0x80000000,
    BindReceiverResp = 0x80000001,
    BindTransmitterResp = 0x80000002,
    QuerySmResp = 0x80000003,
    SubmitSmResp = 0x80000004,
    DeliverSmResp = 0x80000005,
    UnbindResp = 0x80000006,
    ReplaceSmResp = 0x80000007,
    CancelSmResp = 0x80000008,
    BindTransceiverResp = 0x80000009,
    EnquireLinkResp = 0x80000015,
    SubmitMultiResp = 0x80000021,
    DataSmResp = 0x80000103,
    BroadcastSmResp = 0x80000111,
    QueryBroadcastSmResp = 0x80000112,
    CancelBroadcastSmResp = 0x80000113,
    #[num_enum(catch_all)]
    Other(u32),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into() {
        let id: u32 = CommandId::BindReceiver.into();
        assert_eq!(id, 0x00000001);

        let id: u32 = CommandId::Other(0x00000115).into();
        assert_eq!(id, 0x00000115);
    }

    #[test]
    fn from() {
        let id = CommandId::from(0x00000001);
        assert_eq!(id, CommandId::BindReceiver);

        let id = CommandId::from(0x00000115);
        assert_eq!(id, CommandId::Other(0x00000115));
    }
}
