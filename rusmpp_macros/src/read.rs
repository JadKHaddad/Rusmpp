const ATTR_GROUP: &str = "rusmpp_io_read";

#[derive(Debug)]
struct StructReadOptions<'a> {
    name_ident: &'a proc_macro2::Ident,
    ty_ident: &'a proc_macro2::Ident,
    ty: TY,
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
    Ident {
        ident: proc_macro2::Ident,
    },
    IdentMinusAllBeforeLengths {
        ident: proc_macro2::Ident,
    },
    IdentMinusIdentLength {
        ident: proc_macro2::Ident,
        ident2: proc_macro2::Ident,
    },
}


pub fn derive_rusmpp_io_read(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let struct_name = &input.ident;
    let (field_name_idents, read_fields) = extract_field_name_idents_and_read_fields_from_data(&input.data);

    let expanded = quote::quote! {
        #[async_trait::async_trait]
        impl rusmpp_io::io::read::AsyncIoRead for #struct_name {
            async fn async_io_read(buf: &mut rusmpp_io::io::read::AsyncIoReadable) -> Result<Self, rusmpp_io::io::read::IoReadError> {
                #(#read_fields)*;

                Ok(Self {
                    #(#field_name_idents),*
                })
            }
        }
    };

    expanded.into()
}

pub fn derive_rusmpp_io_read_with_length(input: syn::DeriveInput) -> proc_macro::TokenStream {
    let struct_name = &input.ident;
    let (field_name_idents, read_fields) = extract_field_name_idents_and_read_fields_from_data(&input.data);

    let expanded = quote::quote! {
        #[async_trait::async_trait]
        impl rusmpp_io::io::read::AsyncIoReadWithLength for #struct_name {
            async fn async_io_read(buf: &mut rusmpp_io::io::read::AsyncIoReadable, length: usize) -> Result<Self, rusmpp_io::io::read::IoReadError> {
                #(#read_fields)*;

                Ok(Self {
                    #(#field_name_idents),*
                })
            }
        }
    };

    expanded.into()
}

pub fn derive_rusmpp_io_read_with_key(_input: syn::DeriveInput) -> proc_macro::TokenStream {
    panic!("Not implemented yet")
}

fn is_ty_option_or_vec(field_ty: &syn::Type) -> (&syn::Type, bool, bool) {
    match crate::utils::extract_type_from_option_if_exists(field_ty) {
        Some(ty) => (ty, true, false),
        None => match crate::utils::extract_type_from_vec_if_exists(field_ty) {
            Some(ty) => (ty, false, true),
            None => (field_ty, false, false),
        },
    }
}

fn extract_ty_ident_from_type<'a>(ty: &'a syn::Type, name_ident: &syn::Ident) -> &'a syn::Ident {
    match ty {
        syn::Type::Path(syn::TypePath { qself: None, path }) => {
            &path.segments.last().unwrap_or_else(|| panic!("No last segment, field: {name_ident}")).ident
        }
        _ => panic!("Only path types are supported, field: {name_ident}"),
    }
}

fn decide_ty(name_ident: &syn::Ident, is_option: bool, is_vec: bool, length_op: Option<LengthOperation>, key_ident: Option<syn::Ident>, count_ident: Option<syn::Ident>) -> TY{
    match (is_option, is_vec) {
        (true, false) => {
            let ty = match length_op {
                None => panic!("Option field must have length, field: {name_ident}"),
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
                panic!("Option field cannot have count, field: {name_ident}");
            }

            ty
        }
        (false, true) => {
            let ty = match length_op {
                None => match count_ident {
                    None => panic!("Vec field must have length or count, field: {name_ident}"),
                    Some(count_ident) => TY::VecWithCount {
                        count_ident,
                    },
                },
                Some(length_op) => TY::VecWithLength {
                    length_op,
                },
            };

            if key_ident.is_some() {
                panic!("Vec field cannot have key, field: {name_ident}");
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
                panic!("Normal field cannot have key, field: {name_ident}");
            }

            if count_ident.is_some() {
                panic!("Normal field cannot have count, field: {name_ident}");
            }

            ty
        }
        (true, true) => {
            panic!("What the hell!, field: {name_ident}")
        }
    }
}

