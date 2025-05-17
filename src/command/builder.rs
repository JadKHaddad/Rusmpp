use crate::{CommandStatus, Pdu};

use super::inner::Command;

#[derive(Debug, Default)]
pub struct CommandStatusBuilder {
    inner: Command,
}

impl CommandStatusBuilder {
    #[inline]
    pub fn status(mut self, status: CommandStatus) -> SequenceNumberBuilder {
        self.inner.status = status;

        SequenceNumberBuilder { inner: self.inner }
    }
}

#[derive(Debug)]
pub struct SequenceNumberBuilder {
    inner: Command,
}

impl SequenceNumberBuilder {
    #[inline]
    pub fn sequence_number(mut self, sequence_number: u32) -> PduBuilder {
        self.inner.sequence_number = sequence_number;

        PduBuilder { inner: self.inner }
    }
}

#[derive(Debug)]
pub struct PduBuilder {
    inner: Command,
}

impl PduBuilder {
    #[inline]
    pub fn pdu(mut self, pdu: impl Into<Pdu>) -> Command {
        self.inner.set_pdu(pdu);
        self.inner
    }
}
