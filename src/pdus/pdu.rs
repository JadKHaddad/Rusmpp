use super::{
    body::Body,
    types::{
        command_id::{CommandId, HasCommandId},
        command_status::CommandStatus,
    },
};
use crate::{
    ende::{
        decode::{Decode, DecodeError, DecodeWithKey, DecodeWithLength},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pdu {
    command_id: CommandId,
    pub command_status: CommandStatus,
    pub sequence_number: u32,
    body: Body,
}

impl Pdu {
    pub fn new(command_status: CommandStatus, sequence_number: u32, body: Body) -> Self {
        let command_id = body.command_id();

        Self {
            command_id,
            command_status,
            sequence_number,
            body,
        }
    }

    pub fn command_id(&self) -> CommandId {
        self.command_id
    }

    pub fn body(&self) -> &Body {
        &self.body
    }

    pub fn into_body(self) -> Body {
        self.body
    }
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

impl DecodeWithLength for Pdu {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let command_id = tri!(CommandId::decode_from(reader));
        let command_status = tri!(CommandStatus::decode_from(reader));
        let sequence_number = tri!(u32::decode_from(reader));

        let body_length = length.saturating_sub(
            command_id.length() + command_status.length() + sequence_number.length(),
        );

        let body = tri!(Body::decode_from(command_id, reader, body_length));

        Ok(Self {
            command_id,
            command_status,
            sequence_number,
            body,
        })
    }
}
