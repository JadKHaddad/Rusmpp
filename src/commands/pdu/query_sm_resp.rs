use super::Pdu;
use crate::{
    commands::types::message_state::MessageState,
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::{
        c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString,
        u8::EndeU8,
    },
};

#[derive(Default, Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct QuerySmResp {
    /// MC Message ID of the message whose
    /// state is being queried.  
    pub message_id: COctetString<1, 65>,
    /// Date and time when the queried
    /// message reached a final state. For
    /// messages, which have not yet reached
    /// a final state, this field will contain a
    /// single NULL octet.  
    pub final_date: EmptyOrFullCOctetString<17>,
    /// Specifies the status of the queried short
    /// message.
    pub message_state: MessageState,
    /// Where appropriate this holds a network
    /// error code defining the reason for failure
    /// of message delivery.  
    ///
    /// The range of values returned depends
    /// on the underlying telecommunications
    /// network.
    pub error_code: u8,
}

impl QuerySmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        final_date: EmptyOrFullCOctetString<17>,
        message_state: MessageState,
        error_code: u8,
    ) -> Self {
        Self {
            message_id,
            final_date,
            message_state,
            error_code,
        }
    }

    pub fn builder() -> QuerySmRespBuilder {
        QuerySmRespBuilder::new()
    }

    pub fn into_query_sm_resp(self) -> Pdu {
        Pdu::QuerySmResp(self)
    }
}

impl Length for QuerySmResp {
    fn length(&self) -> usize {
        self.message_id.length()
            + self.final_date.length()
            + self.message_state.length()
            + self.error_code.length()
    }
}

impl Encode for QuerySmResp {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.message_id.encode_to(writer));
        tri!(self.final_date.encode_to(writer));
        tri!(self.message_state.encode_to(writer));
        tri!(self.error_code.encode_to(writer));

        Ok(())
    }
}

impl Decode for QuerySmResp {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let message_id = tri!(COctetString::<1, 65>::decode_from(reader));
        let final_date = tri!(EmptyOrFullCOctetString::<17>::decode_from(reader));
        let message_state = tri!(MessageState::decode_from(reader));
        let error_code = tri!(u8::decode_from(reader));

        Ok(Self {
            message_id,
            final_date,
            message_state,
            error_code,
        })
    }
}

#[derive(Default)]
pub struct QuerySmRespBuilder {
    inner: QuerySmResp,
}

impl QuerySmRespBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn message_id(mut self, message_id: COctetString<1, 65>) -> Self {
        self.inner.message_id = message_id;
        self
    }

    pub fn final_date(mut self, final_date: EmptyOrFullCOctetString<17>) -> Self {
        self.inner.final_date = final_date;
        self
    }

    pub fn message_state(mut self, message_state: MessageState) -> Self {
        self.inner.message_state = message_state;
        self
    }

    pub fn error_code(mut self, error_code: u8) -> Self {
        self.inner.error_code = error_code;
        self
    }

    pub fn build(self) -> QuerySmResp {
        self.inner
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::ende::tests::default_encode_decode::<QuerySmResp>();
    }
}
