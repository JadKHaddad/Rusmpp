use proc_macro2::TokenStream;
use syn::{DeriveInput, Ident, parse};

use crate::{
    container_attributes::{DecodeAttributes, TestAttributes},
    repr::{Repr, ReprType},
};

pub fn derive_for_enum(input: &DeriveInput) -> syn::Result<TokenStream> {
    let enum_attrs = EnumAttributes::extract(input)?;

    Ok(enum_attrs.repr.quote(
        &input.ident,
        &enum_attrs.decode_attrs,
        &enum_attrs.test_attrs,
    ))
}

struct EnumAttributes {
    /// #[repr(u8)]
    repr: Repr,
    decode_attrs: DecodeAttributes,
    test_attrs: TestAttributes,
}

impl EnumAttributes {
    fn extract(input: &DeriveInput) -> syn::Result<Self> {
        let mut repr: Option<Repr> = None;
        let mut decode_attrs = DecodeAttributes::default();
        let mut test_attrs = TestAttributes::default();

        for attr in &input.attrs {
            if attr.path().is_ident("repr") {
                attr.parse_args_with(|input: parse::ParseStream| {
                    let ident: Ident = input.parse()?;

                    match ident.to_string().as_str() {
                        "u8" => repr = Some(Repr::new(ReprType::U8)),
                        "u16" => repr = Some(Repr::new(ReprType::U16)),
                        "u32" => repr = Some(Repr::new(ReprType::U32)),
                        other => {
                            return Err(syn::Error::new_spanned(
                                ident,
                                format!(
                                    "unsupported repr: {}, only `u8`, `u16`, and `u32` are allowed",
                                    other
                                ),
                            ));
                        }
                    }

                    Ok(())
                })?;
            } else if attr.path().is_ident("rusmpp") {
                attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("decode") {
                        decode_attrs = DecodeAttributes::extract(meta)?;
                    } else if meta.path.is_ident("test") {
                        test_attrs = TestAttributes::extract(meta)?;
                    }

                    Ok(())
                })?;
            }
        }

        let repr = repr.ok_or_else(|| {
            syn::Error::new_spanned(
                input,
                "enums must have a #[repr(...)] attribute, e.g. #[repr(u8)]",
            )
        })?;

        Ok(Self {
            repr,
            decode_attrs,
            test_attrs,
        })
    }
}
