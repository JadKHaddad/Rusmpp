use rusmpp::Command;

use crate::error::Error;

/// `SMPP` event.
///
/// Events are sent from the open connection through the events stream.
#[derive(Debug)]
pub enum Event {
    /// An error occurred.
    Incoming(Command),
    /// A command was received from the server.
    Error(Error),
}

impl Event {
    pub(crate) const fn incoming(command: Command) -> Self {
        Event::Incoming(command)
    }

    pub(crate) const fn error(error: Error) -> Self {
        Event::Error(error)
    }
}
