use crate::codecs::owned::Encoder;

use super::*;

mod unpacked {
    use super::*;

    #[test]
    fn encode() {
        // c-spell: disable
        let input = r##"Hello world!

@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà

^{}\[~]|€"##;
        // c-spell: enable

        let encoded = Gsm7UnpackedCodec::new()
            .encode(input.as_bytes())
            .expect("Encoding failed");

        #[rustfmt::skip]
            let expected: &[u8] = &[
                // "Hello world!\n\n"
                b'H', b'e', b'l', b'l', b'o', b' ', b'w', b'o', b'r', b'l', b'd', b'!', 0x0a, 0x0a,
                // 00–09, 0b–0c, 0e–0f
                0x00, 0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0b, 0x0c, 0x0e, 0x0f,
                // 10–1f
                0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1a, 0x1c, 0x1d, 0x1e,
                0x1f,
                // ASCII printable range 0x20–0x7f
                // !"#$%&'()*+,-./0123456789:;<=>?@
                b' ', b'!', b'"', b'#', b'$', b'%', b'&', b'\'', b'(', b')', b'*', b'+', b',', b'-',
                b'.', b'/', b'0', b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8', b'9', b':', b';',
                b'<', b'=', b'>', b'?', b'@', 
                // A–Z
                b'A', b'B', b'C', b'D', b'E', b'F', b'G', b'H', b'I', b'J', b'K', b'L', b'M', b'N',
                b'O', b'P', b'Q', b'R', b'S', b'T', b'U', b'V', b'W', b'X', b'Y', b'Z',
                // [\]^_`
                b'[', b'\\', b']', b'^', b'_', b'`', 
                // a–z
                b'a', b'b', b'c', b'd', b'e', b'f', b'g', b'h', b'i', b'j', b'k', b'l', b'm', b'n',
                b'o', b'p', b'q', b'r', b's', b't', b'u', b'v', b'w', b'x', b'y', b'z',
                // {|}~ 0x7f
                b'{', b'|', b'}', b'~', 0x7f, 
                // 0a 0a
                0x0a, 0x0a, 
                // 1b 14 1b ( 1b ) 1b / 1b < 1b = 1b > 1b @ 1b e
                0x1b, 0x14, 0x1b, b'(', 0x1b, b')', 0x1b, b'/', 0x1b, b'<', 0x1b, b'=', 0x1b, b'>',
                0x1b, b'@', 0x1b, b'e',
            ];

        assert_eq!(encoded.as_slice(), expected);
    }

