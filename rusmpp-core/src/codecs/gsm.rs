/// Errors that can occur during GSM 7-bit unpacked encoding.
#[derive(Debug)]
pub enum Gsm7UnpackedEncodeError {
    /// UTF-8 error that occurred during encoding.
    Utf8(core::str::Utf8Error),
    /// Character that cannot be encoded in GSM 7-bit.
    Encode(char),
}

impl core::fmt::Display for Gsm7UnpackedEncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Gsm7UnpackedEncodeError::Utf8(err) => write!(f, "UTF-8 error: {err}"),
            Gsm7UnpackedEncodeError::Encode(ch) => {
                write!(
                    f,
                    "Input contains a character that cannot be encoded in GSM 7-bit: {ch:?}"
                )
            }
        }
    }
}

impl core::error::Error for Gsm7UnpackedEncodeError {}

/// GSM 7-bit encoding and decoding.
#[non_exhaustive]
#[derive(Debug, Clone, Default)]
pub struct Gsm7UnpackedCodec;

impl Gsm7UnpackedCodec {
    /// Creates a new [`Gsm7UnpackedCodec`] encoder/decoder.
    pub const fn new() -> Self {
        Gsm7UnpackedCodec
    }

    /// Looks up the GSM 7-bit value for the given character.
    ///
    /// # Returns
    ///
    /// - `Some(Lookup)` if the character is found in the GSM 7-bit tables.
    /// - `None` if the character is not found.
    fn lookup(ch: char) -> Option<Lookup> {
        if let Some(&val) = lookup::TABLE.get(&ch) {
            Some(Lookup::Standard(val))
        } else if let Some(&val) = lookup::TABLE_EXTENDED.get(&ch) {
            Some(Lookup::Extended(val))
        } else {
            None
        }
    }
}

/// Lookup result for GSM 7-bit encoding.   
enum Lookup {
    /// Standard GSM 7-bit character.
    Standard(u8),
    /// Extended GSM 7-bit character.
    ///
    /// Requires the escape character `0x1B` before the value.
    Extended(u8),
}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl super::owned::SealedEncoder for Gsm7UnpackedCodec {}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl super::owned::Encoder<&[u8]> for Gsm7UnpackedCodec {
    type Error = Gsm7UnpackedEncodeError;

    fn encode(&self, value: &[u8]) -> Result<alloc::vec::Vec<u8>, Self::Error> {
        let value = core::str::from_utf8(value).map_err(Gsm7UnpackedEncodeError::Utf8)?;

        // We double the amount of `bytes` we have in the worst case.
        //
        // If the amount of `bytes` is equals to the amount of `chars`
        //      (str = `[[[`, chars = [`[`, `[`, `[`] bytes = `[[[`)
        //      => we have 6 bytes of space, which is enough for standard/extended chars.
        //
        // If the amount of `bytes` is more than the amount of `chars`
        //      (str = `Ä`, , chars = [`Ä`], bytes = [195, 132])
        //      => we have 4 bytes of space, which is enough for standard/extended chars.
        let mut result = alloc::vec::Vec::with_capacity(value.len() * 2);

        for ch in value.chars() {
            match Self::lookup(ch) {
                Some(Lookup::Standard(val)) => result.push(val),
                Some(Lookup::Extended(val)) => {
                    result.push(0x1B);
                    result.push(val);
                }
                None => return Err(Gsm7UnpackedEncodeError::Encode(ch)),
            }
        }

        result.truncate(result.len());

        Ok(result)
    }

    fn data_coding(&self) -> crate::values::DataCoding {
        crate::values::DataCoding::McSpecific
    }

    fn padding(&self) -> usize {
        // We reserve 1 byte to avoid an escape character being split between payloads
        1
    }
}

mod lookup {
    pub(super) static TABLE_EXTENDED: phf::Map<char, u8> = phf::phf_map! {
        '^' => 0x14,
        '{' => 0x28,
        '}' => 0x29,
        '\\' => 0x2F,
        '[' => 0x3C,
        '~' => 0x3D,
        ']' => 0x3E,
        '|' => 0x40,
        '€' => 0x65,
    };

