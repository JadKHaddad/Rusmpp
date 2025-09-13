crate::create! {
    @[repr = u8]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub struct CallbackNumPresInd {
        pub presentation: Presentation,
        pub screening: Screening,
    }
}

impl CallbackNumPresInd {
    pub fn new(presentation: Presentation, screening: Screening) -> Self {
        Self {
            presentation,
            screening,
        }
    }
}

impl From<u8> for CallbackNumPresInd {
    fn from(value: u8) -> Self {
        Self {
            presentation: Presentation::from(value & 0b00000011),
            screening: Screening::from(value & 0b00001100),
        }
    }
}

impl From<CallbackNumPresInd> for u8 {
    fn from(value: CallbackNumPresInd) -> Self {
        u8::from(value.presentation) | u8::from(value.screening)
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
#[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum Presentation {
        #[default]
        PresentationAllowed = 0b00000000,
        PresentationRestricted = 0b00000001,
        NumberNotAvailable = 0b00000010,
        Other(u8),
    }
}

impl From<u8> for Presentation {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => Presentation::PresentationAllowed,
            0b00000001 => Presentation::PresentationRestricted,
            0b00000010 => Presentation::NumberNotAvailable,
            value => Presentation::Other(value),
        }
    }
}

impl From<Presentation> for u8 {
    fn from(value: Presentation) -> Self {
        match value {
            Presentation::PresentationAllowed => 0b00000000,
            Presentation::PresentationRestricted => 0b00000001,
            Presentation::NumberNotAvailable => 0b00000010,
            Presentation::Other(value) => value,
        }
    }
}

crate::create! {
    #[repr(u8)]
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
    #[cfg_attr(feature = "arbitrary", derive(::arbitrary::Arbitrary))]
    #[cfg_attr(feature = "serde", derive(::serde::Serialize))]
    #[cfg_attr(feature = "serde-deserialize-unchecked", derive(::serde::Deserialize))]
    pub enum Screening {
        #[default]
        NotScreened = 0b00000000,
        VerifiedAndPassed = 0b00000100,
        VerifiedAndFailed = 0b00001000,
        NetworkProvided = 0b00001100,
        Other(u8),
    }
}

impl From<u8> for Screening {
    fn from(value: u8) -> Self {
        match value {
            0b00000000 => Screening::NotScreened,
            0b00000100 => Screening::VerifiedAndPassed,
            0b00001000 => Screening::VerifiedAndFailed,
            0b00001100 => Screening::NetworkProvided,
            value => Screening::Other(value),
        }
    }
}

impl From<Screening> for u8 {
    fn from(value: Screening) -> Self {
        match value {
            Screening::NotScreened => 0b00000000,
            Screening::VerifiedAndPassed => 0b00000100,
            Screening::VerifiedAndFailed => 0b00001000,
            Screening::NetworkProvided => 0b00001100,
            Screening::Other(value) => value,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_u8() {
        let callback_num_pres_ind = CallbackNumPresInd {
            presentation: Presentation::PresentationRestricted,
            screening: Screening::VerifiedAndFailed,
        };

        assert_eq!(u8::from(callback_num_pres_ind), 0b00001001);
    }

    #[test]
    fn from_u8() {
        let callback_num_pres_ind = CallbackNumPresInd::from(0b00001001);

        assert_eq!(
            callback_num_pres_ind,
            CallbackNumPresInd {
                presentation: Presentation::PresentationRestricted,
                screening: Screening::VerifiedAndFailed,
            }
        );
    }

    #[test]
    fn encode_decode() {
        crate::tests::encode_decode_test_instances::<CallbackNumPresInd>();
        crate::tests::encode_decode_test_instances::<Presentation>();
        crate::tests::encode_decode_test_instances::<Screening>();
    }
}
