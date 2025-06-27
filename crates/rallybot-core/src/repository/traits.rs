use crate::{
    models::{Session, SessionType, Venue},
    registration::{Registration, RegistrationStatus},
    user::User,
};
use uuid::Uuid;

#[derive(Debug)]
pub enum RegistrationError {
    SessionNotFound,
    UserNotFound,
    UserNotApproved,
    AlreadyRegistered,
    NotRegistered,
}

#[derive(Debug)]
pub enum SessionError {
    VenueNotFound,
}

#[async_trait::async_trait]
pub trait SessionRepository: Send + Sync {
    async fn list(&self, session_type: Option<SessionType>) -> Vec<Session>;
    async fn get(&self, id: Uuid) -> Option<Session>;
    async fn create(&self, session: Session) -> Result<Session, SessionError>;
    async fn register_user(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<RegistrationStatus, RegistrationError>;
    async fn unregister_user(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), RegistrationError>;
    async fn get_registrations(&self, session_id: Uuid) -> Vec<Registration>;
    async fn get_user_sessions(&self, user_id: Uuid) -> Vec<Session>;
}

#[async_trait::async_trait]
pub trait UserRepository: Send + Sync {
    async fn get(&self, id: Uuid) -> Option<User>;
    async fn get_by_phone(&self, phone: &str) -> Option<User>;
    async fn create(&self, user: User) -> User;
}

#[async_trait::async_trait]
pub trait VenueRepository: Send + Sync {
    async fn get(&self, id: Uuid) -> Option<Venue>;
    async fn list(&self) -> Vec<Venue>;
    async fn create(&self, venue: Venue) -> Venue;
}