use super::Encoded;

#[non_exhaustive]
#[derive(Debug, Clone, Copy)]
/// Default GSM 7-bit alphabet.
pub struct Gsm7BitDefaultAlphabet;

impl Default for Gsm7BitDefaultAlphabet {
    fn default() -> Self {
        Self::new()
    }
}

impl Gsm7BitDefaultAlphabet {
    /// Creates a new [`Gsm7BitDefaultAlphabet`].
    pub const fn new() -> Self {
        Self
    }
}

impl Gsm7BitDefaultAlphabet {
    /// Encodes the given character into GSM 7-bit encoded byte.
    ///
    /// # Returns
    ///
    /// - `Some(Self)` if the character is found in the GSM 7-bit tables.
    /// - `None` if the character is not found.
    pub(super) const fn encode(&self, ch: char) -> Option<Encoded> {
        if let Some(byte) = Standard::encode(ch) {
            Some(Encoded::Standard(byte))
        } else if let Some(byte) = Extended::encode(ch) {
            Some(Encoded::Extended(byte))
        } else {
            None
        }
    }

    /// Decodes the given GSM 7-bit encoded byte into a character.
    ///
    /// # Returns
    ///
    /// - `Some(char)` if the byte is found in the GSM 7-bit tables.
    /// - `None` if the byte is not found.
    pub(super) const fn decode(&self, byte: Encoded) -> Option<char> {
        match byte {
            Encoded::Standard(b) => Standard::decode(b),
            Encoded::Extended(b) => Extended::decode(b),
        }
    }
}

struct Standard;

struct Extended;

impl Standard {
    const fn encode(ch: char) -> Option<u8> {
        let byte = match ch {
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
            _ => return None,
        };

        Some(byte)
    }

    const fn decode(byte: u8) -> Option<char> {
        let ch = match byte {
            0x00 => '@',
            0x01 => '£',
            0x02 => '$',
            0x03 => '¥',
            0x04 => 'è',
            0x05 => 'é',
            0x06 => 'ù',
            0x07 => 'ì',
            0x08 => 'ò',
            0x09 => 'Ç',
            0x0A => '\n',
            0x0B => 'Ø',
            0x0C => 'ø',
            0x0D => '\r',
            0x0E => 'Å',
            0x0F => 'å',
            0x10 => 'Δ',
            0x11 => '_',
            0x12 => 'Φ',
            0x13 => 'Γ',
            0x14 => 'Λ',
            0x15 => 'Ω',
            0x16 => 'Π',
            0x17 => 'Ψ',
            0x18 => 'Σ',
            0x19 => 'Θ',
            0x1A => 'Ξ',
            0x1B => '\u{001B}',
            0x1C => 'Æ',
            0x1D => 'æ',
            0x1E => 'ß',
            0x1F => 'É',
            0x20 => ' ',
            0x21 => '!',
            0x22 => '"',
            0x23 => '#',
            0x24 => '¤',
            0x25 => '%',
            0x26 => '&',
            0x27 => '\'',
            0x28 => '(',
            0x29 => ')',
            0x2A => '*',
            0x2B => '+',
            0x2C => ',',
            0x2D => '-',
            0x2E => '.',
            0x2F => '/',
            0x30 => '0',
            0x31 => '1',
            0x32 => '2',
            0x33 => '3',
            0x34 => '4',
            0x35 => '5',
            0x36 => '6',
            0x37 => '7',
            0x38 => '8',
            0x39 => '9',
            0x3A => ':',
            0x3B => ';',
            0x3C => '<',
            0x3D => '=',
            0x3E => '>',
            0x3F => '?',
            0x40 => '¡',
            0x41 => 'A',
            0x42 => 'B',
            0x43 => 'C',
            0x44 => 'D',
            0x45 => 'E',
            0x46 => 'F',
            0x47 => 'G',
            0x48 => 'H',
            0x49 => 'I',
            0x4A => 'J',
            0x4B => 'K',
            0x4C => 'L',
            0x4D => 'M',
            0x4E => 'N',
            0x4F => 'O',
            0x50 => 'P',
            0x51 => 'Q',
            0x52 => 'R',
            0x53 => 'S',
            0x54 => 'T',
            0x55 => 'U',
            0x56 => 'V',
            0x57 => 'W',
            0x58 => 'X',
            0x59 => 'Y',
            0x5A => 'Z',
            0x5B => 'Ä',
            0x5C => 'Ö',
            0x5D => 'Ñ',
            0x5E => 'Ü',
            0x5F => '§',
            0x60 => '¿',
            0x61 => 'a',
            0x62 => 'b',
            0x63 => 'c',
            0x64 => 'd',
            0x65 => 'e',
            0x66 => 'f',
            0x67 => 'g',
            0x68 => 'h',
            0x69 => 'i',
            0x6A => 'j',
            0x6B => 'k',
            0x6C => 'l',
            0x6D => 'm',
            0x6E => 'n',
            0x6F => 'o',
            0x70 => 'p',
            0x71 => 'q',
            0x72 => 'r',
            0x73 => 's',
            0x74 => 't',
            0x75 => 'u',
            0x76 => 'v',
            0x77 => 'w',
            0x78 => 'x',
            0x79 => 'y',
            0x7A => 'z',
            0x7B => 'ä',
            0x7C => 'ö',
            0x7D => 'ñ',
            0x7E => 'ü',
            0x7F => 'à',
            _ => return None,
        };
        Some(ch)
    }
}

impl Extended {
    const fn encode(ch: char) -> Option<u8> {
        let byte = match ch {
            '^' => 0x14,
            '{' => 0x28,
            '}' => 0x29,
            '\\' => 0x2F,
            '[' => 0x3C,
            '~' => 0x3D,
            ']' => 0x3E,
            '|' => 0x40,
            '€' => 0x65,
            _ => return None,
        };

        Some(byte)
    }

    const fn decode(byte: u8) -> Option<char> {
        let ch = match byte {
            0x14 => '^',
            0x28 => '{',
            0x29 => '}',
            0x2F => '\\',
            0x3C => '[',
            0x3D => '~',
            0x3E => ']',
            0x40 => '|',
            0x65 => '€',
            _ => return None,
        };

        Some(ch)
    }
}
