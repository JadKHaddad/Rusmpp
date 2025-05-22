use crate::error::Error;

#[derive(Debug)]
pub enum Event {
    Error(Error),
}
