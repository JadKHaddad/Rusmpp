#[derive(Debug, Copy, Clone, Default)]
pub enum BindMode {
    #[default]
    Tx,
    Rx,
    TxRx,
}
