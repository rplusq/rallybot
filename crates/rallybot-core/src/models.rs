use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "UPPERCASE")]
#[sqlx(type_name = "session_type")]
pub enum SessionType {
    #[serde(rename = "C")]
    #[sqlx(rename = "C")]
    Coaching,
    #[serde(rename = "S")]
    #[sqlx(rename = "S")]
    Social,
    #[serde(rename = "L")]
    #[sqlx(rename = "L")]
    League,
    #[serde(rename = "X")]
    #[sqlx(rename = "X")]
    Mixed,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Venue {
    pub id: Uuid,
    pub name: String,
    pub address: String,
}

impl Venue {
    pub fn new(name: String, address: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            address,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Session {
    pub id: Uuid,
    pub session_type: SessionType,
    pub datetime: DateTime<Utc>,
    pub duration_minutes: i32,
    pub venue_id: Uuid,
}

impl Session {
    pub fn new(
        session_type: SessionType,
        datetime: DateTime<Utc>,
        duration_minutes: i32,
        venue_id: Uuid,
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
            venue_id,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_valid_durations() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        // Valid durations: 60, 90, 120
        assert!(Session::new(SessionType::Social, datetime, 60, venue_id).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 120, venue_id).is_ok());
    }

    #[test]
    fn session_invalid_duration_too_short() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        let result = Session::new(SessionType::Social, datetime, 30, venue_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duration must be between 60 and 120 minutes");
    }

    #[test]
    fn session_invalid_duration_too_long() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        let result = Session::new(SessionType::Social, datetime, 150, venue_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duration must be between 60 and 120 minutes");
    }

    #[test]
    fn session_invalid_duration_not_30_minute_increment() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        let result = Session::new(SessionType::Social, datetime, 75, venue_id);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duration must be in 30-minute increments");
    }
}