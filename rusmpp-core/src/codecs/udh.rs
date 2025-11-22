/// UDH Type
#[derive(Debug)]
pub enum UdhType {
    /// 8-bit reference number UDH
    EightBit,
    /// 16-bit reference number UDH
    SixteenBit,
}

impl UdhType {
    /// Returns the length of the UDH type in bytes.
    // XXX: Codecs rely on this value to be correct. Using bad (very large) values will cause underflow.
    pub(crate) const fn length(&self) -> usize {
        match self {
            UdhType::EightBit => EightBitUdh::length(),
            UdhType::SixteenBit => SixteenBitUdh::length(),
        }
    }

    pub(crate) const fn udh_unchecked(
        &self,
        reference: u16,
        total_parts: u8,
        part_number: u8,
    ) -> Udh {
        match self {
            UdhType::EightBit => {
                let reference_8bit = (reference & 0xFF) as u8;
                Udh::EightBit(EightBitUdh::new_unchecked(
                    reference_8bit,
                    total_parts,
                    part_number,
                ))
            }
            UdhType::SixteenBit => Udh::SixteenBit(SixteenBitUdh::new_unchecked(
                reference,
                total_parts,
                part_number,
            )),
        }
    }
}

/// User Data Header (UDH)
#[derive(Debug)]
pub enum Udh {
    /// 8-bit reference number UDH
    EightBit(EightBitUdh),
    /// 16-bit reference number UDH
    SixteenBit(SixteenBitUdh),
}

impl Udh {
    /// Converts the UDH to its byte representation.
    pub(crate) const fn bytes(&self) -> UdhBytes {
        match self {
            Udh::EightBit(udh) => UdhBytes::EightBit(udh.bytes()),
            Udh::SixteenBit(udh) => UdhBytes::SixteenBit(udh.bytes()),
        }
    }
}

/// UDH Bytes representation
#[derive(Debug)]
pub enum UdhBytes {
    /// 8-bit UDH bytes
    EightBit([u8; 6]),
    /// 16-bit UDH bytes
    SixteenBit([u8; 7]),
}

impl UdhBytes {
    /// Returns the byte slice representation of the UDH.
    pub(crate) const fn as_bytes(&self) -> &[u8] {
        match self {
            UdhBytes::EightBit(bytes) => bytes,
            UdhBytes::SixteenBit(bytes) => bytes,
        }
    }
}

/// 8-bit UDH
///
/// 8-bit reference number (IEI = 0x00)
///
/// ```txt
/// 05 00 03 RR TP PN
/// │  │  │  │  │  └─ Part number
/// │  │  │  │  └──── Total parts
/// │  │  │  └─────── 8-bit reference number (1 byte)
/// │  │  └────────── IE Data Length = 3
/// │  └───────────── IEI = 00 (8-bit reference)
/// └──────────────── UDH length = 5
/// ```
#[derive(Debug)]
pub struct EightBitUdh {
    reference: u8,
    total_parts: u8,
    part_number: u8,
}

/// Errors that can occur when creating a [`EightBitUdh`].
#[derive(Debug)]
pub enum EightBitUdhError {
    /// The part number is zero.
    PartNumberZero,
    /// The part number exceeds the total number of parts.
    PartNumberExceedsTotalParts(u8, u8),
    /// The total number of parts is zero.
    TotalPartsZero,
}

impl EightBitUdh {
    /// Creates a new [`EightBitUdh`].
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if the invariants are satisfied.
    /// - `Err(EightBitUdhError)` if any invariant is violated.
    const fn new(
        reference: u8,
        total_parts: u8,
        part_number: u8,
    ) -> Result<Self, EightBitUdhError> {
        Self::new_unchecked(reference, total_parts, part_number).assert()
    }

