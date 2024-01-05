use crate::types::c_octet_string::COctetString;
use derive_builder::Builder;
use derive_new::new;
use getset::{Getters, Setters};
use rusmpp_macros::{RusmppIoLength, RusmppIoRead, RusmppIoWrite};

#[derive(
    new,
    Default,
    Getters,
    Setters,
    Builder,
    Debug,
    Clone,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    RusmppIoLength,
    RusmppIoWrite,
    RusmppIoRead,
)]
#[builder(default)]
pub struct Outbind {
    pub system_id: COctetString<1, 16>,
    pub password: COctetString<1, 9>,
}
