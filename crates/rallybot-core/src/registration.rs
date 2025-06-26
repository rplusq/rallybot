use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum RegistrationStatus {
    Confirmed,
    Substitute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub status: RegistrationStatus,
    pub registered_at: DateTime<Utc>,
}

impl Registration {
    pub fn new(user_id: Uuid, session_id: Uuid, status: RegistrationStatus) -> Self {
        Self {
            user_id,
            session_id,
            status,
            registered_at: Utc::now(),
        }
    }
}