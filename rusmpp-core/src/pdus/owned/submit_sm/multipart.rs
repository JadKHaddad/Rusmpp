use alloc::vec::Vec;

use crate::{
    codecs::owned::Encoder,
    types::owned::OctetString,
    values::{ConcatenatedShortMessageType, DataCoding},
};

use super::SubmitSm;

// TODO: We have to deal MessagePayload if set instead of ShortMessage. Do we override? or we ignore it?

#[derive(Debug)]
pub struct SubmitSmMultipartBuilder<'a, E> {
    short_message: &'a [u8],
    sm: SubmitSm,
    encoder: E,
    concatenation_type: ConcatenatedShortMessageType,
}

impl<E> SubmitSmMultipartBuilder<'static, E> {
    pub fn new(sm: SubmitSm, encoder: E) -> SubmitSmMultipartBuilder<'static, E> {
        Self {
            short_message: &[],
            sm,
            encoder,
            concatenation_type: ConcatenatedShortMessageType::u8(0),
        }
    }
}

impl<'a, E> SubmitSmMultipartBuilder<'a, E> {
    pub fn short_message<'b>(self, short_message: &'b [u8]) -> SubmitSmMultipartBuilder<'b, E> {
        SubmitSmMultipartBuilder {
            short_message,
            sm: self.sm,
            encoder: self.encoder,
            concatenation_type: self.concatenation_type,
        }
    }

    /// Sets the reference number for the concatenated short message as [`u8`].
    pub fn reference_u8(mut self, reference: u8) -> Self {
        self.concatenation_type = ConcatenatedShortMessageType::u8(reference);
        self
    }

    /// Sets the reference number for the concatenated short message as [`u16`].
    pub fn reference_u16(mut self, reference: u16) -> Self {
        self.concatenation_type = ConcatenatedShortMessageType::u16(reference);
        self
    }

    /// Sets a custom encoder.
    pub fn encoder<U>(self, encoder: U) -> SubmitSmMultipartBuilder<'a, U> {
        SubmitSmMultipartBuilder {
            short_message: self.short_message,
            sm: self.sm,
            encoder,
            concatenation_type: self.concatenation_type,
        }
    }

    /// Sets the GSM 7-bit unpacked encoder.
    pub fn gsm7_unpacked(self) -> SubmitSmMultipartBuilder<'a, crate::codecs::Gsm7UnpackedCodec> {
        self.encoder(crate::codecs::Gsm7UnpackedCodec::new())
    }
}

impl<'a, E> SubmitSmMultipartBuilder<'a, E>
where
    E: Encoder<&'a [u8]> + 'a,
{
    pub fn build(
        self,
    ) -> Result<impl ExactSizeIterator<Item = SubmitSm>, <E as Encoder<&'a [u8]>>::Error> {
        let encoded = self.encoder.encode(self.short_message)?;

        let part_size = self
            .encoder
            .max_bytes_with_concatenation(self.concatenation_type)
            .get();

        if encoded.len() > part_size {
            let total_parts = encoded.len().div_ceil(part_size);
            let total_parts = total_parts.min(255) as u8;

            return Ok(MultipartIterator::new(
                encoded,
                self.encoder.data_coding(),
                self.sm,
                self.concatenation_type,
                part_size,
                total_parts,
            ));
        }

        Ok(MultipartIterator::new(
            encoded,
            self.encoder.data_coding(),
            self.sm,
            self.concatenation_type,
            part_size,
            1,
        ))
    }
}

struct MultipartIterator {
    idx: usize,
    encoded: Vec<u8>,
    data_coding: DataCoding,
    sm: SubmitSm,
    concatenation_type: ConcatenatedShortMessageType,
    part_size: usize,
    total_parts: u8,
}

impl MultipartIterator {
    const fn new(
        encoded: Vec<u8>,
        data_coding: DataCoding,
        sm: SubmitSm,
        concatenation_type: ConcatenatedShortMessageType,
        part_size: usize,
        total_parts: u8,
    ) -> Self {
        Self {
            idx: 0,
            encoded,
            data_coding,
            sm,
            concatenation_type,
            part_size,
            total_parts,
        }
    }
}

impl Iterator for MultipartIterator {
    type Item = SubmitSm;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.total_parts as usize {
            return None;
        }

        let start = self.idx * self.part_size;
        let end = (start + self.part_size).min(self.encoded.len());
        let chunk = &self.encoded[start..end];

        let part_number = (self.idx + 1) as u8;

        let concatenation = self
            .concatenation_type
            .concatenated_short_message_unchecked(self.total_parts, part_number);

        let mut payload = Vec::with_capacity(concatenation.udh_length() + chunk.len());

        payload.extend_from_slice(concatenation.udh_bytes().as_bytes());
        payload.extend_from_slice(chunk);

        let short_message = OctetString::new_unchecked(payload);

        self.idx += 1;

        Some(
            self.sm
                .clone()
                .with_udhi_indicator()
                .with_short_message(short_message)
                .with_data_coding(self.data_coding),
        )
    }
}

impl ExactSizeIterator for MultipartIterator {
    fn len(&self) -> usize {
        (self.total_parts as usize) - self.idx
    }
}

// TODO: remove this commented code. keep for comments and reference.

// pub fn build<B: FromIterator<SubmitSm>>(self) -> Result<B, <E as Encoder<&'a [u8]>>::Error> {
//     let encoded = self.encoder.encode(self.short_message)?;

//     let should_split = encoded.len() > self.encoder.max_bytes().get();

//     if should_split {
//         let parts = encoded.chunks(
//             self.encoder
//                 .max_bytes_with_concatenation(self.concatenation_type)
//                 .get(),
//         );

//         // XXX: Downcast to u8 to truncate to max 255 parts which we can encode in `total_parts` as `u8`
//         let total_parts = parts.len() as u8;

//         return Ok(parts
//             .into_iter()
//             // XXX: We take only truncated `total_parts` that we can encode in `u8`
//             .take(total_parts as usize)
//             .enumerate()
//             .map(move |(idx, chunk)| {
//                 // XXX: Do not fuck this up
//                 // part_number can not be 0
//                 let part_number = (idx + 1) as u8;

//                 let concatenation = self
//                     .concatenation_type
//                     .concatenated_short_message_unchecked(total_parts, part_number);

//                 let mut payload = Vec::with_capacity(concatenation.udh_length() + chunk.len());

//                 payload.extend_from_slice(concatenation.udh_bytes().as_bytes());
//                 payload.extend_from_slice(chunk);

//                 assert!(payload.len() <= 255, "short_message length must not exceed 255 bytes. This is a bug in the encoder implementation");
//                 // MIN: 0, MAX: 255 => (payload.len() >= MIN) and (payload.len() <= MAX)
//                 let short_message = OctetString::new_unchecked(payload);

//                 self
//                     .builder
//                     .clone()
//                     .build()
//                     .with_udhi_indicator()
//                     .with_short_message(short_message)
//                     .with_data_coding(self.encoder.data_coding())
//             })
//             .collect::<B>());
//     }

//     assert!(
//         encoded.len() <= 255,
//         "short_message length must not exceed 255 bytes. This is a bug in the encoder implementation"
//     );
//     // MIN: 0, MAX: 255 => (encoded.len() >= MIN) and (encoded.len() <= MAX)
//     let short_message = OctetString::new_unchecked(encoded);

//     let sm = self
//         .builder
//         .clone()
//         .build()
//         .with_short_message(short_message)
//         .with_data_coding(self.encoder.data_coding());

//     Ok(core::iter::once(sm).collect::<B>())
// }
