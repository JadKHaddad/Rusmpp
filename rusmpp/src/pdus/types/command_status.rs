use num_enum::{FromPrimitive, IntoPrimitive};

use crate::io::{
    length::IoLength,
    read::{AsyncIoRead, AsyncIoReadable, IoReadError},
    write::{AsyncIoWritable, AsyncIoWrite},
};

use super::command_id::CommandId;

#[derive(thiserror::Error, Debug)]
#[error("Invalid command status. command status should not be 0 when command id is GenericNack. command_status: {status:?}             ,          command_id: {command_id:?}")]
pub struct InvalidCommandStatus {
    status: CommandStatus,
    command_id: CommandId,
}

/// The command_status represents the means by which an ESME or MC sends an error code
/// to its peer. This field is only relevant in response PDUs. Thus PDU requests always have this
/// field set to NULL (0x00000000).
///
/// The command_status field of a SMPP message response indicates the success or failure of
/// a SMPP request. It is relevant only in the SMPP response message and should be set to
/// NULL in SMPP request messages.
///
/// The SMPP Error status codes are returned by the MC in the command_status field of the
/// SMPP message header and in the error_status_code field of a submit_multi_resp message.
#[repr(u32)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum CommandStatus {
    ///No Error.
    ///
    ///Specified in a response PDU to indicate
    ///the success of the corresponding request
    ///PDU.
    EsmeRok = 0x00000000,
    /// Message Length is invalid.
    ///
    /// short_message field or
    /// message_payload TLV has an invalid
    /// length (usually too long for the given MC
    /// or underlying network technology).
    EsmeRinvmsglen = 0x00000001,
    /// Command Length is invalid.
    ///
    /// PDU length is considered invalid, either
    /// because the value is too short or too
    /// large for the given PDU.
    EsmeRinvcmdlen = 0x00000002,
    /// Invalid Command ID.
    ///
    /// Command ID is not recognised, either
    /// because the operation is not supported
    /// or unknown.
    EsmeRinvcmdid = 0x00000003,
    /// Incorrect BIND Status for given command.
    ///
    /// PDU has been sent in the wrong session
    /// state. E.g. sending a submit_sm without
    /// first establishing a Bound_TX session
    /// state.
    EsmeRinvbndsts = 0x00000004,
    /// ESME Already in Bound State.
    ///
    /// A bind request has been issued within a
    /// session that is already bound.
    EsmeRalybnd = 0x00000005,
    /// Invalid Priority Flag.
    ///
    /// Priority flag contains an illegal or
    /// unsupported value.
    EsmeRinvprtflg = 0x00000006,
    /// Invalid Registered Delivery Flag.
    ///
    /// Registered field contains an invalid
    /// setting.
    EsmeRinvregdlvflg = 0x00000007,
    EsmeRsyserr = 0x00000008,
    EsmeRinvsrcadr = 0x0000000A,
    EsmeRinvdstadr = 0x0000000B,
    EsmeRinvmsgid = 0x0000000C,
    EsmeRbindfail = 0x0000000D,
    EsmeRinvpaswd = 0x0000000E,
    EsmeRinvsysid = 0x0000000F,
    EsmeRcancelfail = 0x00000011,
    EsmeRreplacefail = 0x00000013,
    EsmeRmsgqful = 0x00000014,
    EsmeRinvsertyp = 0x00000015,
    EsmeRinvnumdests = 0x00000033,
    EsmeRinvdlname = 0x00000034,
    EsmeRinvdestflag = 0x00000040,
    EsmeRinvsubrep = 0x00000042,
    EsmeRinvesmclass = 0x00000043,
    EsmeRcntsubdl = 0x00000044,
    EsmeRsubmitfail = 0x00000045,
    EsmeRinvsrcton = 0x00000048,
    EsmeRinvsrcnpi = 0x00000049,
    EsmeRinvdstton = 0x00000050,
    EsmeRinvdstnpi = 0x00000051,
    EsmeRinvsystyp = 0x00000053,
    EsmeRinvrepflag = 0x00000054,
    EsmeRinvnummsgs = 0x00000055,
    EsmeRthrottled = 0x00000058,
    EsmeRinvsched = 0x00000061,
    EsmeRinvexpiry = 0x00000062,
    EsmeRinvdftmsgid = 0x00000063,
    EsmeRxTAppn = 0x00000064,
    EsmeRxPAppn = 0x00000065,
    EsmeRxRAppn = 0x00000066,
    EsmeRqueryfail = 0x00000067,
    EsmeRinvtlvstream = 0x000000C0,
    EsmeRtlvnotallwd = 0x000000C1,
    EsmeRinvtlvlen = 0x000000C2,
    EsmeRmissingtlv = 0x000000C3,
    EsmeRinvtlvval = 0x000000C4,
    EsmeRdeliveryfailure = 0x000000FE,
    EsmeRunknownerr = 0x000000FF,
    EsmeRsertypunauth = 0x00000100,
    EsmeRprohibited = 0x00000101,
    EsmeRsertypunavail = 0x00000102,
    EsmeRsertypdenied = 0x00000103,
    EsmeRinvdcs = 0x00000104,
    EsmeRinvsrcaddrsubunit = 0x00000105,
    EsmeRinvdstaddrsubunit = 0x00000106,
    EsmeRinvbcastfreqint = 0x00000107,
    EsmeRinvbcastaliasName = 0x00000108,
    EsmeRinvbcastareafmt = 0x00000109,
    EsmeRinvnumbcastAreas = 0x0000010A,
    EsmeRinvbcastcnttype = 0x0000010B,
    EsmeRinvbcastmsgclass = 0x0000010C,
    EsmeRbcastfail = 0x0000010D,
    EsmeRbcastqueryfail = 0x0000010E,
    EsmeRbcastcancelfail = 0x0000010F,
    EsmeRinvbcastRep = 0x00000110,
    EsmeRinvbcastsrvgrp = 0x00000111,
    EsmeRinvbcastchanind = 0x00000112,
    #[num_enum(catch_all)]
    Other(u32),
}

impl CommandStatus {
    pub fn is_success(&self) -> bool {
        matches!(self, CommandStatus::EsmeRok)
    }

    pub fn validate(&self, command_id: CommandId) -> Result<(), InvalidCommandStatus> {
        if self.is_success() && matches!(command_id, CommandId::GenericNack) {
            return Err(InvalidCommandStatus {
                status: *self,
                command_id,
            });
        }

        Ok(())
    }
}

#[allow(clippy::derivable_impls)]
impl Default for CommandStatus {
    fn default() -> Self {
        CommandStatus::EsmeRok
    }
}

impl IoLength for CommandStatus {
    fn length(&self) -> usize {
        u32::from(*self).length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for CommandStatus {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        u32::from(*self).async_io_write(buf).await
    }
}

#[async_trait::async_trait]
impl AsyncIoRead for CommandStatus {
    async fn async_io_read(buf: &mut AsyncIoReadable) -> Result<Self, IoReadError> {
        u32::async_io_read(buf).await.map(Self::from)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn into() {
        let status: u32 = CommandStatus::EsmeRok.into();
        assert_eq!(status, 0x00000000);

        let status: u32 = CommandStatus::Other(0x00000115).into();
        assert_eq!(status, 0x00000115);
    }

    #[test]
    fn from() {
        let status = CommandStatus::from(0x00000000);
        assert_eq!(status, CommandStatus::EsmeRok);

        let status = CommandStatus::from(0x00000115);
        assert_eq!(status, CommandStatus::Other(0x00000115));
    }
}
