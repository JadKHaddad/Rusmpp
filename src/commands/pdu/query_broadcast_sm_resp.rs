use super::Pdu;
use crate::{
    commands::{
        tlvs::tlv::{query_broadcast_response::QueryBroadcastResponseTlv, SingleTlv, Tlv},
        types::{
            broadcast_area_identifier::BroadcastAreaIdentifier,
            broadcast_area_success::BroadcastAreaSuccess, message_state::MessageState,
        },
    },
    types::COctetString,
};

crate::create! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub struct QueryBroadcastSmResp {
        /// Message ID of the queried message. This must be the MC
        /// assigned Message ID allocated to the original short message
        /// when submitted to the MC by the broadcast_sm, command, and
        /// returned in the broadcast_sm_resp PDU by the MC.
        pub message_id: COctetString<1, 65>,
        /// This field indicates the current status of the broadcast message.
        message_state: SingleTlv<MessageState>,
        /// Identifies one or more target Broadcast Area(s) for which the
        /// status information applies.
        ///
        /// The number of instances of this parameter will be exactly equal
        /// to the number of occurrences of the broadcast_area_identifiers
        /// parameter in the corresponding broadcast_sm.
        broadcast_area_identifier: SingleTlv<BroadcastAreaIdentifier>,
        /// The success rate indicator, defined as the ratio of the
        /// number of BTSs that accepted the message and the total
        /// number of BTSs that should have accepted the message, for
        /// a particular broadcast_area_identifier.
        broadcast_area_success: SingleTlv<BroadcastAreaSuccess>,
        /// Query broadcast response TLVs ([`QueryBroadcastResponseTLV`]).
        @[length = unchecked]
        tlvs: Vec<Tlv>,
    }
}

impl QueryBroadcastSmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        message_state: MessageState,
        broadcast_area_identifier: BroadcastAreaIdentifier,
        broadcast_area_success: BroadcastAreaSuccess,
        tlvs: Vec<impl Into<QueryBroadcastResponseTlv>>,
    ) -> Self {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<Tlv>>();

        let message_state = message_state.into();
        let broadcast_area_identifier = broadcast_area_identifier.into();
        let broadcast_area_success = broadcast_area_success.into();

        Self {
            message_id,
            message_state,
            broadcast_area_identifier,
            broadcast_area_success,
            tlvs,
        }
    }
    //TODO: fix commented out code
    // pub fn message_state(&self) -> MessageState {
    //     *self.message_state.value()
    // }

    pub fn set_message_state(&mut self, message_state: MessageState) {
        self.message_state = message_state.into();
    }

    // pub fn broadcast_area_identifier(&self) -> &BroadcastAreaIdentifier {
    //     self.broadcast_area_identifier.value()
    // }

    pub fn set_broadcast_area_identifier(
        &mut self,
        broadcast_area_identifier: BroadcastAreaIdentifier,
    ) {
        self.broadcast_area_identifier = broadcast_area_identifier.into();
    }

    // pub fn broadcast_area_success(&self) -> BroadcastAreaSuccess {
    //     *self.broadcast_area_success.value()
    // }

    pub fn set_broadcast_area_success(&mut self, broadcast_area_success: BroadcastAreaSuccess) {
        self.broadcast_area_success = broadcast_area_success.into();
    }

    pub fn tlvs(&self) -> &[Tlv] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<impl Into<QueryBroadcastResponseTlv>>) {
        let tlvs = tlvs
            .into_iter()
            .map(Into::into)
            .map(From::from)
            .collect::<Vec<Tlv>>();

        self.tlvs = tlvs;
    }

    pub fn clear_tlvs(&mut self) {
        self.tlvs.clear();
    }

    pub fn push_tlv(&mut self, tlv: impl Into<QueryBroadcastResponseTlv>) {
        let tlv: QueryBroadcastResponseTlv = tlv.into();
        let tlv: Tlv = tlv.into();

        self.tlvs.push(tlv);
    }

    pub fn builder() -> QueryBroadcastSmRespBuilder {
        QueryBroadcastSmRespBuilder::new()
    }
}

impl From<QueryBroadcastSmResp> for Pdu {
    fn from(value: QueryBroadcastSmResp) -> Self {
        Self::QueryBroadcastSmResp(value)
    }
}

#[derive(Debug, Default)]
pub struct QueryBroadcastSmRespBuilder {
    inner: QueryBroadcastSmResp,
}

impl QueryBroadcastSmRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn message_state(mut self, message_state: MessageState) -> Self {
        self.inner.set_message_state(message_state);
        self
    }

    pub fn broadcast_area_identifier(
        mut self,
        broadcast_area_identifier: BroadcastAreaIdentifier,
    ) -> Self {
        self.inner
            .set_broadcast_area_identifier(broadcast_area_identifier);
        self
    }

    pub fn broadcast_area_success(mut self, broadcast_area_success: BroadcastAreaSuccess) -> Self {
        self.inner
            .set_broadcast_area_success(broadcast_area_success);
        self
    }

    pub fn tlvs(mut self, tlvs: Vec<impl Into<QueryBroadcastResponseTlv>>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn clear_tlvs(mut self) -> Self {
        self.inner.clear_tlvs();
        self
    }

    pub fn push_tlv(mut self, tlv: impl Into<QueryBroadcastResponseTlv>) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> QueryBroadcastSmResp {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_with_length_test_instances::<QueryBroadcastSmResp>();
    }
}
