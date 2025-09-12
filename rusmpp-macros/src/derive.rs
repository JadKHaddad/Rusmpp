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
        return Ok(repr.expand(&input.ident, &struct_attrs.decode_attrs));
    }

    let fields: Vec<(&Field, ValidFieldAttributes)> = fields_named
        .named
        .iter()
        .map(
            |field| match FieldAttributes::extract(field).and_then(|a| a.validated()) {
                Ok(attrs) => Ok((field, attrs)),
                Err(err) => Err(syn::Error::new_spanned(field, err)),
            },
        )
        .collect::<Result<_, syn::Error>>()?;

    let expanded = quote! {};

    Ok(expanded)
}

#[derive(Debug)]
enum DecodeAttributes {
    Skip,
    Implement(DecodeImplementation),
}

#[derive(Debug)]
enum DecodeImplementation {
    Owned,
    Borrowed,
    All,
}

impl DecodeAttributes {
    fn extract(meta: syn::meta::ParseNestedMeta<'_>) -> syn::Result<Self> {
        let ident: Ident = meta.value()?.parse()?;

        match ident.to_string().as_str() {
            "skip" => Ok(Self::Skip),
            "owned" => Ok(Self::Implement(DecodeImplementation::Owned)),
            "borrowed" => Ok(Self::Implement(DecodeImplementation::Borrowed)),
            "all" => Ok(Self::Implement(DecodeImplementation::All)),
            other => Err(meta.error(format!(
                "unknown decode attribute: {}, expected skip, owned, borrowed, or all",
                other
            ))),
        }
    }
}

struct StructAttributes {
    /// #[rusmpp(repr = "u8")]
    repr: Option<Repr>,
    /// #[rusmpp(decode = skip|owned|borrowed|all)]
    decode_attrs: DecodeAttributes,
}

impl StructAttributes {
    fn extract(input: &DeriveInput) -> syn::Result<Self> {
        let mut repr: Option<Repr> = None;
        let mut decode_attrs = DecodeAttributes::Implement(DecodeImplementation::All);

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
                }

                Ok(())
            })?;
        }

        Ok(Self { repr, decode_attrs })
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

fn derive_enum(input: &DeriveInput) -> syn::Result<TokenStream> {
    let enum_attrs = EnumAttributes::extract(input)?;

    Ok(enum_attrs
        .repr
        .expand(&input.ident, &enum_attrs.decode_attrs))
}

struct EnumAttributes {
    /// #[repr(u8)]
    repr: Repr,
    /// #[rusmpp(decode = skip|owned|borrowed|all)]
    decode_attrs: DecodeAttributes,
}

impl EnumAttributes {
    fn extract(input: &DeriveInput) -> syn::Result<Self> {
        let mut repr: Option<Repr> = None;
        let mut decode_attrs = DecodeAttributes::Implement(DecodeImplementation::All);

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
                        println!("decode_attrs: {:?}", decode_attrs);
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

        Ok(Self { repr, decode_attrs })
    }
}

struct Repr {
    ident: Ident,
}

impl Repr {
    fn new(repr_type: ReprType) -> Self {
        let ident = repr_type.to_ident();

        Self { ident }
    }
}

enum ReprType {
    U8,
    U16,
    U32,
}

impl ReprType {
    fn to_ident(&self) -> Ident {
        match self {
            Self::U8 => Ident::new("u8", proc_macro2::Span::call_site()),
            Self::U16 => Ident::new("u16", proc_macro2::Span::call_site()),
            Self::U32 => Ident::new("u32", proc_macro2::Span::call_site()),
        }
    }
}

impl Repr {
    fn quote_length_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl ::rusmpp_core::encode::Length for #name {
                fn length(&self) -> usize {
                    #repr_ident::from(*self).length()
                }
            }
        }
    }

    fn quote_encode_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl ::rusmpp_core::encode::Encode for #name {
                fn encode(&self, dst: &mut [u8]) -> usize {
                    #repr_ident::from(*self).encode(dst)
                }
            }
        }
    }

    fn quote_owned_decode_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl ::rusmpp_core::decode::owned::Decode for #name {
                fn decode(src: &[u8]) -> Result<(Self, usize), ::rusmpp_core::decode::DecodeError> {
                    #repr_ident::decode(src).map(|(this, size)| (Self::from(this), size))
                }
            }
        }
    }

    fn quote_borrowed_decode_impl(&self, name: &Ident) -> TokenStream {
        let repr_ident = &self.ident;

        quote! {
            impl<'a> ::rusmpp_core::decode::borrowed::Decode<'a> for #name {
                fn decode(src: &'a [u8]) -> Result<(Self, usize), ::rusmpp_core::decode::DecodeError> {
                    #repr_ident::decode(src).map(|(this, size)| (Self::from(this), size))
                }
            }
        }
    }

    fn quote_decode_impl(&self, name: &Ident, decode_attrs: &DecodeAttributes) -> TokenStream {
        match decode_attrs {
            DecodeAttributes::Skip => quote! {},
            DecodeAttributes::Implement(impl_type) => match impl_type {
                DecodeImplementation::Owned => self.quote_owned_decode_impl(name),
                DecodeImplementation::Borrowed => self.quote_borrowed_decode_impl(name),
                DecodeImplementation::All => {
                    let owned = self.quote_owned_decode_impl(name);
                    let borrowed = self.quote_borrowed_decode_impl(name);

                    quote! {
                        #owned
                        #borrowed
                    }
                }
            },
        }
    }

    fn expand(&self, name: &Ident, decode_attrs: &DecodeAttributes) -> TokenStream {
        let length_impl = self.quote_length_impl(name);
        let encode_impl = self.quote_encode_impl(name);
        let decode_impl = self.quote_decode_impl(name, decode_attrs);

        quote! {
            #length_impl
            #encode_impl
            #decode_impl
        }
    }
}
