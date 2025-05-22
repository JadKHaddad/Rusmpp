use std::sync::Arc;

use rusmpp::session::SessionState;

#[derive(Debug, Clone)]
pub struct SessionStateHolder {
    session_state: Arc<parking_lot::RwLock<SessionState>>,
}

impl SessionStateHolder {
    pub fn new(session_state: SessionState) -> Self {
        Self {
            session_state: Arc::new(parking_lot::RwLock::new(session_state)),
        }
    }

    pub fn get(&self) -> SessionState {
        *self.session_state.read()
    }

    pub fn set(&self, session_state: SessionState) {
        *self.session_state.write() = session_state;
    }
}
