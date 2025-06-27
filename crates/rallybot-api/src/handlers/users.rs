use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use rallybot_core::{Session, User, Gender, SkillLevel, PreferredSide, PlayFrequency, LookingFor};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateUserRequest {
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
}

pub async fn create_user(
    State(state): State<AppState>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), StatusCode> {
    // Check if user with this phone already exists
    if state.user_repository.get_by_phone(&payload.phone_number).await.is_some() {
        return Err(StatusCode::CONFLICT);
    }
    
    let user = User::new(
        payload.first_name,
        payload.last_name,
        payload.phone_number,
        payload.email,
        payload.city,
        payload.photo_url,
        payload.occupation,
        payload.company,
        payload.industry,
        payload.linkedin_url,
        payload.gender,
        payload.skill_levels,
        payload.preferred_side,
        payload.play_frequency,
        payload.looking_for,
    );
    
    state.user_repository.create(user.clone()).await;
    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn get_user_by_phone(
    State(state): State<AppState>,
    Path(phone): Path<String>,
) -> Result<Json<User>, StatusCode> {
    state
        .user_repository
        .get_by_phone(&phone)
        .await
        .map(Json)
        .ok_or(StatusCode::NOT_FOUND)
}

pub async fn get_user_sessions(
    State(state): State<AppState>,
    Path(phone): Path<String>,
) -> Result<Json<Vec<Session>>, StatusCode> {
    let user = state
        .user_repository
        .get_by_phone(&phone)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    let sessions = state.session_repository.get_user_sessions(user.id).await;
    Ok(Json(sessions))
}