    #[test]
    fn cases() {
        // c-spell: disable
        let cases: &[(&str, &[u8])] = &[
        ("", &[]),
        ("1", &[0x31]),
        ("12", &[0x31, 0x32]),
        ("123", &[0x31, 0x32, 0x33]),
        ("1234", &[0x31, 0x32, 0x33, 0x34]),
        ("12345", &[0x31, 0x32, 0x33, 0x34, 0x35]),
        ("123456", &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36]),
        ("1234567", &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37]),
        ("12345678", &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38]),
        ("123456789", &[0x31, 0x32, 0x33, 0x34, 0x35, 0x36, 0x37, 0x38, 0x39]),

        ("12345[6", &[0x31, 0x32, 0x33, 0x34, 0x35, 0x1B, 0x3C, 0x36]),

        (
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur nec nunc venenatis, ultricies ipsum id, volutpat ante. Sed pretium ac metus a interdum metus.",
            b"\x4C\x6F\x72\x65\x6D\x20\x69\x70\x73\x75\x6D\x20\x64\x6F\x6C\x6F\x72\x20\x73\x69\x74\x20\x61\x6D\x65\x74\x2C\x20\x63\x6F\x6E\x73\x65\x63\x74\x65\x74\x75\x72\x20\x61\x64\x69\x70\x69\x73\x63\x69\x6E\x67\x20\x65\x6C\x69\x74\x2E\x20\x43\x75\x72\x61\x62\x69\x74\x75\x72\x20\x6E\x65\x63\x20\x6E\x75\x6E\x63\x20\x76\x65\x6E\x65\x6E\x61\x74\x69\x73\x2C\x20\x75\x6C\x74\x72\x69\x63\x69\x65\x73\x20\x69\x70\x73\x75\x6D\x20\x69\x64\x2C\x20\x76\x6F\x6C\x75\x74\x70\x61\x74\x20\x61\x6E\x74\x65\x2E\x20\x53\x65\x64\x20\x70\x72\x65\x74\x69\x75\x6D\x20\x61\x63\x20\x6D\x65\x74\x75\x73\x20\x61\x20\x69\x6E\x74\x65\x72\x64\x75\x6D\x20\x6D\x65\x74\x75\x73\x2E",
        ),

        ("\n", &[0x0A]),
        ("\r", &[0x0D]),

        (
            "^{}\\[~]|€",
            &[
                0x1B, 0x14,
                0x1B, 0x28,
                0x1B, 0x29,
                0x1B, 0x2F,
                0x1B, 0x3C,
                0x1B, 0x3D,
                0x1B, 0x3E,
                0x1B, 0x40,
                0x1B, 0x65,
            ],
        ),

        (
            "@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !\"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà",
            b"\x00\x01\x02\x03\x04\x05\x06\x07\x08\x09\x0B\x0C\x0E\x0F\x10\x11\x12\x13\x14\x15\x16\x17\x18\x19\x1A\x1C\x1D\x1E\x1F\x20\x21\x22\x23\x24\x25\x26\x27\x28\x29\x2A\x2B\x2C\x2D\x2E\x2F\x30\x31\x32\x33\x34\x35\x36\x37\x38\x39\x3A\x3B\x3C\x3D\x3E\x3F\x40\x41\x42\x43\x44\x45\x46\x47\x48\x49\x4A\x4B\x4C\x4D\x4E\x4F\x50\x51\x52\x53\x54\x55\x56\x57\x58\x59\x5A\x5B\x5C\x5D\x5E\x5F\x60\x61\x62\x63\x64\x65\x66\x67\x68\x69\x6A\x6B\x6C\x6D\x6E\x6F\x70\x71\x72\x73\x74\x75\x76\x77\x78\x79\x7A\x7B\x7C\x7D\x7E\x7F",
        ),
    ];
        // c-spell: enable

        for (text, expected) in cases {
            let encoded = Gsm7UnpackedCodec::new()
                .encode(text.as_bytes())
                .expect("Encoding failed");

            assert_eq!(encoded, *expected, "Failed for text: {text:?}");
        }
    }
}

mod packed {
    use super::*;

