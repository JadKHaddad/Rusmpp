use rusmpp::{Command, CommandId, CommandStatus};

pub trait CommandExt: Sized {
    fn ok(self) -> Result<Self, Box<Self>>;

    fn matches(self, command_id: CommandId) -> Result<Self, Box<Self>>;

    fn ok_and_matches(self, command_id: CommandId) -> Result<Self, Box<Self>> {
        self.ok().and_then(|this| this.matches(command_id))
    }
}

impl CommandExt for Command {
    fn ok(self) -> Result<Self, Box<Self>> {
        if matches!(self.status(), CommandStatus::EsmeRok) {
            return Ok(self);
        }

        Err(Box::new(self))
    }

    fn matches(self, command_id: CommandId) -> Result<Self, Box<Self>> {
        if self.id() == command_id {
            return Ok(self);
        }

        Err(Box::new(self))
    }
}
