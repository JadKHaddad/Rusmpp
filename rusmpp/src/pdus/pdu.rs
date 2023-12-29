use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

use super::{
    body::pdu_body::PduBody,
    types::{
        command_id::CommandId,
        command_status::{CommandStatus, InvalidCommandStatus},
        sequence_number::{InvalidSequenceNumber, SequenceNumber},
    },
};
use crate::{
    io::length::IoLength, io::read::AsyncIoReadWithLength,
    types::no_fixed_size_octet_string::NoFixedSizeOctetString,
};

#[derive(thiserror::Error, Debug)]
pub enum InvalidPdu {
    #[error(transparent)]
    InvalidCommandStatus(#[from] InvalidCommandStatus),
    #[error(transparent)]
    InvalidSequenceNumber(#[from] InvalidSequenceNumber),
}

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoLength, RusmppIoWrite, RusmppIoRead,
)]
pub struct Pdu {
    command_length: u32,
    command_id: CommandId,
    command_status: CommandStatus,
    sequence_number: SequenceNumber,
    #[rusmpp_io_read(key=command_id, length=(command_length - all_before))]
    body: Option<PduBody>,
    #[rusmpp_io_length(skip)]
    #[rusmpp_io_write(skip)]
    #[rusmpp_io_read(length=(body_len - body))]
    byte_overflow: NoFixedSizeOctetString,
}

impl Pdu {
    pub fn new(
        command_status: CommandStatus,
        sequence_number: SequenceNumber,
        body: PduBody,
    ) -> Result<Self, InvalidPdu> {
        let command_id = body.command_id();

        // crate::types::u32::SIZE = 4 is the size of command_length itself as a field
        let command_length = (crate::types::u32::SIZE
            + command_id.length()
            + command_status.length()
            + sequence_number.length()
            + body.length()) as u32;

        let pdu = Self {
            command_length,
            command_id,
            command_status,
            sequence_number,
            body: Some(body),
            byte_overflow: NoFixedSizeOctetString::empty(),
        };

        pdu.validate()?;

        Ok(pdu)
    }

    pub fn new_without_body(
        command_id: CommandId,
        command_status: CommandStatus,
        sequence_number: SequenceNumber,
    ) -> Result<Self, InvalidPdu> {
        let command_length = (crate::types::u32::SIZE
            + command_id.length()
            + command_status.length()
            + sequence_number.length()) as u32;

        let pdu = Self {
            command_length,
            command_id,
            command_status,
            sequence_number,
            body: None,
            byte_overflow: NoFixedSizeOctetString::empty(),
        };

        pdu.validate()?;

        Ok(pdu)
    }

    pub fn validate(&self) -> Result<(), InvalidPdu> {
        self.command_status.validate(self.command_id)?;
        self.sequence_number.validate(self.command_id)?;

        Ok(())
    }

    pub fn command_length(&self) -> u32 {
        self.command_length
    }

    pub fn command_id(&self) -> CommandId {
        self.command_id
    }

    pub fn command_status(&self) -> CommandStatus {
        self.command_status
    }

    pub fn sequence_number(&self) -> SequenceNumber {
        self.sequence_number
    }

    pub fn body(&self) -> Option<&PduBody> {
        self.body.as_ref()
    }

    pub fn into_body(self) -> Option<PduBody> {
        self.body
    }

    pub fn byte_overflow(&self) -> &NoFixedSizeOctetString {
        &self.byte_overflow
    }

    pub fn into_parts(self) -> (CommandId, CommandStatus, SequenceNumber, Option<PduBody>) {
        (
            self.command_id,
            self.command_status,
            self.sequence_number,
            self.body,
        )
    }
}
