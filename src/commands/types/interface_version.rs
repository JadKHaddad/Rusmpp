use crate::{TlvValue, Tlv};

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub enum InterfaceVersion {
        Smpp3_3OrEarlier(u8),
        #[default]
        Smpp3_4 = 0x34,
        Smpp5_0 = 0x50,
        Other(u8),
    }
}

impl InterfaceVersion {
    #[inline]
    pub fn downcast_from_tlv_value(value: &TlvValue) -> Option<Self> {
        match value {
            TlvValue::ScInterfaceVersion(interface_version) => Some(*interface_version),
            _ => None,
        }
    }

    #[inline]
    pub fn downcast_from_tlv(tlv: &Tlv) -> Option<Self> {
        tlv.value().and_then(Self::downcast_from_tlv_value)
    }
}

impl From<InterfaceVersion> for u8 {
    fn from(value: InterfaceVersion) -> Self {
        match value {
            InterfaceVersion::Smpp3_3OrEarlier(value) => value,
            InterfaceVersion::Smpp3_4 => 0x34,
            InterfaceVersion::Smpp5_0 => 0x50,
            InterfaceVersion::Other(value) => value,
        }
    }
}

impl From<u8> for InterfaceVersion {
    fn from(value: u8) -> Self {
        match value {
            0x00..=0x33 => Self::Smpp3_3OrEarlier(value),
            0x34 => Self::Smpp3_4,
            0x50 => Self::Smpp5_0,
            _ => Self::Other(value),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::tests::default_encode_decode::<InterfaceVersion>();
    }
}
