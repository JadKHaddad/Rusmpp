// /// Implement the [`Length`](crate::ende::length::Length) and [`Encode`](crate::ende::encode::Encode) traits for a struct.
// ///
// /// # Help
// ///
// /// ```ignore
// /// create! {
// ///    /// This is a doc comment
// ///    ///
// ///    /// More comments
// ///    #[derive(Debug, Clone)]
// ///    pub struct Foo  {
// ///        /// Identifies the ESME system
// ///        /// requesting to bind with the MC.
// ///        pub system_id: COctetString<1, 16>,
// ///        /// Identifies the version of the `SMPP`
// ///        /// protocol supported by the ESME.
// ///        pub interface_version: InterfaceVersion,
// ///        /// Type of Number (TON) for ESME
// ///        /// address(es) served via this `SMPP` session.
// ///        ///
// ///        /// Set to NULL (Unknown) if not known.
// ///        pub addr_ton: Ton,
// ///    }
// /// }
// /// ```
// /// expands to:
// ///
// /// ```ignore
// /// /// This is a doc comment
// /// ///
// /// /// More comments
// /// #[derive(Debug, Clone)]
// /// pub struct Foo  {
// ///     /// Identifies the ESME system
// ///     /// requesting to bind with the MC.
// ///     pub system_id: COctetString<1, 16>,
// ///     /// Identifies the version of the `SMPP`
// ///     /// protocol supported by the ESME.
// ///     pub interface_version: InterfaceVersion,
// ///     /// Type of Number (TON) for ESME
// ///     /// address(es) served via this `SMPP` session.
// ///     ///
// ///     /// Set to NULL (Unknown) if not known.
// ///     pub addr_ton: Ton,
// /// }
// ///
// /// impl crate::ende::length::Length for Foo {
// ///     fn length(&self) -> usize {
// ///         let mut length = 0;
// ///
// ///         length += self.system_id.length();
// ///         length += self.interface_version.length();
// ///         length += self.addr_ton.length();
// ///
// ///         length
// ///     }
// /// }
// ///
// /// impl crate::ende::encode::Encode for Foo {
// ///     fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), crate::ende::encode::EncodeError> {
// ///         crate::tri!(self.system_id.encode_to(writer));
// ///         crate::tri!(self.interface_version.encode_to(writer));
// ///         crate::tri!(self.addr_ton.encode_to(writer));
// ///
// ///         Ok(())
// ///     }
// /// }
// /// ```
// macro_rules! create {
//     (
//         $(#[$struct_meta:meta])*
//         $struct_vis:vis struct $struct_ident:ident {
//             $(
//                 $(#[$field_meta:meta])*
//                 $field_vis:vis $field_ident:ident: $field_ty:ty,)*
//         }
//     ) => {
//         $(#[$struct_meta])*
//         $struct_vis struct $struct_ident {
//             $(
//                 $(#[$field_meta])*
//                 $field_vis $field_ident: $field_ty,)*
//         }

//         impl $crate::ende::length::Length for $struct_ident {
//             fn length(&self) -> usize {
//                 let mut length = 0;

//                 $(
//                     length += self.$field_ident.length();
//                 )*

//                 length
//             }
//         }

//         impl $crate::ende::encode::Encode for $struct_ident {
//             fn encode_to<W: std::io::Write>(&self, writer: &mut W) -> Result<(), $crate::ende::encode::EncodeError> {
//                 $(
//                     $crate::tri!(self.$field_ident.encode_to(writer));
//                 )*

//                 Ok(())
//             }
//         }
//     };
// }

