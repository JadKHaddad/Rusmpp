use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

use crate::{
    enums::{derive_rusmpp_for_enum, derive_tlv_value_for_enum},
    structs::derive_rusmpp_for_struct,
};

pub fn rusmpp(input: DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            syn::Fields::Named(fields_named) => derive_rusmpp_for_struct(&input, fields_named),
            _ => Err(syn::Error::new_spanned(
                &input.ident,
                "Rusmpp can only be derived for structs with named fields",
            )),
        },
        Data::Enum(_) => derive_rusmpp_for_enum(&input),
        _ => Err(syn::Error::new_spanned(
            &input.ident,
            "Rusmpp can only be derived for enums or structs with named fields",
        )),
    }
}

pub fn tlv_value(input: DeriveInput) -> syn::Result<TokenStream> {
    match &input.data {
        Data::Enum(data_enum) => derive_tlv_value_for_enum(&input, data_enum),
        _ => Err(syn::Error::new_spanned(
            &input.ident,
            "TlvValue can only be derived for enums",
        )),
    }
}
