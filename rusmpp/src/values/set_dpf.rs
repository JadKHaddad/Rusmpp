crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum SetDpf {
        NotRequested = 0,
        #[default]
        Requested = 1,
        Other(u8),
    }
}

impl From<u8> for SetDpf {
    fn from(value: u8) -> Self {
        match value {
            0 => SetDpf::NotRequested,
            1 => SetDpf::Requested,
            value => SetDpf::Other(value),
        }
    }
}

impl From<SetDpf> for u8 {
    fn from(value: SetDpf) -> Self {
        match value {
            SetDpf::NotRequested => 0,
            SetDpf::Requested => 1,
            SetDpf::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<SetDpf>();
    }
}
