use rusmpp::Command;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to connect to the server: {0}")]
    Connect(#[source] std::io::Error),
    // This happen when the response timer expires.
    // e.g. We send a bind request and the server doesn't respond.
    #[error("Request timed out: {request:?}")]
    RequestTimeout { request: Box<Command> },
    // This happen when we get any other status code than esmeRok.
    #[error("Unexpected response from the server: request: {request:?}, response: {response:?}")]
    UnexpectedResponse {
        request: Box<Command>,
        response: Box<Command>,
    },
}
