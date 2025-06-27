use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Player skill level in padel
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "skill_level")]
pub enum SkillLevel {
    /// A - Beginner: New to padel, learning basic shots, positioning, and rules
    #[serde(rename = "A")]
    #[sqlx(rename = "A")]
    Beginner,
    /// B - Low Intermediate (M6/F6): Can sustain short rallies, understands basic tactics but lacks consistency
    #[serde(rename = "B")]
    #[sqlx(rename = "B")]
    LowIntermediate,
    /// C - Intermediate (M5/F5): Has reasonable consistency on forehand and backhand, understands positioning but struggles with faster-paced play
    #[serde(rename = "C")]
    #[sqlx(rename = "C")]
    Intermediate,
    /// D - Upper-Intermediate (M4/F4): More consistent, beginning to use lobs, volleys, and tactical positioning effectively
    #[serde(rename = "D")]
    #[sqlx(rename = "D")]
    UpperIntermediate,
    /// E - Advanced (M4+/F4+): Good control, can vary shots, comfortable at the net, and understands offensive/defensive transitions
    #[serde(rename = "E")]
    #[sqlx(rename = "E")]
    Advanced,
    /// F - High Advanced (M3/F3): Strong tactical awareness, executes smashes, viboras, and bandejas effectively, can handle high-paced play
    #[serde(rename = "F")]
    #[sqlx(rename = "F")]
    HighAdvanced,
    /// G - Expert (M2/F2): Very strong in all areas, plays fast-paced matches with high consistency and tactical intelligence
    #[serde(rename = "G")]
    #[sqlx(rename = "G")]
    Expert,
    /// H - Elite (M1/F1): Tournament-level player, highly skilled in strategy, shot placement, and game psychology
    #[serde(rename = "H")]
    #[sqlx(rename = "H")]
    Elite,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "gender")]
pub enum Gender {
    #[sqlx(rename = "Male")]
    Male,
    #[sqlx(rename = "Female")]
    Female
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[sqlx(type_name = "preferred_side")]
pub enum PreferredSide {
    #[sqlx(rename = "Right")]
    Right,
    #[sqlx(rename = "Left")]
    Left,
    #[sqlx(rename = "Flexible")]
    Flexible,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "play_frequency")]
pub enum PlayFrequency {
    #[sqlx(rename = "NeverPlayed")]
    NeverPlayed,
    #[sqlx(rename = "FewTimesMonth")]
    FewTimesMonth,
    #[sqlx(rename = "OnceWeek")]
    OnceWeek,
    #[sqlx(rename = "SeveralTimesWeek")]
    SeveralTimesWeek,
}

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "looking_for")]
pub enum LookingFor {
    #[sqlx(rename = "BusinessOpportunities")]
    BusinessOpportunities,
    #[sqlx(rename = "SocialConnections")]
    SocialConnections,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: String,
    pub email: String,
    pub city: String,
    pub photo_url: Option<String>,
    pub occupation: String,
    pub company: String,
    pub industry: String,
    pub linkedin_url: String,
    pub gender: Gender,
    pub skill_levels: Vec<SkillLevel>,
    pub preferred_side: PreferredSide,
    pub play_frequency: PlayFrequency,
    pub looking_for: Vec<LookingFor>,
    pub is_approved: bool,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(
        first_name: String,
        last_name: String,
        phone_number: String,
        email: String,
        city: String,
        photo_url: Option<String>,
        occupation: String,
        company: String,
        industry: String,
        linkedin_url: String,
        gender: Gender,
        skill_levels: Vec<SkillLevel>,
        preferred_side: PreferredSide,
        play_frequency: PlayFrequency,
        looking_for: Vec<LookingFor>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            first_name,
            last_name,
            phone_number,
            email,
            city,
            photo_url,
            occupation,
            company,
            industry,
            linkedin_url,
            gender,
            skill_levels,
            preferred_side,
            play_frequency,
            looking_for,
            is_approved: false,
            created_at: Utc::now(),
        }
    }

    pub fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }
}