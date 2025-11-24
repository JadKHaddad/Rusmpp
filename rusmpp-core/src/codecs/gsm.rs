use crate::sealed::Sealed;

/// Errors that can occur during GSM 7-bit unpacked encoding.
#[derive(Debug)]
#[non_exhaustive]
pub enum Gsm7EncodeError {
    /// UTF-8 error that occurred during encoding.
    Utf8(core::str::Utf8Error),
    /// Character that cannot be encoded in GSM 7-bit.
    Encode(char),
}

impl core::fmt::Display for Gsm7EncodeError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Gsm7EncodeError::Utf8(err) => write!(f, "UTF-8 error: {err}"),
            Gsm7EncodeError::Encode(ch) => {
                write!(
                    f,
                    "Input contains a character that cannot be encoded in GSM 7-bit: {ch:?}"
                )
            }
        }
    }
}

impl core::error::Error for Gsm7EncodeError {}

/// GSM 7-bit encoding and decoding.
#[non_exhaustive]
#[derive(Debug, Clone)]
pub struct Gsm7UnpackedCodec;

impl Default for Gsm7UnpackedCodec {
    fn default() -> Self {
        Self::new()
    }
}

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

impl Sealed for Gsm7UnpackedCodec {}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl super::owned::Encoder<&[u8]> for Gsm7UnpackedCodec {
    type Error = Gsm7EncodeError;

    fn encode(&self, value: &[u8]) -> Result<alloc::vec::Vec<u8>, Self::Error> {
        let value = core::str::from_utf8(value).map_err(Gsm7EncodeError::Utf8)?;

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
                None => return Err(Gsm7EncodeError::Encode(ch)),
            }
        }

        result.truncate(result.len());

        Ok(result)
    }

    fn data_coding(&self) -> crate::values::DataCoding {
        crate::values::DataCoding::McSpecific
    }

    fn tolerance(&self) -> usize {
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

/// GSM 7-bit packed encoding and decoding.
///
/// # GSM 03.38 §6.1.2.3.1
///
/// ## Packing of 7 bits
///
/// Packing of 7 bits characters in USSD strings are done in the same way as for SMS (chapter 6.1.2.1).The
/// character stream is bit padded to octet boundary with binary zeroes.
///
/// ## Note
/// When multiple of 7 octets are sent the receiving entity has no knowledge weather the
/// last 7 bits are padding bits or is an Internet "at" character.
///
/// The sending entity may optional add an "end of transmission" character after the last USSD character and
/// then bitpadding to octet boundary before transmission in order to avoid the problem.
///
/// The CR character defined in the default alphabet in chapter 6.2 shall be used as the "end of transmission"
/// sign.
///
/// Old mobiles will perform carriage return after displaying the last USSD character received.
#[non_exhaustive]
#[derive(Debug)]
pub struct Gsm7PackedCodec {
    inner: Gsm7UnpackedCodec,
    end_of_transmission: bool,
}

impl Default for Gsm7PackedCodec {
    fn default() -> Self {
        Self::new()
    }
}

impl Gsm7PackedCodec {
    /// Creates a new [`Gsm7PackedCodec`] encoder/decoder.
    ///
    /// # Defaults
    ///
    /// - `end_of_transmission`: `false`
    pub const fn new() -> Self {
        Gsm7PackedCodec {
            inner: Gsm7UnpackedCodec::new(),
            end_of_transmission: false,
        }
    }

    /// Returns whether end-of-transmission character (0x1A) padding is enabled.
    pub fn end_of_transmission(&self) -> bool {
        self.end_of_transmission
    }

    /// Sets whether to add end-of-transmission character (0x1A) padding.
    pub fn with_end_of_transmission(mut self, enable: bool) -> Self {
        self.end_of_transmission = enable;
        self
    }
}

impl Sealed for Gsm7PackedCodec {}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
impl super::owned::Encoder<&[u8]> for Gsm7PackedCodec {
    type Error = Gsm7EncodeError;

    fn encode(&self, value: &[u8]) -> Result<alloc::vec::Vec<u8>, Self::Error> {
        let septets = self.inner.encode(value)?;

        Ok(self.pack(&septets))
    }

    fn data_coding(&self) -> crate::values::DataCoding {
        crate::values::DataCoding::McSpecific
    }

    fn tolerance(&self) -> usize {
        1 // reserve 1 byte for escape character split prevention
    }
}

impl Gsm7PackedCodec {
    #[cfg(any(test, feature = "alloc"))]
    #[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
    fn pack(&self, septets: &[u8]) -> alloc::vec::Vec<u8> {
        let n_octets = (septets.len() * 7).div_ceil(8);
        let mut packed = alloc::vec::Vec::with_capacity(n_octets);

        let mut carry: u16 = 0;
        let mut carry_bits: u8 = 0;

        for &septet in septets {
            carry |= (septet as u16) << carry_bits;
            carry_bits += 7;

            while carry_bits >= 8 {
                packed.push((carry & 0xFF) as u8);
                carry >>= 8;
                carry_bits -= 8;
            }
        }

        if carry_bits > 0 {
            packed.push(carry as u8);
        }

        if self.end_of_transmission {
            let total_bits = septets.len() * 7;

            if total_bits % 8 == 1 {
                if let Some(last) = packed.last_mut() {
                    if *last == 0x00 || *last == 0x01 {
                        *last |= 0x0D << 1; // insert CR << 1 = 0x1A
                    }
                }
            }
        }

        packed
    }
}

#[cfg(test)]
mod tests;
