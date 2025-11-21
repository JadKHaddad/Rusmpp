/// User Data Header (UDH)
#[derive(Debug)]
pub enum Udh<'a> {
    /// 8-bit reference number UDH
    EightBit(EightBitUdh),
    /// 16-bit reference number UDH
    SixteenBit(SixteenBitUdh),
    /// Other UDH represented as raw bytes
    Other(&'a [u8]),
}

impl Udh<'_> {
    /// Length of the UDH in bytes.
    pub(crate) const fn length(&self) -> usize {
        match self {
            Udh::EightBit(udh) => udh.length(),
            Udh::SixteenBit(udh) => udh.length(),
            Udh::Other(bytes) => bytes.len(),
        }
    }

    /// Converts the UDH to its byte representation.
    pub(crate) const fn bytes(&self) -> UdhBytes<'_> {
        match self {
            Udh::EightBit(udh) => UdhBytes::EightBit(udh.bytes()),
            Udh::SixteenBit(udh) => UdhBytes::SixteenBit(udh.bytes()),
            Udh::Other(bytes) => UdhBytes::Other(bytes),
        }
    }
}

pub enum UdhBytes<'a> {
    EightBit([u8; 6]),
    SixteenBit([u8; 7]),
    Other(&'a [u8]),
}

impl<'a> UdhBytes<'a> {
    /// Returns the byte slice representation of the UDH.
    pub(crate) const fn as_bytes(&'a self) -> &'a [u8] {
        match self {
            UdhBytes::EightBit(bytes) => bytes,
            UdhBytes::SixteenBit(bytes) => bytes,
            UdhBytes::Other(bytes) => bytes,
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
    pub const fn new(reference: u8, total_parts: u8, part_number: u8) -> Option<Self> {
        if part_number == 0 || part_number > total_parts || total_parts == 0 {
            return None;
        }

        Some(Self {
            reference,
            total_parts,
            part_number,
        })
    }

    /// Length of the UDH in bytes.
    pub(crate) const fn length(&self) -> usize {
        6
    }

    /// Converts the UDH to its byte representation.
    pub(crate) const fn bytes(&self) -> [u8; 6] {
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
    pub const fn new(reference: u16, total_parts: u8, part_number: u8) -> Option<Self> {
        if part_number == 0 || part_number > total_parts || total_parts == 0 {
            return None;
        }

        Some(Self {
            reference,
            total_parts,
            part_number,
        })
    }

    /// Length of the UDH in bytes.
    pub(crate) const fn length(&self) -> usize {
        7
    }

    pub(crate) const fn bytes(&self) -> [u8; 7] {
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
