use crate::{
    models::{Session, SessionType, Venue},
    registration::Registration,
    user::User,
};
use std::sync::Arc;
use uuid::Uuid;

#[async_trait::async_trait]
pub trait Storage: Send + Sync {
    // Session operations
    async fn get_session(&self, id: Uuid) -> Option<Session>;
    async fn list_sessions(&self, session_type: Option<SessionType>) -> Vec<Session>;
    async fn create_session(&self, session: Session);
    
    // User operations
    async fn get_user(&self, id: Uuid) -> Option<User>;
    async fn get_user_by_phone(&self, phone: &str) -> Option<User>;
    async fn create_user(&self, user: User);
    
    // Registration operations
    async fn get_registrations(&self, session_id: Uuid) -> Vec<Registration>;
    async fn get_user_registrations(&self, user_id: Uuid) -> Vec<Registration>;
    async fn create_registration(&self, registration: Registration);
    async fn registration_exists(&self, session_id: Uuid, user_id: Uuid) -> bool;
    
    // Venue operations
    async fn get_venue(&self, id: Uuid) -> Option<Venue>;
    async fn list_venues(&self) -> Vec<Venue>;
    async fn create_venue(&self, venue: Venue);
}

// Implement Storage for Arc<S> where S: Storage
#[async_trait::async_trait]
impl<S: Storage> Storage for Arc<S> {
    async fn get_session(&self, id: Uuid) -> Option<Session> {
        (**self).get_session(id).await
    }

    async fn list_sessions(&self, session_type: Option<SessionType>) -> Vec<Session> {
        (**self).list_sessions(session_type).await
    }

    async fn create_session(&self, session: Session) {
        (**self).create_session(session).await
    }

    async fn get_user(&self, id: Uuid) -> Option<User> {
        (**self).get_user(id).await
    }

    async fn get_user_by_phone(&self, phone: &str) -> Option<User> {
        (**self).get_user_by_phone(phone).await
    }

    async fn create_user(&self, user: User) {
        (**self).create_user(user).await
    }

    async fn get_registrations(&self, session_id: Uuid) -> Vec<Registration> {
        (**self).get_registrations(session_id).await
    }

    async fn get_user_registrations(&self, user_id: Uuid) -> Vec<Registration> {
        (**self).get_user_registrations(user_id).await
    }

    async fn create_registration(&self, registration: Registration) {
        (**self).create_registration(registration).await
    }

    async fn registration_exists(&self, session_id: Uuid, user_id: Uuid) -> bool {
        (**self).registration_exists(session_id, user_id).await
    }

    async fn get_venue(&self, id: Uuid) -> Option<Venue> {
        (**self).get_venue(id).await
    }

    async fn list_venues(&self) -> Vec<Venue> {
        (**self).list_venues().await
    }

    async fn create_venue(&self, venue: Venue) {
        (**self).create_venue(venue).await
    }
}