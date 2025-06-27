use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;
use crate::user::SkillLevel;

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
    pub skill_level: Option<SkillLevel>,
}

impl Session {
    pub fn new(
        session_type: SessionType,
        datetime: DateTime<Utc>,
        duration_minutes: i32,
        venue_id: Uuid,
        skill_level: Option<SkillLevel>,
    ) -> Result<Self, &'static str> {
        if duration_minutes < 60 || duration_minutes > 120 {
            return Err("Duration must be between 60 and 120 minutes");
        }
        if duration_minutes % 30 != 0 {
            return Err("Duration must be in 30-minute increments");
        }
        
        // Validate skill_level based on session_type
        match session_type {
            SessionType::Mixed => {
                if skill_level.is_some() {
                    return Err("Mixed sessions cannot have a skill level");
                }
            }
            SessionType::Coaching | SessionType::Social | SessionType::League => {
                if skill_level.is_none() {
                    return Err("Coaching, Social, and League sessions must have a skill level");
                }
            }
        }
        
        Ok(Self {
            id: Uuid::new_v4(),
            session_type,
            datetime,
            duration_minutes,
            venue_id,
            skill_level,
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
        assert!(Session::new(SessionType::Social, datetime, 60, venue_id, Some(SkillLevel::Intermediate)).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id, Some(SkillLevel::Advanced)).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 120, venue_id, Some(SkillLevel::Beginner)).is_ok());
    }

    #[test]
    fn session_invalid_duration_too_short() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        let result = Session::new(SessionType::Social, datetime, 30, venue_id, Some(SkillLevel::Intermediate));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duration must be between 60 and 120 minutes");
    }

    #[test]
    fn session_invalid_duration_too_long() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        let result = Session::new(SessionType::Social, datetime, 150, venue_id, Some(SkillLevel::Intermediate));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duration must be between 60 and 120 minutes");
    }

    #[test]
    fn session_invalid_duration_not_30_minute_increment() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        let result = Session::new(SessionType::Social, datetime, 75, venue_id, Some(SkillLevel::Intermediate));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Duration must be in 30-minute increments");
    }

    #[test]
    fn session_skill_level_required_for_coaching() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        // Coaching session without skill level should fail
        let result = Session::new(SessionType::Coaching, datetime, 90, venue_id, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Coaching, Social, and League sessions must have a skill level");
        
        // Coaching session with skill level should succeed
        assert!(Session::new(SessionType::Coaching, datetime, 90, venue_id, Some(SkillLevel::Intermediate)).is_ok());
    }

    #[test]
    fn session_skill_level_required_for_social() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        // Social session without skill level should fail
        let result = Session::new(SessionType::Social, datetime, 90, venue_id, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Coaching, Social, and League sessions must have a skill level");
        
        // Social session with skill level should succeed
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id, Some(SkillLevel::Advanced)).is_ok());
    }

    #[test]
    fn session_skill_level_required_for_league() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        // League session without skill level should fail
        let result = Session::new(SessionType::League, datetime, 90, venue_id, None);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Coaching, Social, and League sessions must have a skill level");
        
        // League session with skill level should succeed
        assert!(Session::new(SessionType::League, datetime, 90, venue_id, Some(SkillLevel::UpperIntermediate)).is_ok());
    }

    #[test]
    fn session_mixed_cannot_have_skill_level() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        // Mixed session with skill level should fail
        let result = Session::new(SessionType::Mixed, datetime, 90, venue_id, Some(SkillLevel::Intermediate));
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "Mixed sessions cannot have a skill level");
        
        // Mixed session without skill level should succeed
        assert!(Session::new(SessionType::Mixed, datetime, 90, venue_id, None).is_ok());
    }

    #[test]
    fn session_all_skill_levels_work() {
        let venue_id = Uuid::new_v4();
        let datetime = Utc::now();
        
        // Test all skill levels work with non-Mixed sessions
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id, Some(SkillLevel::Beginner)).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id, Some(SkillLevel::LowIntermediate)).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id, Some(SkillLevel::Intermediate)).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id, Some(SkillLevel::UpperIntermediate)).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id, Some(SkillLevel::Advanced)).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id, Some(SkillLevel::HighAdvanced)).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id, Some(SkillLevel::Expert)).is_ok());
        assert!(Session::new(SessionType::Social, datetime, 90, venue_id, Some(SkillLevel::Elite)).is_ok());
    }
}