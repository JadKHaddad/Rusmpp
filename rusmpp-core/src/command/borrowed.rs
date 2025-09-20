use rusmpp_macros::Rusmpp;

use crate::{CommandId, CommandStatus, pdus::borrowed::Pdu};

/// `SMPP` command.
///
/// The following PDU example illustrates how a `SMPP` PDU is decoded:
///
/// Sample PDU (Values are shown in Hex format):
///
/// 00 00 00 2F 00 00 00 02 00 00 00 00 00 00 00 01
///
/// 53 4D 50 50 33 54 45 53 54 00 73 65 63 72 65 74
///
/// 30 38 00 53 55 42 4D 49 54 31 00 50 01 01 00
///
/// The 16-octet header would be decoded as follows:
///
/// | Octets | Description |
/// | ------ | ----------- |
/// | 00 00 00 2F | Command Length (47) |
/// | 00 00 00 02 | Command ID (bind_transmitter) |
/// | 00 00 00 00 | Command Status (0) |
/// | 00 00 00 01 | Sequence Number (1)|
///
/// The remaining data represents the PDU body (which in this example relates to the
/// bind_transmitter PDU). This is diagnosed as follows:
///
/// | Octets | Value |
/// | ------ | ----- |
/// | 53 4D 50 50 33 54 45 53 54 00 | system_id (“SMPP3TEST”) |
/// | 73 65 63 72 65 74 30 38 00    | password (“secret08”) |
/// | 53 55 42 4D 49 54 31 00       | system_type (“SUBMIT1”) |
/// | 50                            | interface_version (0x50 “V5.0 compliant”) |
/// | 01                            | addr_ton (0x01) |
/// | 01                            | addr_npi (0x01) |
/// | 00                            | addr_range (NULL) |
#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Rusmpp)]
#[rusmpp(decode = borrowed)]
#[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
#[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
pub struct Command<'a> {
    /// See [`CommandId`]
    id: CommandId,
    /// See [`CommandStatus`]
    pub status: CommandStatus,
    /// The sequence_number represents a means of uniquely
    /// identifying each PDU within a `SMPP` session. It also provides a means of correlating request
    /// and response PDUs based on matching sequence number.
    pub sequence_number: u32,
    /// See [`Pdu`]
    ///
    /// Optional because incoming commands may not have a PDU.
    #[rusmpp(key = id, length = "unchecked")]
    pdu: Option<Pdu<'a>>,
}
