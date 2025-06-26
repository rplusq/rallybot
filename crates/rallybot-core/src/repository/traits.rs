use crate::{
    models::{Session, SessionType, Venue},
    registration::{Registration, RegistrationStatus},
    user::User,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug)]
pub enum RegistrationError {
    SessionNotFound,
    UserNotFound,
    UserNotApproved,
    AlreadyRegistered,
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

pub struct InMemoryRepository {
    sessions: Arc<Mutex<Vec<Session>>>,
    users: Arc<Mutex<Vec<User>>>,
    registrations: Arc<Mutex<Vec<Registration>>>,
    venues: Arc<Mutex<Vec<Venue>>>,
}

impl InMemoryRepository {
    pub fn new() -> Self {
        Self {
            sessions: Arc::new(Mutex::new(Vec::new())),
            users: Arc::new(Mutex::new(Vec::new())),
            registrations: Arc::new(Mutex::new(Vec::new())),
            venues: Arc::new(Mutex::new(Vec::new())),
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

    async fn create(&self, session: Session) -> Result<Session, SessionError> {
        let mut sessions = self.sessions.lock().await;
        sessions.push(session.clone());
        Ok(session)
    }

    async fn register_user(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<RegistrationStatus, RegistrationError> {
        // Check session exists
        let sessions = self.sessions.lock().await;
        if !sessions.iter().any(|s| s.id == session_id) {
            return Err(RegistrationError::SessionNotFound);
        }
        drop(sessions);

        // Check user exists and is approved
        let users = self.users.lock().await;
        let user = users
            .iter()
            .find(|u| u.id == user_id)
            .ok_or(RegistrationError::UserNotFound)?;
        
        if !user.is_approved {
            return Err(RegistrationError::UserNotApproved);
        }
        drop(users);

        // Check if already registered
        let mut registrations = self.registrations.lock().await;
        if registrations
            .iter()
            .any(|r| r.session_id == session_id && r.user_id == user_id)
        {
            return Err(RegistrationError::AlreadyRegistered);
        }

        // Count confirmed registrations
        let confirmed_count = registrations
            .iter()
            .filter(|r| r.session_id == session_id && r.status == RegistrationStatus::Confirmed)
            .count();

        // Determine status based on capacity
        let status = if confirmed_count < 4 {
            RegistrationStatus::Confirmed
        } else {
            RegistrationStatus::Substitute
        };

        // Create registration
        let registration = Registration::new(user_id, session_id, status);
        registrations.push(registration);

        Ok(status)
    }

    async fn get_registrations(&self, session_id: Uuid) -> Vec<Registration> {
        let registrations = self.registrations.lock().await;
        registrations
            .iter()
            .filter(|r| r.session_id == session_id)
            .cloned()
            .collect()
    }

    async fn get_user_sessions(&self, user_id: Uuid) -> Vec<Session> {
        let registrations = self.registrations.lock().await;
        let session_ids: Vec<Uuid> = registrations
            .iter()
            .filter(|r| r.user_id == user_id)
            .map(|r| r.session_id)
            .collect();
        drop(registrations);

        let sessions = self.sessions.lock().await;
        sessions
            .iter()
            .filter(|s| session_ids.contains(&s.id))
            .cloned()
            .collect()
    }
}

#[async_trait::async_trait]
impl UserRepository for InMemoryRepository {
    async fn get(&self, id: Uuid) -> Option<User> {
        let users = self.users.lock().await;
        users.iter().find(|u| u.id == id).cloned()
    }

    async fn get_by_phone(&self, phone: &str) -> Option<User> {
        let users = self.users.lock().await;
        users.iter().find(|u| u.phone_number == phone).cloned()
    }

    async fn create(&self, user: User) -> User {
        let mut users = self.users.lock().await;
        users.push(user.clone());
        user
    }
}

#[async_trait::async_trait]
impl VenueRepository for InMemoryRepository {
    async fn get(&self, id: Uuid) -> Option<Venue> {
        let venues = self.venues.lock().await;
        venues.iter().find(|v| v.id == id).cloned()
    }

    async fn list(&self) -> Vec<Venue> {
        let venues = self.venues.lock().await;
        venues.clone()
    }

    async fn create(&self, venue: Venue) -> Venue {
        let mut venues = self.venues.lock().await;
        venues.push(venue.clone());
        venue
    }
}