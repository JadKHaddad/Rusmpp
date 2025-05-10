// // TODO: delete this file
// use crate::{
//     ende::{
//         decode::{Decode2, DecodeError2, DecodeWithLength2},
//         encode::Encode2,
//         length::Length,
//     },
//     types::{AnyOctetString, COctetString, EmptyOrFullCOctetString, OctetString},
// };

// #[derive(Debug)]
// struct A {
//     b_size: u32,
//     b: AnyOctetString,
//     c: u8,
//     d: u16,
//     e: u32,
//     c_octet: COctetString<1, 16>,
//     emp: EmptyOrFullCOctetString<6>,
//     octet_string_size: u32,
//     octet_string: OctetString<0, 13>,
// }

// impl Length for A {
//     fn length(&self) -> usize {
//         self.b_size.length()
//             + self.b.length()
//             + self.c.length()
//             + self.d.length()
//             + self.e.length()
//             + self.c_octet.length()
//             + self.emp.length()
//             + self.octet_string_size.length()
//             + self.octet_string.length()
//     }
// }

// impl crate::ende::encode::Encode2 for A {
//     fn encode(&self, dst: &mut [u8]) -> usize {
//         let size = 0;

//         let size = self.b_size.encode_move(dst, size);
//         let size = self.b.encode_move(dst, size);
//         let size = self.c.encode_move(dst, size);
//         let size = self.d.encode_move(dst, size);
//         let size = self.e.encode_move(dst, size);
//         let size = self.c_octet.encode_move(dst, size);
//         let size = self.emp.encode_move(dst, size);
//         let size = self.octet_string_size.encode_move(dst, size);
//         let size = self.octet_string.encode_move(dst, size);

//         size
//     }
// }

// impl Decode2 for A {
//     fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError2> {
//         let size = 0;

//         let (b_size, size) = Decode2::decode_move(src, size)?;
//         let (b, size) = DecodeWithLength2::decode_move(src, b_size as usize, size)?;
//         let (c, size) = Decode2::decode_move(src, size)?;
//         let (d, size) = Decode2::decode_move(src, size)?;
//         let (e, size) = Decode2::decode_move(src, size)?;
//         let (c_octet, size) = Decode2::decode_move(src, size)?;
//         let (emp, size) = Decode2::decode_move(src, size)?;
//         let (octet_string_size, size) = Decode2::decode_move(src, size)?;
//         let (octet_string, size) =
//             DecodeWithLength2::decode_move(src, octet_string_size as usize, size)?;

//         Ok((
//             A {
//                 b_size,
//                 b,
//                 c,
//                 d,
//                 e,
//                 c_octet,
//                 emp,
//                 octet_string_size,
//                 octet_string,
//             },
//             size,
//         ))
//     }
// }

// #[test]
// fn test() {
//     let a = A {
//         b_size: 2,
//         b: AnyOctetString::new(b"Hi"),
//         c: 1,
//         d: 15,
//         e: 256,
//         c_octet: COctetString::new(b"Hallo\0").unwrap(),
//         emp: EmptyOrFullCOctetString::new(b"Hello\0").unwrap(),
//         octet_string_size: 13,
//         octet_string: OctetString::<0, 13>::new(b"Hello\0World!\0").unwrap(),
//     };

//     let mut dst = vec![0; a.length()];

//     let size = a.encode(dst.as_mut_slice());

//     println!("{:?}", &dst[..size]);

//     println!();

//     let a = A::decode(dst.as_mut_slice());

//     println!("{a:?}");
// }

#![allow(dead_code)]
#![allow(clippy::disallowed_names)]

use crate::{
    decode::{Decode, DecodeError, DecodeWithKeyOptional, DecodeWithLength},
    types::AnyOctetString,
};

#[derive(Debug, PartialEq, Eq)]
enum Foo {
    A,
    B(u16),
    C(AnyOctetString),
}

impl DecodeWithKeyOptional for Foo {
    type Key = u32;

