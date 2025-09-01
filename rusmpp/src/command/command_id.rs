crate::create! {
    #[repr(u32)]
    @[skip_test]
    /// Identifies the `SMPP` operation e.g. submit_sm, bind_transmitter etc.
    ///
    /// The [`CommandId`] is encoded as a 4-octet integer value.
    ///
    /// [`CommandId`]s for request PDUs are allocated from a range of numbers; 0x00000000 to
    /// 0x000001FF.
    ///
    /// [`CommandId`]s for response PDUs are allocated from a range of numbers; 0x80000000 to
    /// 0x800001FF.
    ///
    /// The relationship between the [`CommandId`] for a request PDU and its associated response
    /// PDU is that bit 31 is cleared for the request and set for the response. For example,
    /// replace_sm has a [`CommandId`] = 0x00000007 and its’ response PDU replace_sm_resp has
    /// a [`CommandId`] = 0x80000007.
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    #[cfg_attr(test, derive(strum_macros::EnumIter))]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
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
        Other(u32),
    }
}

impl CommandId {
    /// Returns true if this [`CommandId`] represents a request PDU.
    pub fn is_operation(self) -> bool {
        let id: u32 = self.into();
        id & 0x80000000 == 0x00000000
    }

    /// Returns true if this [`CommandId`] represents a response PDU.
    pub fn is_response(self) -> bool {
        let id: u32 = self.into();
        id & 0x80000000 == 0x80000000
    }

    /// Returns the matching request [`CommandId`]
    ///
    /// Note that this function should be used only on response Ids.
    pub fn matching_request(self) -> CommandId {
        let id: u32 = self.into();
        (id & 0x0FFFFFFF).into()
    }

    /// Returns the matching response [`CommandId`]
    ///
    /// Note that this function should be used only on request Ids.
    /// If the command does not have a response, then it will return [`CommandId::Other`].
    pub fn matching_response(self) -> CommandId {
        let id: u32 = self.into();
        (id | 0x80000000).into()
    }
}

impl From<u32> for CommandId {
    fn from(value: u32) -> Self {
        match value {
            0x00000001 => CommandId::BindReceiver,
            0x00000002 => CommandId::BindTransmitter,
            0x00000003 => CommandId::QuerySm,
            0x00000004 => CommandId::SubmitSm,
            0x00000005 => CommandId::DeliverSm,
            0x00000006 => CommandId::Unbind,
            0x00000007 => CommandId::ReplaceSm,
            0x00000008 => CommandId::CancelSm,
            0x00000009 => CommandId::BindTransceiver,
            0x0000000B => CommandId::Outbind,
            0x00000015 => CommandId::EnquireLink,
            0x00000021 => CommandId::SubmitMulti,
            0x00000102 => CommandId::AlertNotification,
            0x00000103 => CommandId::DataSm,
            0x00000111 => CommandId::BroadcastSm,
            0x00000112 => CommandId::QueryBroadcastSm,
            0x00000113 => CommandId::CancelBroadcastSm,
            0x80000000 => CommandId::GenericNack,
            0x80000001 => CommandId::BindReceiverResp,
            0x80000002 => CommandId::BindTransmitterResp,
            0x80000003 => CommandId::QuerySmResp,
            0x80000004 => CommandId::SubmitSmResp,
            0x80000005 => CommandId::DeliverSmResp,
            0x80000006 => CommandId::UnbindResp,
            0x80000007 => CommandId::ReplaceSmResp,
            0x80000008 => CommandId::CancelSmResp,
            0x80000009 => CommandId::BindTransceiverResp,
            0x80000015 => CommandId::EnquireLinkResp,
            0x80000021 => CommandId::SubmitMultiResp,
            0x80000103 => CommandId::DataSmResp,
            0x80000111 => CommandId::BroadcastSmResp,
            0x80000112 => CommandId::QueryBroadcastSmResp,
            0x80000113 => CommandId::CancelBroadcastSmResp,
            other => CommandId::Other(other),
        }
    }
}

impl From<CommandId> for u32 {
    fn from(value: CommandId) -> Self {
        match value {
            CommandId::BindReceiver => 0x00000001,
            CommandId::BindTransmitter => 0x00000002,
            CommandId::QuerySm => 0x00000003,
            CommandId::SubmitSm => 0x00000004,
            CommandId::DeliverSm => 0x00000005,
            CommandId::Unbind => 0x00000006,
            CommandId::ReplaceSm => 0x00000007,
            CommandId::CancelSm => 0x00000008,
            CommandId::BindTransceiver => 0x00000009,
            CommandId::Outbind => 0x0000000B,
            CommandId::EnquireLink => 0x00000015,
            CommandId::SubmitMulti => 0x00000021,
            CommandId::AlertNotification => 0x00000102,
            CommandId::DataSm => 0x00000103,
            CommandId::BroadcastSm => 0x00000111,
            CommandId::QueryBroadcastSm => 0x00000112,
            CommandId::CancelBroadcastSm => 0x00000113,
            CommandId::GenericNack => 0x80000000,
            CommandId::BindReceiverResp => 0x80000001,
            CommandId::BindTransmitterResp => 0x80000002,
            CommandId::QuerySmResp => 0x80000003,
            CommandId::SubmitSmResp => 0x80000004,
            CommandId::DeliverSmResp => 0x80000005,
            CommandId::UnbindResp => 0x80000006,
            CommandId::ReplaceSmResp => 0x80000007,
            CommandId::CancelSmResp => 0x80000008,
            CommandId::BindTransceiverResp => 0x80000009,
            CommandId::EnquireLinkResp => 0x80000015,
            CommandId::SubmitMultiResp => 0x80000021,
            CommandId::DataSmResp => 0x80000103,
            CommandId::BroadcastSmResp => 0x80000111,
            CommandId::QueryBroadcastSmResp => 0x80000112,
            CommandId::CancelBroadcastSmResp => 0x80000113,
            CommandId::Other(other) => other,
        }
    }
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

    #[test]
    fn is_operation() {
        assert!(CommandId::BindReceiver.is_operation());
        assert!(CommandId::Outbind.is_operation());
        assert!(!CommandId::BindReceiverResp.is_operation());
    }

    #[test]
    fn is_response() {
        assert!(!CommandId::Outbind.is_response());
        assert!(!CommandId::SubmitSm.is_response());
        assert!(CommandId::SubmitSmResp.is_response());
    }

    #[test]
    fn get_matching_request() {
        assert_eq!(
            CommandId::BroadcastSmResp.matching_request(),
            CommandId::BroadcastSm
        );
    }

    #[test]
    fn get_matching_response() {
        assert_eq!(
            CommandId::BroadcastSm.matching_response(),
            CommandId::BroadcastSmResp
        );
        assert_eq!(
            CommandId::Outbind.matching_response(),
            CommandId::Other(0x8000000B)
        );
    }
}
