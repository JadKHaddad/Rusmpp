use core::panic;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field};

mod length;
mod primitive;
mod read;
mod utils;
mod write;

#[proc_macro_derive(RusmppIoLength, attributes(rusmpp_io_length))]
pub fn derive_rusmpp_io_length(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Enum(_) => length::derive_rusmpp_io_length_enum(input),
        syn::Data::Struct(_) => length::derive_rusmpp_io_length_struct(input),
        _ => panic!("Only enums and structs are supported"),
    }
}

#[proc_macro_derive(RusmppIoWrite, attributes(rusmpp_io_write))]
pub fn derive_rusmpp_io_write(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Enum(_) => write::derive_rusmpp_io_write_enum(input),
        syn::Data::Struct(_) => write::derive_rusmpp_io_write_struct(input),
        _ => panic!("Only enums and structs are supported"),
    }
}

#[proc_macro_derive(RusmppIoRead, attributes(rusmpp_io_read))]
pub fn derive_rusmpp_io_read(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Struct(_) => read::derive_rusmpp_io_read(input),
        _ => panic!("Only structs are supported"),
    }
}

#[proc_macro_derive(RusmppIoReadLength, attributes(rusmpp_io_read))]
pub fn derive_rusmpp_io_read_with_length(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Struct(_) => read::derive_rusmpp_io_read_with_length(input),
        _ => panic!("Only structs are supported"),
    }
}

#[proc_macro_derive(RusmppIoReadKey, attributes(rusmpp_io_read))]
pub fn derive_rusmpp_io_read_with_key(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    match input.data {
        syn::Data::Enum(_) => read::derive_rusmpp_io_read_with_key(input),
        _ => panic!("Only enums are supported"),
    }
}

#[proc_macro_derive(RusmppIoU8)]
pub fn derive_rusmpp_io_u8(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    utils::panic_if_not_enum_or_struct(&input);

    let name = &input.ident;
    let primitive = primitive::PrimitiveType::U8;
    primitive::derive_rusmpp_io_primitive(name, primitive)
}

#[proc_macro_derive(RusmppIoU16)]
pub fn derive_rusmpp_io_u16(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    utils::panic_if_not_enum_or_struct(&input);

    let name = &input.ident;
    let primitive = primitive::PrimitiveType::U16;
    primitive::derive_rusmpp_io_primitive(name, primitive)
}

#[proc_macro_derive(RusmppIoU32)]
pub fn derive_rusmpp_io_u32(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    utils::panic_if_not_enum_or_struct(&input);

    let name = &input.ident;
    let primitive = primitive::PrimitiveType::U32;
    primitive::derive_rusmpp_io_primitive(name, primitive)
}

// ------------

struct FiledOptions {
    skip_length: bool,
    skip_write: bool,
}

#[proc_macro_derive(RusmppIo, attributes(rusmpp_io))]
pub fn derive_rusmpp_io(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let struct_name = &input.ident;

    let fields = if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = input.data
    {
        named
    } else {
        panic!("Only structs with named fields are supported");
    };

    let fileds_with_skip: Vec<(&Field, FiledOptions)> = fields
        .into_iter()
        .map(|field| {
            let mut skip_length = false;
            let mut skip_write = false;
            for attr in field.attrs.iter() {
                let mut token_stream = attr.tokens.clone().into_iter();
                if let TokenTree::Group(group) = token_stream.next().expect("No group found") {
                    let tokens = group.stream();
                    for token in tokens.into_iter() {
                        if let TokenTree::Ident(ident) = token {
                            if !["skip_length", "skip_write"].contains(&&*ident.to_string()) {
                                panic!("Only skip_length and skip_write are supported");
                            }
                            if ident == "skip_length" {
                                skip_length = true;
                            }
                            if ident == "skip_write" {
                                skip_write = true;
                            }
                        };
                    }
                };
            }

            (
                field,
                FiledOptions {
                    skip_length,
                    skip_write,
                },
            )
        })
        .collect();

    let io_length_fields = fileds_with_skip.iter().map(|(field, skip)| {
        let field_name = &field.ident;

        if skip.skip_length {
            return quote! {};
        }

        quote! {
            self.#field_name.length() +
        }
    });

    let io_wirte_fields = fileds_with_skip.iter().map(|(field, skip)| {
        let field_name = &field.ident;

        if skip.skip_write {
            return quote! {};
        }

        quote! {
            self.#field_name.async_io_write(buf).await?;
        }
    });

    let expanded = quote! {
        impl rusmpp_io::io::length::IoLength for #struct_name {
            fn length(&self) -> usize {
                #(#io_length_fields)* 0
            }
        }

        #[async_trait::async_trait]
        impl rusmpp_io::io::write::AsyncIoWrite for #struct_name {
            async fn async_io_write(&self, buf: &mut rusmpp_io::io::write::AsyncIoWritable) -> std::io::Result<()> {
                #(#io_wirte_fields)*;

                Ok(())
            }
        }
    };

    expanded.into()
}
