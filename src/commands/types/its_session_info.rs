use crate::{
    ende::decode::{Decode, DecodeError},
    impl_length_encode, tri,
};

impl_length_encode! {
    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub struct ItsSessionInfo {
        pub session_number: u8,
        pub sequence_number: u8,
    }
}

impl ItsSessionInfo {
    pub fn new(session_number: u8, sequence_number: u8) -> Self {
        Self {
            session_number,
            sequence_number,
        }
    }
}

impl Decode for ItsSessionInfo {
    fn decode_from<R: std::io::Read>(reader: &mut R) -> Result<Self, DecodeError>
    where
        Self: Sized,
    {
        let session_number = tri!(u8::decode_from(reader));
        let sequence_number = tri!(u8::decode_from(reader));

        Ok(Self {
            session_number,
            sequence_number,
        })
    }
}
