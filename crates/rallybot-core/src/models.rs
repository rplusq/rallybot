use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum SessionType {
    #[serde(rename = "C")]
    Coaching,
    #[serde(rename = "S")]
    Social,
    #[serde(rename = "L")]
    League,
    #[serde(rename = "X")]
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
    pub name: String,
    pub address: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub session_type: SessionType,
    pub datetime: DateTime<Utc>,
    pub duration_minutes: u32,
    pub venue: Venue,
}

impl Session {
    pub fn new(
        session_type: SessionType,
        datetime: DateTime<Utc>,
        duration_minutes: u32,
        venue: Venue,
    ) -> Result<Self, &'static str> {
        if duration_minutes < 60 || duration_minutes > 120 {
            return Err("Duration must be between 60 and 120 minutes");
        }
        if duration_minutes % 30 != 0 {
            return Err("Duration must be in 30-minute increments");
        }
        
        Ok(Self {
            id: Uuid::new_v4(),
            session_type,
            datetime,
            duration_minutes,
            venue,
        })
    }
}