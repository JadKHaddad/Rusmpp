use crate::{
    io::{
        length::IoLength,
        read::{AsyncIoRead, AsyncIoReadWithLength, AsyncIoReadable, IoReadError},
        write::{AsyncIoWritable, AsyncIoWrite},
    },
    pdus::tlvs::tlv::{MessageSubmissionRequestTLV, TLV},
};

use super::s_sm::SSm;

#[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SubmitSm {
    ssm: SSm,
    tlvs: Vec<TLV>,
}

impl SubmitSm {
    pub fn new(ssm: SSm, tlvs: Vec<MessageSubmissionRequestTLV>) -> Self {
        let tlvs = tlvs.into_iter().map(|v| v.into()).collect::<Vec<TLV>>();
        let ssm = SSm::check_for_message_payload_and_update(ssm, &tlvs);

        Self { ssm, tlvs }
    }

    pub fn ssm(&self) -> &SSm {
        &self.ssm
    }

    pub fn tlvs(&self) -> &[TLV] {
        &self.tlvs
    }

    pub fn into_parts(self) -> (SSm, Vec<TLV>) {
        (self.ssm, self.tlvs)
    }
}

impl IoLength for SubmitSm {
    fn length(&self) -> usize {
        self.ssm.length() + self.tlvs.length()
    }
}

#[async_trait::async_trait]
impl AsyncIoWrite for SubmitSm {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        self.ssm.async_io_write(buf).await?;
        self.tlvs.async_io_write(buf).await?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl AsyncIoReadWithLength for SubmitSm {
    async fn async_io_read(buf: &mut AsyncIoReadable, length: usize) -> Result<Self, IoReadError> {
        let ssm = SSm::async_io_read(buf).await?;

        let tlvs_expected_length = length.saturating_sub(ssm.length());

        let tlvs = Vec::<TLV>::async_io_read(buf, tlvs_expected_length).await?;

        Ok(Self { ssm, tlvs })
    }
}
