pub(crate) trait Length {
    /// Returns the length of the encoded data in bytes.
    fn length(&self) -> usize;
}
