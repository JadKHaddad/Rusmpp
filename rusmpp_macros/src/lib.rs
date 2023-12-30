use core::panic;
use proc_macro::TokenStream;
use syn::{parse_macro_input, DeriveInput};

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
