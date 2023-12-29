use rusmpp_macros::RusmppIoX;

use crate::{
    io::{length::IoLength, read::AsyncIoReadWithLength},
    types::no_fixed_size_octet_string::NoFixedSizeOctetString,
};

use super::{
    body::pdu_body::PduBody,
    types::{
        command_id::CommandId,
        command_status::{CommandStatus, InvalidCommandStatus},
        sequence_number::{InvalidSequenceNumber, SequenceNumber},
    },
};

#[derive(thiserror::Error, Debug)]
pub enum InvalidPdu {
    #[error(transparent)]
    InvalidCommandStatus(#[from] InvalidCommandStatus),
    #[error(transparent)]
    InvalidSequenceNumber(#[from] InvalidSequenceNumber),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoX)]
pub struct Pdu {
    command_length: u32,
    command_id: CommandId,
    command_status: CommandStatus,
    sequence_number: SequenceNumber,
    #[rusmpp_io_x(key=command_id, length=(command_length - all_before))]
    body: Option<PduBody>,
    #[rusmpp_io_x(skip_length, skip_write, length=(body_len - body))]
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

// #[async_trait::async_trait]
// impl AsyncIoRead for Pdu {
//     async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
//         let command_length = u32::async_io_read(buf).await?;
//         let command_id = CommandId::async_io_read(buf).await?;
//         let command_status = CommandStatus::async_io_read(buf).await?;
//         let sequence_number = SequenceNumber::async_io_read(buf).await?;

//         let body_expected_len = (command_length as usize).saturating_sub(
//             command_length.length()
//                 + command_id.length()
//                 + command_status.length()
//                 + sequence_number.length(),
//         );

//         let body =
//             option::async_io_read_with_key_optional(command_id, buf, body_expected_len).await?;

//         let overflow_len = body_expected_len.saturating_sub(body.length());
//         let byte_overflow = NoFixedSizeOctetString::async_io_read(buf, overflow_len).await?;

//         Ok(Self {
//             command_length,
//             command_id,
//             command_status,
//             sequence_number,
//             body,
//             byte_overflow,
//         })
//     }
// }
