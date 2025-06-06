use crate::{CloseRequest, PendingResponses, RegisteredRequest, Request, UnregisteredRequest};

#[allow(clippy::large_enum_variant)]
#[derive(Debug)]
pub enum Action {
    Request(Request),
    /// Removes a pending response from the connection's pending responses map.
    Remove(u32),
    /// The connection will stop reading from the server, stop time keeping, close the requests channel, flush pending requests and terminate.
    Close(CloseRequest),
    /// Retrieves pending responses from the connection.
    PendingResponses(PendingResponses),
}

impl Action {
    pub const fn registered_request(request: RegisteredRequest) -> Self {
        Self::Request(Request::Registered(request))
    }

    pub const fn unregistered_request(request: UnregisteredRequest) -> Self {
        Self::Request(Request::Unregistered(request))
    }
}
