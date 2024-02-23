use super::Pdu;
use crate::{
    commands::{
        tlvs::{
            tlv::{query_broadcast_response::QueryBroadcastResponseTLV, TLV},
            tlv_value::TLVValue,
        },
        types::{
            broadcast_area_identifier::BroadcastAreaIdentifier,
            broadcast_area_success::BroadcastAreaSuccess, message_state::MessageState,
        },
    },
    ende::{
        decode::{Decode, DecodeError, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::c_octet_string::COctetString,
};

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
    message_state: TLV,
    /// [`TLVValue::BroadcastAreaIdentifier`].
    ///
    /// Identifies one or more target Broadcast Area(s) for which the
    /// status information applies.
    ///
    /// The number of instances of this parameter will be exactly equal
    /// to the number of occurrences of the broadcast_area_identifiers
    /// parameter in the corresponding broadcast_sm.
    broadcast_area_identifier: TLV,
    /// [`TLVValue::BroadcastAreaSuccess`].
    ///
    /// The success rate indicator, defined as the ratio of the
    /// number of BTSs that accepted the message and the total
    /// number of BTSs that should have accepted the message, for
    /// a particular broadcast_area_identifier.
    broadcast_area_success: TLV,
    /// Query broadcast response TLVs ([`QueryBroadcastResponseTLV`]).
    tlvs: Vec<TLV>,
}

impl QueryBroadcastSmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        message_state: MessageState,
        broadcast_area_identifier: BroadcastAreaIdentifier,
        broadcast_area_success: BroadcastAreaSuccess,
        tlvs: Vec<QueryBroadcastResponseTLV>,
    ) -> Self {
        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

        let message_state = TLV::new(TLVValue::MessageState(message_state));

        let broadcast_area_identifier =
            TLV::new(TLVValue::BroadcastAreaIdentifier(broadcast_area_identifier));

        let broadcast_area_success =
            TLV::new(TLVValue::BroadcastAreaSuccess(broadcast_area_success));

        Self {
            message_id,
            message_state,
            broadcast_area_identifier,
            broadcast_area_success,
            tlvs,
        }
    }

    pub fn message_state(&self) -> &TLV {
        &self.message_state
    }

    pub fn set_message_state(&mut self, message_state: MessageState) {
        self.message_state = TLV::new(TLVValue::MessageState(message_state));
    }

    pub fn broadcast_area_identifier(&self) -> &TLV {
        &self.broadcast_area_identifier
    }

    pub fn set_broadcast_area_identifier(
        &mut self,
        broadcast_area_identifier: BroadcastAreaIdentifier,
    ) {
        self.broadcast_area_identifier =
            TLV::new(TLVValue::BroadcastAreaIdentifier(broadcast_area_identifier));
    }

    pub fn broadcast_area_success(&self) -> &TLV {
        &self.broadcast_area_success
    }

    pub fn set_broadcast_area_success(&mut self, broadcast_area_success: BroadcastAreaSuccess) {
        self.broadcast_area_success =
            TLV::new(TLVValue::BroadcastAreaSuccess(broadcast_area_success));
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn set_tlvs(&mut self, tlvs: Vec<QueryBroadcastResponseTLV>) {
        let tlvs = tlvs
            .into_iter()
            .map(|value| value.into())
            .collect::<Vec<TLV>>();

        self.tlvs = tlvs;
    }

    pub fn push_tlv(&mut self, tlv: QueryBroadcastResponseTLV) {
        let tlv = tlv.into();

        self.tlvs.push(tlv);
    }

    pub fn builder() -> QueryBroadcastSmRespBuilder {
        QueryBroadcastSmRespBuilder::new()
    }

    pub fn into_query_broadcast_sm_resp(self) -> Pdu {
        Pdu::QueryBroadcastSmResp(self)
    }
}

impl Default for QueryBroadcastSmResp {
    fn default() -> Self {
        Self {
            message_id: Default::default(),
            message_state: TLV::new(TLVValue::MessageState(Default::default())),
            broadcast_area_identifier: TLV::new(TLVValue::BroadcastAreaIdentifier(
                Default::default(),
            )),
            broadcast_area_success: TLV::new(TLVValue::BroadcastAreaSuccess(Default::default())),
            tlvs: Default::default(),
        }
    }
}

impl Length for QueryBroadcastSmResp {
    fn length(&self) -> usize {
        self.message_id.length()
            + self.message_state.length()
            + self.broadcast_area_identifier.length()
            + self.broadcast_area_success.length()
            + self.tlvs.length()
    }
}

impl Encode for QueryBroadcastSmResp {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.message_id.encode_to(writer));
        tri!(self.message_state.encode_to(writer));
        tri!(self.broadcast_area_identifier.encode_to(writer));
        tri!(self.broadcast_area_success.encode_to(writer));
        tri!(self.tlvs.encode_to(writer));

        Ok(())
    }
}

impl DecodeWithLength for QueryBroadcastSmResp {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let message_id = tri!(COctetString::<1, 65>::decode_from(reader));
        let message_state = tri!(TLV::decode_from(reader));
        let broadcast_area_identifier = tri!(TLV::decode_from(reader));
        let broadcast_area_success = tri!(TLV::decode_from(reader));

        let tlvs_length = length
            .saturating_sub(message_id.length())
            .saturating_sub(message_state.length())
            .saturating_sub(broadcast_area_identifier.length())
            .saturating_sub(broadcast_area_success.length());

        let tlvs = tri!(Vec::<TLV>::decode_from(reader, tlvs_length));

        Ok(Self {
            message_id,
            message_state,
            broadcast_area_identifier,
            broadcast_area_success,
            tlvs,
        })
    }
}

#[derive(Default)]
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

    pub fn tlvs(mut self, tlvs: Vec<QueryBroadcastResponseTLV>) -> Self {
        self.inner.set_tlvs(tlvs);
        self
    }

    pub fn push_tlv(mut self, tlv: QueryBroadcastResponseTLV) -> Self {
        self.inner.push_tlv(tlv);
        self
    }

    pub fn build(self) -> QueryBroadcastSmResp {
        self.inner
    }
}
