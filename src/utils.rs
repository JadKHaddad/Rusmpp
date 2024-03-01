/// A simple struct to print the hex representation of a byte slice.
pub struct HexFormatter<'a>(pub &'a [u8]);

impl std::fmt::Debug for HexFormatter<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // Produces: [0x00, 0x00, 0x00, 0x6F]
        // write!(f, "[")?;
        // for (i, byte) in self.0.iter().enumerate() {
        //     write!(f, "0x{byte:02X?}")?;
        //     if i < self.0.len() - 1 {
        //         write!(f, ", ")?;
        //     }
        // }
        // write!(f, "]")?;
        // Ok(())

        // Produces: [00, 00, 00, 6F]
        write!(f, "{:02X?}", self.0)
    }
}
