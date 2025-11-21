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
    // XXX: Encoders rely on this value to be correct. Using bad (very large) values will cause underflow.
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

pub enum UdhBytes {
    EightBit([u8; 6]),
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

impl EightBitUdh {
    /// Creates a new [`EightBitUdh`].
    ///
    /// # Returns
    ///
    /// - `Some(EightBitUdh)` if the parameters are valid.
    /// - `None` if the parameters are invalid (e.g., part_number is 0 or greater than total_parts).
    #[allow(dead_code)] // allowed dead_code to keep the invariants documented
    const fn new(reference: u8, total_parts: u8, part_number: u8) -> Option<Self> {
        if part_number == 0 || part_number > total_parts || total_parts == 0 {
            return None;
        }

        Some(Self::new_unchecked(reference, total_parts, part_number))
    }

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

impl SixteenBitUdh {
    /// Creates a new [`SixteenBitUdh`].
    ///
    /// # Returns
    ///
    /// - `Some(SixteenBitUdh)` if the parameters are valid.
    /// - `None` if the parameters are invalid (e.g., part_number is 0 or greater than total_parts).
    #[allow(dead_code)] // allowed dead_code to keep the invariants documented
    fn new(reference: u16, total_parts: u8, part_number: u8) -> Option<Self> {
        if part_number == 0 || part_number > total_parts || total_parts == 0 {
            return None;
        }

        Some(Self::new_unchecked(reference, total_parts, part_number))
    }

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
}
