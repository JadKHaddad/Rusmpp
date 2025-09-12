use proc_macro2::TokenStream;
use syn::{Data, DeriveInput};

use crate::{enum_::derive_enum, struct_::derive_struct};

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
