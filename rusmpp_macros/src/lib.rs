use core::panic;
use proc_macro::TokenStream;
use proc_macro2::TokenTree;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Field};

struct FiledOptions {
    skip_length: bool,
    skip_write: bool,
}

#[derive(Debug)]
enum DeriveType {
    Normal,
    WithLength,
    WithKey,
}

#[derive(Debug)]
struct FieldOptionsX<'a> {
    name_ident: &'a proc_macro2::Ident,
    ty_ident: &'a proc_macro2::Ident,
    ty: TY,
    length_options: LengthOptions,
    write_options: WriteOptions,
}

#[derive(Debug)]
struct LengthOptions {
    skip: bool,
}

#[derive(Debug)]
struct WriteOptions {
    skip: bool,
}

#[derive(Debug)]
enum TY {
    Normal,
    NormalWithLength {
        length_op: LengthOperation,
    },
    Option {
        length_op: LengthOperation,
    },
    OptionWithKey {
        length_op: LengthOperation,
        key_ident: proc_macro2::Ident,
    },
    VecWithLength {
        length_op: LengthOperation,
    },
    VecWithCount {
        count_ident: proc_macro2::Ident,
    },
}

#[derive(Debug)]
enum LengthOperation {
    Ident { ident: proc_macro2::Ident },
    IdentMinusAllBeforeLengths { ident: proc_macro2::Ident },
    IdentMinusIdentLength { ident: proc_macro2::Ident, ident2: proc_macro2::Ident },
}

