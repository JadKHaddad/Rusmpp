use num_enum::{FromPrimitive, IntoPrimitive};
use rusmpp_macros::RusmppIoU8;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default, RusmppIoU8)]
pub struct CallbackNumPresInd {
    pub presentation: Presentation,
    pub screening: Screening,
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

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum Presentation {
    PresentationAllowed = 0b00000000,
    PresentationRestricted = 0b00000001,
    NumberNotAvailable = 0b00000010,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for Presentation {
    fn default() -> Self {
        Presentation::PresentationAllowed
    }
}

#[repr(u8)]
#[derive(
    Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, IntoPrimitive, FromPrimitive,
)]
pub enum Screening {
    NotScreened = 0b00000000,
    VerivfiedAndPassed = 0b00000100,
    VerivfiedAndFailed = 0b00001000,
    NetworkProvided = 0b00001100,
    #[num_enum(catch_all)]
    Other(u8),
}

#[allow(clippy::derivable_impls)]
impl Default for Screening {
    fn default() -> Self {
        Screening::NotScreened
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_u8() {
        let callback_num_pres_ind = CallbackNumPresInd {
            presentation: Presentation::PresentationRestricted,
            screening: Screening::VerivfiedAndFailed,
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
                screening: Screening::VerivfiedAndFailed,
            }
        );
    }
}
