// TODO: We have to deal MessagePayload if set instead of ShortMessage. Do we override? or we ignore it?

use rusmpp_core::{
    pdus::owned::SubmitSm, types::owned::OctetString,
    udhs::owned::concatenation::ConcatenatedShortMessageType, values::DataCoding,
};

use crate::{
    codecs::{gsm::Gsm7BitUnpacked, owned::Encoder},
    concatenation::owned::{Concatenation, Concatenator, multipart::errors::MultipartError},
};

#[derive(Debug)]
pub struct SubmitSmMultipartBuilder<'a, E> {
    short_message: &'a str,
    max_short_message_size: usize,
    data_coding: Option<DataCoding>,
    sm: SubmitSm,
    encoder: E,
    concatenation_type: ConcatenatedShortMessageType,
}

impl<'a, E> SubmitSmMultipartBuilder<'a, E> {
    pub fn short_message<'b>(self, short_message: &'b str) -> SubmitSmMultipartBuilder<'b, E> {
        SubmitSmMultipartBuilder {
            short_message,
            max_short_message_size: self.max_short_message_size,
            data_coding: self.data_coding,
            sm: self.sm,
            encoder: self.encoder,
            concatenation_type: self.concatenation_type,
        }
    }

    /// Override the default max short message size.
    ///
    /// See [`SubmitSm::default_max_short_message_size`].
    pub fn max_short_message_size(mut self, size: usize) -> Self {
        self.max_short_message_size = size;
        self
    }

    /// Override the default data coding provided by the encoder.
    ///
    /// If not set, the encoder's data coding will be used.
    ///
    /// Set this value when using a custom encoder function.
    pub fn data_coding(mut self, data_coding: DataCoding) -> Self {
        self.data_coding = Some(data_coding);
        self
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
    ///
    /// See [`Self::data_coding`] to override data coding if needed.
    pub fn encoder<U>(self, encoder: U) -> SubmitSmMultipartBuilder<'a, U> {
        SubmitSmMultipartBuilder {
            short_message: self.short_message,
            max_short_message_size: self.max_short_message_size,
            data_coding: self.data_coding,
            sm: self.sm,
            encoder,
            concatenation_type: self.concatenation_type,
        }
    }

    /// Sets the GSM 7-bit unpacked encoder.
    pub fn gsm7bit_unpacked(self) -> SubmitSmMultipartBuilder<'a, Gsm7BitUnpacked> {
        self.encoder(Gsm7BitUnpacked::new())
    }
}

impl<'a, E> SubmitSmMultipartBuilder<'a, E>
where
    E: Concatenator + 'a,
{
    pub fn build(
        self,
    ) -> Result<
        (), // impl Iterator<Item = SubmitSm>,
        MultipartError<<E as Encoder>::Error, <E as Concatenator>::Error>,
    > {
        let data_coding = self.data_coding.unwrap_or(self.encoder.data_coding());

        let encoded = self
            .encoder
            .encode(self.short_message)
            .map_err(MultipartError::encode)?;

        let concatenation = self
            .encoder
            .concatenate(
                encoded,
                self.max_short_message_size as u8,
                self.concatenation_type.udh_length() as u8,
            )
            .map_err(MultipartError::concatenation)?;

        match concatenation {
            Concatenation::Single(bytes) => {
                let short_message = OctetString::new(bytes)
                    .expect("Encoder produced invalid short message. This is a bug in the encoder");

                let sm = self
                    .sm
                    .with_short_message(short_message)
                    .with_data_coding(data_coding);
            }
            Concatenation::Concatenated(iter) => {
                let iter = iter.enumerate().map(|(index, bytes)| todo!());
            }
        }

        Ok(())
    }
}
