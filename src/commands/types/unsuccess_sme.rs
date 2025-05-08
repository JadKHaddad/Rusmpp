use super::{command_status::CommandStatus, npi::Npi, ton::Ton};
use crate::{
    create, tri,
    types::{c_octet_string::COctetString, u32::EndeU32, u8::EndeU8},
};

create! {
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
