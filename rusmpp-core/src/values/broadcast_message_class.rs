crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum BroadcastMessageClass {
        #[default]
        NoClassSpecified = 0x00,
        Class1 = 0x01,
        Class2 = 0x02,
        Class3 = 0x03,
        Other(u8),
    }
}

impl From<u8> for BroadcastMessageClass {
    fn from(value: u8) -> Self {
        match value {
            0x00 => BroadcastMessageClass::NoClassSpecified,
            0x01 => BroadcastMessageClass::Class1,
            0x02 => BroadcastMessageClass::Class2,
            0x03 => BroadcastMessageClass::Class3,
            value => BroadcastMessageClass::Other(value),
        }
    }
}

impl From<BroadcastMessageClass> for u8 {
    fn from(value: BroadcastMessageClass) -> Self {
        match value {
            BroadcastMessageClass::NoClassSpecified => 0x00,
            BroadcastMessageClass::Class1 => 0x01,
            BroadcastMessageClass::Class2 => 0x02,
            BroadcastMessageClass::Class3 => 0x03,
            BroadcastMessageClass::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<BroadcastMessageClass>();
    }
}
