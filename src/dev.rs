use crate::{
    ende::{
        decode::{Decode2, DecodeWithLength2},
        encode::Encode2,
        length::Length,
    },
    types::{AnyOctetString, COctetString, EmptyOrFullCOctetString},
};

#[derive(Debug)]
struct A {
    b_size: u32,
    b: AnyOctetString,
    c: u8,
    d: u16,
    e: u32,
    c_octet: COctetString<1, 16>,
    emp: EmptyOrFullCOctetString<6>,
}

impl Length for A {
    fn length(&self) -> usize {
        self.b_size.length()
            + self.b.length()
            + self.c.length()
            + self.d.length()
            + self.e.length()
            + self.c_octet.length()
            + self.emp.length()
    }
}

impl crate::ende::encode::Encode2 for A {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;

        let size = self.b_size.encode_move(dst, size);
        let size = self.b.encode_move(dst, size);
        let size = self.c.encode_move(dst, size);
        let size = self.d.encode_move(dst, size);
        let size = self.e.encode_move(dst, size);
        let size = self.c_octet.encode_move(dst, size);
        let size = self.emp.encode_move(dst, size);

        size
    }
}

impl crate::ende::decode::Decode2 for A {
    fn decode(src: &mut [u8]) -> Result<(Self, usize), crate::ende::decode::DecodeError2> {
        let size = 0;

        let (b_size, size) = u32::decode_move(src, size)?;
        let (b, size) = AnyOctetString::decode_move(src, b_size as usize, size)?;
        let (c, size) = u8::decode_move(src, size)?;
        let (d, size) = u16::decode_move(src, size)?;
        let (e, size) = u32::decode_move(src, size)?;
        let (c_octet, size) = COctetString::decode_move(src, size)?;
        let (emp, size) = EmptyOrFullCOctetString::decode_move(src, size)?;

        Ok((
            A {
                b_size,
                b,
                c,
                d,
                e,
                c_octet,
                emp,
            },
            size,
        ))
    }
}

#[test]
fn test() {
    let a = A {
        b_size: 2,
        b: AnyOctetString::new(b"Hi"),
        c: 1,
        d: 15,
        e: 256,
        c_octet: COctetString::new(b"Hallo\0").unwrap(),
        emp: EmptyOrFullCOctetString::new(b"Hello\0").unwrap(),
    };

    let mut dst = vec![0; a.length()];

    let size = a.encode(dst.as_mut_slice());

    println!("{:?}", &dst[..size]);

    println!();

    let a = A::decode(dst.as_mut_slice());

    println!("{a:?}");
}
