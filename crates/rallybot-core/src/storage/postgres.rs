use super::Storage;
use crate::{
    models::{Session, SessionType, Venue},
    registration::{Registration, RegistrationStatus},
    user::{Gender, LookingFor, PlayFrequency, PreferredSide, SkillLevel, User},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use uuid::Uuid;

#[derive(Clone)]
pub struct PostgresStorage {
    pool: PgPool,
}

impl PostgresStorage {
    pub async fn new(database_url: &str) -> Result<Self, sqlx::Error> {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(database_url)
            .await?;

        Ok(Self { pool })
    }

    pub fn new_with_pool(pool: PgPool) -> Self {
        Self { pool }
    }
}

#[async_trait::async_trait]
impl Storage for PostgresStorage {
    async fn get_session(&self, id: Uuid) -> Option<Session> {
        sqlx::query_as!(
            Session,
            r#"
            SELECT id, session_type as "session_type: SessionType", datetime, duration_minutes, venue_id, skill_level as "skill_level: SkillLevel"
            FROM sessions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()
    }

    async fn list_sessions(&self, session_type: Option<SessionType>) -> Vec<Session> {
        match session_type {
            Some(st) => {
                sqlx::query_as!(
                    Session,
                    r#"
                    SELECT id, session_type as "session_type: SessionType", datetime, duration_minutes, venue_id, skill_level as "skill_level: SkillLevel"
                    FROM sessions
                    WHERE session_type = $1
                    ORDER BY datetime
                    "#,
                    st as SessionType
                )
                .fetch_all(&self.pool)
                .await
                .unwrap_or_default()
            }
            None => {
                sqlx::query_as!(
                    Session,
                    r#"
                    SELECT id, session_type as "session_type: SessionType", datetime, duration_minutes, venue_id, skill_level as "skill_level: SkillLevel"
                    FROM sessions
                    ORDER BY datetime
                    "#
                )
                .fetch_all(&self.pool)
                .await
                .unwrap_or_default()
            }
        }
    }

    async fn create_session(&self, session: Session) {
        let _ = sqlx::query!(
            r#"
            INSERT INTO sessions (id, session_type, datetime, duration_minutes, venue_id, skill_level)
            VALUES ($1, $2, $3, $4, $5, $6)
            "#,
            session.id,
            session.session_type as SessionType,
            session.datetime,
            session.duration_minutes as i32,
            session.venue_id,
            session.skill_level as _
        )
        .execute(&self.pool)
        .await;
    }

    async fn get_user(&self, id: Uuid) -> Option<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, first_name, last_name, phone_number, email, city, photo_url,
                   occupation, company, industry, linkedin_url, gender as "gender: Gender",
                   skill_levels as "skill_levels: Vec<SkillLevel>",
                   preferred_side as "preferred_side: PreferredSide",
                   play_frequency as "play_frequency: PlayFrequency",
                   looking_for as "looking_for: Vec<LookingFor>",
                   is_approved, created_at
            FROM users
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()
    }

    async fn get_user_by_phone(&self, phone: &str) -> Option<User> {
        sqlx::query_as!(
            User,
            r#"
            SELECT id, first_name, last_name, phone_number, email, city, photo_url,
                   occupation, company, industry, linkedin_url, gender as "gender: Gender",
                   skill_levels as "skill_levels: Vec<SkillLevel>",
                   preferred_side as "preferred_side: PreferredSide",
                   play_frequency as "play_frequency: PlayFrequency",
                   looking_for as "looking_for: Vec<LookingFor>",
                   is_approved, created_at
            FROM users
            WHERE phone_number = $1
            "#,
            phone
        )
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()
    }

    async fn create_user(&self, user: User) {
        let _ = sqlx::query!(
            r#"
            INSERT INTO users (id, first_name, last_name, phone_number, email, city,
                             photo_url, occupation, company, industry, linkedin_url, gender,
                             skill_levels, preferred_side, play_frequency, looking_for,
                             is_approved, created_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18)
            "#,
            user.id,
            user.first_name,
            user.last_name,
            user.phone_number,
            user.email,
            user.city,
            user.photo_url,
            user.occupation,
            user.company,
            user.industry,
            user.linkedin_url,
            user.gender as Gender,
            &user.skill_levels as &Vec<SkillLevel>,
            user.preferred_side as PreferredSide,
            user.play_frequency as PlayFrequency,
            &user.looking_for as &Vec<LookingFor>,
            user.is_approved,
            user.created_at
        )
        .execute(&self.pool)
        .await;
    }

    async fn get_registrations(&self, session_id: Uuid) -> Vec<Registration> {
        sqlx::query_as!(
            Registration,
            r#"
            SELECT id, user_id, session_id, status as "status: RegistrationStatus", created_at
            FROM registrations
            WHERE session_id = $1
            ORDER BY created_at
            "#,
            session_id
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default()
    }

    async fn get_user_registrations(&self, user_id: Uuid) -> Vec<Registration> {
        sqlx::query_as!(
            Registration,
            r#"
            SELECT id, user_id, session_id, status as "status: RegistrationStatus", created_at
            FROM registrations
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(&self.pool)
        .await
        .unwrap_or_default()
    }

    async fn create_registration(&self, registration: Registration) {
        let _ = sqlx::query!(
            r#"
            INSERT INTO registrations (id, user_id, session_id, status, created_at)
            VALUES ($1, $2, $3, $4, $5)
            "#,
            registration.id,
            registration.user_id,
            registration.session_id,
            registration.status as RegistrationStatus,
            registration.created_at
        )
        .execute(&self.pool)
        .await;
    }

    async fn registration_exists(&self, session_id: Uuid, user_id: Uuid) -> bool {
        sqlx::query!(
            r#"
            SELECT EXISTS(
                SELECT 1 FROM registrations
                WHERE session_id = $1 AND user_id = $2
            ) as "exists!"
            "#,
            session_id,
            user_id
        )
        .fetch_one(&self.pool)
        .await
        .map(|r| r.exists)
        .unwrap_or(false)
    }

    async fn delete_registration(&self, session_id: Uuid, user_id: Uuid) -> bool {
        sqlx::query!(
            r#"
            DELETE FROM registrations
            WHERE session_id = $1 AND user_id = $2
            "#,
            session_id,
            user_id
        )
        .execute(&self.pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .unwrap_or(false)
    }

    async fn update_registration(&self, registration: Registration) -> bool {
        sqlx::query!(
            r#"
            UPDATE registrations
            SET status = $3, created_at = $4
            WHERE session_id = $1 AND user_id = $2
            "#,
            registration.session_id,
            registration.user_id,
            registration.status as RegistrationStatus,
            registration.created_at
        )
        .execute(&self.pool)
        .await
        .map(|result| result.rows_affected() > 0)
        .unwrap_or(false)
    }

    async fn get_venue(&self, id: Uuid) -> Option<Venue> {
        sqlx::query_as!(
            Venue,
            "SELECT id, name, address FROM venues WHERE id = $1",
            id
        )
        .fetch_optional(&self.pool)
        .await
        .ok()
        .flatten()
    }

    async fn list_venues(&self) -> Vec<Venue> {
        sqlx::query_as!(Venue, "SELECT id, name, address FROM venues ORDER BY name")
            .fetch_all(&self.pool)
            .await
            .unwrap_or_default()
    }

    async fn create_venue(&self, venue: Venue) {
        let _ = sqlx::query!(
            "INSERT INTO venues (id, name, address) VALUES ($1, $2, $3)",
            venue.id,
            venue.name,
            venue.address
        )
        .execute(&self.pool)
        .await;
    }
}
