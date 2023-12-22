#[derive(thiserror::Error, Debug)]
#[error("Invalid sm default msg id. sm default msg id must be between 1 and 255 inclusive. sm default msg id: {sm_default_msg_id}")]

pub struct InvalidSmDefaultMsgId {
    sm_default_msg_id: u8,
}

/// The sm_default_msg_id parameter specifies the MC index of a pre-defined (‘canned’)
/// message.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SmDefaultMsgId {
    pub value: u8,
}

impl SmDefaultMsgId {
    pub fn new(value: u8) -> Self {
        Self { value }
    }

    pub fn validate(&self) -> Result<(), InvalidSmDefaultMsgId> {
        if self.value == 0 {
            return Err(InvalidSmDefaultMsgId {
                sm_default_msg_id: self.value,
            });
        }

        Ok(())
    }
}