macro_rules! create {
    // Default
    (
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $struct_ident:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_ident:ident: $field_ty:ty,
            )*
        }
    ) => {
        $crate::create!(@create_struct_with_length_and_encode {
            $(#[$struct_meta])*
            $struct_vis $struct_ident
            $(
                $(#[$field_attr])*
                $field_vis $field_ident $field_ty,
            )*
        });

        impl $crate::Decode for $struct_ident {
            fn decode(src: &[u8]) -> Result<(Self, usize), $crate::errors::DecodeError> {
                let size = 0;

                $(
                    $crate::create!(@match_field
                        {
                            $field_ident,
                            src, length, size
                        }
                    );
                )*

                Ok((
                    Self {
                        $($field_ident,)*
                    },
                    size,
                ))
            }
        }
    };

    (
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $struct_ident:ident {
            $(
                $(#[$field_attr:meta])*
                $(@[length = $length:ident])?
                $(@[count = $count:ident])?
                $field_vis:vis $field_ident:ident: $field_ty:ty,
            )*
        }
    ) => {
        $crate::create!(@create_struct_with_length_and_encode {
            $(#[$struct_meta])*
            $struct_vis $struct_ident
            $(
                $(#[$field_attr])*
                $field_vis $field_ident $field_ty,
            )*
        });

        impl $crate::DecodeWithLength for $struct_ident {
            fn decode(src: &[u8], length: usize) -> Result<(Self, usize), $crate::errors::DecodeError> {
                let size = 0;

                $(
                    $crate::create!(@match_field
                        {
                            $(@[length = $length])?
                            $(@[count = $count])?
                            $field_ident,
                            src, length, size
                        }
                    );
                )*

                Ok((
                    Self {
                        $($field_ident,)*
                    },
                    size,
                ))
            }
        }
    };

    (
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $struct_ident:ident {
            $(
                $(#[$field_attr:meta])*
                $(@[key = $key:ident, length = $length:ident])?
                $field_vis:vis $field_ident:ident: $field_ty:ty,
            )*
        }
    ) => {
        $crate::create!(@create_struct_with_length_and_encode {
            $(#[$struct_meta])*
            $struct_vis $struct_ident
            $(
                $(#[$field_attr])*
                $field_vis $field_ident $field_ty,
            )*
        });

        // Implements DecodeWithLength or Decode depending on the length:
        // If it's unchecked, it implements DecodeWithLength.
        // If it's and ident of a field, it implements Decode.
        $crate::create!(@create_decode_with_key {
            $struct_ident
            $(
                $(#[$field_attr])*
                $(@[key = $key, length = $length])?
                $field_ident $field_ty,
            )*
        });

    };

    // `impl Decode` generation for single field.
    // Example: SmeAddress
    (
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $struct_ident:ident {
            $(
                $(@[$skip:ident])?
                $(#[$skipped_field_attr:meta])*
                $skipped_field_vis:vis $skipped_field_ident:ident: $skipped_field_ty:ty,
            )?
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_ident:ident: $field_ty:ty,
            )*
        }
    ) => {
        $crate::create!(@create_struct_with_length_and_encode {
            $(#[$struct_meta])*
            $struct_vis $struct_ident
            $(
                $(#[$skipped_field_attr])*
                $skipped_field_vis $skipped_field_ident $skipped_field_ty,
            )*
            $(
                $(#[$field_attr])*
                $field_vis $field_ident $field_ty,
            )*
        });

        impl $crate::Decode for $struct_ident {
            fn decode(src: &[u8]) -> Result<(Self, usize), $crate::errors::DecodeError> {
                let size = 0;

                $(
                    $crate::create!(@match_field
                        {
                            $field_ident,
                            src, length, size
                        }
                    );
                )*

                // If a struct contains a @skip field, it is required to have a `new` function, that does not take the skipped field as an argument.
                Ok((
                    Self::new(
                        $($field_ident,)*
                    ),
                    size,
                ))
            }
        }
    };

    // Skip `impl Decode` generation for the whole struct.
    // @skip: must be applied before the docs.
    // Every other attribute must be applied after the docs.
    (
        @[$skip:ident]
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $struct_ident:ident {
            $(
                $(@[$skip0:ident])?
                $(#[$field_attr:meta])*
                $(@[length = $length:ident])?
                $(@[count = $count:ident])?
                $(@[key = $key:ident, length = $length0:ident])?
                $field_vis:vis $field_ident:ident: $field_ty:ty,
            )*
        }
    ) => {
        $crate::create!(@create_struct_with_length_and_encode {
            $(#[$struct_meta])*
            $struct_vis $struct_ident
            $(
                $(#[$field_attr])*
                $field_vis $field_ident $field_ty,
            )*
        });
    };

    // Impl `Length`, `Encode` and `Decode` for a struct, based on its Into/From u8
    // The struct must be `Copy`, `Into<u8>` and `From<u8>`
    (
        @[repr = u8]
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $struct_ident:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_ident:ident: $field_ty:ty,
            )*
        }
    ) => {
        $(#[$struct_meta])*
        $struct_vis struct $struct_ident {
            $(
                $(#[$field_attr])*
                $field_vis $field_ident: $field_ty,
            )*
        }

        $crate::create!(@repr{
            $struct_ident, u8
        });
    };

    (@create_struct_with_length_and_encode {
        $(#[$struct_meta:meta])*
        $struct_vis:vis $struct_ident:ident
        $(
            $(#[$field_attr:meta])*
            $field_vis:vis $field_ident:ident $field_ty:ty,
        )*

    }) => {
        $(#[$struct_meta])*
        $struct_vis struct $struct_ident {
            $(
                $(#[$field_attr])*
                $field_vis $field_ident: $field_ty,
            )*
        }

        impl $crate::Length for $struct_ident {
            fn length(&self) -> usize {
                let mut length = 0;

                $(
                    length +=  $crate::Length::length(&self.$field_ident);
                )*

                length
            }
        }

        impl $crate::Encode for $struct_ident {
            fn encode(&self, dst: &mut [u8]) -> usize {
                let size = 0;

                $(
                    let size = $crate::EncodeExt::encode_move(&self.$field_ident, dst, size);
                )*

                size
            }
        }
    };

    (@create_decode_with_key {
        $struct_ident:ident
        $(
            $(#[$field_attr:meta])*
            $(@[key = $key:ident, length = unchecked])?
            $field_ident:ident $field_ty:ty,
        )*
    }) => {
        impl $crate::DecodeWithLength for $struct_ident {
            fn decode(src: &[u8], length: usize) -> Result<(Self, usize), $crate::errors::DecodeError> {
                let size = 0;

                $(
                    $crate::create!(@match_field
                        {
                            $(@[key = $key, length = unchecked])?
                            $field_ident,
                            src, length, size
                        }
                    );
                )*

                Ok((
                    Self {
                        $($field_ident,)*
                    },
                    size,
                ))
            }
        }
    };

    (@create_decode_with_key {
        $struct_ident:ident
        $(
            $(#[$field_attr:meta])*
            $(@[key = $key:ident, length = $length:ident])?
            $field_ident:ident $field_ty:ty,
        )*
    }) => {
        impl $crate::Decode for $struct_ident {
            fn decode(src: &[u8]) -> Result<(Self, usize), $crate::errors::DecodeError> {
                let size = 0;

                $(
                    $crate::create!(@match_field
                        {
                            $(@[key = $key, length = $length])?
                            $field_ident,
                            src, length, size
                        }
                    );
                )*

                Ok((
                    Self {
                        $($field_ident,)*
                    },
                    size,
                ))
            }
        }
    };

    // Example: BroadcastSmResp, SubmitSm
    (@match_field {
        @[length = unchecked]
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::DecodeWithLengthExt::decode_move(
            $src, $len.saturating_sub($size), $size
        )?;
    };

    // Example: AlertNotification
    (@match_field {
        @[length = checked]
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::DecodeExt::length_checked_decode_move(
            $src, $len.saturating_sub($size), $size
        )?
        .map(|(this, size)| (Some(this), size))
        .unwrap_or((None, $size));
    };

    // Example: SubmitSm
    (@match_field {
        @[length = $length_ident:ident]
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::DecodeWithLengthExt::decode_move(
            $src, $length_ident as usize, $size
        )?;
    };

    // Example: Command
    (@match_field {
        @[key = $key:ident, length = unchecked]
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::DecodeWithKeyOptionalExt::decode_move(
            $key, $src, $len.saturating_sub($size), $size
        )?
        .map(|(this, size)| (Some(this), size))
        .unwrap_or((None, $size));
    };

    // Example: TLV
    (@match_field {
        @[key = $key:ident, length = $length_ident:ident]
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::DecodeWithKeyExt::optional_length_checked_decode_move(
            $key, $src, $length_ident as usize, $size
        )?
        .map(|(this, size)| (Some(this), size))
        .unwrap_or((None, $size));
    };

    // Example: SubmitMultiResp
    (@match_field {
        @[count = $count:ident]
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::DecodeExt::counted_move(
            $src, $count as usize, $size
        )?;
    };

    // Example: CancelSm
    (@match_field {
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::DecodeExt::decode_move($src, $size)?;
    };

    // Enums u8
    (
        #[repr(u8)]
        $(#[$enum_meta:meta])*
        $enum_vis:vis enum $enum_ident:ident {
            $($enum_body:tt)*
        }
    ) => {
        #[repr(u8)]
        $(#[$enum_meta])*
        $enum_vis enum $enum_ident {
            $($enum_body)*
        }

        $crate::create!(@repr{
            $enum_ident, u8
        });
    };

    // Enums u16
    (
        #[repr(u16)]
        $(#[$enum_meta:meta])*
        $enum_vis:vis enum $enum_ident:ident {
            $($enum_body:tt)*
        }
    ) => {
        #[repr(u16)]
        $(#[$enum_meta])*
        $enum_vis enum $enum_ident {
            $($enum_body)*
        }

        $crate::create!(@repr{
            $enum_ident, u16
        });
    };

    // Enums u32
    (
        #[repr(u32)]
        $(#[$enum_meta:meta])*
        $enum_vis:vis enum $enum_ident:ident {
            $($enum_body:tt)*
        }
    ) => {
        #[repr(u32)]
        $(#[$enum_meta])*
        $enum_vis enum $enum_ident {
            $($enum_body)*
        }

        $crate::create!(@repr{
            $enum_ident, u32
        });
    };

    (@repr {
        $name:ident, $repr:ident
    }) => {
        impl $crate::Length for $name {
            fn length(&self) -> usize {
                $repr::from(*self).length()
            }
        }

        impl $crate::Encode for $name {
            fn encode(&self, dst: &mut [u8]) -> usize {
                $repr::from(*self).encode(dst)
            }
        }

        impl $crate::Decode for $name {
            fn decode(src: &[u8]) -> Result<(Self, usize), $crate::errors::DecodeError> {
                $repr::decode(src).map(|(this, size)| (Self::from(this), size))
            }
        }
    };
}

pub(super) use create;
