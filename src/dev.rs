use crate::{
    ende::{
        decode::{Decode2, DecodeError2, DecodeWithLength2},
        encode::Encode2,
        length::Length,
    },
    types::{AnyOctetString, COctetString, EmptyOrFullCOctetString, OctetString},
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
    octet_string_size: u32,
    octet_string: OctetString<0, 13>,
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
            + self.octet_string_size.length()
            + self.octet_string.length()
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
        let size = self.octet_string_size.encode_move(dst, size);
        let size = self.octet_string.encode_move(dst, size);

        size
    }
}

impl Decode2 for A {
    fn decode(src: &mut [u8]) -> Result<(Self, usize), DecodeError2> {
        let size = 0;

        let (b_size, size) = Decode2::decode_move(src, size)?;
        let (b, size) = DecodeWithLength2::decode_move(src, b_size as usize, size)?;
        let (c, size) = Decode2::decode_move(src, size)?;
        let (d, size) = Decode2::decode_move(src, size)?;
        let (e, size) = Decode2::decode_move(src, size)?;
        let (c_octet, size) = Decode2::decode_move(src, size)?;
        let (emp, size) = Decode2::decode_move(src, size)?;
        let (octet_string_size, size) = Decode2::decode_move(src, size)?;
        let (octet_string, size) =
            DecodeWithLength2::decode_move(src, octet_string_size as usize, size)?;

        Ok((
            A {
                b_size,
                b,
                c,
                d,
                e,
                c_octet,
                emp,
                octet_string_size,
                octet_string,
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
        octet_string_size: 13,
        octet_string: OctetString::<0, 13>::new(b"Hello\0World!\0").unwrap(),
    };

    let mut dst = vec![0; a.length()];

    let size = a.encode(dst.as_mut_slice());

    println!("{:?}", &dst[..size]);

    println!();

    let a = A::decode(dst.as_mut_slice());

    println!("{a:?}");
}
