use std::{collections::HashMap, sync::Arc};

use rusmpp::Command;
use tokio::sync::oneshot;

use crate::error::Error;

pub type PendingResponses =
    Arc<parking_lot::Mutex<HashMap<u32, oneshot::Sender<Result<Command, Error>>>>>;
