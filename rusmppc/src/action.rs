use rusmpp::Command;
use tokio::sync::oneshot;
use tokio_util::sync::CancellationToken;

use crate::error::Error;

#[derive(Debug)]
pub enum Action {
    SendCommand(SendCommandAction),
}

#[derive(Debug)]
pub struct SendCommandAction {
    command: Command,
    // Should be cancelled, if a client request was dropped with tokio::select!.
    // So the connection task will cancel the command task.
    cancellation_token: CancellationToken,
    // Error: RequestTimeout or UnexpectedResponse.
    response: oneshot::Receiver<Result<Command, Error>>,
}
