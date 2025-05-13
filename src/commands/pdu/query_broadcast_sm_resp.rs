use super::Pdu;
use crate::{
    commands::{
        tlvs::{
            tlv::{query_broadcast_response::QueryBroadcastResponseTlv, Tlv},
            tlv_value::TlvValue,
        },
        types::{
            broadcast_area_identifier::BroadcastAreaIdentifier,
            broadcast_area_success::BroadcastAreaSuccess, message_state::MessageState,
        },
    },
    types::COctetString,
};

crate::create! {
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct QueryBroadcastSmResp {
        /// Message ID of the queried message. This must be the MC
        /// assigned Message ID allocated to the original short message
        /// when submitted to the MC by the broadcast_sm, command, and
        /// returned in the broadcast_sm_resp PDU by the MC.
        pub message_id: COctetString<1, 65>,
        /// [`TLVValue::MessageState`].
        ///
        /// This field indicates the current status of the broadcast message.
        message_state: Tlv,
        /// [`TLVValue::BroadcastAreaIdentifier`].
        ///
        /// Identifies one or more target Broadcast Area(s) for which the
        /// status information applies.
        ///
        /// The number of instances of this parameter will be exactly equal
        /// to the number of occurrences of the broadcast_area_identifiers
        /// parameter in the corresponding broadcast_sm.
        broadcast_area_identifier: Tlv,
        /// [`TLVValue::BroadcastAreaSuccess`].
        ///
        /// The success rate indicator, defined as the ratio of the
        /// number of BTSs that accepted the message and the total
        /// number of BTSs that should have accepted the message, for
        /// a particular broadcast_area_identifier.
        broadcast_area_success: Tlv,
        /// Query broadcast response TLVs ([`QueryBroadcastResponseTLV`]).
        @[length = unchecked]
        tlvs: Vec<Tlv>,
    }
}

// TODO: add the downcast for these tlvs
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

        let message_state = Tlv::new(TlvValue::MessageState(message_state));

        let broadcast_area_identifier =
            Tlv::new(TlvValue::BroadcastAreaIdentifier(broadcast_area_identifier));

        let broadcast_area_success =
            Tlv::new(TlvValue::BroadcastAreaSuccess(broadcast_area_success));

        Self {
            message_id,
            message_state,
            broadcast_area_identifier,
            broadcast_area_success,
            tlvs,
        }
    }

    pub fn message_state(&self) -> &Tlv {
        &self.message_state
    }

    pub fn set_message_state(&mut self, message_state: MessageState) {
        self.message_state = Tlv::new(TlvValue::MessageState(message_state));
    }

    pub fn broadcast_area_identifier(&self) -> &Tlv {
        &self.broadcast_area_identifier
    }

    pub fn set_broadcast_area_identifier(
        &mut self,
        broadcast_area_identifier: BroadcastAreaIdentifier,
    ) {
        self.broadcast_area_identifier =
            Tlv::new(TlvValue::BroadcastAreaIdentifier(broadcast_area_identifier));
    }

    pub fn broadcast_area_success(&self) -> &Tlv {
        &self.broadcast_area_success
    }

    pub fn set_broadcast_area_success(&mut self, broadcast_area_success: BroadcastAreaSuccess) {
        self.broadcast_area_success =
            Tlv::new(TlvValue::BroadcastAreaSuccess(broadcast_area_success));
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

impl Default for QueryBroadcastSmResp {
    fn default() -> Self {
        Self {
            message_id: Default::default(),
            message_state: Tlv::new(TlvValue::MessageState(Default::default())),
            broadcast_area_identifier: Tlv::new(TlvValue::BroadcastAreaIdentifier(
                Default::default(),
            )),
            broadcast_area_success: Tlv::new(TlvValue::BroadcastAreaSuccess(Default::default())),
            tlvs: Default::default(),
        }
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
