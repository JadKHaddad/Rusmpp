use crate::codecs::gsm::alphabet::Gsm7BitAlphabet;

#[non_exhaustive]
#[derive(Debug)]
pub struct Gsm7Bit {
    alphabet: Gsm7BitAlphabet,
}

impl Default for Gsm7Bit {
    fn default() -> Self {
        Self::new()
    }
}

impl Gsm7Bit {
    /// Creates a new [`Gsm7Bit`] codec with [`Gsm7BitAlphabet::Default`].
    pub const fn new() -> Self {
        Self {
            alphabet: Gsm7BitAlphabet::default(),
        }
    }

    pub const fn with_alphabet(mut self, alphabet: Gsm7BitAlphabet) -> Self {
        self.alphabet = alphabet;
        self
    }
}
