use super::PduBody;
use rusmpp_io::io::write::{AsyncIoWritable, AsyncIoWrite, IoWritable, IoWrite};

#[async_trait::async_trait]
impl AsyncIoWrite for PduBody {
    async fn async_io_write(&self, buf: &mut AsyncIoWritable) -> std::io::Result<()> {
        match self {
            PduBody::BindTransmitter(b) => b.async_io_write(buf).await,
            PduBody::BindTransmitterResp(b) => b.async_io_write(buf).await,
            PduBody::BindReceiver(b) => b.async_io_write(buf).await,
            PduBody::BindReceiverResp(b) => b.async_io_write(buf).await,
            PduBody::BindTransceiver(b) => b.async_io_write(buf).await,
            PduBody::BindTransceiverResp(b) => b.async_io_write(buf).await,
            PduBody::Outbind(b) => b.async_io_write(buf).await,
            PduBody::AlertNotification(b) => b.async_io_write(buf).await,
            PduBody::SubmitSm(b) => b.async_io_write(buf).await,
            PduBody::SubmitSmResp(b) => b.async_io_write(buf).await,
            PduBody::QuerySm(b) => b.async_io_write(buf).await,
            PduBody::QuerySmResp(b) => b.async_io_write(buf).await,
            PduBody::DeliverSm(b) => b.async_io_write(buf).await,
            PduBody::DeliverSmResp(b) => b.async_io_write(buf).await,
            PduBody::DataSm(b) => b.async_io_write(buf).await,
            PduBody::DataSmResp(b) => b.async_io_write(buf).await,
            PduBody::CancelSm(b) => b.async_io_write(buf).await,
            PduBody::ReplaceSm(b) => b.async_io_write(buf).await,
            PduBody::SubmitMulti(b) => b.async_io_write(buf).await,
            PduBody::SubmitMultiResp(b) => b.async_io_write(buf).await,
            PduBody::BroadcastSm(b) => b.async_io_write(buf).await,
            PduBody::BroadcastSmResp(b) => b.async_io_write(buf).await,
            PduBody::QueryBroadcastSm(b) => b.async_io_write(buf).await,
            PduBody::QueryBroadcastSmResp(b) => b.async_io_write(buf).await,
            PduBody::CancelBroadcastSm(b) => b.async_io_write(buf).await,
            PduBody::Other { body, .. } => body.async_io_write(buf).await,
        }
    }
}

impl IoWrite for PduBody {
    fn io_write(&self, buf: &mut IoWritable) -> std::io::Result<()> {
        match self {
            PduBody::BindTransmitter(b) => b.io_write(buf),
            PduBody::BindTransmitterResp(b) => b.io_write(buf),
            PduBody::BindReceiver(b) => b.io_write(buf),
            PduBody::BindReceiverResp(b) => b.io_write(buf),
            PduBody::BindTransceiver(b) => b.io_write(buf),
            PduBody::BindTransceiverResp(b) => b.io_write(buf),
            PduBody::Outbind(b) => b.io_write(buf),
            PduBody::AlertNotification(b) => b.io_write(buf),
            PduBody::SubmitSm(b) => b.io_write(buf),
            PduBody::SubmitSmResp(b) => b.io_write(buf),
            PduBody::QuerySm(b) => b.io_write(buf),
            PduBody::QuerySmResp(b) => b.io_write(buf),
            PduBody::DeliverSm(b) => b.io_write(buf),
            PduBody::DeliverSmResp(b) => b.io_write(buf),
            PduBody::DataSm(b) => b.io_write(buf),
            PduBody::DataSmResp(b) => b.io_write(buf),
            PduBody::CancelSm(b) => b.io_write(buf),
            PduBody::ReplaceSm(b) => b.io_write(buf),
            PduBody::SubmitMulti(b) => b.io_write(buf),
            PduBody::SubmitMultiResp(b) => b.io_write(buf),
            PduBody::BroadcastSm(b) => b.io_write(buf),
            PduBody::BroadcastSmResp(b) => b.io_write(buf),
            PduBody::QueryBroadcastSm(b) => b.io_write(buf),
            PduBody::QueryBroadcastSmResp(b) => b.io_write(buf),
            PduBody::CancelBroadcastSm(b) => b.io_write(buf),
            PduBody::Other { body, .. } => body.io_write(buf),
        }
    }
}
