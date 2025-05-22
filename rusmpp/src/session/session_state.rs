/// The [`SessionState`] represents the state of an ESME session in the `SMPP` 5.0 protocol.
///
/// The session state determines what operations are allowed at any given point in the
/// communication between an ESME (External Short Message Entity) and an MC (Message Center).
///
/// The session state transitions are triggered by bind, unbind, and outbind operations.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
pub enum SessionState {
    /// CLOSED state.
    ///
    /// This is the initial state before any connection is established.
    /// In this state, no communication is possible between the ESME and MC.
    #[default]
    Closed,

    /// OPEN state.
    ///
    /// This state is entered after a connection is established between
    /// the ESME and MC, but before any `SMPP` bind operation is performed.
    /// In this state, only bind operations are allowed.
    Open,

    /// BOUND_TX state (Transmitter mode).
    ///
    /// This state is entered after a successful bind_transmitter operation.
    /// In this state, the ESME can send messages to the MC but cannot receive messages.
    BoundTx,

    /// BOUND_RX state (Receiver mode).
    ///
    /// This state is entered after a successful bind_receiver operation.
    /// In this state, the ESME can receive messages from the MC but cannot send messages.
    BoundRx,

    /// BOUND_TRX state (Transceiver mode).
    ///
    /// This state is entered after a successful bind_transceiver operation.
    /// In this state, the ESME can both send messages to and receive messages from the MC.
    BoundTrx,

    /// OUTBOUND state.
    ///
    /// This state is entered after an MC initiates an outbind operation to an ESME.
    /// The ESME must respond with a bind_receiver or bind_transceiver operation.
    /// In this state, no messaging operations are allowed until the ESME completes the binding process.
    Outbound,

    /// UNBOUND state.
    ///
    /// This state is entered after an unbind operation is initiated by either the ESME or MC.
    /// The session is in the process of being terminated, but the unbind_resp has not yet been sent.
    /// No messaging operations are allowed in this state.
    Unbound,
}
