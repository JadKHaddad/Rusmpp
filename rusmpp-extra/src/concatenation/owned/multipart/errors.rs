#[derive(Debug, thiserror::Error)]
pub enum MultipartError<E, C> {
    #[error("Encode error: {0}")]
    Encode(E),
    #[error("Concatenation error: {0}")]
    Concatenation(C),
}

impl<E, C> MultipartError<E, C> {
    pub(crate) const fn encode(error: E) -> Self {
        Self::Encode(error)
    }

    pub(crate) const fn concatenation(error: C) -> Self {
        Self::Concatenation(error)
    }
}
