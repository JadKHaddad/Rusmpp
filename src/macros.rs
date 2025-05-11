/// Implement the [`Length`](crate::encode::Length), [`Encode`](crate::encode::Encode) and [`Decode`](crate::decode::Decode) traits for enums and structs.
///
/// # Implementation
///
/// ## Enums
///
/// - Must be `#[repr(u8)]`, `#[repr(u16)]` or `#[repr(u32)]`.
/// - Must implement (`Into<u8>`, `From<u8>`), (`Into<u16>`, `From<u16>`) or (`Into<u32>`, `From<u32>`) traits.
///
/// ```ignore
/// create! {
///     #[repr(u8)]
///     pub enum DestFlag {
///         SmeAddress = 0x01,
///         DistributionListName = 0x02,
///         Other(u8),
///     }
/// }
/// ```
///
/// `expands to:`
///
/// ```ignore
/// #[repr(u8)]
/// pub enum DestFlag {
///     SmeAddress = 0x01,
///     DistributionListName = 0x02,
///     Other(u8),
/// }
///
/// impl crate::encode::Length for DestFlag {
///     fn length(&self) -> usize {
///         u8::from(*self).length()
///     }
/// }
///
/// impl crate::encode::Encode for DestFlag {
///     fn encode(&self, dst: &mut [u8]) -> usize {
///         u8::from(*self).encode(dst)
///     }
/// }
///
/// impl crate::decode::Decode for DestFlag {
///     fn decode(src: &[u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
///         u8::decode(src).map(|(this, size)| (Self::from(this), size))
///     }
/// }
/// ```
///
/// ## Structs:
/// ```ignore
/// create! {
///     pub struct CancelSm {
///         pub service_type: ServiceType,
///         pub message_id: COctetString<1, 65>,
///         pub other: u8,
///     }
/// }
/// ```
///
/// `expands to:`
///
/// ```ignore
/// pub struct CancelSm {
///     pub service_type: ServiceType,
///     pub message_id: COctetString<1, 65>,
///     pub other: u8,
/// }
///
/// impl crate::encode::Length for CancelSm {
///     fn length(&self) -> usize {
///         let mut length = 0;
///         length += crate::encode::Length::length(&self.service_type);
///         length += crate::encode::Length::length(&self.message_id);
///         length += crate::encode::Length::length(&self.other);
///         length
///     }
/// }
///
/// impl crate::encode::Encode for CancelSm {
///     fn encode(&self, dst: &mut [u8]) -> usize {
///         let size = 0;
///         let size = crate::encode::EncodeExt::encode_move(
///             &self.service_type,
///             dst,
///             size,
///         );
///         let size = crate::encode::EncodeExt::encode_move(
///             &self.message_id,
///             dst,
///             size,
///         );
///         let size = crate::encode::EncodeExt::encode_move(&self.other, dst, size);
///         size
///     }
/// }
///
/// impl crate::decode::Decode for CancelSm {
///     fn decode(src: &[u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
///         let size = 0;
///         let (service_type, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         let (message_id, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         let (other, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         Ok((
///             Self {
///                 service_type,
///                 message_id,
///                 other,
///             },
///             size,
///         ))
///     }
/// }
/// ```
///
/// ### ```@[repr = u8]```
///
/// Implement [`Decode`](crate::decode::Decode) for a struct based on its `Into<u8>` and `From<u8>` traits.
///
/// ```ignore
/// create! {
///     @[repr = u8]
///     pub struct CallbackNumPresInd {
///         pub presentation: Presentation,
///         pub screening: Screening,
///     }
/// }
/// ```
///
/// `expands to:`
///
/// ```ignore
/// pub struct CallbackNumPresInd {
///     pub presentation: Presentation,
///     pub screening: Screening,
/// }
///
/// impl crate::encode::Length for CallbackNumPresInd {
///     fn length(&self) -> usize {
///         u8::from(*self).length()
///     }
/// }
///
/// impl crate::encode::Encode for CallbackNumPresInd {
///     fn encode(&self, dst: &mut [u8]) -> usize {
///         u8::from(*self).encode(dst)
///     }
/// }
///
/// impl crate::decode::Decode for CallbackNumPresInd {
///     fn decode(src: &[u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
///         u8::decode(src).map(|(this, size)| (Self::from(this), size))
///     }
/// }
/// ```
///
/// ### `@[skip]`
///
/// Skip a marked field from the `impl Decode` generation.
///
/// The struct must have a `new` function that does not take the skipped field as an argument.
///
/// ```ignore
/// create! {
///     pub struct DistributionListName {
///         @[skip]
///         dest_flag: DestFlag,
///         pub dl_name: COctetString<1, 21>,
///     }
/// }
/// ```
/// `expands to:`
///
/// ```ignore
/// impl crate::decode::Decode for DistributionListName {
///     fn decode(src: &[u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
///         let size = 0;
///         let (dl_name, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         Ok((Self::new(dl_name), size))
///     }
/// }
/// ```
/// ### `@[length = unchecked]`
///
/// Decode a field without checking its length.
///
/// ```ignore
/// crate::create! {
///     pub struct BroadcastAreaIdentifier {
///         pub format: BroadcastAreaFormat,
///         @[length = unchecked]
///         pub area: OctetString<0, 100>,
///     }
/// }
/// ```
///
/// `expands to:`
///
/// ```ignore
/// impl crate::decode::DecodeWithLength for BroadcastAreaIdentifier {
///     fn decode(
///         src: &[u8],
///         length: usize,
///     ) -> Result<(Self, usize), crate::decode::DecodeError> {
///         let size = 0;
///         let (format, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         let (area, size) = crate::decode::DecodeWithLengthExt::decode_move(
///             src,
///             length.saturating_sub(size),
///             size,
///         )?;
///         Ok((Self { format, area }, size))
///     }
/// }
/// ```
///
/// ### `@[length = checked]`
///
/// Decode a field with `length_checked_decode`.
///
/// ```ignore
/// create! {
///     pub struct MsValidity {
///         pub validity_behavior: MsValidityBehavior,
///         @[length = checked]
///         pub validity_information: Option<MsValidityInformation>,
///     }
/// }
/// ```
///
/// `expands to:`
///
/// ```ignore
/// impl crate::decode::DecodeWithLength for MsValidity {
///     fn decode(
///         src: &[u8],
///         length: usize,
///     ) -> Result<(Self, usize), crate::decode::DecodeError> {
///         let size = 0;
///         let (validity_behavior, size) = crate::decode::DecodeExt::decode_move(
///             src,
///             size,
///         )?;
///         let (validity_information, size) = crate::decode::DecodeExt::length_checked_decode_move(
///                 src,
///                 length.saturating_sub(size),
///                 size,
///             )?
///             .map(|(this, size)| (Some(this), size))
///             .unwrap_or((None, size));
///         Ok((
///             Self {
///                 validity_behavior,
///                 validity_information,
///             },
///             size,
///         ))
///     }
/// }
/// ```
///
/// ### `@[length = ident]`
///
/// Decode a field with length based on another field (`ident`) in struct.
///
/// ```ignore
/// create! {
///     pub struct SubmitSm {
///         pub other: u8,
///         sm_length: u8,
///         @[length = sm_length]
///         short_message: OctetString<0, 255>,
///     }
/// }
/// ```
///
/// `expands to:`
///
/// ```ignore
/// impl crate::decode::DecodeWithLength for SubmitSm {
///     fn decode(
///         src: &[u8],
///         length: usize,
///     ) -> Result<(Self, usize), crate::decode::DecodeError> {
///         let size = 0;
///         let (other, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         let (sm_length, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         let (short_message, size) = crate::decode::DecodeWithLengthExt::decode_move(
///             src,
///             sm_length as usize,
///             size,
///         )?;
///         Ok((
///             Self {
///                 other,
///                 sm_length,
///                 short_message,
///             },
///             size,
///         ))
///     }
/// }
/// ```
///
/// ### `@[key = ident, length = unchecked]`
///
/// Decode a field with key (`ident`) and length without checking its length.
///
/// ```ignore
/// create! {
///     pub struct Command {
///         command_id: CommandId,
///         pub command_status: CommandStatus,
///         pub sequence_number: u32,
///         @[key = command_id, length = unchecked]
///         pdu: Option<Pdu>,
///     }
/// }
/// ```
///
/// `expands to:`
///
/// ```ignore
/// impl crate::decode::DecodeWithLength for Command {
///     fn decode(
///         src: &[u8],
///         length: usize,
///     ) -> Result<(Self, usize), crate::decode::DecodeError> {
///         let size = 0;
///         let (command_id, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         let (command_status, size) = crate::decode::DecodeExt::decode_move(
///             src,
///             size,
///         )?;
///         let (sequence_number, size) = crate::decode::DecodeExt::decode_move(
///             src,
///             size,
///         )?;
///         let (pdu, size) = crate::decode::DecodeWithKeyOptionalExt::decode_move(
///                 command_id,
///                 src,
///                 length.saturating_sub(size),
///                 size,
///             )?
///             .map(|(this, size)| (Some(this), size))
///             .unwrap_or((None, size));
///         Ok((
///             Self {
///                 command_id,
///                 command_status,
///                 sequence_number,
///                 pdu,
///             },
///             size,
///         ))
///     }
/// }
/// ```
///
/// ### `@[key = k_ident, length = l_ident]`
///
/// Decode a field with key (`k_ident`) and length based on another field (`l_ident`) in struct.
///
/// ```ignore
/// create! {
///     pub struct Tlv {
///         tag: TlvTag,
///         value_length: u16,
///         @[key = tag, length = value_length]
///         value: Option<TlvValue>,
///     }
/// }
/// ```
///
/// `expands to:`
///
/// ```ignore
/// impl crate::decode::Decode for Tlv {
///     fn decode(src: &[u8]) -> Result<(Self, usize), crate::decode::DecodeError> {
///         let size = 0;
///         let (tag, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         let (value_length, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         let (value, size) = crate::decode::DecodeWithKeyExt::optional_length_checked_decode_move(
///                 tag,
///                 src,
///                 value_length as usize,
///                 size,
///             )?
///             .map(|(this, size)| (Some(this), size))
///             .unwrap_or((None, size));
///         Ok((Self { tag, value_length, value }, size))
///     }
/// }
/// ```
///
/// ### `@[count = ident]`
///
/// Decode a vector of values with count based on another field (`attr`) in struct.
///
/// ```ignore
/// create! {
///     pub struct SubmitMulti {
///         pub other: u8,
///         number_of_dests: u8,
///         @[count = number_of_dests]
///         dest_address: Vec<DestAddress>,
///     }
/// }
/// ```
///
/// `expands to:`
///
/// ```ignore
/// impl crate::decode::DecodeWithLength for SubmitMulti {
///     fn decode(
///         src: &[u8],
///         length: usize,
///     ) -> Result<(Self, usize), crate::decode::DecodeError> {
///         let size = 0;
///         let (other, size) = crate::decode::DecodeExt::decode_move(src, size)?;
///         let (number_of_dests, size) = crate::decode::DecodeExt::decode_move(
///             src,
///             size,
///         )?;
///         let (dest_address, size) = crate::decode::DecodeExt::counted_move(
///             src,
///             number_of_dests as usize,
///             size,
///         )?;
///         Ok((
///             Self {
///                 other,
///                 number_of_dests,
///                 dest_address,
///             },
///             size,
///         ))
///     }
/// }
/// ```
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

    (@create_decode_with_key {
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

    (@create_decode_with_key {
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

pub(super) use create;
