use crate::codecs::{
    owned::Encoder,
    ucs2::{Ucs2, Ucs2ConcatenateError, Ucs2EncodeError},
};

mod encode {
    use super::*;

    #[test]
    fn encode() {
        // TODO: cases and stuff
    }

    mod error {
        use super::*;

        #[test]
        fn unencodable_character() {
            let message = "Hi 😀";

            let encoder = Ucs2::new();

            let err = encoder.encode(message).unwrap_err();

            assert!(matches!(err, Ucs2EncodeError::UnencodableCharacter))
        }
    }
}

mod concatenate {
    use super::*;

    mod error {

        use crate::concatenation::owned::Concatenator;

        use super::*;

        // We have to concatenate but the part size was zero
        #[test]
        fn zero_part_size() {
            let message = "1234567";
            let max_message_size = 6;
            let part_header_size = 6;

            let encoder = Ucs2::new();

            let err = encoder
                .concatenate(message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(err, Ucs2ConcatenateError::PartCapacityExceeded))
        }

        #[test]
        fn zero_message_size() {
            let message = "1234567";
            let max_message_size = 0;
            let part_header_size = 6;

            let encoder = Ucs2::new();

            let err = encoder
                .concatenate(message, max_message_size, part_header_size)
                .unwrap_err();

            assert!(matches!(err, Ucs2ConcatenateError::PartCapacityExceeded))
        }

        #[test]
        fn parts_count_exceeded() {
            // TODO
        }

        mod no_split {
            use super::*;

            #[test]
            fn character_no_split() {
                // TODO
            }
        }

        mod split {
            use super::*;

            #[test]
            fn character_split() {
                // TODO
            }
        }
    }
}