    pub(super) static TABLE: phf::Map<char, u8> = phf::phf_map! {
        '@' => 0x00,
        '£' => 0x01,
        '$' => 0x02,
        '¥' => 0x03,
        'è' => 0x04,
        'é' => 0x05,
        'ù' => 0x06,
        'ì' => 0x07,
        'ò' => 0x08,
        'Ç' => 0x09,
        '\n' => 0x0a,
        'Ø' => 0x0b,
        'ø' => 0x0c,
        '\r' => 0x0d,
        'Å' => 0x0e,
        'å' => 0x0f,
        'Δ' => 0x10,
        '_' => 0x11,
        'Φ' => 0x12,
        'Γ' => 0x13,
        'Λ' => 0x14,
        'Ω' => 0x15,
        'Π' => 0x16,
        'Ψ' => 0x17,
        'Σ' => 0x18,
        'Θ' => 0x19,
        'Ξ' => 0x1A,
        '\u{001B}' => 0x1B,
        'Æ' => 0x1C,
        'æ' => 0x1D,
        'ß' => 0x1E,
        'É' => 0x1F,
        ' ' => 0x20,
        '!' => 0x21,
        '"' => 0x22,
        '#' => 0x23,
        '¤' => 0x24,
        '%' => 0x25,
        '&' => 0x26,
        '\'' => 0x27,
        '(' => 0x28,
        ')' => 0x29,
        '*' => 0x2A,
        '+' => 0x2B,
        ',' => 0x2C,
        '-' => 0x2D,
        '.' => 0x2E,
        '/' => 0x2F,
        '0' => 0x30,
        '1' => 0x31,
        '2' => 0x32,
        '3' => 0x33,
        '4' => 0x34,
        '5' => 0x35,
        '6' => 0x36,
        '7' => 0x37,
        '8' => 0x38,
        '9' => 0x39,
        ':' => 0x3A,
        ';' => 0x3B,
        '<' => 0x3C,
        '=' => 0x3D,
        '>' => 0x3E,
        '?' => 0x3F,
        '¡' => 0x40,
        'A' => 0x41,
        'B' => 0x42,
        'C' => 0x43,
        'D' => 0x44,
        'E' => 0x45,
        'F' => 0x46,
        'G' => 0x47,
        'H' => 0x48,
        'I' => 0x49,
        'J' => 0x4A,
        'K' => 0x4B,
        'L' => 0x4C,
        'M' => 0x4D,
        'N' => 0x4E,
        'O' => 0x4F,
        'P' => 0x50,
        'Q' => 0x51,
        'R' => 0x52,
        'S' => 0x53,
        'T' => 0x54,
        'U' => 0x55,
        'V' => 0x56,
        'W' => 0x57,
        'X' => 0x58,
        'Y' => 0x59,
        'Z' => 0x5A,
        'Ä' => 0x5B,
        'Ö' => 0x5C,
        'Ñ' => 0x5D,
        'Ü' => 0x5E,
        '§' => 0x5F,
        '¿' => 0x60,
        'a' => 0x61,
        'b' => 0x62,
        'c' => 0x63,
        'd' => 0x64,
        'e' => 0x65,
        'f' => 0x66,
        'g' => 0x67,
        'h' => 0x68,
        'i' => 0x69,
        'j' => 0x6A,
        'k' => 0x6B,
        'l' => 0x6C,
        'm' => 0x6D,
        'n' => 0x6E,
        'o' => 0x6F,
        'p' => 0x70,
        'q' => 0x71,
        'r' => 0x72,
        's' => 0x73,
        't' => 0x74,
        'u' => 0x75,
        'v' => 0x76,
        'w' => 0x77,
        'x' => 0x78,
        'y' => 0x79,
        'z' => 0x7A,
        'ä' => 0x7B,
        'ö' => 0x7C,
        'ñ' => 0x7D,
        'ü' => 0x7E,
        'à' => 0x7F,
    };
}

#[cfg(test)]
mod tests {
    use crate::codecs::owned::Encoder;

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
}
