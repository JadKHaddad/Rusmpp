/// Docs
///
/// More docs
pub struct Command {
    /// Docs
    ///
    /// More docs
    command_id: CommandId,
    pub command_status: CommandStatus,
    pub sequence_number: u32,
    /// Docs
    ///
    /// More docs
    pdu: Option<Pdu>,
}
#[automatically_derived]
impl ::core::fmt::Debug for Command {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "Command",
            "command_id",
            &self.command_id,
            "command_status",
            &self.command_status,
            "sequence_number",
            &self.sequence_number,
            "pdu",
            &&self.pdu,
        )
    }
}
pub struct CommandParts {
    pub command_id: CommandId,
    pub command_status: CommandStatus,
    pub sequence_number: u32,
    pub pdu: Option<Pdu>,
}
#[automatically_derived]
impl ::core::fmt::Debug for CommandParts {
    #[inline]
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        ::core::fmt::Formatter::debug_struct_field4_finish(
            f,
            "CommandParts",
            "command_id",
            &self.command_id,
            "command_status",
            &self.command_status,
            "sequence_number",
            &self.sequence_number,
            "pdu",
            &&self.pdu,
        )
    }
}
impl CommandParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        command_id: CommandId,
        command_status: CommandStatus,
        sequence_number: u32,
        pdu: Option<Pdu>,
    ) -> Self {
        Self {
            command_id,
            command_status,
            sequence_number,
            pdu,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (CommandId, CommandStatus, u32, Option<Pdu>) {
        (self.command_id, self.command_status, self.sequence_number, self.pdu)
    }
}
impl Command {
    #[inline]
    pub fn into_parts(self) -> CommandParts {
        CommandParts {
            command_id: self.command_id,
            command_status: self.command_status,
            sequence_number: self.sequence_number,
            pdu: self.pdu,
        }
    }
}
impl ::rusmpp::encode::Length for Command {
    fn length(&self) -> usize {
        let mut length = 0;
        length += ::rusmpp::encode::Length::length(&self.command_id);
        length += ::rusmpp::encode::Length::length(&self.command_status);
        length += ::rusmpp::encode::Length::length(&self.sequence_number);
        length += ::rusmpp::encode::Length::length(&self.pdu);
        length
    }
}
impl ::rusmpp::encode::Encode for Command {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.command_id, dst, size);
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.command_status,
            dst,
            size,
        );
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.sequence_number,
            dst,
            size,
        );
        let size = ::rusmpp::encode::EncodeExt::encode_move(&self.pdu, dst, size);
        size
    }
}
impl ::rusmpp::decode::DecodeWithLength for Command {
    fn decode(
        src: &[u8],
        length: usize,
    ) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        let size = 0;
        let (command_id, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::decode_move(src, size),
            ::rusmpp::fields::SmppField::command_id,
        )?;
        let (command_status, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::decode_move(src, size),
            ::rusmpp::fields::SmppField::command_status,
        )?;
        let (sequence_number, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
            ::rusmpp::decode::DecodeExt::decode_move(src, size),
            ::rusmpp::fields::SmppField::sequence_number,
        )?;
        let (pdu, size) = ::rusmpp::decode::DecodeErrorExt::map_as_source(
                ::rusmpp::decode::DecodeWithKeyOptionalExt::decode_move(
                    command_id,
                    src,
                    length.saturating_sub(size),
                    size,
                ),
                ::rusmpp::fields::SmppField::pdu,
            )?
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((
            Self {
                command_id,
                command_status,
                sequence_number,
                pdu,
            },
            size,
        ))
    }
}