    /// Asserts the invariants of the UDH.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if the invariants are satisfied.
    /// - `Err(EightBitUdhError)` if any invariant is violated.
    const fn assert(self) -> Result<Self, EightBitUdhError> {
        if self.part_number == 0 {
            return Err(EightBitUdhError::PartNumberZero);
        }

        if self.part_number > self.total_parts {
            return Err(EightBitUdhError::PartNumberExceedsTotalParts(
                self.part_number,
                self.total_parts,
            ));
        }

        if self.total_parts == 0 {
            return Err(EightBitUdhError::TotalPartsZero);
        }

        Ok(self)
    }

    /// Creates a new [`EightBitUdh`] without checking invariants.
    const fn new_unchecked(reference: u8, total_parts: u8, part_number: u8) -> Self {
        Self {
            reference,
            total_parts,
            part_number,
        }
    }

    /// Length of the UDH in bytes.
    const fn length() -> usize {
        6
    }

    /// Converts the UDH to its byte representation.
    const fn bytes(&self) -> [u8; 6] {
        [
            0x05, // UDH length (following bytes = 5)
            0x00, // IEI = 8-bit ref number
            0x03, // IE Data Length = 3 bytes
            self.reference,
            self.total_parts,
            self.part_number,
        ]
    }

    /// Decodes a [`EightBitUdh`] from a byte slice without checking invariants.
    ///
    /// # Returns
    ///
    /// - `Some((Self, usize))` if decoding is successful.
    /// - `None` if the byte slice is too short or does not match the expected format.
    const fn decode_unchecked(bytes: &[u8]) -> Option<(Self, usize)> {
        if bytes.len() < Self::length() {
            return None;
        }

        if bytes[0] != 0x05 || bytes[1] != 0x00 || bytes[2] != 0x03 {
            return None;
        }

        let reference = bytes[3];
        let total_parts = bytes[4];
        let part_number = bytes[5];

        Some((
            Self::new_unchecked(reference, total_parts, part_number),
            Self::length(),
        ))
    }

    /// Decodes a [`EightBitUdh`] from a byte slice, checking invariants.
    ///
    /// # Returns
    ///
    /// - `Some(Ok((Self, usize)))` if decoding and invariants are successful.
    /// - `Some(Err(EightBitUdhError))` if invariants are violated.
    /// - `None` if the byte slice is too short or does not match the expected format.
    const fn decode(bytes: &[u8]) -> Option<Result<(Self, usize), EightBitUdhError>> {
        match Self::decode_unchecked(bytes) {
            Some((udh, len)) => match udh.assert() {
                Ok(udh) => Some(Ok((udh, len))),
                Err(err) => Some(Err(err)),
            },
            None => None,
        }
    }
}

/// 16-bit UDH
///
/// 16-bit reference number (IEI = 0x08)
///
/// ```txt
/// 06 08 04 RRH RRL TP PN
/// │  │  │  │   │   │  └─ Part number
/// │  │  │  │   │   └──── Total parts
/// │  │  │  │   └──────── Low byte of 16-bit reference
/// │  │  │  └──────────── High byte of 16-bit reference
/// │  │  └─────────────── IE Data Length = 4
/// │  └────────────────── IEI = 08 (16-bit reference)
/// └───────────────────── UDH length = 6
/// ```
#[derive(Debug)]
pub struct SixteenBitUdh {
    reference: u16,
    total_parts: u8,
    part_number: u8,
}

/// Errors that can occur when creating a [`SixteenBitUdh`].
#[derive(Debug)]
pub enum SixteenBitUdhError {
    /// The part number is zero.
    PartNumberZero,
    /// The part number exceeds the total number of parts.
    PartNumberExceedsTotalParts(u8, u8),
    /// The total number of parts is zero.
    TotalPartsZero,
}

impl SixteenBitUdh {
    /// Creates a new [`SixteenBitUdh`].
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if the invariants are satisfied.
    /// - `Err(SixteenBitUdhError)` if any invariant is violated.
    const fn new(
        reference: u16,
        total_parts: u8,
        part_number: u8,
    ) -> Result<Self, SixteenBitUdhError> {
        Self::new_unchecked(reference, total_parts, part_number).assert()
    }