    // c-spell: disable
    const CASES: &[(&str, &[u8])] = &[
        ("", &[]),
        ("1", &[0x31]),
        ("12", &[0x31, 0x19]),
        ("123", &[0x31, 0xD9, 0x0C]),
        ("1234", &[0x31, 0xD9, 0x8C, 0x06]),
        ("12345", &[0x31, 0xD9, 0x8C, 0x56, 0x03]),
        ("123456", &[0x31, 0xD9, 0x8C, 0x56, 0xB3, 0x01]),
        ("12345678", &[0x31, 0xD9, 0x8C, 0x56, 0xB3, 0xDD, 0x70]),
        ("123456789", &[0x31, 0xD9, 0x8C, 0x56, 0xB3, 0xDD, 0x70, 0x39]),

        (
            "Lorem ipsum dolor sit amet, consectetur adipiscing elit. Curabitur nec nunc venenatis, ultricies ipsum id, volutpat ante. Sed pretium ac metus a interdum metus.",
            b"\xCC\xB7\xBC\xDC\x06\xA5\xE1\xF3\x7A\x1B\x44\x7E\xB3\xDF\x72\xD0\x3C\x4D\x07\x85\xDB\x65\x3A\x0B\x34\x7E\xBB\xE7\xE5\x31\xBD\x4C\xAF\xCB\x41\x61\x72\x1A\x9E\x9E\x8F\xD3\xEE\x33\xA8\xCC\x4E\xD3\x5D\xA0\x61\x5D\x1E\x16\xA7\xE9\x75\x39\xC8\x5D\x1E\x83\xDC\x75\xF7\x18\x64\x2F\xBB\xCB\xEE\x30\x3D\x3D\x67\x81\xEA\x6C\xBA\x3C\x3D\x4E\x97\xE7\xA0\x34\x7C\x5E\x6F\x83\xD2\x64\x16\xC8\xFE\x66\xD7\xE9\xF0\x30\x1D\x14\x76\xD3\xCB\x2E\xD0\xB4\x4C\x06\xC1\xE5\x65\x7A\xBA\xDE\x06\x85\xC7\xA0\x76\x99\x5E\x9F\x83\xC2\xA0\xB4\x9B\x5E\x96\x93\xEB\x6D\x50\xBB\x4C\xAF\xCF\x5D",
        ),

        ("\n", &[0x0A]),
        ("\r", &[0x0D]),

        (
            "^{}\\[~]|€",
            &[
                0x1B, 0xCA, 0x06, 0xB5, 0x49, 0x6D, 0x5E, 0x1B,
                0xDE, 0xA6, 0xB7, 0xF1, 0x6D, 0x80, 0x9B, 0x32,
            ],
        ),

        (
            "@£$¥èéùìòÇØøÅåΔ_ΦΓΛΩΠΨΣΘΞÆæßÉ !\"#¤%&'()*+,-./0123456789:;<=>?¡ABCDEFGHIJKLMNOPQRSTUVWXYZÄÖÑÜ§¿abcdefghijklmnopqrstuvwxyzäöñüà",
            b"\x80\x80\x60\x40\x28\x18\x0E\x88\xC4\x82\xE1\x78\x40\x22\x92\x09\xA5\x62\xB9\x60\x32\x1A\x4E\xC7\xF3\x01\x85\x44\x23\x52\xC9\x74\x42\xA5\x54\x2B\x56\xCB\xF5\x82\xC5\x64\x33\x5A\xCD\x76\xC3\xE5\x74\x3B\x5E\xCF\xF7\x03\x06\x85\x43\x62\xD1\x78\x44\x26\x95\x4B\x66\xD3\xF9\x84\x46\xA5\x53\x6A\xD5\x7A\xC5\x66\xB5\x5B\x6E\xD7\xFB\x05\x87\xC5\x63\x72\xD9\x7C\x46\xA7\xD5\x6B\x76\xDB\xFD\x86\xC7\xE5\x73\x7A\xDD\x7E\xC7\xE7\xF5\x7B\x7E\xDF\xFF\x07",
        ),
    ];

    const END_OF_TRANSMISSION: &[(&str, &[u8])] =
        &[("1234567", &[0x31, 0xD9, 0x8C, 0x56, 0xB3, 0xDD, 0x1A])];

    const NO_END_OF_TRANSMISSION: &[(&str, &[u8])] =
        &[("1234567", &[0x31, 0xD9, 0x8C, 0x56, 0xB3, 0xDD, 0x00])];

    // c-spell: enable

    mod with_end_of_transmission {
        use super::*;

        #[test]
        fn cases() {
            let cases = CASES.iter().chain(END_OF_TRANSMISSION.iter());

            for (text, expected) in cases {
                let encoded = Gsm7PackedCodec::new()
                    .with_end_of_transmission(true)
                    .encode(text.as_bytes())
                    .expect("Encoding failed");

                assert_eq!(encoded, *expected, "Failed for text: {text:?}");
            }
        }
    }

    mod no_end_of_transmission {
        use super::*;

        #[test]
        fn cases() {
            let cases = CASES.iter().chain(NO_END_OF_TRANSMISSION.iter());

            for (text, expected) in cases {
                let encoded = Gsm7PackedCodec::new()
                    .with_end_of_transmission(false)
                    .encode(text.as_bytes())
                    .expect("Encoding failed");

                assert_eq!(encoded, *expected, "Failed for text: {text:?}");
            }
        }
    }
}
