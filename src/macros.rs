/// Implements [`Length`](crate::encode::Length), [`Encode`](crate::encode::Encode), [`Decode`](crate::decode::Decode) and `TestInstance` for structs and enums.
///
/// # Enums
///
/// Enums must be annotated with `#[repr(u8)]`, `#[repr(u16)]`, or `#[repr(u32)]`, and implement the appropriate `Into`/`From` conversions.
///
/// ## Decoding attributes
///
/// - `@[skip_test]`: Skip impl `TestInstance` for the enum.
///
/// # Structs
///
/// ## Decoding attributes
///
/// ### Struct attributes
///
/// - `@[skip]`: Skip impl `Decode` for the struct.
/// - `@[skip_test]`: Skip impl `TestInstance` for the struct.
/// - `@[repr = u8]`: Use the `From<u8>`/`Into<u8>` representation for decoding.
///
/// ### Field attributes
///
/// - `@[skip]`: Skip decoding the field (requires a corresponding `new` constructor that does not take the skipped field as an argument).
/// - `@[length = unchecked]`: Decode without length checks.
/// - `@[length = checked]`: Decode using `length_checked_decode`.
/// - `@[length = ident]`: Use the value of another field (`ident`) as the length for decoding.
/// - `@[key = ident, length = unchecked]`: Decode using a key and unchecked length.
/// - `@[key = ident, length = ident]`: Decode using a key and the value of another field (`ident`) as the length.
/// - `@[count = ident]`: Decode a vector of values, where `ident` is the number of elements to decode.
///
/// # Examples
///
/// See `tests/expand`.
#[macro_export]
#[doc(hidden)]
macro_rules! create {
    // Default
    (
        $(@[$skip_test:ident])?
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $struct_ident:ident {
            $(
                $(#[$field_attr:meta])*
                $field_vis:vis $field_ident:ident: $field_ty:ty,
            )*
        }
    ) => {
        $crate::create!(@create_struct_with_length_and_encode_and_test {
            $(#[$struct_meta])*
            $struct_vis $struct_ident
            $(
                $(#[$field_attr])*
                $field_vis $field_ident $field_ty,
            )*
        });

        $crate::create!(@impl_test_instances {
            $(@[$skip_test])?
            $struct_ident
        });

        impl $crate::decode::Decode for $struct_ident {
            fn decode(src: &[u8]) -> Result<(Self, usize), $crate::decode::DecodeError> {
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
        $(@[$skip_test:ident])?
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
        $crate::create!(@create_struct_with_length_and_encode_and_test {
            $(#[$struct_meta])*
            $struct_vis $struct_ident
            $(
                $(#[$field_attr])*
                $field_vis $field_ident $field_ty,
            )*
        });

        $crate::create!(@impl_test_instances {
            $(@[$skip_test])?
            $struct_ident
        });

        impl $crate::decode::DecodeWithLength for $struct_ident {
            fn decode(src: &[u8], length: usize) -> Result<(Self, usize), $crate::decode::DecodeError> {
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
        $(@[$skip_test:ident])?
        $(#[$struct_meta:meta])*
        $struct_vis:vis struct $struct_ident:ident {
            $(
                $(#[$field_attr:meta])*
                $(@[key = $key:ident, length = $length:ident])?
                $field_vis:vis $field_ident:ident: $field_ty:ty,
            )*
        }
    ) => {
        $crate::create!(@create_struct_with_length_and_encode_and_test {
            $(#[$struct_meta])*
            $struct_vis $struct_ident
            $(
                $(#[$field_attr])*
                $field_vis $field_ident $field_ty,
            )*
        });

        $crate::create!(@impl_test_instances {
            $(@[$skip_test])?
            $struct_ident
        });

        // Implements DecodeWithLength or Decode depending on the length:
        // If it's unchecked, it implements DecodeWithLength.
        // If it's and ident of a field, it implements Decode.
        $crate::create!(@impl_decode_with_key {
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
        $(@[$skip_test:ident])?
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
        $crate::create!(@create_struct_with_length_and_encode_and_test {
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

        $crate::create!(@impl_test_instances {
            $(@[$skip_test])?
            $struct_ident
        });

        impl $crate::decode::Decode for $struct_ident {
            fn decode(src: &[u8]) -> Result<(Self, usize), $crate::decode::DecodeError> {
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
        $(@[$skip_test:ident])?
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
        $crate::create!(@create_struct_with_length_and_encode_and_test {
            $(#[$struct_meta])*
            $struct_vis $struct_ident
            $(
                $(#[$field_attr])*
                $field_vis $field_ident $field_ty,
            )*
        });

        $crate::create!(@impl_test_instances {
            $(@[$skip_test])?
            $struct_ident
        });
    };

    // Impl `Length`, `Encode` and `Decode` for a struct, based on its Into/From u8
    // The struct must be `Copy`, `Into<u8>` and `From<u8>`
    (
        @[repr = u8]
        $(@[$skip_test:ident])?
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

        $crate::create!(@impl_test_instances {
            $(@[$skip_test])?
            $struct_ident
        });
    };

    (@create_struct_with_length_and_encode_and_test {
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

        impl $crate::encode::Length for $struct_ident {
            fn length(&self) -> usize {
                let mut length = 0;

                $(
                    length +=  $crate::encode::Length::length(&self.$field_ident);
                )*

                length
            }
        }

        impl $crate::encode::Encode for $struct_ident {
            fn encode(&self, dst: &mut [u8]) -> usize {
                let size = 0;

                $(
                    let size = $crate::encode::EncodeExt::encode_move(&self.$field_ident, dst, size);
                )*

                size
            }
        }
    };

    (@impl_test_instances {
        $name:ident
    }) => {
        #[cfg(test)]
        impl $crate::tests::TestInstance for $name {}
    };

    (@impl_test_instances {
        @[$skip_test:ident]
        $name:ident
    }) => {};

    (@impl_decode_with_key {
        $struct_ident:ident
        $(
            $(#[$field_attr:meta])*
            $(@[key = $key:ident, length = unchecked])?
            $field_ident:ident $field_ty:ty,
        )*
    }) => {
        impl $crate::decode::DecodeWithLength for $struct_ident {
            fn decode(src: &[u8], length: usize) -> Result<(Self, usize), $crate::decode::DecodeError> {
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

    (@impl_decode_with_key {
        $struct_ident:ident
        $(
            $(#[$field_attr:meta])*
            $(@[key = $key:ident, length = $length:ident])?
            $field_ident:ident $field_ty:ty,
        )*
    }) => {
        impl $crate::decode::Decode for $struct_ident {
            fn decode(src: &[u8]) -> Result<(Self, usize), $crate::decode::DecodeError> {
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
        let ($field_ident, $size) = $crate::decode::DecodeWithLengthExt::decode_move(
            $src, $len.saturating_sub($size), $size
        )?;
    };

    // Example: AlertNotification
    (@match_field {
        @[length = checked]
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::decode::DecodeExt::length_checked_decode_move(
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
        let ($field_ident, $size) = $crate::decode::DecodeWithLengthExt::decode_move(
            $src, $length_ident as usize, $size
        )?;
    };

    // Example: Command
    (@match_field {
        @[key = $key:ident, length = unchecked]
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::decode::DecodeWithKeyOptionalExt::decode_move(
            $key, $src, $len.saturating_sub($size), $size
        )?
        .map(|(this, size)| (Some(this), size))
        .unwrap_or((None, $size));
    };

    // Example: Tlv
    (@match_field {
        @[key = $key:ident, length = $length_ident:ident]
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::decode::DecodeWithKeyExt::optional_length_checked_decode_move(
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
        let ($field_ident, $size) = $crate::decode::DecodeExt::counted_move(
            $src, $count as usize, $size
        )?;
    };

    // Example: CancelSm
    (@match_field {
        $field_ident:ident,
        $src:ident, $len:ident, $size:ident
    }) => {
        let ($field_ident, $size) = $crate::decode::DecodeExt::decode_move($src, $size)?;
    };

    // Enums u8
    (
        #[repr(u8)]
        $(@[$skip_test:ident])?
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

        $crate::create!(@impl_test_instances {
            $(@[$skip_test])?
            $enum_ident
        });
    };

    // Enums u16
    (
        #[repr(u16)]
        $(@[$skip_test:ident])?
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

        $crate::create!(@impl_test_instances {
            $(@[$skip_test])?
            $enum_ident
        });
    };

    // Enums u32
    (
        #[repr(u32)]
        $(@[$skip_test:ident])?
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

        $crate::create!(@impl_test_instances {
            $(@[$skip_test])?
            $enum_ident
        });
    };

    (@repr {
        $name:ident, $repr:ident
    }) => {
        impl $crate::encode::Length for $name {
            fn length(&self) -> usize {
                $repr::from(*self).length()
            }
        }

        impl $crate::encode::Encode for $name {
            fn encode(&self, dst: &mut [u8]) -> usize {
                $repr::from(*self).encode(dst)
            }
        }

        impl $crate::decode::Decode for $name {
            fn decode(src: &[u8]) -> Result<(Self, usize), $crate::decode::DecodeError> {
                $repr::decode(src).map(|(this, size)| (Self::from(this), size))
            }
        }
    };
}
