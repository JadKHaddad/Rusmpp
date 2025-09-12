use proc_macro2::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Field, FieldsNamed, Ident, Lit, parse};

pub fn derive(input: DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => derive_struct(&input, fields_named),
            _ => Err(syn::Error::new_spanned(
                &input.ident,
                "Rusmpp can only be derived for structs with named fields",
            )),
        },
        Data::Enum(_) => derive_enum(&input),
        _ => Err(syn::Error::new_spanned(
            &input.ident,
            "Rusmpp can only be derived for enums or structs with named fields",
        )),
    }
}

fn derive_struct(input: &DeriveInput, fields_named: &FieldsNamed) -> syn::Result<TokenStream> {
    let struct_attrs = StructAttributes::extract(input)?;

    if let Some(repr) = struct_attrs.repr {
        return Ok(repr.expand(&input.ident));
    }

    let fields: Vec<(&Field, FieldAttributes)> = fields_named
        .named
        .iter()
        .map(|field| {
            let attrs = FieldAttributes::extract(field)?;

            Ok((field, attrs))
        })
        .collect::<Result<_, syn::Error>>()?;

    let expanded = quote! {};

    Ok(expanded)
}

struct StructAttributes {
    /// #[rusmpp(repr = "u8")]
    repr: Option<Repr>,
    /// #[rusmpp(skip_decode)]
    skip_decode: bool,
}

impl StructAttributes {
    fn extract(input: &DeriveInput) -> syn::Result<Self> {
        let mut repr: Option<Repr> = None;
        let mut skip_decode = false;

        for attr in &input.attrs {
            if !attr.path().is_ident("rusmpp") {
                continue;
            }

            attr.parse_nested_meta(|meta| {
                if meta.path.is_ident("repr") {
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
                    skip_decode = true;
                }

                Ok(())
            })?;
        }

        Ok(Self { repr, skip_decode })
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
}

enum Length {
    Unchecked,
    Checked,
    Ident(Ident),
}

fn derive_enum(input: &DeriveInput) -> syn::Result<TokenStream> {
    let enum_attrs = EnumAttributes::extract(input)?;

    Ok(enum_attrs.repr.expand(&input.ident))
}

struct EnumAttributes {
    /// #[repr(u8)]
    repr: Repr,
}

impl EnumAttributes {
    fn extract(input: &DeriveInput) -> syn::Result<Self> {
        let mut repr: Option<Repr> = None;

        for attr in &input.attrs {
            if attr.path().is_ident("repr") {
                attr.parse_args_with(|input: parse::ParseStream| {
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
                })?;
            }
        }

        let repr = repr.ok_or_else(|| {
            syn::Error::new_spanned(
                input,
                "enums must have a #[repr(...)] attribute, e.g. #[repr(u8)]",
            )
        })?;

        Ok(Self { repr })
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

    fn expand(&self, name: &Ident) -> TokenStream {
        let repr_ident = self.to_ident();

        quote! {
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

            impl ::rusmpp_core::decode::owned::Decode for #name {
                fn decode(src: &[u8]) -> Result<(Self, usize), ::rusmpp_core::decode::DecodeError> {
                    #repr_ident::decode(src).map(|(this, size)| (Self::from(this), size))
                }
            }
        }
    }
}
