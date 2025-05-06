//! Mirror of the `std::io` module where `std` feature is disabled.

#[derive(Debug)]
pub enum Error {
    Eof,
}

impl core::fmt::Display for Error {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Error::Eof => write!(f, "EOF"),
        }
    }
}

impl core::error::Error for Error {}

pub trait Write {
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error>;

    fn write_all(&mut self, mut buf: &[u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            match self.write(buf) {
                Ok(0) => {
                    return Err(Error::Eof);
                }
                Ok(n) => buf = &buf[n..],
                Err(e) => return Err(e),
            }
        }
        Ok(())
    }
}

pub trait Read {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error>;

    fn read_exact(&mut self, mut buf: &mut [u8]) -> Result<(), Error> {
        while !buf.is_empty() {
            match self.read(buf) {
                Ok(0) => break,
                Ok(n) => {
                    buf = &mut buf[n..];
                }
                Err(e) => return Err(e),
            }
        }
        if !buf.is_empty() {
            Err(Error::Eof)
        } else {
            Ok(())
        }
    }

    fn bytes(self) -> Bytes<Self>
    where
        Self: Sized,
    {
        Bytes { inner: self }
    }
}

pub struct Bytes<R> {
    inner: R,
}

impl<R: Read> Iterator for Bytes<R> {
    type Item = Result<u8, Error>;

    fn next(&mut self) -> Option<Result<u8, Error>> {
        let mut byte = 0;

        match self.inner.read(core::slice::from_mut(&mut byte)) {
            Ok(0) => None,
            Ok(..) => Some(Ok(byte)),
            Err(e) => Some(Err(e)),
        }
    }
}

impl<T> Read for &mut T
where
    T: Read,
{
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        (*self).read(buf)
    }
}

impl Read for &[u8] {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, Error> {
        let amt = core::cmp::min(buf.len(), self.len());
        let (a, b) = self.split_at(amt);

        if amt == 1 {
            buf[0] = a[0];
        } else {
            buf[..amt].copy_from_slice(a);
        }

        *self = b;
        Ok(amt)
    }
}

impl Write for &mut [u8] {
    #[inline]
    fn write(&mut self, data: &[u8]) -> Result<usize, Error> {
        let amt = core::cmp::min(data.len(), self.len());
        let (a, b) = core::mem::take(self).split_at_mut(amt);
        a.copy_from_slice(&data[..amt]);
        *self = b;
        Ok(amt)
    }
}

// Used for Encode::encode_to_vec()
#[cfg(feature = "alloc")]
impl Write for ::alloc::vec::Vec<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> Result<usize, Error> {
        self.extend_from_slice(buf);

        Ok(buf.len())
    }
}

#[cfg(test)]
pub use std::io::Cursor;

#[cfg(test)]
mod impl_std {
    impl super::Write for ::std::io::Cursor<::std::vec::Vec<u8>> {
        fn write(&mut self, buf: &[u8]) -> Result<usize, super::Error> {
            Ok(::std::io::Write::write(self, buf).expect("std::io::Error"))
        }
    }

    impl super::Read for ::std::io::Cursor<::std::vec::Vec<u8>> {
        fn read(&mut self, buf: &mut [u8]) -> Result<usize, super::Error> {
            Ok(::std::io::Read::read(self, buf).expect("std::io::Error"))
        }
    }
}
