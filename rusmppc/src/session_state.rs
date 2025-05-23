use std::sync::{
    Arc,
    atomic::{AtomicU32, Ordering},
};

use rusmpp::session::SessionState;

#[derive(Debug, Clone)]
pub struct SessionStateHolder {
    inner: Arc<SessionStateInner>,
}

impl SessionStateHolder {
    pub fn new(session_state: SessionState) -> Self {
        Self {
            inner: Arc::new(SessionStateInner {
                session_state: parking_lot::RwLock::new(session_state),
                sequence_number: AtomicU32::new(1),
            }),
        }
    }

    pub fn session_state(&self) -> SessionState {
        *self.inner.session_state.read()
    }

    pub fn set_session_state(&self, session_state: SessionState) {
        *self.inner.session_state.write() = session_state;
    }

    pub fn sequence_number(&self) -> u32 {
        self.inner.sequence_number.load(Ordering::Relaxed)
    }

    pub fn next_sequence_number(&self) -> u32 {
        self.inner.sequence_number.fetch_add(1, Ordering::Relaxed)
    }
}

#[derive(Debug)]
pub struct SessionStateInner {
    session_state: parking_lot::RwLock<SessionState>,
    sequence_number: AtomicU32,
}
