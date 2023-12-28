use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field};

struct FiledOptions {
    skip_length: bool,
    skip_write: bool,
}

struct FiledOptionsX {
    type_ident: proc_macro2::Ident,
    length_ident: Option<proc_macro2::Ident>,
    key_ident: Option<proc_macro2::Ident>,
    count_ident: Option<proc_macro2::Ident>,
    is_option: bool,
    is_vec: bool,
    skip_length: bool,
    skip_write: bool,
}

enum TY<'a> {
    Normal {
        ty_ident: &'a proc_macro2::Ident,
    },
    NormalWithLength {
        ty_ident: &'a proc_macro2::Ident,
        length_ident: &'a proc_macro2::Ident,
    },
    Option {
        ty_ident: &'a proc_macro2::Ident,
        length_ident: &'a proc_macro2::Ident,
    },
    OptionWithKey {
        ty_ident: &'a proc_macro2::Ident,
        length_ident: &'a proc_macro2::Ident,
        key_ident: &'a proc_macro2::Ident,
    },
    Vec {
        ty_ident: &'a proc_macro2::Ident,
        length_ident: &'a proc_macro2::Ident,
    },
    VecWithCount {
        ty_ident: &'a proc_macro2::Ident,
        length_ident: &'a proc_macro2::Ident,
        count_ident: &'a proc_macro2::Ident,
    },
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

#[proc_macro_derive(RusmppIoX, attributes(rusmpp_io_x))]
pub fn derive_rusmpp_io_x(input: TokenStream) -> TokenStream {
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

    let fileds_with_options: Vec<(&Field, FiledOptionsX)> = fields
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

            let (ty, is_option, is_vec) = match extract_type_from_option_if_exists(&field.ty) {
                Some(ty) => (ty.clone(), true, false),
                None => match extract_type_from_vec_if_exists(&field.ty) {
                    Some(ty) => (ty.clone(), false, true),
                    None => (field.ty.clone(), false, false),
                },
            };

            let type_ident = match &ty {
                syn::Type::Path(syn::TypePath { qself: None, path }) => {
                    path.segments.last().expect("No last segment").ident.clone()
                }
                _ => panic!("Only path types are supported"),
            };

            (
                field,
                FiledOptionsX {
                    type_ident,
                    length_ident: None,
                    key_ident: None,
                    count_ident: None,
                    is_option,
                    is_vec,
                    skip_length,
                    skip_write,
                },
            )
        })
        .collect();

    let io_length_fields = fileds_with_options.iter().map(|(field, field_options)| {
        let field_name = &field.ident;

        if field_options.skip_length {
            return quote! {};
        }

        quote! {
            self.#field_name.length() +
        }
    });

    let io_wirte_fields = fileds_with_options.iter().map(|(field, field_options)| {
        let field_name = &field.ident;

        if field_options.skip_write {
            return quote! {};
        }

        quote! {
            self.#field_name.async_io_write(buf).await?;
        }
    });

    let io_read_fields = fileds_with_options.iter().map(|(field, field_options)| {
        let field_name = &field.ident;
        let field_type = &field_options.type_ident;
        let is_option = field_options.is_option;

        println!("field_name: {:?}", field_name);
        println!("field_type: {:?}, is option: {}", field_type, is_option);
        println!();

        quote! {
            let #field_name = #field_type::async_io_read(buf).await?;
        }
    });

    let field_names = fileds_with_options.iter().map(|(field, _)| &field.ident);

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

        #[async_trait::async_trait]
        impl rusmpp_io::io::read::AsyncIoRead for #struct_name {
            async fn async_io_read(buf: &mut rusmpp_io::io::read::AsyncIoReadable) -> Result<Self, rusmpp_io::io::read::IoReadError> {
                #(#io_read_fields)*;

                Ok(Self {
                    #(#field_names),*
                })
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
        impl rusmpp_io::io::length::IoLength for #name {
            fn length(&self) -> usize {
                #primitive::from(*self).length()
            }
        }

        #[async_trait::async_trait]
        impl rusmpp_io::io::write::AsyncIoWrite for #name {
            async fn async_io_write(&self, buf: &mut rusmpp_io::io::write::AsyncIoWritable) -> std::io::Result<()> {
                #primitive::from(*self).async_io_write(buf).await
            }
        }

        #[async_trait::async_trait]
        impl rusmpp_io::io::read::AsyncIoRead for #name {
            async fn async_io_read(buf: &mut rusmpp_io::io::read::AsyncIoReadable) -> Result<Self, rusmpp_io::io::read::IoReadError> {
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

/// Checks if the type is a wrapper type like Option or Vec
/// and returns the inner type.
/// If the type is not a wrapper type, it returns None.
/// For Option: ["Option", "std:option:Option", "core:option:Option"].
/// For Vec: ["Vec", "std:vec:Vec", "core:vec:Vec"].
fn extract_type_if_exists<'a>(ty: &'a syn::Type, types: &[&str]) -> Option<&'a syn::Type> {
    if let syn::Type::Path(syn::TypePath { qself: None, path }) = ty {
        let segments_str = &path
            .segments
            .iter()
            .map(|segment| segment.ident.to_string())
            .collect::<Vec<_>>()
            .join(":");

        let wrapper_segment = types
            .iter()
            .find(|s| segments_str == *s)
            .and_then(|_| path.segments.last());

        let inner_type = wrapper_segment
            .and_then(|path_seg| match &path_seg.arguments {
                syn::PathArguments::AngleBracketed(syn::AngleBracketedGenericArguments {
                    args,
                    ..
                }) => args.first(),
                _ => None,
            })
            .and_then(|generic_arg| match generic_arg {
                syn::GenericArgument::Type(ty) => Some(ty),
                _ => None,
            });
        return inner_type;
    }
    None
}

fn extract_type_from_option_if_exists(ty: &syn::Type) -> Option<&syn::Type> {
    extract_type_if_exists(ty, &["Option", "std:option:Option", "core:option:Option"])
}

fn extract_type_from_vec_if_exists(ty: &syn::Type) -> Option<&syn::Type> {
    extract_type_if_exists(
        ty,
        &[
            "Vec",
            "std:vec:Vec",
            "core:vec:Vec",
            "std:vec:vec",
            "core:vec:vec",
        ],
    )
}
