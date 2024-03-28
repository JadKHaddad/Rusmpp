/// Our custom `try!` macro aka `?`, to get rid of [`std::convert::From`]/[`std::convert::Into`] used by the `?` operator.
#[macro_export]
macro_rules! tri {
    ($e:expr $(,)?) => {
        match $e {
            core::result::Result::Ok(value) => value,
            core::result::Result::Err(err) => {
                return core::result::Result::Err(err);
            }
        }
    };
}

/// Implement the [`Length`](crate::ende::length::Length) and [`Encode`](crate::ende::encode::Encode) traits for a struct.
///
/// # Help
///
/// ```ignore
/// impl_length_encode! {
///    /// This is a doc comment
///    ///
///    /// More comments
///    #[derive(Debug, Clone)]
///    pub struct Foo  {
///        /// Identifies the ESME system
///        /// requesting to bind with the MC.
///        pub system_id: COctetString<1, 16>,
///        /// Identifies the version of the SMPP
///        /// protocol supported by the ESME.
///        pub interface_version: InterfaceVersion,
///        /// Type of Number (TON) for ESME
///        /// address(es) served via this SMPP session.
///        ///
///        /// Set to NULL (Unknown) if not known.
///        pub addr_ton: Ton,
///    }
/// }
/// ```
/// expands to:
///
/// ```ignore
/// /// This is a doc comment
/// ///
/// /// More comments
/// #[derive(Debug, Clone)]
/// pub struct Foo  {
///     /// Identifies the ESME system
///     /// requesting to bind with the MC.
///     pub system_id: COctetString<1, 16>,
///     /// Identifies the version of the SMPP
///     /// protocol supported by the ESME.
///     pub interface_version: InterfaceVersion,
///     /// Type of Number (TON) for ESME
///     /// address(es) served via this SMPP session.
///     ///
///     /// Set to NULL (Unknown) if not known.
///     pub addr_ton: Ton,
/// }
///
/// impl crate::ende::length::Length for Foo {
///     fn length(&self) -> usize {
///         let mut length = 0;
///
///         length += self.system_id.length();
///         length += self.interface_version.length();
///         length += self.addr_ton.length();
///
///         length
///     }
/// }
///
/// impl crate::ende::encode::Encode for Foo {
///     fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::ende::encode::EncodeError> {
///         crate::tri!(self.system_id.encode_to(writer));
///         crate::tri!(self.interface_version.encode_to(writer));
///         crate::tri!(self.addr_ton.encode_to(writer));
///
///         Ok(())
///     }
/// }
/// ```
#[macro_export]
macro_rules! impl_length_encode {
    (
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $struct_ident:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field_ident:ident: $field_ty:ty,)*
        }
    ) => {
        $(#[$struct_meta])*
        $struct_vis struct $struct_ident {
            $(
                $(#[$field_meta])*
                $field_vis $field_ident: $field_ty,)*
        }

        impl $crate::ende::length::Length for $struct_ident {
            fn length(&self) -> usize {
                let mut length = 0;

                $(
                    length += self.$field_ident.length();
                )*

                length
            }
        }

        impl $crate::ende::encode::Encode for $struct_ident {
            fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), $crate::ende::encode::EncodeError> {
                $(
                    $crate::tri!(self.$field_ident.encode_to(writer));
                )*

                Ok(())
            }
        }
    };
}
