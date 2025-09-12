use proc_macro2::TokenStream;
use quote::quote;
use syn::{DeriveInput, Field, FieldsNamed, Ident, Lit};

use crate::{
    container_attributes::{DecodeAttributes, TestAttributes},
    parts,
    repr::{Repr, ReprType},
};

pub fn derive_for_struct(
    input: &DeriveInput,
    fields_named: &FieldsNamed,
) -> syn::Result<TokenStream> {
    let struct_attrs = StructAttributes::extract(input)?;

    let parts = parts::quote_parts(&input.ident, fields_named);

    if let Some(repr) = struct_attrs.repr {
        let repr_expanded = repr.quote(
            &input.ident,
            &struct_attrs.decode_attrs,
            &struct_attrs.test_attrs,
        );

        let expanded = quote! {
            #parts
            #repr_expanded
        };

        return Ok(expanded);
    }

    let fields = fields_named.named.iter().map(|field| {
        match FieldAttributes::extract(field).and_then(|a| a.validated()) {
            Ok(attrs) => Ok((field, attrs)),
            Err(err) => Err(syn::Error::new_spanned(field, err)),
        }
    });

    let expanded = quote! {
        #parts
    };

    Ok(expanded)
}

struct StructAttributes {
    /// #[rusmpp(repr = "u8")]
    repr: Option<Repr>,
    /// #[rusmpp(decode = skip|owned|borrowed|all)]
    decode_attrs: DecodeAttributes,
    /// #[rusmpp(test = skip|owned|borrowed|all)]
    test_attrs: TestAttributes,
}

impl StructAttributes {
    fn extract(input: &DeriveInput) -> syn::Result<Self> {
        let mut repr: Option<Repr> = None;
        let mut decode_attrs = DecodeAttributes::default();
        let mut test_attrs = TestAttributes::default();

        for attr in &input.attrs {
            if !attr.path().is_ident("rusmpp") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("repr") {
                    let lit: Lit = meta.value()?.parse()?;

                    match lit {
                        Lit::Str(s) => match s.value().as_str() {
                            "u8" => repr = Some(Repr::new(ReprType::U8)),
                            "u16" => repr = Some(Repr::new(ReprType::U16)),
                            "u32" => repr = Some(Repr::new(ReprType::U32)),
                            other => {
                                return Err(meta.error(format!(
                                    "unsupported repr: {}, only `u8`, `u16`, and `u32` are allowed",
                                    other
                                )));
                            }
                        },
                        _ => {
                            return Err(meta.error("repr must be a string literal like \"u8\""));
                        }
                    }
                } else if meta.path.is_ident("decode") {
                    decode_attrs = DecodeAttributes::extract(meta)?;
                } else if meta.path.is_ident("test") {
                    test_attrs = TestAttributes::extract(meta)?;
                }

                Ok(())
            })?;
        }

        Ok(Self {
            repr,
            decode_attrs,
            test_attrs,
        })
    }
}

struct FieldAttributes {
    skip_decode: bool,
    length: Option<Length>,
    key: Option<Ident>,
    count: Option<Ident>,
}

impl FieldAttributes {
    fn extract(field: &Field) -> syn::Result<Self> {
        let mut skip_decode = false;
        let mut length = None;
        let mut key = None;
        let mut count = None;

        for attr in &field.attrs {
            if !attr.path().is_ident("rusmpp") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("skip_decode") {
                    skip_decode = true;
                } else if meta.path.is_ident("length") {
                    let value = meta.value()?;

                    if let Ok(lit) = value.parse()
                        && let Lit::Str(s) = lit
                    {
                        match s.value().as_str() {
                            "unchecked" => length = Some(Length::Unchecked),
                            "checked" => length = Some(Length::Checked),
                            _ => {
                                return Err(meta.error(
                                    "length must be \"unchecked\", \"checked\", or an identifier",
                                ));
                            }
                        }
                    }

                    if length.is_none() {
                        let ident: Ident = value.parse()?;
                        length = Some(Length::Ident(ident));
                    }
                } else if meta.path.is_ident("key") {
                    let ident: Ident = meta.value()?.parse()?;
                    key = Some(ident);
                } else if meta.path.is_ident("count") {
                    let ident: Ident = meta.value()?.parse()?;
                    count = Some(ident);
                } else {
                    return Err(meta.error("unknown attribute"));
                }

                Ok(())
            })?;
        }

        Ok(Self {
            skip_decode,
            length,
            key,
            count,
        })
    }

    fn validated(self) -> syn::Result<ValidFieldAttributes> {
        let Self {
            skip_decode,
            length,
            key,
            count,
        } = self;

        if skip_decode {
            if length.is_some() || key.is_some() || count.is_some() {
                return Err(syn::Error::new(
                    proc_macro2::Span::call_site(),
                    "skip_decode cannot be combined with length, key, or count",
                ));
            }

            return Ok(ValidFieldAttributes::SkipDecode);
        }

        match (length, key, count) {
            (Some(Length::Unchecked), None, None) => Ok(ValidFieldAttributes::LengthUnchecked),
            (Some(Length::Checked), None, None) => Ok(ValidFieldAttributes::LengthChecked),
            (Some(Length::Ident(length)), None, None) => {
                Ok(ValidFieldAttributes::LengthIdent { length })
            }
            (Some(Length::Unchecked), Some(key), None) => {
                Ok(ValidFieldAttributes::KeyLengthUnchecked { key })
            }
            (Some(Length::Ident(length)), Some(key), None) => {
                Ok(ValidFieldAttributes::KeyLengthIdent { key, length })
            }
            (None, None, Some(count)) => Ok(ValidFieldAttributes::Count { count }),
            (None, None, None) => Ok(ValidFieldAttributes::None),
            _ => Err(syn::Error::new(
                proc_macro2::Span::call_site(),
                "invalid combination of field attributes",
            )),
        }
    }
}

enum Length {
    Unchecked,
    Checked,
    Ident(Ident),
}

enum ValidFieldAttributes {
    None,
    SkipDecode,
    LengthUnchecked,
    LengthChecked,
    LengthIdent { length: Ident },
    KeyLengthUnchecked { key: Ident },
    KeyLengthIdent { key: Ident, length: Ident },
    Count { count: Ident },
}