fn collect_read_options_from_named_fields(
    named_fields: &syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> Vec<StructReadOptions> {
    named_fields
        .into_iter()
        .map(|field| {
            let name_ident = field.ident.as_ref().expect("No field name");
            let mut key_ident = None;
            let mut length_op = None;
            let mut count_ident = None;
            
            for attr in field.attrs.iter() {
                if attr.path.segments[0].ident != ATTR_GROUP {
                    continue;
                }

                let mut token_stream = attr.tokens.clone().into_iter();
                if let proc_macro2::TokenTree::Group(group) =
                    token_stream.next().unwrap_or_else(|| panic!("Expected parenthesis for attribute group: {ATTR_GROUP}, field: {name_ident}"))
                {
                    let tokens = group.stream();
                    let mut iter = tokens.into_iter();
                    while let Some(token) = iter.next() {
                        if let proc_macro2::TokenTree::Ident(ident) = token {
                            if !["key", "length", "count"]
                                .contains(&&*ident.to_string())
                            {   
                                panic!("Unknown attribute: {ident}. Only key=<value>, length=<value> and count=<value> are supported, field: {name_ident}");
                            }
                            if ident == "key" {
                                if let proc_macro2::TokenTree::Punct(punct) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("Expected '=' for key attribute, field: {name_ident}"))
                                {
                                    if punct.as_char() != '=' {
                                        panic!("Only '=' is supported for key attribute, field: {name_ident}");
                                    }
                                } else {
                                    panic!("Expected '=' for key attribute, field: {name_ident}");
                                };

                                if let proc_macro2::TokenTree::Ident(value) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("No value found for key attribute, field: {name_ident}"))
                                {
                                    key_ident = Some(proc_macro2::Ident::new(
                                        &value.to_string(),
                                        proc_macro2::Span::call_site(),
                                    ));
                                } else {
                                    panic!("Unsupported value for key attribute, field: {name_ident}");
                                };
                            }

                            if ident == "count" {
                                if let proc_macro2::TokenTree::Punct(punct) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("Expected '=' for count attribute, field: {name_ident}"))
                                {
                                    if punct.as_char() != '=' {
                                        panic!("Only '=' is supported for count attribute, field: {name_ident}");
                                    }
                                } else {
                                    panic!("Expected '=' for count attribute, field: {name_ident}");
                                };

                                if let proc_macro2::TokenTree::Ident(value) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("No value found for count attribute, field: {name_ident}"))
                                {
                                    count_ident = Some(proc_macro2::Ident::new(
                                        &value.to_string(),
                                        proc_macro2::Span::call_site(),
                                    ));
                                } else {
                                    panic!("Unsupported value for count attribute, field: {name_ident}");
                                };
                            
                            }

                            if ident == "length" {
                                if let proc_macro2::TokenTree::Punct(punct) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("Expected '=' for length attribute, field: {name_ident}"))
                                {
                                    if punct.as_char() != '=' {
                                        panic!("Only '=' is supported for length attribute, field: {name_ident}");
                                    }
                                } else {
                                    panic!("Expected '=' for length attribute, field: {name_ident}");
                                };

                                if let proc_macro2::TokenTree::Group(length_group) = iter
                                    .next()
                                    .unwrap_or_else(|| panic!("Expected parenthesis for length attribute, field: {name_ident}"))
                                    
                                {
                                    let length_tokens = length_group.stream();
                                    let mut length_iter = length_tokens.into_iter();
                                    while let Some(length_token) = length_iter.next() {
                                        if let proc_macro2::TokenTree::Ident(length_ident) = length_token {
                                            if length_ident == "all_before" {
                                                panic!("'all_before' must be subtracted from another value, field: {name_ident}");
                                            }
                                            match length_iter.next() {
                                                None => {
                                                    // this is field
                                                    length_op = Some(LengthOperation::Ident {
                                                        ident: length_ident.clone(),
                                                    });
                                                },

                                                Some(proc_macro2::TokenTree::Punct(punct)) => {
                                                    if punct.as_char() != '-' {
                                                        panic!("Only '-' is supported for a field length, field: {name_ident}");
                                                    }

                                                    if let proc_macro2::TokenTree::Ident(value) = length_iter
                                                            .next()
                                                            .unwrap_or_else(|| panic!("No value found for a field length, field: {name_ident}"))
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
                                                            panic!("Unsupported value length, field: {name_ident}");
                                                        };
                                                },
                                                _ => {
                                                    panic!("Unsupported value length, field: {name_ident}");
                                                }
                                            }
                                        };
                                    }
                                } else {
                                    panic!("Expected parenthesis for length attribute, field: {name_ident}");
                                };
                            }
                        };
                    }
                }else {
                    panic!("Expected parenthesis for attribute group: {ATTR_GROUP}, field: {name_ident}")
                }
            }

            let (field_ty, is_option, is_vec) = is_ty_option_or_vec(&field.ty);
            let ty_ident = extract_ty_ident_from_type(field_ty, name_ident);
            let ty = decide_ty(name_ident, is_option, is_vec, length_op, key_ident, count_ident);

            StructReadOptions {
                name_ident,
                ty_ident,
                ty,
            }
        })
        .collect()
}