#[proc_macro_derive(RusmppIoX, attributes(rusmpp_io_x))]
pub fn derive_rusmpp_io_x(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let mut derive_type = DeriveType::Normal;

    for attr in &input.attrs {
        if &attr.path.segments[0].ident == "rusmpp_io_x" {
            let mut token_stream = attr.tokens.clone().into_iter();
            if let TokenTree::Group(group) = token_stream.next().expect("No group found") {
                let tokens = group.stream();
                let mut iter = tokens.into_iter();
                while let Some(token) = iter.next() {
                    if let TokenTree::Ident(ident) = token {
                        if ident != "derive" {
                            panic!("Only derive is supported");
                        }

                        if let TokenTree::Punct(punct) = iter
                            .next()
                            .unwrap_or_else(|| panic!("No punct found for derive"))
                        {
                            if punct.as_char() != '=' {
                                panic!("Only '=' is supported for derive");
                            }
                        } else {
                            panic!("Only Punct is supported for derive");
                        };

                        if let TokenTree::Ident(value) = iter
                            .next()
                            .unwrap_or_else(|| panic!("No value found for derive"))
                        {
                            if value == "key" {
                                derive_type = DeriveType::WithKey;
                            } else if value == "length" {
                                derive_type = DeriveType::WithLength;
                            } else {
                                panic!("Only key and length are supported for derive");
                            }
                        } else {
                            panic!("Only Ident is supported for derive");
                        };

                        if iter.next().is_some() {
                            panic!("Only one value is supported for derive");
                        }
                    }
                }
            }
        }
    }

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

    let fileds_with_options: Vec<FieldOptionsX> = fields
        .into_iter()
        .map(|field| {
            let name_ident = field.ident.as_ref().expect("No field name");

            let mut skip_length = false;
            let mut skip_write = false;
            let mut key_ident = None;
            let mut length_op = None;
            let mut count_ident = None;
            for attr in field.attrs.iter() {
                let mut token_stream = attr.tokens.clone().into_iter();
                if let TokenTree::Group(group) =
                    token_stream.next().expect("No group found: {name}")
                {
                    let tokens = group.stream();
                    let mut iter = tokens.into_iter();
                    while let Some(token) = iter.next() {
                        if let TokenTree::Ident(ident) = token {
                            if !["skip_length", "skip_write", "key", "length", "count"]
                                .contains(&&*ident.to_string())
                            {
                                panic!("Only skip_length, skip_write, key=<value>, length=<value> and count=<value> are supported: {name_ident}");
                            }

                            if ident == "skip_length" {
                                skip_length = true;
                            }

                            if ident == "skip_write" {
                                skip_write = true;
                            }

                            if ident == "key" {
                                if let TokenTree::Punct(punct) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("No punct found for key: {name_ident}"))
                                {
                                    if punct.as_char() != '=' {
                                        panic!("Only '=' is supported for key: {name_ident}");
                                    }
                                } else {
                                    panic!("Only Punct is supported for key: {name_ident}");
                                };

                                if let TokenTree::Ident(value) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("No value found for key: {name_ident}"))
                                {
                                    key_ident = Some(proc_macro2::Ident::new(
                                        &value.to_string(),
                                        proc_macro2::Span::call_site(),
                                    ));
                                } else {
                                    panic!("Only Ident is supported for key: {name_ident}");
                                };
                            }

                            if ident == "length" {
                                if let TokenTree::Punct(punct) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("No punct found for length: {name_ident}"))
                                {
                                    if punct.as_char() != '=' {
                                        panic!("Only '=' is supported for length: {name_ident}");
                                    }
                                } else {
                                    panic!("Only Punct is supported for length: {name_ident}");
                                };

                                if let TokenTree::Group(length_group) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("No group found for length: {name_ident}"))
                                {
                                    let length_tokens = length_group.stream();
                                    let mut length_iter = length_tokens.into_iter();
                                    while let Some(length_token) = length_iter.next() {
                                        if let TokenTree::Ident(length_ident) = length_token {
                                            if length_ident == "all_before" {
                                                panic!("all_before must be subtracted from another value: {name_ident}");
                                            }
                                            match length_iter.next() {
                                                None => {
                                                    // this is field
                                                    length_op = Some(LengthOperation::Ident {
                                                        ident: length_ident.clone(),
                                                    });
                                                },

                                                Some(TokenTree::Punct(punct)) => {
                                                    if punct.as_char() != '-' {
                                                        panic!("Only '-' is supported for a field length: {name_ident}");
                                                    }

                                                    if let TokenTree::Ident(value) = length_iter
                                                            .next()
                                                            .unwrap_or_else(|| panic!("No value found for a field length: {name_ident}"))
                                                        {
                                                            if value == "all_before" {
                                                                // this is field - all_before
                                                                length_op = Some(LengthOperation::IdentMinusAllBeforeLengths {
                                                                    ident: length_ident.clone(),});
 
                                                                 break;
                                                             }else {
                                                                // this is field - field
                                                                length_op = Some(LengthOperation::IdentMinusIdentLength {
                                                                    ident: length_ident.clone(),
                                                                    ident2: proc_macro2::Ident::new(
                                                                        &value.to_string(),
                                                                        proc_macro2::Span::call_site(),
                                                                    ),
                                                                });
                                                             }

                                                        } else {
                                                            panic!("Only Ident is supported for length value: {name_ident}");
                                                        };
                                                },
                                                _ => {
                                                    panic!("Only Punct is supported for incoming: {name_ident}");
                                                }
                                            }
                                        };
                                    }
                                } else {
                                    panic!("Only group is supported for length: {name_ident}");
                                };
                            }

                            if ident == "count" {
                                if let TokenTree::Punct(punct) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("No punct found for count: {name_ident}"))
                                {
                                    if punct.as_char() != '=' {
                                        panic!("Only '=' is supported for count: {name_ident}");
                                    }
                                } else {
                                    panic!("Only Punct is supported for count: {name_ident}");
                                };

                                if let TokenTree::Ident(value) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("No value found for count: {name_ident}"))
                                {
                                    count_ident = Some(proc_macro2::Ident::new(
                                        &value.to_string(),
                                        proc_macro2::Span::call_site(),
                                    ));
                                } else {
                                    panic!("Only Ident is supported for count: {name_ident}");
                                };
                            }
                        };
                    }
                };
            }

            let (ty, is_option, is_vec) = match extract_type_from_option_if_exists(&field.ty) {
                Some(ty) => (ty, true, false),
                None => match extract_type_from_vec_if_exists(&field.ty) {
                    Some(ty) => (ty, false, true),
                    None => (&field.ty, false, false),
                },
            };

            let ty_ident = match &ty {
                syn::Type::Path(syn::TypePath { qself: None, path }) => {
                    &path.segments.last().expect("No last segment").ident
                }
                _ => panic!("Only path types are supported"),
            };

            let ty = match (is_option, is_vec) {
                (true, false) => {
                    let ty = match length_op {
                        None => panic!("Option field must have length: {name_ident}"),
                        Some(length_op) => match key_ident {
                            None => TY::Option {
                                length_op,
                            },
                            Some(key_ident) => TY::OptionWithKey {
                                length_op,
                                key_ident,
                            },
                        },
                    };

                    if count_ident.is_some() {
                        panic!("Option field cannot have count: {name_ident}");
                    }

                    ty
                }
                (false, true) => {
                    let ty = match length_op {
                        None => match count_ident {
                            None => panic!("Vec field must have length or count: {name_ident}"),
                            Some(count_ident) => TY::VecWithCount {
                                count_ident,
                            },
                        },
                        Some(length_op) => TY::VecWithLength {
                            length_op,
                        },
                    };

                    if key_ident.is_some() {
                        panic!("Vec field cannot have key: {name_ident}");
                    }

                    ty
                }

                (false, false) => {
                    let ty = match length_op {
                        None => TY::Normal,
                        Some(length_op) => TY::NormalWithLength {
                            length_op,
                        },
                    };

                    if key_ident.is_some() {
                        panic!("Normal field cannot have key: {name_ident}");
                    }

                    if count_ident.is_some() {
                        panic!("Normal field cannot have count: {name_ident}");
                    }

                    ty
                }
                (true, true) => {
                    panic!("What the hell!")
                }
            };

            FieldOptionsX {
                name_ident,
                ty_ident,
                ty,
                length_options: LengthOptions { skip: skip_length },
                write_options: WriteOptions { skip: skip_write },
            }
        })
        .collect();

    let io_length_fields = fileds_with_options.iter().map(|options| {
        let field_name = &options.name_ident;

        if options.length_options.skip {
            return quote! {};
        }

        quote! {
            self.#field_name.length() +
        }
    });

    let io_wirte_fields = fileds_with_options.iter().map(|options| {
        let field_name = &options.name_ident;

        if options.write_options.skip {
            return quote! {};
        }

        quote! {
            self.#field_name.async_io_write(buf).await?;
        }
    });

    // println!("{:#?}", fileds_with_options);
    let mut field_name_idents = Vec::new();
    let io_read_fields = fileds_with_options.iter().map(|options| {
        let field_name_ident = &options.name_ident;
        let field_ty_ident = &options.ty_ident;

        let token_stream = match &options.ty {
            TY::Normal => quote! {
                let #field_name_ident = #field_ty_ident::async_io_read(buf).await?;
            },
            TY::NormalWithLength { length_op } => {
                let length_ident = create_length_ident(field_name_ident);
                let set_length = set_length(&length_ident, length_op, &field_name_idents);
                quote!{
                    #set_length
                    let #field_name_ident = #field_ty_ident::async_io_read(buf, #length_ident).await?;
                }
            },
            TY::Option { length_op } => {
                let length_ident = create_length_ident(field_name_ident);
                let set_length = set_length(&length_ident, length_op, &field_name_idents);
                quote!{
                    #set_length
                    let #field_name_ident = rusmmp_io::types::option::async_io_read(buf, #length_ident).await?;
                }
            },
            TY::OptionWithKey {
                length_op,
                key_ident,
            } => {
                let length_ident = create_length_ident(field_name_ident);
                let set_length = set_length(&length_ident, length_op, &field_name_idents);
                quote!{
                    #set_length
                    let #field_name_ident = rusmmp_io::types::option::async_io_read_with_key_optional(#key_ident, buf, #length_ident).await?;
                }
            },
            TY::VecWithLength { length_op } => {
                let length_ident = create_length_ident(field_name_ident);
                let set_length = set_length(&length_ident, length_op, &field_name_idents);
                quote!{
                    #set_length
                    let #field_name_ident = Vec::<#field_ty_ident>::async_io_read(buf, #length_ident).await?;
                }
            }
            TY::VecWithCount { count_ident } => quote! {
                let dest_address = rusmmp_io::types::vec::read_counted::<#field_ty_ident>(buf, #count_ident.into()).await?;
            },
        };

        field_name_idents.push(field_name_ident);

        token_stream
    });


    let (read_trait, read_function_signature) = io_read_decide_trait_and_function_signature(derive_type);

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
        impl #read_trait for #struct_name {
            async fn #read_function_signature -> Result<Self, rusmpp_io::io::read::IoReadError> {
                #(#io_read_fields)*;

                Ok(Self {
                    #(#field_name_idents),*
                })
            }
        }
    };

    expanded.into()
}

