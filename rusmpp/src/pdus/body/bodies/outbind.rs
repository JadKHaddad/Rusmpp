use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

use crate::types::c_octet_string::COctetString;

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoLength, RusmppIoWrite, RusmppIoRead,
)]
pub struct Outbind {
    pub system_id: COctetString<1, 16>,
    pub password: COctetString<1, 9>,
}

impl Outbind {
    pub fn new(system_id: COctetString<1, 16>, password: COctetString<1, 9>) -> Self {
        Self {
            system_id,
            password,
        }
    }
}
