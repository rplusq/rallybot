use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "registration_status")]
pub enum RegistrationStatus {
    #[sqlx(rename = "Confirmed")]
    Confirmed,
    #[sqlx(rename = "Substitute")]
    Substitute,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registration {
    pub id: Uuid,
    pub user_id: Uuid,
    pub session_id: Uuid,
    pub status: RegistrationStatus,
    pub created_at: DateTime<Utc>,
}

impl Registration {
    pub fn new(user_id: Uuid, session_id: Uuid, status: RegistrationStatus) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            session_id,
            status,
            created_at: Utc::now(),
        }
    }
}