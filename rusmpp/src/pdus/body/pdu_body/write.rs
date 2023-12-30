use super::PduBody;
use rusmpp_io::io::write::{AsyncIoWritable, AsyncIoWrite};

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