    fn decode(
        key: Self::Key,
        src: &[u8],
        length: usize,
    ) -> Result<Option<(Self, usize)>, DecodeError> {
        if length == 0 {
            match key {
                0x00000000 => return Ok(Some((Foo::A, 0))),
                _ => return Ok(None),
            }
        }

        match key {
            0x01020304 => {
                let (a, size) = u16::decode(src)?;

                Ok(Some((Foo::B(a), size)))
            }
            0x04030201 => {
                let (b, size) = AnyOctetString::decode(src, length)?;

                Ok(Some((Foo::C(b), size)))
            }
            _ => Err(DecodeError::UnsupportedKey { key }),
        }
    }
}

#[test]
fn foo() {
    // Received over the wire
    let length = 4;

    // Key is A
    let buf = &[
        0x00, 0x00, 0x00, 0x00, // Key
        0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
    ];

    let index = 0;

    let (key, size) = u32::decode(buf).unwrap();
    let index = index + size;

    let (foo, size) = Foo::decode(key, &buf[index..], length - index)
        .unwrap()
        .unwrap();
    let index = index + size;

    let expected = Foo::A;

    assert_eq!(size, 0);
    assert_eq!(foo, expected);
    assert_eq!(&buf[index..], &[0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);

    // Received over the wire
    let length = 4;

    // Key is B, but the received length indicates no value
    let buf = &[
        0x01, 0x02, 0x03, 0x04, // Key
        0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
    ];

    let index = 0;

    let (key, size) = u32::decode(buf).unwrap();
    let index = index + size;

    let value = Foo::decode(key, &buf[index..], length - index).unwrap();

    assert!(value.is_none());
    assert_eq!(&buf[index..], &[0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B]);

    // Received over the wire
    let length = 8;

    // Key is B
    let buf = &[
        0x01, 0x02, 0x03, 0x04, // Key
        0x05, 0x06, // Value
        0x07, 0x08, 0x09, 0x0A, 0x0B, // Rest
    ];

    let index = 0;

    let (key, size) = u32::decode(buf).unwrap();
    let index = index + size;

    let (foo, size) = Foo::decode(key, &buf[index..], length - index)
        .unwrap()
        .unwrap();
    let index = index + size;

    let expected = Foo::B(0x0506);

    assert_eq!(size, 2);
    assert_eq!(foo, expected);
    assert_eq!(&buf[index..], &[0x07, 0x08, 0x09, 0x0A, 0x0B]);

    // Received over the wire
    let length = 8;

    // Key is C
    let buf = &[
        0x04, 0x03, 0x02, 0x01, // Key
        0x05, 0x06, 0x07, 0x08, // Value
        0x09, 0x0A, 0x0B, // Rest
    ];

    let index = 0;

    let (key, size) = u32::decode(buf).unwrap();
    let index = index + size;

    let (foo, size) = Foo::decode(key, &buf[index..], length - index)
        .unwrap()
        .unwrap();
    let index = index + size;

    let expected = Foo::C(AnyOctetString::new([0x05, 0x06, 0x07, 0x08]));

    assert_eq!(size, 4);
    assert_eq!(foo, expected);
    assert_eq!(&buf[index..], &[0x09, 0x0A, 0x0B]);
}

// impl Decode for Foo {
//     fn decode(src: &[u8]) -> Result<(Self, usize), DecodeError> {
//         let index = 0;

//         let (a, size) = u8::decode(&src[index..])?;
//         let index = index + size;

//         let (b, size) = u16::decode(&src[index..])?;
//         let index = index + size;

//         let (c, size) = u32::decode(&src[index..])?;
//         let index = index + size;

//         Ok((Foo { a, b, c }, index))
//     }
// }

// fn foo() {
//     let buf = &[0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08];

//     let expected = Foo {
//         a: 0x01,
//         b: 0x0203,
//         c: 0x04050607,
//     };

//     let (foo, size) = Foo::decode(buf).unwrap();

//     assert_eq!(size, 7);
//     assert_eq!(foo, expected);
//     assert_eq!(&buf[size..], &[0x08]);
// }
