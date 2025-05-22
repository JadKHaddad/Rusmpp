#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Failed to connect to the server: {0}")]
    Connect(#[source] std::io::Error),
}
