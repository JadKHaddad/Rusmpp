/// `SMPP` bind modes.
#[derive(Debug, Copy, Clone, Default)]
pub enum BindMode {
    /// Transmitter.
    #[default]
    Tx,
    /// Receiver.
    Rx,
    /// Transceiver (both transmitter and receiver).
    Trx,
}
