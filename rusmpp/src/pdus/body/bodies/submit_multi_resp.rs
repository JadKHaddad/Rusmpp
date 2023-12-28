use rusmpp_macros::RusmppIo;

use rusmpp_io::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
    },
    types::{c_octet_string::COctetString, vec},
};

use crate::pdus::{
    tlvs::tlv::{MessageSubmissionRequestTLV, TLV},
    types::unsuccess_sme::UnsuccessSme,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, RusmppIo)]
pub struct SubmitMultiResp {
    message_id: COctetString<1, 65>,
    no_unsuccess: u8,
    unsuccess_sme: Vec<UnsuccessSme>,
    tlvs: Vec<TLV>,
}

impl SubmitMultiResp {
    pub fn new(
        message_id: COctetString<1, 65>,
        unsuccess_sme: Vec<UnsuccessSme>,
        tlvs: Vec<MessageSubmissionRequestTLV>,
    ) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        let no_unsuccess = unsuccess_sme.len() as u8;
        Self {
            message_id,
            no_unsuccess,
            unsuccess_sme,
            tlvs,
        }
    }

    pub fn message_id(&self) -> &COctetString<1, 65> {
        &self.message_id
    }

    pub fn no_unsuccess(&self) -> u8 {
        self.no_unsuccess
    }

    pub fn unsuccess_sme(&self) -> &[UnsuccessSme] {
        &self.unsuccess_sme
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn into_parts(self) -> (COctetString<1, 65>, u8, Vec<UnsuccessSme>, Vec<TLV>) {
        (
            self.message_id,
            self.no_unsuccess,
            self.unsuccess_sme,
            self.tlvs,
        )
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for SubmitMultiResp {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let message_id = COctetString::async_io_read(buf).await?;
        let no_unsuccess = u8::async_io_read(buf).await?;
        let unsuccess_sme = vec::read_counted::<UnsuccessSme>(buf, no_unsuccess as usize).await?;

        let tlvs_expected_len = length
            .saturating_sub(message_id.length())
            .saturating_sub(no_unsuccess.length())
            .saturating_sub(unsuccess_sme.length());

        let tlvs = Vec::<TLV>::async_io_read(buf, tlvs_expected_len).await?;

        Ok(Self {
            message_id,
            no_unsuccess,
            unsuccess_sme,
            tlvs,
        })
    }
}
