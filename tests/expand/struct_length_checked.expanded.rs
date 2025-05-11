pub struct MsValidity {
    pub validity_behavior: MsValidityBehavior,
    pub validity_information: Option<MsValidityInformation>,
}
impl ::rusmpp::encode::Length for MsValidity {
    fn length(&self) -> usize {
        let mut length = 0;
        length += ::rusmpp::encode::Length::length(&self.validity_behavior);
        length += ::rusmpp::encode::Length::length(&self.validity_information);
        length
    }
}
impl ::rusmpp::encode::Encode for MsValidity {
    fn encode(&self, dst: &mut [u8]) -> usize {
        let size = 0;
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.validity_behavior,
            dst,
            size,
        );
        let size = ::rusmpp::encode::EncodeExt::encode_move(
            &self.validity_information,
            dst,
            size,
        );
        size
    }
}
impl ::rusmpp::decode::DecodeWithLength for MsValidity {
    fn decode(
        src: &[u8],
        length: usize,
    ) -> Result<(Self, usize), ::rusmpp::decode::DecodeError> {
        let size = 0;
        let (validity_behavior, size) = ::rusmpp::decode::DecodeExt::decode_move(
            src,
            size,
        )?;
        let (validity_information, size) = ::rusmpp::decode::DecodeExt::length_checked_decode_move(
                src,
                length.saturating_sub(size),
                size,
            )?
            .map(|(this, size)| (Some(this), size))
            .unwrap_or((None, size));
        Ok((
            Self {
                validity_behavior,
                validity_information,
            },
            size,
        ))
    }
}
