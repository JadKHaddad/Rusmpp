use crate::Event;

use super::error::ReconnectingError;

#[derive(Debug)]
pub enum ReconnectingEvent {
    Connection(Event),
    Error(ReconnectingError),
    Reconnected,
    Disconnected,
}

impl ReconnectingEvent {
    pub(crate) const fn error(error: ReconnectingError) -> Self {
        Self::Error(error)
    }
}

impl From<Event> for ReconnectingEvent {
    fn from(event: Event) -> Self {
        Self::Connection(event)
    }
}
