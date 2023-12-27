use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::RusmppIoU32;

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
    Debug,
    Copy,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    IntoPrimitive,
    FromPrimitive,
    RusmppIoU32,
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
    /// System Error.
    ///
    /// MC system error indicating that all or part
    /// of the MC is currently unavailable. This
    /// can be returned in any response PDU.
    EsmeRsyserr = 0x00000008,
    /// Invalid Source Address.
    ///
    /// Source address of message is
    /// considered invalid. Usually this is
    /// because the field is either too long or
    /// contains invalid characters.
    EsmeRinvsrcadr = 0x0000000A,
    /// Invalid Destination Address.
    ///
    /// Destination address of message is
    /// considered invalid. Usually this is
    /// because the field is either zero length,
    /// too long or contains invalid characters.
    EsmeRinvdstadr = 0x0000000B,
    /// Message ID is invalid.
    ///
    /// Message ID specified in cancel_sm,
    /// query_sm or other operations is invalid.
    EsmeRinvmsgid = 0x0000000C,
    /// Bind Failed.
    ///
    /// A generic failure scenario for a bind
    /// attempt. This may be due to a
    /// provisioning error, incorrect password or
    /// other reason. A MC will typically return
    /// this error for an invalid system_id,
    /// system_type, password or other attribute
    /// that may cause a bind failure.
    EsmeRbindfail = 0x0000000D,
    /// Invalid Password.
    ///
    /// Password field in bind PDU is invalid.
    /// This is usually returned when the length
    /// is too short or too long. It is not supposed
    /// to be returned when the ESME has
    /// specified the incorrect password.
    EsmeRinvpaswd = 0x0000000E,
    /// Invalid System ID.
    ///
    /// The System ID field in bind PDU is
    /// invalid. This is usually returned when the
    /// length is too short or too long. It is not
    /// supposed to be returned when the ESME
    /// has specified the incorrect system id.
    EsmeRinvsysid = 0x0000000F,
    /// Cancel SM Failed.
    ///
    /// Generic failure error for cancel_sm
    /// operation.
    EsmeRcancelfail = 0x00000011,
    /// Replace SM Failed.
    ///
    /// Generic failure for replace_sm operation.
    EsmeRreplacefail = 0x00000013,
    /// Message Queue Full.
    ///
    /// Used to indicate a resource error within
    /// the MC. This may be interpreted as the
    /// maximum number of messages
    /// addressed to a single destination or a
    /// global maximum of undelivered
    /// messages within the MC.
    EsmeRmsgqful = 0x00000014,
    /// Invalid Service Type.
    ///
    /// Service type is rejected either because it
    /// is not recognised by the MC or because
    /// its length is not within the defined range.
    EsmeRinvsertyp = 0x00000015,
    /// Invalid number of destinations.
    ///
    /// The number_of_dests field in the
    /// submit_multi PDU is invalid.
    EsmeRinvnumdests = 0x00000033,
    /// Invalid Distribution List name.
    ///
    /// The dl_name field specified in the
    /// submit_multi PDU is either invalid, or
    /// non-existent.
    EsmeRinvdlname = 0x00000034,
    /// Destination flag is invalid (submit_multi).
    ///
    /// The dest_flag field in the submit_multi
    /// PDU has been encoded with an invalid
    /// setting.
    EsmeRinvdestflag = 0x00000040,
    /// Submit w/replace functionality has been
    /// requested where it is either unsupported
    /// or inappropriate for the particular MC.
    /// This can typically occur with submit_multi
    /// where the context of “replace if present”
    /// is often a best effort operation and MCs
    /// may not support the feature in
    /// submit_multi.
    ///
    /// Another reason for returning this error
    /// would be where the feature has been
    /// denied to an ESME.
    EsmeRinvsubrep = 0x00000042,
    /// Invalid esm_class field data.
    ///
    /// The esm_class field has an unsupported
    /// setting.
    EsmeRinvesmclass = 0x00000043,
    /// Cannot Submit to Distribution List.
    ///
    /// Distribution lists are not supported, are
    /// denied or unavailable.
    EsmeRcntsubdl = 0x00000044,
    /// submit_sm, data_sm or submit_multi
    /// failed.
    ///
    /// Generic failure error for submission
    /// operations.
    EsmeRsubmitfail = 0x00000045,
    /// Invalid Source address TON.
    ///
    /// The source TON of the message is either
    /// invalid or unsupported.
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
