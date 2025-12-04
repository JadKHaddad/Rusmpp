/// Character that cannot be encoded in the target encoding.
#[derive(Debug, thiserror::Error)]
#[error("Character '{character}' cannot be encoded in target encoding.")]
pub struct UnencodableCharacterError {
    /// The unencodable character.
    pub character: char,
}

impl UnencodableCharacterError {
    /// Creates a new [`UnencodableCharacter`] error.
    pub(crate) const fn new(character: char) -> Self {
        Self { character }
    }
}
