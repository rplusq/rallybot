use crate::models::{Session, SessionType};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn list(&self, session_type: Option<SessionType>) -> Vec<Session>;
    async fn get(&self, id: Uuid) -> Option<Session>;
    async fn create(&self, session: Session) -> Session;
}

pub struct InMemoryRepository {
    sessions: Arc<Mutex<Vec<Session>>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[async_trait::async_trait]
impl SessionRepository for InMemoryRepository {
    async fn list(&self, session_type: Option<SessionType>) -> Vec<Session> {
        let sessions = self.sessions.lock().await;
        match session_type {
            Some(st) => sessions
                .iter()
                .filter(|s| s.session_type == st)
                .cloned()
                .collect(),
            None => sessions.clone(),
        }
    }

    async fn get(&self, id: Uuid) -> Option<Session> {
        let sessions = self.sessions.lock().await;
        sessions.iter().find(|s| s.id == id).cloned()
    }

    async fn create(&self, session: Session) -> Session {
        let mut sessions = self.sessions.lock().await;
        sessions.push(session.clone());
        session
    }
}