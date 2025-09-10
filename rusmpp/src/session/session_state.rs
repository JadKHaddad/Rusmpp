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
    pub const fn is_bound(self) -> bool {
        matches!(self, Self::BoundTx | Self::BoundRx | Self::BoundTrx)
    }

    /// Determines whether the current session state allows sending a given SMPP command as an ESME.
    ///
    /// # Arguments
    ///
    /// * `command` - The SMPP command to check.
    ///
    /// # Returns true if an ESME in that state can send this command.
    ///
    /// This follows the 2.4 Operation Matrix of the SMPP 5.0 specification
    pub const fn can_send_as_esme(self, command: CommandId) -> bool {
        match self {
            SessionState::Closed => false,
            SessionState::Open | SessionState::Outbound => {
                matches!(
                    command,
                    CommandId::BindReceiver
                        | CommandId::BindTransmitter
                        | CommandId::BindTransceiver
                        | CommandId::EnquireLink
                        | CommandId::EnquireLinkResp
                        | CommandId::GenericNack
                )
            }
            SessionState::BoundTx => {
                matches!(
                    command,
                    CommandId::BroadcastSm
                        | CommandId::CancelBroadcastSm
                        | CommandId::CancelSm
                        | CommandId::DataSm
                        | CommandId::EnquireLink
                        | CommandId::EnquireLinkResp
                        | CommandId::GenericNack
                        | CommandId::QueryBroadcastSm
                        | CommandId::QuerySm
                        | CommandId::ReplaceSm
                        | CommandId::SubmitMulti
                        | CommandId::SubmitSm
                        | CommandId::Unbind
                        | CommandId::UnbindResp
                )
            }
            SessionState::BoundRx => {
                matches!(
                    command,
                    CommandId::DataSmResp
                        | CommandId::DeliverSmResp
                        | CommandId::EnquireLink
                        | CommandId::EnquireLinkResp
                        | CommandId::GenericNack
                        | CommandId::Unbind
                        | CommandId::UnbindResp
                )
            }
            SessionState::BoundTrx => {
                SessionState::BoundTx.can_send_as_esme(command)
                    || SessionState::BoundRx.can_send_as_esme(command)
            }
            SessionState::Unbound => {
                matches!(
                    command,
                    CommandId::EnquireLink | CommandId::EnquireLinkResp | CommandId::GenericNack
                )
            }
        }
    }

    /// Determines whether the current session state allows an ESME to receive a given SMPP command.
    ///
    /// # Arguments
    ///
    /// * `command` - The SMPP command to check.
    ///
    /// # Returns true if an ESME in that state can receive this command.
    ///
    /// This follows the 2.4 Operation Matrix of the SMPP 5.0 specification
    pub const fn can_receive_as_esme(self, command: CommandId) -> bool {
        self.can_send_as_mc(command)
    }

    /// Determines whether the current session state allows a MC to send a given SMPP command.
    ///
    /// # Arguments
    ///
    /// * `command` - The SMPP command to check.
    ///
    /// # Returns true if a MC in that state can send this command.
    ///
    /// This follows the 2.4 Operation Matrix of the SMPP 5.0 specification
    pub const fn can_send_as_mc(self, command: CommandId) -> bool {
        match self {
            SessionState::Closed => false,
            SessionState::Open => {
                matches!(
                    command,
                    CommandId::BindReceiverResp
                        | CommandId::BindTransmitterResp
                        | CommandId::BindTransceiverResp
                        | CommandId::EnquireLink
                        | CommandId::EnquireLinkResp
                        | CommandId::GenericNack
                        | CommandId::Outbind
                )
            }
            SessionState::Outbound => {
                matches!(
                    command,
                    CommandId::BindReceiverResp
                        | CommandId::BindTransmitterResp
                        | CommandId::BindTransceiverResp
                        | CommandId::EnquireLink
                        | CommandId::EnquireLinkResp
                        | CommandId::GenericNack
                )
            }
            SessionState::BoundTx => {
                matches!(
                    command,
                    CommandId::BroadcastSmResp
                        | CommandId::CancelBroadcastSmResp
                        | CommandId::CancelSmResp
                        | CommandId::DataSmResp
                        | CommandId::EnquireLink
                        | CommandId::EnquireLinkResp
                        | CommandId::GenericNack
                        | CommandId::QueryBroadcastSmResp
                        | CommandId::QuerySmResp
                        | CommandId::ReplaceSmResp
                        | CommandId::SubmitMultiResp
                        | CommandId::SubmitSmResp
                        | CommandId::Unbind
                        | CommandId::UnbindResp
                )
            }
            SessionState::BoundRx => {
                matches!(
                    command,
                    CommandId::AlertNotification
                        | CommandId::DataSm
                        | CommandId::DeliverSm
                        | CommandId::EnquireLink
                        | CommandId::EnquireLinkResp
                        | CommandId::GenericNack
                        | CommandId::Unbind
                        | CommandId::UnbindResp
                )
            }
            SessionState::BoundTrx => {
                SessionState::BoundTx.can_send_as_mc(command)
                    || SessionState::BoundRx.can_send_as_mc(command)
            }
            SessionState::Unbound => {
                matches!(
                    command,
                    CommandId::EnquireLink | CommandId::EnquireLinkResp | CommandId::GenericNack
                )
            }
        }
    }

    /// Determines whether the current session state allows a MC to receive a given SMPP command.
    ///
    /// # Arguments
    ///
    /// * `command` - The SMPP command to check.
    ///
    /// # Returns true if a MC in that state can receive this command.
    ///
    /// This follows the 2.4 Operation Matrix of the SMPP 5.0 specification
    pub const fn can_receive_as_mc(self, command: CommandId) -> bool {
        self.can_send_as_esme(command)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use strum::IntoEnumIterator;

    const ESME_BOUND_TX_COMMANDS: [CommandId; 9] = [
        CommandId::BroadcastSm,
        CommandId::CancelBroadcastSm,
        CommandId::CancelSm,
        CommandId::DataSm,
        CommandId::QueryBroadcastSm,
        CommandId::QuerySm,
        CommandId::ReplaceSm,
        CommandId::SubmitMulti,
        CommandId::SubmitSm,
    ];

    const ESME_BOUND_RX_COMMANDS: [CommandId; 2] =
        [CommandId::DataSmResp, CommandId::DeliverSmResp];

    #[test]
    fn test_is_bound() {
        assert!(!SessionState::Closed.is_bound());
        assert!(SessionState::BoundTx.is_bound());
        assert!(SessionState::BoundRx.is_bound());
        assert!(SessionState::BoundTrx.is_bound());
        assert!(!SessionState::Open.is_bound());
        assert!(!SessionState::Outbound.is_bound());
        assert!(!SessionState::Unbound.is_bound());
    }

    #[test]
    fn test_status_close() {
        for command in CommandId::iter() {
            assert!(!SessionState::Closed.can_send_as_esme(command));
            assert!(!SessionState::Closed.can_send_as_mc(command));
            assert!(!SessionState::Closed.can_receive_as_esme(command));
            assert!(!SessionState::Closed.can_receive_as_mc(command));
        }
    }

    #[test]
    fn test_link_nack() {
        for command in [
            CommandId::GenericNack,
            CommandId::EnquireLink,
            CommandId::EnquireLinkResp,
        ] {
            for state in [
                SessionState::Open,
                SessionState::Outbound,
                SessionState::BoundTx,
                SessionState::BoundRx,
                SessionState::BoundTrx,
                SessionState::Unbound,
            ] {
                assert!(state.can_send_as_esme(command));
                assert!(state.can_send_as_mc(command));
                assert!(state.can_receive_as_esme(command));
                assert!(state.can_receive_as_mc(command));
            }
        }
    }

    #[test]
    fn test_open_outbound() {
        for state in [SessionState::Open, SessionState::Outbound] {
            assert!(state.can_send_as_esme(CommandId::BindTransmitter));
            assert!(state.can_send_as_esme(CommandId::BindTransceiver));
            assert!(state.can_send_as_esme(CommandId::BindReceiver));
            assert!(state.can_send_as_mc(CommandId::BindTransmitterResp));
            assert!(state.can_send_as_mc(CommandId::BindTransceiverResp));
            assert!(state.can_send_as_mc(CommandId::BindReceiverResp));
        }
    }

    #[test]
    fn test_tx() {
        for command in ESME_BOUND_TX_COMMANDS {
            assert!(SessionState::BoundTx.can_send_as_esme(command));
            assert!(SessionState::BoundTx.can_receive_as_esme(command.matching_response()));
            assert!(SessionState::BoundTx.can_receive_as_mc(command));
            assert!(SessionState::BoundTx.can_send_as_mc(command.matching_response()));
        }
    }

    #[test]
    fn test_rx() {
        for command in ESME_BOUND_RX_COMMANDS {
            assert!(SessionState::BoundRx.can_send_as_esme(command));
            assert!(SessionState::BoundRx.can_receive_as_esme(command.matching_request()));
            assert!(SessionState::BoundRx.can_receive_as_mc(command));
            assert!(SessionState::BoundRx.can_send_as_mc(command.matching_request()));
        }
    }

    #[test]
    fn test_trx() {
        for command in ESME_BOUND_TX_COMMANDS {
            assert!(SessionState::BoundTrx.can_send_as_esme(command));
            assert!(SessionState::BoundTrx.can_receive_as_esme(command.matching_response()));
            assert!(SessionState::BoundTrx.can_receive_as_mc(command));
            assert!(SessionState::BoundTrx.can_send_as_mc(command.matching_response()));
        }
        for command in ESME_BOUND_RX_COMMANDS {
            assert!(SessionState::BoundTrx.can_send_as_esme(command));
            assert!(SessionState::BoundTrx.can_receive_as_esme(command.matching_request()));
            assert!(SessionState::BoundTrx.can_receive_as_mc(command));
            assert!(SessionState::BoundTrx.can_send_as_mc(command.matching_request()));
        }
    }
}