fn collect_read_options_from_data(
    data: &syn::Data,
) -> Vec<StructReadOptions> {
    let named_fields = crate::utils::extract_struct_named_fields_from_data(data);
    collect_read_options_from_named_fields(named_fields)
}


fn create_length_ident(field_name_ident: &syn::Ident) -> proc_macro2::Ident {
    proc_macro2::Ident::new(
        &format!("{}_len", field_name_ident),
        proc_macro2::Span::call_site(),
    )
}

fn set_length(length_ident: &syn::Ident, length_op: &LengthOperation, prev_field_name_idents: &[&syn::Ident]) -> proc_macro2::TokenStream {
    match length_op {
        LengthOperation::Ident { ident } => quote::quote! {
           let #length_ident: usize =  (#ident as usize); 
        },
        LengthOperation::IdentMinusIdentLength { ident, ident2 } => {
            quote::quote! {
                let #length_ident: usize =  (#ident as usize).saturating_sub(#ident2.length());
            }
        },
        LengthOperation::IdentMinusAllBeforeLengths { ident } => {
            let field_name_idents_saturating_sub = prev_field_name_idents.iter().map(|field_name_ident| {
                quote::quote! {
                    .saturating_sub(#field_name_ident.length())
                }
            });

            quote::quote! {
                let #length_ident: usize = (#ident as usize)#(#field_name_idents_saturating_sub)*;
            }
        },
    }
}


fn map_options_to_token_stream(options: &StructReadOptions, prev_field_name_idents: &[&syn::Ident]) -> proc_macro2::TokenStream {
    let field_name_ident = &options.name_ident;
    let field_ty_ident = &options.ty_ident;

    match &options.ty {
        TY::Normal => quote::quote! {
            let #field_name_ident = #field_ty_ident::async_io_read(buf).await?;
        },
        TY::NormalWithLength { length_op } => {
            let length_ident = create_length_ident(field_name_ident);
            let set_length = set_length(&length_ident, length_op, prev_field_name_idents);
            quote::quote!{
                #set_length
                let #field_name_ident = #field_ty_ident::async_io_read(buf, #length_ident).await?;
            }
        },
        TY::Option { length_op } => {
            let length_ident = create_length_ident(field_name_ident);
            let set_length = set_length(&length_ident, length_op, prev_field_name_idents);
            quote::quote!{
                #set_length
                let #field_name_ident = rusmpp_io::types::option::async_io_read(buf, #length_ident).await?;
            }
        },
        TY::OptionWithKey {
            length_op,
            key_ident,
        } => {
            let length_ident = create_length_ident(field_name_ident);
            let set_length = set_length(&length_ident, length_op, prev_field_name_idents);
            quote::quote!{
                #set_length
                let #field_name_ident = rusmpp_io::types::option::async_io_read_with_key_optional(#key_ident, buf, #length_ident).await?;
            }
        },
        TY::VecWithLength { length_op } => {
            let length_ident = create_length_ident(field_name_ident);
            let set_length = set_length(&length_ident, length_op, prev_field_name_idents);
            quote::quote!{
                #set_length
                let #field_name_ident = Vec::<#field_ty_ident>::async_io_read(buf, #length_ident).await?;
            }
        }
        TY::VecWithCount { count_ident } => quote::quote! {
            let #field_name_ident = rusmpp_io::types::vec::async_read_counted::<#field_ty_ident>(buf, #count_ident.into()).await?;
        },
    }
}

fn extract_field_name_idents_and_read_fields_from_data(
    data: &syn::Data,
) -> (Vec<&syn::Ident>, Vec<proc_macro2::TokenStream>) {
    let fields_with_options = collect_read_options_from_data(data);
    let mut field_name_idents: Vec<&proc_macro2::Ident> = Vec::new();
    let read_fields: Vec<proc_macro2::TokenStream> = fields_with_options.iter().map(|options| {
        let field_name_ident = &options.name_ident;
        let token_stream = map_options_to_token_stream(options, &field_name_idents);
        field_name_idents.push(field_name_ident);
        token_stream
    }).collect();

    (field_name_idents, read_fields)
}