use crate::{
    models::{Session, SessionType},
    storage::Storage,
};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug, PartialEq)]
pub enum SessionError {
    VenueNotFound,
}

pub struct SessionService<S> {
    storage: Arc<S>,
}

impl<S: Storage> SessionService<S> {
    pub fn new(storage: Arc<S>) -> Self {
        Self { storage }
    }

    pub async fn create_session(&self, session: Session) -> Result<Session, SessionError> {
        // Validate venue exists
        if self.storage.get_venue(session.venue_id).await.is_none() {
            return Err(SessionError::VenueNotFound);
        }
        
        self.storage.create_session(session.clone()).await;
        Ok(session)
    }

    pub async fn get_session(&self, id: Uuid) -> Option<Session> {
        self.storage.get_session(id).await
    }

    pub async fn list_sessions(&self, session_type: Option<SessionType>) -> Vec<Session> {
        self.storage.list_sessions(session_type).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{SessionType, Venue},
        storage::InMemoryStorage,
    };
    use chrono::Utc;

    async fn setup_test_storage() -> Arc<InMemoryStorage> {
        let storage = Arc::new(InMemoryStorage::new());
        
        // Create test venue
        let venue = Venue::new(
            "Test Padel Club".to_string(),
            "123 Test Street".to_string(),
        );
        storage.create_venue(venue.clone()).await;
        
        // Create various sessions
        let sessions = vec![
            Session::new(SessionType::Social, Utc::now(), 90, venue.id).unwrap(),
            Session::new(SessionType::Coaching, Utc::now(), 60, venue.id).unwrap(),
            Session::new(SessionType::Social, Utc::now(), 120, venue.id).unwrap(),
            Session::new(SessionType::League, Utc::now(), 90, venue.id).unwrap(),
        ];
        
        for session in sessions {
            storage.create_session(session).await;
        }
        
        storage
    }

    #[tokio::test]
    async fn filter_sessions_by_type() {
        let storage = setup_test_storage().await;
        let service = SessionService::new(storage);
        
        // Test that filtering is handled by storage
        let social_sessions = service.list_sessions(Some(SessionType::Social)).await;
        assert_eq!(social_sessions.len(), 2);
        assert!(social_sessions.iter().all(|s| s.session_type == SessionType::Social));
        
        let all_sessions = service.list_sessions(None).await;
        assert_eq!(all_sessions.len(), 4);
    }

    #[tokio::test]
    async fn create_session_with_valid_venue_succeeds() {
        let storage = Arc::new(InMemoryStorage::new());
        let service = SessionService::new(storage.clone());
        
        // Create venue first
        let venue = Venue::new("Test Venue".to_string(), "Test Address".to_string());
        storage.create_venue(venue.clone()).await;
        
        // Create session with valid venue
        let session = Session::new(
            SessionType::Social,
            Utc::now(),
            90,
            venue.id,
        ).unwrap();
        
        let result = service.create_session(session.clone()).await;
        assert!(result.is_ok());
        
        let created = result.unwrap();
        assert_eq!(created.venue_id, venue.id);
    }

    #[tokio::test]
    async fn create_session_with_invalid_venue_fails() {
        let storage = Arc::new(InMemoryStorage::new());
        let service = SessionService::new(storage);
        
        // Try to create session with non-existent venue
        let fake_venue_id = Uuid::new_v4();
        let session = Session::new(
            SessionType::Social,
            Utc::now(),
            90,
            fake_venue_id,
        ).unwrap();
        
        let result = service.create_session(session).await;
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), SessionError::VenueNotFound);
    }
}