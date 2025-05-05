/// A simple struct to print the hex representation of a byte slice.
pub struct HexFormatter<'a>(pub &'a [u8]);

impl HexFormatter<'_> {
    /// Produces: [0x00, 0x00, 0x00, 0x6F]
    #[cfg(feature = "pretty-hex-fmt")]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.is_empty() {
            write!(f, "[]")?;

            return Ok(());
        }

        if self.0.len() == 1 {
            write!(f, "[0x{:02X?}]", self.0[0])?;

            return Ok(());
        }

        write!(f, "[")?;
        for i in 0..self.0.len() - 1 {
            write!(f, "0x{:02X?}, ", self.0[i])?;
        }
        write!(f, "0x{:02X}]", self.0[self.0.len() - 1])?;

        Ok(())
    }

    /// Produces: [00, 00, 00, 6F]
    #[cfg(not(feature = "pretty-hex-fmt"))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02X?}", self.0)
    }
}

impl core::fmt::Debug for HexFormatter<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.fmt(f)
    }
}