fn io_read_decide_trait_and_function_signature(derive_type: DeriveType) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match derive_type {
        DeriveType::Normal => (quote! {rusmpp_io::io::read::AsyncIoRead}, quote! {async_io_read(buf: &mut rusmpp_io::io::read::AsyncIoReadable)}),
        DeriveType::WithLength => (quote! {rusmpp_io::io::read::AsyncIoReadWithLength}, quote! {async_io_read(buf: &mut rusmpp_io::io::read::AsyncIoReadable, length: usize)}),
        DeriveType::WithKey => panic!("WithKey is not supported"),
    }
}

fn create_length_ident(field_name_ident: &syn::Ident) -> proc_macro2::Ident {
    proc_macro2::Ident::new(
        &format!("{}_len", field_name_ident),
        proc_macro2::Span::call_site(),
    )
}

fn set_length(length_ident: &syn::Ident, length_op: &LengthOperation, prev_field_name_idents: &[&syn::Ident]) -> proc_macro2::TokenStream {
    match length_op {
        LengthOperation::Ident { ident } => quote! {
           let #length_ident: usize = #ident.into();
        },
        LengthOperation::IdentMinusIdentLength { ident, ident2 } => {
            quote! {
                let #length_ident: usize = #ident.into().saturating_sub(#ident2.length());
            }
        },
        LengthOperation::IdentMinusAllBeforeLengths { ident } => {
            let field_name_idents_saturating_sub = prev_field_name_idents.iter().map(|field_name_ident| {
                quote! {
                    .saturating_sub(#field_name_ident.length())
                }
            });

            quote! {
                let #length_ident: usize = #ident.into() #(#field_name_idents_saturating_sub)*;
            }
        },
    }
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
