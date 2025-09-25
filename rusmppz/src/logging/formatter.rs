//! Simple formatter for byte slices.

/// A simple struct for debugging a byte slice.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Formatter<'a>(pub &'a [u8]);

impl Formatter<'_> {
    /// Produces: [0x00, 0x00, 0x00, 0x6F]
    #[cfg(all(feature = "pretty-hex-fmt", not(feature = "char-fmt")))]
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

    /// Produces: ['0', '0', '0', 'o']
    #[cfg(all(feature = "char-fmt", not(feature = "pretty-hex-fmt")))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        if self.0.is_empty() {
            write!(f, "[]")?;

            return Ok(());
        }

        if self.0.len() == 1 {
            write!(f, "[{:?}]", self.0[0] as char)?;

            return Ok(());
        }

        write!(f, "[")?;
        for i in 0..self.0.len() - 1 {
            write!(f, "{:?}, ", self.0[i] as char)?;
        }
        write!(f, "{:?}]", self.0[self.0.len() - 1] as char)?;

        Ok(())
    }

    /// Produces: [00, 00, 00, 6F]
    #[cfg(any(
        all(not(feature = "pretty-hex-fmt"), not(feature = "char-fmt")),
        all(feature = "pretty-hex-fmt", feature = "char-fmt")
    ))]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02X?}", self.0)
    }
}

impl core::fmt::Debug for Formatter<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.fmt(f)
    }
}
