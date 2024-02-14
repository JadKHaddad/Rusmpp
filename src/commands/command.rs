//! The following PDU example illustrates how a SMPP PDU is decoded:
//!
//! Sample PDU (Values are shown in Hex format):
//! * 00 00 00 2F 00 00 00 02 00 00 00 00 00 00 00 01
//! * 53 4D 50 50 33 54 45 53 54 00 73 65 63 72 65 74
//! * 30 38 00 53 55 42 4D 49 54 31 00 50 01 01 00
//!
//! The 16-octet header would be decoded as follows:
//! * 00 00 00 2F         Command Length      0x0000002F
//! * 00 00 00 02         Command ID          0x00000002 (bind_transmitter)
//! * 00 00 00 00         Command Status      0x00000000
//! * 00 00 00 01         Sequence Number     0x00000001
//!
//! The remaining data represents the PDU body (which in this example relates to the
//! bind_transmitter PDU).
//!
//! This is diagnosed as follows:
//! * 53 4D 50 50 33 54 45 53 54 00           system_id (“SMPP3TEST”)
//! * 73 65 63 72 65 74 30 38 00              password (“secret08”)
//! * 53 55 42 4D 49 54 31 00                 system_type (“SUBMIT1”)
//! * 50                                      interface_version (0x50 “V5.0 compliant”)
//! * 01                                      addr_ton (0x01)
//! * 01                                      addr_npi (0x01)
//! * 00                                      addr_range (NULL)

use super::{
    pdu::Pdu,
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
pub struct Command {
    /// See [`CommandId`]
    command_id: CommandId,
    /// See [`CommandStatus`]
    pub command_status: CommandStatus,
    /// The sequence_number represents a means of uniquely
    /// identifying each PDU within a SMPP session. It also provides a means of correlating request
    /// and response PDUs based on matching sequence number.
    pub sequence_number: u32,
    /// See [`Pdu`]
    pdu: Pdu,
}

impl Command {
    pub fn new(command_status: CommandStatus, sequence_number: u32, pdu: Pdu) -> Self {
        let command_id = pdu.command_id();

        Self {
            command_id,
            command_status,
            sequence_number,
            pdu,
        }
    }

    pub fn command_id(&self) -> CommandId {
        self.command_id
    }

    pub fn body(&self) -> &Pdu {
        &self.pdu
    }

    pub fn into_body(self) -> Pdu {
        self.pdu
    }
}

impl Length for Command {
    fn length(&self) -> usize {
        self.command_id.length()
            + self.command_status.length()
            + self.sequence_number.length()
            + self.pdu.length()
    }
}

impl Encode for Command {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.command_id.encode_to(writer));
        tri!(self.command_status.encode_to(writer));
        tri!(self.sequence_number.encode_to(writer));
        tri!(self.pdu.encode_to(writer));

        Ok(())
    }
}

impl DecodeWithLength for Command {
    fn decode_from<R: std::io::Read>(reader: &mut R, length: usize) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let command_id = tri!(CommandId::decode_from(reader));
        let command_status = tri!(CommandStatus::decode_from(reader));
        let sequence_number = tri!(u32::decode_from(reader));

        let pdu_length = length.saturating_sub(
            command_id.length() + command_status.length() + sequence_number.length(),
        );

        let pdu = tri!(Pdu::decode_from(command_id, reader, pdu_length));

        Ok(Self {
            command_id,
            command_status,
            sequence_number,
            pdu,
        })
    }
}
