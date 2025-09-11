use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Ident, Lit, parse, parse_macro_input};

/// Implements `Length`, `Encode` and `Decode` for structs and enums.
///
/// # Enums
///
/// ## Container attributes
///
/// Enums must be annotated with `#[repr(u8)]`, `#[repr(u16)]`, or `#[repr(u32)]`, and implement the appropriate `Into`/`From` conversions.
///
/// # Structs
///
/// ## Container attributes
///
/// - `#[rusmpp(repr = "u8")]`: Use the `From<u8>`/`Into<u8>` representation for decoding.
/// - `#[rusmpp(skip_decode)]`: Skip implementing the `Decode` trait.
///
/// ## Field attributes
///
/// - `#[rusmpp(skip)]`: Skip decoding the field (requires a corresponding `new` constructor that does not take the skipped field as an argument).
/// - `#[rusmpp(length = "unchecked")]`: Decode without length checks.
/// - `#[rusmpp(length = "checked")]`: Decode using `length_checked_decode`.
/// - `#[rusmpp(length = ident)]`: Use the value of another field (`ident`) as the length for decoding.
/// - `#[rusmpp(key = ident, length = "unchecked")]`: Decode using a key and unchecked length.
/// - `#[rusmpp(key = ident, length = ident)]`: Decode using a key and the value of another field (`ident`) as the length.
/// - `#[rusmpp(count = ident)]`: Decode a vector of values, where `ident` is the number of elements to decode.
///
/// # Examples
///
/// See `tests/expand`.
#[proc_macro_derive(Rusmpp)]
pub fn rusmpp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = &input.ident;

    match &input.data {
        Data::Struct(_) => {
            // extract #[rusmpp(repr = "u8")] if exists. default: no repr
            let mut repr: Option<Repr> = None;

            // extract #[rusmpp(skip_decode)] if exists. default: false
            let mut skip_decode = false;

            for attr in &input.attrs {
                if !attr.path().is_ident("rusmpp") {
                    continue;
                }

                let result = attr.parse_nested_meta(|meta| {
                    if meta.path.is_ident("repr") {
                        // #[rusmpp(repr = "u8")]
                        let lit: Lit = meta.value()?.parse()?;

                        match lit {
                            Lit::Str(s) => match s.value().as_str() {
                                "u8" => repr = Some(Repr::U8),
                                other => {
                                    return Err(meta.error(format!(
                                        "unsupported repr: {}, only `u8` allowed",
                                        other
                                    )));
                                }
                            },
                            _ => {
                                return Err(meta.error("repr must be a string literal like \"u8\""));
                            }
                        }
                    } else if meta.path.is_ident("skip_decode") {
                        // #[rusmpp(skip_decode)]
                        skip_decode = true;
                    }

                    Ok(())
                });

                if let Err(err) = result {
                    err.to_compile_error();
                }
            }

            let expanded = quote! {};

            TokenStream::from(expanded)
        }

        Data::Enum(_) => {
            // extract #[repr(u8)] if exists. default: no repr
            let mut repr: Option<Repr> = None;

            for attr in &input.attrs {
                if attr.path().is_ident("repr") {
                    let result = attr.parse_args_with(|input: parse::ParseStream| {
                        let ident: Ident = input.parse()?;

                        match ident.to_string().as_str() {
                            "u8" => repr = Some(Repr::U8),
                            other => {
                                return Err(syn::Error::new_spanned(
                                    ident,
                                    format!("unsupported repr: {}, only `u8` allowed", other),
                                ));
                            }
                        }

                        Ok(())
                    });

                    if let Err(err) = result {
                        err.to_compile_error();
                    }
                }
            }

            let repr = repr.expect("enums must have a #[repr(...)] attribute");
            let repr_ident = repr.to_ident();

            let expanded = quote! {
                impl ::rusmpp_core::encode::Length for #name {
                    fn length(&self) -> usize {
                        #repr_ident::from(*self).length()
                    }
                }

                impl ::rusmpp_core::encode::Encode for #name {
                    fn encode(&self, dst: &mut [u8]) -> usize {
                        #repr_ident::from(*self).encode(dst)
                    }
                }

                impl ::rusmpp_core::decode::Decode for #name {
                    fn decode(src: &[u8]) -> Result<(Self, usize), ::rusmpp_core::decode::DecodeError> {
                        #repr_ident::decode(src).map(|(this, size)| (Self::from(this), size))
                    }
                }
            };

            TokenStream::from(expanded)
        }

        _ => panic!("Rusmpp can only be derived for enums or structs with named fields"),
    }
}

enum Repr {
    U8,
}

impl Repr {
    fn to_ident(&self) -> Ident {
        match self {
            Repr::U8 => Ident::new("u8", proc_macro2::Span::call_site()),
        }
    }
}
