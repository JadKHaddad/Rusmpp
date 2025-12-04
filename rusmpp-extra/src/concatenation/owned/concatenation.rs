use rusmpp_core::types::owned::OctetString;

/// Represents either a single encoded message or an iterator over concatenated message parts.
pub enum Concatenation<Iter> {
    /// A single encoded message.
    Single(OctetString<0, 255>),
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

impl<Iter: Iterator<Item = OctetString<0, 255>>> Concatenation<Iter> {
    /// Creates a new [`Concatenation::Single`] instance.
    pub(crate) const fn single(value: OctetString<0, 255>) -> Self {
        Self::Single(value)
    }

    /// Creates a new [`Concatenation::Concatenated`] instance.
    pub(crate) const fn concatenated(value: Iter) -> Self {
        Self::Concatenated(value)
    }

    #[cfg(test)]
    /// Converts the [`Concatenation`] into a vector of `OctetString<0, 255>`.
    pub(crate) fn into_vec(self) -> alloc::vec::Vec<OctetString<0, 255>> {
        match self {
            Self::Single(part) => alloc::vec![part],
            Self::Concatenated(iter) => iter.collect(),
        }
    }
}
