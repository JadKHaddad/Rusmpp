//! SMPP commands.
//!
//! The following PDU example illustrates how a SMPP PDU is decoded:
//!
//! Sample PDU (Values are shown in Hex format):
//!
//! 00 00 00 2F 00 00 00 02 00 00 00 00 00 00 00 01
//!
//! 53 4D 50 50 33 54 45 53 54 00 73 65 63 72 65 74
//!
//! 30 38 00 53 55 42 4D 49 54 31 00 50 01 01 00
//!
//! The 16-octet header would be decoded as follows:
//!
//! | Octets | Description |
//! | ------ | ----------- |
//! | 00 00 00 2F | Command Length (47) |
//! | 00 00 00 02 | Command ID (bind_transmitter) |
//! | 00 00 00 00 | Command Status (0) |
//! | 00 00 00 01 | Sequence Number (1)|
//!
//! The remaining data represents the PDU body (which in this example relates to the
//! bind_transmitter PDU). This is diagnosed as follows:
//!
//! | Octets | Value |
//! | ------ | ----- |
//! | 53 4D 50 50 33 54 45 53 54 00 | system_id (“SMPP3TEST”) |
//! | 73 65 63 72 65 74 30 38 00    | password (“secret08”) |
//! | 53 55 42 4D 49 54 31 00       | system_type (“SUBMIT1”) |
//! | 50                            | interface_version (0x50 “V5.0 compliant”) |
//! | 01                            | addr_ton (0x01) |
//! | 01                            | addr_npi (0x01) |
//! | 00                            | addr_range (NULL) |

use super::{
    pdu::Pdu,
    types::{
        command_id::{CommandId, HasCommandId},
        command_status::CommandStatus,
    },
};
use crate::{
    ende::{
        decode::{Decode, DecodeError, DecodeWithKeyOptional, DecodeWithLength},
        length::Length,
    },
    impl_length_encode, tri,
    types::u32::EndeU32,
};

impl_length_encode! {
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
        ///
        /// Optional because incoming commands may not have a PDU.
        pdu: Option<Pdu>,
    }
}

impl Command {
    pub fn new(command_status: CommandStatus, sequence_number: u32, pdu: Pdu) -> Self {
        let command_id = pdu.command_id();

        Self {
            command_id,
            command_status,
            sequence_number,
            pdu: Some(pdu),
        }
    }

    pub fn command_id(&self) -> CommandId {
        self.command_id
    }

    pub fn pdu(&self) -> Option<&Pdu> {
        self.pdu.as_ref()
    }

    pub fn take_pdu(&mut self) -> Option<Pdu> {
        self.pdu.take()
    }

    pub fn set_pdu(&mut self, pdu: Pdu) {
        self.command_id = pdu.command_id();
        self.pdu = Some(pdu);
    }

    pub fn builder() -> CommandStatusBuilder {
        CommandStatusBuilder {
            inner: Command {
                command_id: CommandId::BindTransmitter,
                command_status: CommandStatus::EsmeRok,
                sequence_number: 0,
                pdu: None,
            },
        }
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

        let pdu = tri!(Pdu::decode_from(command_id, reader, pdu_length),);

        Ok(Self {
            command_id,
            command_status,
            sequence_number,
            pdu,
        })
    }
}

pub struct CommandStatusBuilder {
    inner: Command,
}

impl CommandStatusBuilder {
    pub fn command_status(mut self, command_status: CommandStatus) -> SequenceNumberBuilder {
        self.inner.command_status = command_status;

        SequenceNumberBuilder { inner: self.inner }
    }
}

pub struct SequenceNumberBuilder {
    inner: Command,
}

impl SequenceNumberBuilder {
    pub fn sequence_number(mut self, sequence_number: u32) -> PduBuilder {
        self.inner.sequence_number = sequence_number;

        PduBuilder { inner: self.inner }
    }
}

pub struct PduBuilder {
    inner: Command,
}

impl PduBuilder {
    pub fn pdu(mut self, pdu: Pdu) -> Self {
        self.inner.set_pdu(pdu);
        self
    }

    pub fn build(self) -> Command {
        self.inner
    }
}
