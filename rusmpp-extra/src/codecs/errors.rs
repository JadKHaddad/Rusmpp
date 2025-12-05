//! Errors related to character encoding.

/// Character that cannot be encoded in the target encoding.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
#[error("Character '{character}' cannot be encoded in target encoding.")]
pub struct UnencodableCharacterError {
    /// The unencodable character.
    pub character: char,
}

impl UnencodableCharacterError {
    /// Creates a new [`UnencodableCharacterError`] error.
    pub(crate) const fn new(character: char) -> Self {
        Self { character }
    }
}
