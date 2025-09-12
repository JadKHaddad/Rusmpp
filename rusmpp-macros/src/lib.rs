use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod derive;

// TODO: parts

/// Implements `Length`, `Encode` and `Decode` for structs and enums
/// and creates parts structs for structs with `new` and `raw` methods and adds `into_parts` method to the original struct.
///
/// # Enums
///
/// ## Container attributes
///
/// Enums must be annotated with `#[repr(u8)]`, `#[repr(u16)]`, or `#[repr(u32)]`, and implement the appropriate `Into`/`From` conversions.
///
/// # Structs
///
/// ## Container attributes
///
/// - `#[rusmpp(repr = "u8")]`: Use the `From<u8>`/`Into<u8>` representation for decoding.
/// - `#[rusmpp(skip_decode)]`: Skip implementing the `Decode` trait if the struct is not annotated with a `repr`.
///
/// ## Field attributes
///
/// - `#[rusmpp(skip_decode)]`: Skip decoding the field (requires a corresponding `new` constructor that does not take the skipped field as an argument).
/// - `#[rusmpp(length = "unchecked")]`: Decode without length checks.
/// - `#[rusmpp(length = "checked")]`: Decode using `length_checked_decode`.
/// - `#[rusmpp(length = ident)]`: Use the value of another field (`ident`) as the length for decoding.
/// - `#[rusmpp(key = ident, length = "unchecked")]`: Decode using a key and unchecked length.
/// - `#[rusmpp(key = ident, length = ident)]`: Decode using a key and the value of another field (`ident`) as the length.
/// - `#[rusmpp(count = ident)]`: Decode a vector of values, where `ident` is the number of elements to decode.
///
/// # Examples
///
/// See `tests/expand`.
#[proc_macro_derive(Rusmpp, attributes(rusmpp))]
pub fn rusmpp(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    derive::derive(input)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
