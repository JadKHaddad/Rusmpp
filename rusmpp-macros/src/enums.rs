use proc_macro2::TokenStream;
use quote::quote;
use syn::{DataEnum, DeriveInput, Fields, Ident, parse};

use crate::{
    container_attributes::{DecodeAttributes, FromIntoAttributes, TestAttributes},
    repr::{Repr, ReprType},
};

pub fn derive_rusmpp_for_enum(input: &DeriveInput) -> syn::Result<TokenStream> {
    let enum_attrs = EnumAttributes::extract(input)?;

    Ok(enum_attrs.repr.quote_rusmpp(
        input,
        enum_attrs.from_into_attrs,
        &enum_attrs.decode_attrs,
        &enum_attrs.test_attrs,
    ))
}

struct EnumAttributes {
    /// #[repr(u8)]
    repr: Repr,
    from_into_attrs: FromIntoAttributes,
    decode_attrs: DecodeAttributes,
    test_attrs: TestAttributes,
}

impl EnumAttributes {
    fn extract(input: &DeriveInput) -> syn::Result<Self> {
        let mut repr: Option<Repr> = None;
        let mut from_into_attrs = FromIntoAttributes::default();
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
                    } else if meta.path.is_ident("from_into") {
                        from_into_attrs = FromIntoAttributes::extract(meta)?;
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
            from_into_attrs,
            decode_attrs,
            test_attrs,
        })
    }
}

pub fn derive_tlv_value_for_enum(
    input: &DeriveInput,
    data_enum: &DataEnum,
) -> syn::Result<TokenStream> {
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = &input.generics.split_for_impl();

    // Collect match arms
    let mut tag_arms = Vec::new();
    let mut value_arms = Vec::new();
    let mut has_other_variant = false;

    for variant in &data_enum.variants {
        let v_ident = &variant.ident;

        match &variant.fields {
            Fields::Unnamed(fields) if fields.unnamed.len() == 1 => {
                tag_arms.push(quote! {
                    #ident::#v_ident(_) => TlvTag::#v_ident,
                });

                value_arms.push(quote! {
                    #ident::#v_ident(value) => TlvValue::#v_ident(value),
                });
            }
            Fields::Named(fields) => {
                if v_ident == "Other" {
                    if has_other_variant {
                        return Err(syn::Error::new_spanned(
                            v_ident,
                            "Duplicate 'Other' variant found. Only one is allowed.",
                        ));
                    }

                    // Check that it has exactly two fields named tag/value of correct types
                    let mut has_tag = false;
                    let mut has_value = false;

                    for field in &fields.named {
                        let name = field.ident.as_ref().unwrap().to_string();
                        match name.as_str() {
                            "tag" => has_tag = true,
                            "value" => has_value = true,
                            _ => {
                                return Err(syn::Error::new_spanned(
                                    &field.ident,
                                    "Unexpected field in 'Other' variant. Expected only { tag, value }.",
                                ));
                            }
                        }
                    }

                    if !(has_tag && has_value) {
                        return Err(syn::Error::new_spanned(
                            &variant.ident,
                            "The 'Other' variant must have fields { tag: TlvTag, value: AnyOctetString }.",
                        ));
                    }

                    has_other_variant = true;

                    tag_arms.push(quote! {
                        #ident::Other { tag, .. } => *tag,
                    });

                    value_arms.push(quote! {
                        #ident::Other { tag, value } => TlvValue::Other { tag, value },
                    });
                }
            }
            _ => {
                return Err(syn::Error::new_spanned(
                    &variant.ident,
                    "TlvValue can only be derived for tuple variants with a single field or a named 'Other' variant.",
                ));
            }
        }
    }

    Ok(quote! {
        impl #impl_generics #ident #ty_generics #where_clause {
            pub const fn tag(&self) -> TlvTag {
                match self {
                    #(#tag_arms)*
                }
            }
        }

        impl #impl_generics From<#ident #ty_generics> for TlvValue #ty_generics #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                match value {
                    #(#value_arms)*
                }
            }
        }

        impl #impl_generics From<#ident #ty_generics> for Tlv #ty_generics #where_clause {
            fn from(value: #ident #ty_generics) -> Self {
                Self::new(TlvValue::from(value))
            }
        }
    })
}
