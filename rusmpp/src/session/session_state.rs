use crate::CommandId;

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

impl SessionState {
    /// Returns true if the session is a bound state.
    ///
    /// One of the following states:
    /// [`SessionState::BoundTx`], [`SessionState::BoundRx`] or [`SessionState::BoundTrx`].
    pub fn is_bound(self) -> bool {
        self == Self::BoundTx || self == Self::BoundRx || self == Self::BoundTrx
    }

    pub fn can_send_as_esme(self, command: CommandId) -> bool {
        match self {
            SessionState::Closed => false,
            SessionState::Open | SessionState::Outbound => {
                command == CommandId::BindReceiver
                    || command == CommandId::BindTransmitter
                    || command == CommandId::BindTransceiver
                    || command == CommandId::EnquireLink
                    || command == CommandId::EnquireLinkResp
                    || command == CommandId::GenericNack
            }
            SessionState::BoundTx => {
                command == CommandId::BroadcastSm
                    || command == CommandId::CancelBroadcastSm
                    || command == CommandId::CancelSm
                    || command == CommandId::DataSm
                    || command == CommandId::EnquireLink
                    || command == CommandId::EnquireLinkResp
                    || command == CommandId::GenericNack
                    || command == CommandId::QueryBroadcastSm
                    || command == CommandId::QuerySm
                    || command == CommandId::ReplaceSm
                    || command == CommandId::SubmitMulti
                    || command == CommandId::SubmitSm
                    || command == CommandId::Unbind
                    || command == CommandId::UnbindResp
            }
            SessionState::BoundRx => {
                command == CommandId::DataSmResp
                    || command == CommandId::DeliverSmResp
                    || command == CommandId::EnquireLink
                    || command == CommandId::EnquireLinkResp
                    || command == CommandId::GenericNack
                    || command == CommandId::Unbind
                    || command == CommandId::UnbindResp
            }
            SessionState::BoundTrx => {
                SessionState::BoundTx.can_send_as_esme(command)
                    || SessionState::BoundRx.can_send_as_esme(command)
            }
            SessionState::Unbound => {
                command == CommandId::EnquireLink
                    || command == CommandId::EnquireLinkResp
                    || command == CommandId::GenericNack
            }
        }
    }

    pub fn can_receive_as_esme(self, command: CommandId) -> bool {
        self.can_send_as_mc(command)
    }

    pub fn can_send_as_mc(self, command: CommandId) -> bool {
        match self {
            SessionState::Closed => false,
            SessionState::Open => {
                command == CommandId::BindReceiverResp
                    || command == CommandId::BindTransmitterResp
                    || command == CommandId::BindTransceiverResp
                    || command == CommandId::EnquireLink
                    || command == CommandId::EnquireLinkResp
                    || command == CommandId::GenericNack
                    || command == CommandId::Outbind
            }
            SessionState::Outbound => {
                command == CommandId::BindReceiverResp
                    || command == CommandId::BindTransmitterResp
                    || command == CommandId::BindTransceiverResp
                    || command == CommandId::EnquireLink
                    || command == CommandId::EnquireLinkResp
                    || command == CommandId::GenericNack
            }
            SessionState::BoundTx => {
                command == CommandId::BroadcastSmResp
                    || command == CommandId::CancelBroadcastSmResp
                    || command == CommandId::CancelSmResp
                    || command == CommandId::DataSmResp
                    || command == CommandId::EnquireLink
                    || command == CommandId::EnquireLinkResp
                    || command == CommandId::GenericNack
                    || command == CommandId::QueryBroadcastSmResp
                    || command == CommandId::QuerySmResp
                    || command == CommandId::ReplaceSmResp
                    || command == CommandId::SubmitMultiResp
                    || command == CommandId::SubmitSmResp
                    || command == CommandId::Unbind
                    || command == CommandId::UnbindResp
            }
            SessionState::BoundRx => {
                command == CommandId::AlertNotification
                    || command == CommandId::DataSm
                    || command == CommandId::DeliverSm
                    || command == CommandId::EnquireLink
                    || command == CommandId::EnquireLinkResp
                    || command == CommandId::GenericNack
                    || command == CommandId::Unbind
                    || command == CommandId::UnbindResp
            }
            SessionState::BoundTrx => {
                SessionState::BoundTx.can_send_as_mc(command)
                    || SessionState::BoundRx.can_send_as_mc(command)
            }
            SessionState::Unbound => {
                command == CommandId::EnquireLink
                    || command == CommandId::EnquireLinkResp
                    || command == CommandId::GenericNack
            }
        }
    }

    pub fn can_receive_as_mc(self, command: CommandId) -> bool {
        self.can_send_as_mc(command)
    }
}
