use tokio::sync::oneshot;

use crate::error::Error;

#[derive(Debug)]
pub struct PendingResponses {
    pub ack: oneshot::Sender<Result<Vec<u32>, Error>>,
}

impl PendingResponses {
    pub fn new() -> (Self, oneshot::Receiver<Result<Vec<u32>, Error>>) {
        let (ack, rx) = oneshot::channel();

        (Self { ack }, rx)
    }
}
