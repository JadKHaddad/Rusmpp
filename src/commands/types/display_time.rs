crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    pub enum DisplayTime {
        Temporary = 0,
        #[default]
        Default = 1,
        Invoke = 2,
        Other(u8),
    }
}

impl From<u8> for DisplayTime {
    fn from(value: u8) -> Self {
        match value {
            0 => DisplayTime::Temporary,
            1 => DisplayTime::Default,
            2 => DisplayTime::Invoke,
            value => DisplayTime::Other(value),
        }
    }
}

impl From<DisplayTime> for u8 {
    fn from(value: DisplayTime) -> Self {
        match value {
            DisplayTime::Temporary => 0,
            DisplayTime::Default => 1,
            DisplayTime::Invoke => 2,
            DisplayTime::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_encode_decode() {
        crate::tests::default_encode_decode::<DisplayTime>();
    }
}
