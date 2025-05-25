use rusmpp::Command;
use tokio::sync::oneshot;

use crate::error::Error;

/// Actions are used to communicate between the client and the connection.
///
/// An action is sent from a client to the open connection.
/// The connection will process the action and send a response back to the client.
#[derive(Debug)]
pub enum Action {
    SendCommand(SendCommand),
    /// Command will be sent without waiting for a response. e.g. `GenericNack`.
    SendCommandNoResponse(SendCommandNoResponse),
    /// When a request times out, or the future is dropped, the corresponding
    /// sequence number should be removed from the pending responses to prevent memory leaks.
    RemoveSequenceNumber(u32),
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
