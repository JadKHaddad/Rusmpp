use super::{command_status::CommandStatus, npi::Npi, ton::Ton};
use crate::{
    ende::{
        decode::{Decode, DecodeError},
        encode::{Encode, EncodeError},
        length::Length,
    },
    tri,
    types::{c_octet_string::COctetString, u32::EndeU32, u8::EndeU8},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct UnsuccessSme {
    /// Type of number for destination.
    pub dest_addr_ton: Ton,
    /// Numbering Plan Indicator for destination.
    pub dest_addr_npi: Npi,
    /// Destination Address of SME.
    pub destination_addr: COctetString<1, 21>,
    /// Indicates the success or failure of the [`Pdu::SubmitMulti`](type@crate::commands::pdu::Pdu::SubmitMulti) request to this SME address.
    pub error_status_code: CommandStatus,
}

impl UnsuccessSme {
    pub fn new(
        dest_addr_ton: Ton,
        dest_addr_npi: Npi,
        destination_addr: COctetString<1, 21>,
        error_status_code: CommandStatus,
    ) -> Self {
        Self {
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            error_status_code,
        }
    }
}

impl Length for UnsuccessSme {
    fn length(&self) -> usize {
        self.dest_addr_ton.length()
            + self.dest_addr_npi.length()
            + self.destination_addr.length()
            + self.error_status_code.length()
    }
}

impl Encode for UnsuccessSme {
    fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), EncodeError> {
        tri!(self.dest_addr_ton.encode_to(writer));
        tri!(self.dest_addr_npi.encode_to(writer));
        tri!(self.destination_addr.encode_to(writer));
        tri!(self.error_status_code.encode_to(writer));

        Ok(())
    }
}

impl Decode for UnsuccessSme {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let dest_addr_ton = tri!(Ton::decode_from(reader));
        let dest_addr_npi = tri!(Npi::decode_from(reader));
        let destination_addr = tri!(COctetString::<1, 21>::decode_from(reader));
        let error_status_code = tri!(CommandStatus::decode_from(reader));

        Ok(Self {
            dest_addr_ton,
            dest_addr_npi,
            destination_addr,
            error_status_code,
        })
    }
}
