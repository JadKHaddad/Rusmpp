use rusmpp_macros::RusmppIo;

use crate::io::read::{AsyncIoRead, AsyncIoReadable, IoReadError};

use super::command_id::CommandId;

#[derive(thiserror::Error, Debug)]
#[error("Invalid sequence number. sequence number should not be 0 when command id is not GenericNack. sequence_number: {value:?}, command_id: {command_id:?}")]
pub struct InvalidSequenceNumber {
    value: u32,
    command_id: CommandId,
}

/// A sequence number allows a response PDU to be correlated with a request PDU. The
/// associated SMPP response PDU must preserve this field. The allowed sequence_number
/// range is from 0x00000001 to 0x7FFFFFFF. In the event of a session using the full range of
/// values for the sequence_number, the ESME or MC should wrap around to 0x00000001. The
/// value 0x00000000 is recommended for use when issuing a generic_nack where the original
/// PDU was deemed completely invalid and its PDU header, was not used to derive a
/// sequence_number for the response PDU
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct SequenceNumber {
    pub value: u32,
}

impl SequenceNumber {
    pub fn new(value: u32) -> Self {
        Self { value }
    }

    pub fn validate(&self, command_id: CommandId) -> Result<(), InvalidSequenceNumber> {
        if self.value == 0 && command_id != CommandId::GenericNack {
            return Err(InvalidSequenceNumber {
                value: self.value,
                command_id,
            });
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for SequenceNumber {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u32::async_io_read(buf).await.map(Self::from)
    }
}

impl From<u32> for SequenceNumber {
    fn from(value: u32) -> Self {
        Self { value }
    }
}

impl From<SequenceNumber> for u32 {
    fn from(value: SequenceNumber) -> Self {
        value.value
    }
}
