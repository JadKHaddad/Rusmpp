use tokio::io::AsyncReadExt;

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadWithKeyOptional, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
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

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Pdu {
    command_length: u32,
    command_id: CommandId,
    command_status: CommandStatus,
    sequence_number: SequenceNumber,
    body: Option<PduBody>,
    byte_overflow: Vec<u8>,
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
            byte_overflow: vec![],
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
            byte_overflow: vec![],
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
}

impl IoLength for Pdu {
    fn length(&self) -> usize {
        self.command_length.length()
            + self.command_id.length()
            + self.command_status.length()
            + self.sequence_number.length()
            + self.body.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for Pdu {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.command_length.async_io_write(buf).await?;
        self.command_id.async_io_write(buf).await?;
        self.command_status.async_io_write(buf).await?;
        self.sequence_number.async_io_write(buf).await?;
        self.body.async_io_write(buf).await?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for Pdu {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        let command_length = u32::async_io_read(buf).await?;
        let command_id = CommandId::async_io_read(buf).await?;
        let command_status = CommandStatus::async_io_read(buf).await?;
        let sequence_number = SequenceNumber::async_io_read(buf).await?;

        let body_expected_len = (command_length as usize).saturating_sub(
            command_length.length()
                + command_id.length()
                + command_status.length()
                + sequence_number.length(),
        );

        let (body, byte_overflow) = if body_expected_len > 0 {
            let body = PduBody::async_io_read(command_id, buf, body_expected_len).await?;

            let mut byte_overflow = vec![0; body_expected_len.saturating_sub(body.length())];
            buf.read_exact(&mut byte_overflow).await?;

            (body, byte_overflow)
        } else {
            (None, vec![])
        };

        Ok(Self {
            command_length,
            command_id,
            command_status,
            sequence_number,
            body,
            byte_overflow,
        })
    }
}
