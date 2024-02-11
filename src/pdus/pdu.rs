use super::{
    body::Body,
    types::{command_id::CommandId, command_status::CommandStatus},
};
use crate::{
    io::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pdu {
    pub command_id: CommandId,
    pub command_status: CommandStatus,
    pub sequence_number: u32,
    pub body: Option<Body>,
}

impl Length for Pdu {
    fn length(&self) -> usize {
        self.command_id.length()
            + self.command_status.length()
            + self.sequence_number.length()
            + self.body.length()
    }
}

impl Encode for Pdu {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.command_id.encode_to(writer));
        tri!(self.command_status.encode_to(writer));
        tri!(self.sequence_number.encode_to(writer));
        tri!(self.body.encode_to(writer));

        Ok(())
    }
}

impl Decode for Pdu {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        todo!()
    }
}
