use alloc::vec::Vec;

/// Represents either a single encoded message or an iterator over concatenated message parts.
pub enum Concatenation<Iter> {
    /// A single encoded message.
    Single(Vec<u8>),
    /// An iterator over concatenated message parts.
    Concatenated(Iter),
}

impl<Iter> core::fmt::Debug for Concatenation<Iter> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Single(value) => f.debug_tuple("Single").field(value).finish(),
            Self::Concatenated(_) => f.debug_tuple("Concatenated").field(&"...").finish(),
        }
    }
}

impl<Iter: Iterator<Item = Vec<u8>>> Concatenation<Iter> {
    /// Creates a new [`Concatenation::Single`] instance.
    pub(crate) const fn single(value: Vec<u8>) -> Self {
        Self::Single(value)
    }

    /// Creates a new [`Concatenation::Concatenated`] instance.
    pub(crate) const fn concatenated(value: Iter) -> Self {
        Self::Concatenated(value)
    }

    #[cfg(test)]
    /// Collects the [`Concatenation`] into a vector of `Vec<u8>`.
    pub(crate) fn collect(self) -> Vec<Vec<u8>> {
        match self {
            Self::Single(part) => alloc::vec![part],
            Self::Concatenated(iter) => iter.collect(),
        }
    }
}
