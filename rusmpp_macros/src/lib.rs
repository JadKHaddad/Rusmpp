use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field};

struct Skip {
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

    let fields: Vec<&Field> = fields.iter().collect();
    let fileds_with_skip: Vec<(&Field, Skip)> = fields
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
                Skip {
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
        impl crate::io::length::IoLength for #struct_name {
            fn length(&self) -> usize {
                #(#io_length_fields)* 0
            }
        }

        #[async_trait::async_trait]
        impl crate::io::write::AsyncIoWrite for #struct_name {
            async fn async_io_write(&self, buf: &mut crate::io::write::AsyncIoWritable) -> std::io::Result<()> {
                #(#io_wirte_fields)*;

                Ok(())
            }
        }
    };

    expanded.into()
}

fn panic_if_not_enum_or_struct(input: &DeriveInput) {
    match input.data {
        syn::Data::Enum(_) => {}
        syn::Data::Struct(_) => {}
        _ => panic!("Only enums and structs are supported"),
    }
}

enum Primitive {
    U8,
    U16,
    U32,
}

impl quote::ToTokens for Primitive {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        match self {
            Primitive::U8 => quote! { u8 },
            Primitive::U16 => quote! { u16 },
            Primitive::U32 => quote! { u32 },
        }
        .to_tokens(tokens)
    }
}

fn derive_rusmpp_io_primitive(name: &proc_macro2::Ident, primitive: Primitive) -> TokenStream {
    let expanded = quote! {
        impl crate::io::length::IoLength for #name {
            fn length(&self) -> usize {
                #primitive::from(*self).length()
            }
        }

        #[async_trait::async_trait]
        impl crate::io::write::AsyncIoWrite for #name {
            async fn async_io_write(&self, buf: &mut crate::io::write::AsyncIoWritable) -> std::io::Result<()> {
                #primitive::from(*self).async_io_write(buf).await
            }
        }

        #[async_trait::async_trait]
        impl crate::io::read::AsyncIoRead for #name {
            async fn async_io_read(buf: &mut crate::io::read::AsyncIoReadable) -> Result<Self, crate::io::read::IoReadError> {
                #primitive::async_io_read(buf).await.map(Self::from)
            }
        }
    };

    expanded.into()
}

#[proc_macro_derive(RusmppIoU8)]
pub fn derive_rusmpp_io_u8(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    panic_if_not_enum_or_struct(&input);

    let name = &input.ident;
    let primitive = Primitive::U8;
    derive_rusmpp_io_primitive(name, primitive)
}

#[proc_macro_derive(RusmppIoU16)]
pub fn derive_rusmpp_io_u16(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    panic_if_not_enum_or_struct(&input);

    let name = &input.ident;
    let primitive = Primitive::U16;
    derive_rusmpp_io_primitive(name, primitive)
}

#[proc_macro_derive(RusmppIoU32)]
pub fn derive_rusmpp_io_u32(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    panic_if_not_enum_or_struct(&input);

    let name = &input.ident;
    let primitive = Primitive::U32;
    derive_rusmpp_io_primitive(name, primitive)
}
