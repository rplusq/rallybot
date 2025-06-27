use super::Storage;
use crate::{
    models::{Session, SessionType, Venue},
    registration::Registration,
    user::User,
};
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

pub struct InMemoryStorage {
    sessions: Arc<Mutex<Vec<Session>>>,
    users: Arc<Mutex<Vec<User>>>,
    registrations: Arc<Mutex<Vec<Registration>>>,
    venues: Arc<Mutex<Vec<Venue>>>,
}

impl InMemoryStorage {
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
impl Storage for InMemoryStorage {
    async fn get_session(&self, id: Uuid) -> Option<Session> {
        let sessions = self.sessions.lock().await;
        sessions.iter().find(|s| s.id == id).cloned()
    }

    async fn list_sessions(&self, session_type: Option<SessionType>) -> Vec<Session> {
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

    async fn create_session(&self, session: Session) {
        let mut sessions = self.sessions.lock().await;
        sessions.push(session);
    }

    async fn get_user(&self, id: Uuid) -> Option<User> {
        let users = self.users.lock().await;
        users.iter().find(|u| u.id == id).cloned()
    }

    async fn get_user_by_phone(&self, phone: &str) -> Option<User> {
        let users = self.users.lock().await;
        users.iter().find(|u| u.phone_number == phone).cloned()
    }

    async fn create_user(&self, user: User) {
        let mut users = self.users.lock().await;
        users.push(user);
    }

    async fn get_registrations(&self, session_id: Uuid) -> Vec<Registration> {
        let registrations = self.registrations.lock().await;
        registrations
            .iter()
            .filter(|r| r.session_id == session_id)
            .cloned()
            .collect()
    }

    async fn get_user_registrations(&self, user_id: Uuid) -> Vec<Registration> {
        let registrations = self.registrations.lock().await;
        registrations
            .iter()
            .filter(|r| r.user_id == user_id)
            .cloned()
            .collect()
    }

    async fn create_registration(&self, registration: Registration) {
        let mut registrations = self.registrations.lock().await;
        registrations.push(registration);
    }

    async fn registration_exists(&self, session_id: Uuid, user_id: Uuid) -> bool {
        let registrations = self.registrations.lock().await;
        registrations
            .iter()
            .any(|r| r.session_id == session_id && r.user_id == user_id)
    }

    async fn delete_registration(&self, session_id: Uuid, user_id: Uuid) -> bool {
        let mut registrations = self.registrations.lock().await;
        let initial_len = registrations.len();
        registrations.retain(|r| !(r.session_id == session_id && r.user_id == user_id));
        registrations.len() < initial_len
    }

    async fn update_registration(&self, registration: Registration) -> bool {
        let mut registrations = self.registrations.lock().await;
        if let Some(pos) = registrations.iter().position(|r| 
            r.session_id == registration.session_id && r.user_id == registration.user_id
        ) {
            registrations[pos] = registration;
            true
        } else {
            false
        }
    }

    async fn get_venue(&self, id: Uuid) -> Option<Venue> {
        let venues = self.venues.lock().await;
        venues.iter().find(|v| v.id == id).cloned()
    }

    async fn list_venues(&self) -> Vec<Venue> {
        let venues = self.venues.lock().await;
        venues.clone()
    }

    async fn create_venue(&self, venue: Venue) {
        let mut venues = self.venues.lock().await;
        venues.push(venue);
    }
}