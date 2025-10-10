# Rusmpp-Macros

![Build Status](https://github.com/JadKHaddad/Rusmpp/actions/workflows/build-and-test.yml/badge.svg)
[![crates.io](https://img.shields.io/crates/v/rusmpp-macros.svg)](https://crates.io/crates/rusmpp-macros)
[![Crates.io (MSRV)](https://img.shields.io/crates/msrv/rusmpp-macros)](https://crates.io/crates/rusmpp-macros)
[![docs.rs](https://docs.rs/rusmpp/badge.svg)](https://docs.rs/rusmpp-macros)
[![Crates.io (Downloads)](https://img.shields.io/crates/d/rusmpp-macros)](https://crates.io/crates/rusmpp-macros)
[![Crates.io (License)](https://img.shields.io/crates/l/rusmpp-macros)](https://crates.io/crates/rusmpp-macros)

Procedural macros for `rusmpp-core`. Used to derive traits defined in `rusmpp-core` and implement boilerplate code for `SMPP` types.

## Note

This crate assumes that the traits are defined in the crate itself under the modules `decode`, `encode` and `tests`.

You should not depend on this crate directly as it is strongly coupled with `rusmpp-core`.

## Example

```rust
#[derive(Rusmpp)]
#[rusmpp(decode = owned)]
pub struct Command {
    id: CommandId,
    pub status: CommandStatus,
    pub sequence_number: u32,
    #[rusmpp(key = id, length = "unchecked")]
    pdu: Option<Pdu>,
}
```

This will expand to:

```rust
#[derive(Debug)]
pub struct CommandParts {
    pub id: CommandId,
    pub status: CommandStatus,
    pub sequence_number: u32,
    pub pdu: Option<Pdu>,
}

impl CommandParts {
    #[inline]
    #[allow(clippy::too_many_arguments)]
    pub const fn new(
        id: CommandId,
        status: CommandStatus,
        sequence_number: u32,
        pdu: Option<Pdu>,
    ) -> Self {
        Self {
            id,
            status,
            sequence_number,
            pdu,
        }
    }
    #[inline]
    #[allow(unused_parens)]
    pub fn raw(self) -> (CommandId, CommandStatus, u32, Option<Pdu>) {
        (self.id, self.status, self.sequence_number, self.pdu)
    }
}

impl Command {
    #[inline]
    pub fn into_parts(self) -> CommandParts {
        CommandParts {
            id: self.id,
            status: self.status,
            sequence_number: self.sequence_number,
            pdu: self.pdu,
        }
    }
}

impl crate::encode::Length for Command {
    fn length(&self) -> usize {
        let mut length = 0;
        length += crate::encode::Length::length(&self.id);
        length += crate::encode::Length::length(&self.status);
        length += crate::encode::Length::length(&self.sequence_number);
        length += crate::encode::Length::length(&self.pdu);
        length
    }
}

impl crate::encode::Encode for Command {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = crate::encode::EncodeExt::encode_move(&self.id, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.status, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.sequence_number, dst, size);
        let size = crate::encode::EncodeExt::encode_move(&self.pdu, dst, size);
        size
    }
}

impl crate::decode::owned::DecodeWithLength for Command {
    fn decode(src: &[u8], length: usize) -> Result<(Self, usize), crate::decode::DecodeError> {
        let size = 0;
        let (id, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::owned::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::id,
        )?;
        let (status, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::owned::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::status,
        )?;
        let (sequence_number, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::owned::DecodeExt::decode_move(src, size),
            crate::fields::SmppField::sequence_number,
        )?;
        let (pdu, size) = crate::decode::DecodeErrorExt::map_as_source(
            crate::decode::owned::DecodeWithKeyOptionalExt::decode_move(
                id,
                src,
                length.saturating_sub(size),
                size,
            ),
            crate::fields::SmppField::pdu,
        )?
        .map(|(this, size)| (Some(this), size))
        .unwrap_or((None, size));
        Ok((
            Self {
                id,
                status,
                sequence_number,
                pdu,
            },
            size,
        ))
    }
}
```

Notice the `crate` prefix. This is because the macro assumes that the traits are defined in the same crate.

Macro attributes are documented in the macro code itself.

## License

Licensed under either of

- Apache License, Version 2.0. [LICENSE-APACHE](../LICENSE-APACHE) or [Apache-2.0 license](http://apache.org/licenses/LICENSE-2.0)
- MIT license. [LICENSE-MIT](../LICENSE-MIT) or [MIT license](http://opensource.org/licenses/MIT)

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
