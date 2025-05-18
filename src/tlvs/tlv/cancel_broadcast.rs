use crate::values::{BroadcastContentType, UserMessageReference};

crate::create_tlv_value! {
    #[non_exhaustive]
    #[derive(Debug, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
    pub enum CancelBroadcastTlvValue {
        /// Specifies the content type of the message.
        BroadcastContentType(BroadcastContentType),
        /// ESME assigned message reference number.
        ///
        /// Note: The message_id field should be set to NULL if
        /// using the user_message_reference TLV.
        UserMessageReference(UserMessageReference),
    }
}