    /// Asserts the invariants of the UDH.
    ///
    /// # Returns
    ///
    /// - `Ok(Self)` if the invariants are satisfied.
    /// - `Err(SixteenBitUdhError)` if any invariant is violated.
    const fn assert(self) -> Result<Self, SixteenBitUdhError> {
        if self.part_number == 0 {
            return Err(SixteenBitUdhError::PartNumberZero);
        }

        if self.part_number > self.total_parts {
            return Err(SixteenBitUdhError::PartNumberExceedsTotalParts(
                self.part_number,
                self.total_parts,
            ));
        }

        if self.total_parts == 0 {
            return Err(SixteenBitUdhError::TotalPartsZero);
        }

        Ok(self)
    }

    /// Creates a new [`SixteenBitUdh`] without checking invariants.
    const fn new_unchecked(reference: u16, total_parts: u8, part_number: u8) -> Self {
        Self {
            reference,
            total_parts,
            part_number,
        }
    }

    /// Length of the UDH in bytes.
    const fn length() -> usize {
        7
    }

    /// Converts the UDH to its byte representation.
    const fn bytes(&self) -> [u8; 7] {
        [
            0x06,                          // UDH length (following bytes = 6)
            0x08,                          // IEI = 16-bit ref number
            0x04,                          // IE Data Length = 4 bytes
            (self.reference >> 8) as u8,   // Ref high
            (self.reference & 0xFF) as u8, // Ref low
            self.total_parts,
            self.part_number,
        ]
    }

    /// Decodes a [`SixteenBitUdh`] from a byte slice without checking invariants.
    ///
    /// # Returns
    ///
    /// - `Some((Self, usize))` if decoding is successful.
    /// - `None` if the byte slice is too short or does not match the expected format.
    const fn decode_unchecked(bytes: &[u8]) -> Option<(Self, usize)> {
        if bytes.len() < Self::length() {
            return None;
        }

        if bytes[0] != 0x06 || bytes[1] != 0x08 || bytes[2] != 0x04 {
            return None;
        }

        let reference = ((bytes[3] as u16) << 8) | (bytes[4] as u16);
        let total_parts = bytes[5];
        let part_number = bytes[6];

        Some((
            Self::new_unchecked(reference, total_parts, part_number),
            Self::length(),
        ))
    }

    /// Decodes a [`SixteenBitUdh`] from a byte slice, checking invariants.
    ///
    /// # Returns
    ///
    /// - `Some(Ok((Self, usize)))` if decoding and invariants are successful.
    /// - `Some(Err(SixteenBitUdhError))` if invariants are violated.
    /// - `None` if the byte slice is too short or does not match the expected format.
    const fn decode(bytes: &[u8]) -> Option<Result<(Self, usize), SixteenBitUdhError>> {
        match Self::decode_unchecked(bytes) {
            Some((udh, len)) => match udh.assert() {
                Ok(udh) => Some(Ok((udh, len))),
                Err(err) => Some(Err(err)),
            },
            None => None,
        }
    }
}

impl core::fmt::Display for EightBitUdhError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            EightBitUdhError::PartNumberZero => {
                write!(f, "Part number cannot be zero")
            }
            EightBitUdhError::PartNumberExceedsTotalParts(part_number, total_parts) => {
                write!(
                    f,
                    "Part number {} exceeds total parts {}",
                    part_number, total_parts
                )
            }
            EightBitUdhError::TotalPartsZero => {
                write!(f, "Total parts cannot be zero")
            }
        }
    }
}

impl core::fmt::Display for SixteenBitUdhError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            SixteenBitUdhError::PartNumberZero => {
                write!(f, "Part number cannot be zero")
            }
            SixteenBitUdhError::PartNumberExceedsTotalParts(part_number, total_parts) => {
                write!(
                    f,
                    "Part number {} exceeds total parts {}",
                    part_number, total_parts
                )
            }
            SixteenBitUdhError::TotalPartsZero => {
                write!(f, "Total parts cannot be zero")
            }
        }
    }
}

impl core::error::Error for SixteenBitUdhError {}
