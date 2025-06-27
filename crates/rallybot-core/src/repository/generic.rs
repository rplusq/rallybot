use crate::{
    models::{Session, SessionType, Venue},
    registration::{Registration, RegistrationStatus},
    services::{RegistrationService, SessionService},
    storage::Storage,
    user::User,
};
use std::sync::Arc;
use uuid::Uuid;

use super::{RegistrationError, SessionError, SessionRepository, UserRepository, VenueRepository};

pub struct Repository<S: Storage> {
    storage: Arc<S>,
    registration_service: RegistrationService<Arc<S>>,
    session_service: SessionService<S>,
}

impl<S: Storage> Repository<S> {
    pub fn new(storage: Arc<S>) -> Self {
        let registration_service = RegistrationService::new(storage.clone());
        let session_service = SessionService::new(storage.clone());
        Self {
            storage,
            registration_service,
            session_service,
        }
    }
}

#[async_trait::async_trait]
impl<S: Storage> SessionRepository for Repository<S> {
    async fn list(&self, session_type: Option<SessionType>) -> Vec<Session> {
        self.storage.list_sessions(session_type).await
    }

    async fn get(&self, id: Uuid) -> Option<Session> {
        self.storage.get_session(id).await
    }

    async fn create(&self, session: Session) -> Result<Session, SessionError> {
        match self.session_service.create_session(session).await {
            Ok(session) => Ok(session),
            Err(crate::services::session::SessionError::VenueNotFound) => {
                Err(SessionError::VenueNotFound)
            }
        }
    }

    async fn register_user(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<RegistrationStatus, RegistrationError> {
        self.registration_service
            .register_user(session_id, user_id)
            .await
    }

    async fn unregister_user(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), RegistrationError> {
        self.registration_service
            .unregister_user(session_id, user_id)
            .await
    }

    async fn get_registrations(&self, session_id: Uuid) -> Vec<Registration> {
        self.registration_service
            .get_session_registrations(session_id)
            .await
    }

    async fn get_user_sessions(&self, user_id: Uuid) -> Vec<Session> {
        let session_ids = self.registration_service.get_user_sessions(user_id).await;
        let mut sessions = Vec::new();
        for id in session_ids {
            if let Some(session) = self.storage.get_session(id).await {
                sessions.push(session);
            }
        }
        sessions
    }
}

#[async_trait::async_trait]
impl<S: Storage> UserRepository for Repository<S> {
    async fn get(&self, id: Uuid) -> Option<User> {
        self.storage.get_user(id).await
    }

    async fn get_by_phone(&self, phone: &str) -> Option<User> {
        self.storage.get_user_by_phone(phone).await
    }

    async fn create(&self, user: User) -> User {
        self.storage.create_user(user.clone()).await;
        user
    }
}

#[async_trait::async_trait]
impl<S: Storage> VenueRepository for Repository<S> {
    async fn get(&self, id: Uuid) -> Option<Venue> {
        self.storage.get_venue(id).await
    }

    async fn list(&self) -> Vec<Venue> {
        self.storage.list_venues().await
    }

    async fn create(&self, venue: Venue) -> Venue {
        self.storage.create_venue(venue.clone()).await;
        venue
    }
}