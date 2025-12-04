use rusmpp_core::types::owned::OctetString;

/// Represents either a single encoded message or an iterator over concatenated message parts.
#[derive(Debug)]
pub enum Concatenation<Iter: Iterator<Item = OctetString<0, 255>>> {
    /// A single encoded message.
    Single(OctetString<0, 255>),
    /// An iterator over concatenated message parts.
    Concatenated(Iter),
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
}
