use rusmpp::Command;
use tokio::sync::oneshot;

use crate::error::Error;

#[derive(Debug)]
pub enum Action {
    SendCommand(SendCommand),
    /// Command will be sent without waiting for a response. e.g. `GenericNack`.
    SendCommandNoResponse(SendCommandNoResponse),
}

#[derive(Debug)]
pub struct SendCommand {
    pub command: Command,
    pub response: oneshot::Sender<Result<Command, Error>>,
    // We do not use a cancellation token to cancel the outgoing request.
    // Because this is most likely going to break the connection on the server side.
    // So dropping the request future will not cancel the request.
}

impl SendCommand {
    pub fn new(command: Command) -> (Self, oneshot::Receiver<Result<Command, Error>>) {
        let (response, rx) = oneshot::channel();

        (Self { command, response }, rx)
    }
}

#[derive(Debug)]
pub struct SendCommandNoResponse {
    pub command: Command,
    pub response: oneshot::Sender<Result<(), Error>>,
}

impl SendCommandNoResponse {
    pub fn new(command: Command) -> (Self, oneshot::Receiver<Result<(), Error>>) {
        let (response, rx) = oneshot::channel();

        (Self { command, response }, rx)
    }
}
