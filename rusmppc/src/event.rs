use rusmpp::Command;

use crate::error::Error;

/// `SMPP` event.
///
/// Events are sent from the open connection through the events stream.
#[derive(Debug)]
pub enum Event {
    /// An error occurred.
    Error(Error),
    /// A command was received from the server.
    Command(Command),
}
