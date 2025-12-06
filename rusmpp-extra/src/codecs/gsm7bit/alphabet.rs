mod default;

pub use default::Gsm7BitDefaultAlphabet;

/// GSM 7-bit alphabet.
#[derive(Debug, Clone, Copy)]
#[non_exhaustive]
pub enum Gsm7BitAlphabet {
    /// Default GSM 7-bit alphabet.
    Default(Gsm7BitDefaultAlphabet),
}

impl Default for Gsm7BitAlphabet {
    fn default() -> Self {
        Self::default()
    }
}

impl Gsm7BitAlphabet {
    pub const fn default() -> Self {
        Self::Default(Gsm7BitDefaultAlphabet::new())
    }

    /// Encodes the given character into GSM 7-bit encoded byte.
    ///
    /// # Returns
    ///
    /// - `Some(Encoded)` if the character is found in the GSM 7-bit tables.
    /// - `None` if the character is not found.
    const fn encode(&self, ch: char) -> Option<Encoded> {
        match self {
            Self::Default(alphabet) => alphabet.encode(ch),
        }
    }

    /// Decodes the given GSM 7-bit encoded byte into a character.
    ///
    /// # Returns
    ///
    /// - `Some(char)` if the byte is found in the GSM 7-bit tables.
    /// - `None` if the byte is not found.
    const fn decode(&self, byte: Encoded) -> Option<char> {
        match self {
            Self::Default(alphabet) => alphabet.decode(byte),
        }
    }
}

#[cfg(any(test, feature = "alloc"))]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
mod impl_owned {
    use core::ops::ControlFlow;

    use alloc::string::String;

    use super::*;

    impl Gsm7BitAlphabet {
        /// Encodes the given message into a vector of GSM 7-bit encoded bytes.
        ///
        /// # Errors
        ///
        /// - Returns `Err(char)` if a character in the message cannot be encoded.
        pub(crate) fn encode_to_vec(&self, message: &str) -> Result<alloc::vec::Vec<u8>, char> {
            // We double the amount of `bytes` we have in the worst case.
            //
            // If the amount of `bytes` is equals to the amount of `chars`
            //      (str = `[[[`, chars = [`[`, `[`, `[`] bytes = `[[[`)
            //      => we have 6 bytes of space, which is enough for standard/extended chars.
            //
            // If the amount of `bytes` is more than the amount of `chars`
            //      (str = `Ä`, , chars = [`Ä`], bytes = [195, 132])
            //      => we have 4 bytes of space, which is enough for standard/extended chars.
            let mut encoded = alloc::vec::Vec::with_capacity(message.len() * 2);

            for ch in message.chars() {
                match self.encode(ch) {
                    Some(Encoded::Standard(byte)) => encoded.push(byte),
                    Some(Encoded::Extended(byte)) => {
                        encoded.push(0x1B);
                        encoded.push(byte);
                    }
                    None => return Err(ch),
                }
            }

            encoded.truncate(encoded.len());

            Ok(encoded)
        }

        // TODO: docs
        pub(crate) fn decode_to_string(&self, bytes: &[u8]) -> Result<(String, Option<u8>), u8> {
            let decoded = String::with_capacity(bytes.len());

            self.decode_into_string(bytes, decoded)
        }

        // TODO: docs
        fn decode_into_string(
            &self,
            bytes: &[u8],
            mut decoded: String,
        ) -> Result<(String, Option<u8>), u8> {
            let mut i = 0;

            while i < bytes.len() {
                let byte = bytes[i];

                if byte == 0x1B {
                    i += 1;

                    if i >= bytes.len() {
                        return Ok((decoded, Some(0x1B)));
                    }

                    let byte = bytes[i];

                    match self.decode(Encoded::Extended(byte)) {
                        Some(ch) => decoded.push(ch),
                        None => return Err(byte),
                    }
                } else {
                    match self.decode(Encoded::Standard(byte)) {
                        Some(ch) => decoded.push(ch),
                        None => return Err(byte),
                    }
                }

                i += 1;
            }

            Ok((decoded, None))
        }

        // TODO: docs
        pub(crate) fn step_decode_to_string(
            &self,
            input: ControlFlow<(String, Option<u8>), (&[u8], String, Option<u8>)>,
        ) -> Result<(String, Option<u8>), u8> {
            match input {
                ControlFlow::Break((decoded, escape)) => Ok((decoded, escape)),
                ControlFlow::Continue((bytes, mut decoded, escape)) => match escape {
                    Some(escape) => {
                        if bytes.is_empty() {
                            return Ok((decoded, Some(escape)));
                        }

                        let escaped = [escape, bytes[0]];

                        // We do not care about the escape here, because 2 escapes are invalid.
                        let (part_decoded, _) = self.decode_to_string(&escaped)?;

                        decoded.push_str(&part_decoded);

                        // Continue decoding the rest of the bytes
                        let (decoded, escape) = self.decode_into_string(&bytes[1..], decoded)?;

                        Ok((decoded, escape))
                    }
                    None => {
                        let (part_decoded, escape) = self.decode_to_string(bytes)?;

                        decoded.push_str(&part_decoded);

                        Ok((decoded, escape))
                    }
                },
            }
        }
    }
}

/// Encoded GSM 7-bit character.
#[derive(Debug, Clone, Copy)]
pub enum Encoded {
    /// Standard GSM 7-bit character.
    Standard(u8),
    /// Extended GSM 7-bit character.
    ///
    /// Requires the escape character `0x1B` before the value.
    Extended(u8),
}
