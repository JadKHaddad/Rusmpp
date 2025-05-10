pub trait Length {
    fn length(&self) -> usize;
}

pub trait Encode: Length {
    /// Encode a value to a slice
    ///
    /// Implementors are allowed to panic if the slice is not big enough to hold the encoded value. If `dst.len()` < [`Length::length`]
    fn encode(&self, dst: &mut [u8]) -> usize;
}

pub(crate) trait EncodeExt: Encode {
    fn encode_move(&self, dst: &mut [u8], size: usize) -> usize {
        size + self.encode(&mut dst[size..])
    }
}

impl<T: Encode> EncodeExt for T {}

const _: () = {
    impl<T: Length> Length for Vec<T> {
        fn length(&self) -> usize {
            self.iter().map(Length::length).sum()
        }
    }

    impl<T: Length> Length for Option<T> {
        fn length(&self) -> usize {
            match self {
                Some(value) => value.length(),
                None => 0,
            }
        }
    }

    impl<T: Encode> Encode for Option<T> {
        fn encode(&self, dst: &mut [u8]) -> usize {
            match self {
                Some(value) => value.encode(dst),
                None => 0,
            }
        }
    }

    impl<T: Encode> Encode for Vec<T> {
        fn encode(&self, dst: &mut [u8]) -> usize {
            let mut size = 0;

            for item in self {
                size += item.encode_move(dst, size);
            }

            size
        }
    }
};
