use rusmpp::Command;
use tokio::sync::oneshot;

use crate::error::Error;

#[derive(Debug)]
pub enum Action {
    SendCommand(SendCommandAction),
}

#[derive(Debug)]
pub struct SendCommandAction {
    command: Command,
    response: oneshot::Receiver<Result<Command, Error>>,
    // We do not use a cancellation token to cancel the outgoing request.
    // Because this is most likely going to break the connection on the server side.
    // So dropping the request future will not cancel the request.
}
