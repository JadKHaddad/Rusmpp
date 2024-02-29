/// A simple struct to print the hex representation of a byte slice.
pub struct BytesHexPrinter<'a>(pub &'a [u8]);

impl std::fmt::Debug for BytesHexPrinter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02x?}", self.0)
    }
}
