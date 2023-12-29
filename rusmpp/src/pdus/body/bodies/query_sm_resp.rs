use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

use crate::{
    pdus::tlvs::tlv_values::message_state::MessageState,
    types::{c_octet_string::COctetString, empty_or_full_c_octet_string::EmptyOrFullCOctetString},
};

#[derive(
    Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIoLength, RusmppIoWrite, RusmppIoRead,
)]
pub struct QuerySmResp {
    pub message_id: COctetString<1, 65>,
    pub final_date: EmptyOrFullCOctetString<17>,
    pub message_state: MessageState,
    pub error_code: u8,
}

impl QuerySmResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        final_date: EmptyOrFullCOctetString<17>,
        message_state: MessageState,
        error_code: u8,
    ) -> Self {
        Self {
            message_id,
            final_date,
            message_state,
            error_code,
        }
    }
}
