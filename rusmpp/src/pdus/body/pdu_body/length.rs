use super::PduBody;
use rusmpp_io::io::length::IoLength;

impl IoLength for PduBody {
    fn length(&self) -> usize {
        match self {
            PduBody::BindTransmitter(b) => b.length(),
            PduBody::BindTransmitterResp(b) => b.length(),
            PduBody::BindReceiver(b) => b.length(),
            PduBody::BindReceiverResp(b) => b.length(),
            PduBody::BindTransceiver(b) => b.length(),
            PduBody::BindTransceiverResp(b) => b.length(),
            PduBody::Outbind(b) => b.length(),
            PduBody::AlertNotification(b) => b.length(),
            PduBody::SubmitSm(b) => b.length(),
            PduBody::SubmitSmResp(b) => b.length(),
            PduBody::QuerySm(b) => b.length(),
            PduBody::QuerySmResp(b) => b.length(),
            PduBody::DeliverSm(b) => b.length(),
            PduBody::DeliverSmResp(b) => b.length(),
            PduBody::DataSm(b) => b.length(),
            PduBody::DataSmResp(b) => b.length(),
            PduBody::CancelSm(b) => b.length(),
            PduBody::ReplaceSm(b) => b.length(),
            PduBody::SubmitMulti(b) => b.length(),
            PduBody::SubmitMultiResp(b) => b.length(),
            PduBody::BroadcastSm(b) => b.length(),
            PduBody::BroadcastSmResp(b) => b.length(),
            PduBody::QueryBroadcastSm(b) => b.length(),
            PduBody::QueryBroadcastSmResp(b) => b.length(),
            PduBody::CancelBroadcastSm(b) => b.length(),
            PduBody::Other { body, .. } => body.length(),
        }
    }
}
