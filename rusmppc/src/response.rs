use tokio::sync::oneshot;

#[derive(Debug)]
pub struct PendingResponses {
    pub ack: oneshot::Sender<Vec<u32>>,
}

impl PendingResponses {
    pub fn new() -> (Self, oneshot::Receiver<Vec<u32>>) {
        let (ack, rx) = oneshot::channel();

        (Self { ack }, rx)
    }
}
