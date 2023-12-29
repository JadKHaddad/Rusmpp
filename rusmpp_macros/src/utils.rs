#[derive(Debug)]
pub struct StructSkipOptions<'a> {
    pub name_ident: &'a proc_macro2::Ident,
    pub skip: bool,
}

pub fn collect_skip_options_from_named_fields<'a>(
    attr_group: &str,
    named_fields: &'a syn::punctuated::Punctuated<syn::Field, syn::Token![,]>,
) -> Vec<StructSkipOptions<'a>> {
    named_fields
        .into_iter()
        .map(|field| {
            let name_ident = field.ident.as_ref().expect("No field name");
            let mut skip = false;

            for attr in field.attrs.iter() {
                if attr.path.segments[0].ident != attr_group {
                    continue;
                }

                let mut token_stream = attr.tokens.clone().into_iter();
                if let proc_macro2::TokenTree::Group(group) =
                    token_stream.next().expect("No group found: {name}")
                {
                    let tokens = group.stream();
                    let iter = tokens.into_iter();
                    for token in iter {
                        if let proc_macro2::TokenTree::Ident(ident) = token {
                            if ident != "skip" {
                                panic!("Unknown attribute: {ident} for field: {name_ident}")
                            }

                            skip = true;

                            break;
                        } else {
                            panic!("Unknown attribute: {token} for field: {name_ident}")
                        }
                    }
                };
            }

            StructSkipOptions { name_ident, skip }
        })
        .collect()
}

pub fn extract_struct_named_fields_from_data(
    data: &syn::Data,
) -> &syn::punctuated::Punctuated<syn::Field, syn::Token![,]> {
    if let syn::Data::Struct(syn::DataStruct {
        fields: syn::Fields::Named(syn::FieldsNamed { ref named, .. }),
        ..
    }) = data
    {
        named
    } else {
        panic!("Only structs with named fields are supported")
    }
}

pub fn collect_skip_options_from_from_data<'a>(
    attr_group: &str,
    data: &'a syn::Data,
) -> Vec<StructSkipOptions<'a>> {
    let named_fields = extract_struct_named_fields_from_data(data);
    collect_skip_options_from_named_fields(attr_group, named_fields)
}

pub fn panic_if_not_enum_or_struct(input: &syn::DeriveInput) {
    match input.data {
        syn::Data::Enum(_) => {}
        syn::Data::Struct(_) => {}
        _ => panic!("Only enums and structs are supported"),
    }
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

pub fn extract_type_from_option_if_exists(ty: &syn::Type) -> Option<&syn::Type> {
    extract_type_if_exists(ty, &["Option", "std:option:Option", "core:option:Option"])
}

pub fn extract_type_from_vec_if_exists(ty: &syn::Type) -> Option<&syn::Type> {
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
