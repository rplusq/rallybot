use crate::state::AppState;
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::Json,
};
use rallybot_core::{
    Registration, RegistrationError, RegistrationStatus, Session, SessionError, SessionType,
    SkillLevel,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Deserialize)]
pub struct ListSessionsQuery {
    #[serde(rename = "type")]
    pub session_type: Option<SessionType>,
    #[serde(default)]
    pub summary: bool,
}

#[derive(Deserialize)]
pub struct CreateSessionRequest {
    pub session_type: SessionType,
    pub datetime: chrono::DateTime<chrono::Utc>,
    pub duration_minutes: i32,
    pub venue_id: Uuid,
    pub skill_level: Option<SkillLevel>,
}

pub async fn list_sessions(
    State(state): State<AppState>,
    Query(params): Query<ListSessionsQuery>,
) -> Json<Vec<Session>> {
    let sessions = state.session_repository.list(params.session_type).await;
    Json(sessions)
}

pub async fn create_session(
    State(state): State<AppState>,
    Json(payload): Json<CreateSessionRequest>,
) -> Result<(StatusCode, Json<Session>), StatusCode> {
    let session = Session::new(
        payload.session_type,
        payload.datetime,
        payload.duration_minutes,
        payload.venue_id,
        payload.skill_level,
    )
    .map_err(|_| StatusCode::BAD_REQUEST)?;

    match state.session_repository.create(session).await {
        Ok(created) => Ok((StatusCode::CREATED, Json(created))),
        Err(SessionError::VenueNotFound) => Err(StatusCode::BAD_REQUEST),
    }
}

#[derive(Deserialize)]
pub struct RegisterRequest {
    pub phone_number: String,
}

#[derive(Serialize)]
pub struct RegisterResponse {
    pub status: RegistrationStatus,
    pub message: String,
}

pub async fn register_for_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<RegisterRequest>,
) -> Result<Json<RegisterResponse>, (StatusCode, String)> {
    // Find user by phone
    let user = state
        .user_repository
        .get_by_phone(&payload.phone_number)
        .await
        .ok_or((StatusCode::NOT_FOUND, "User not found".to_string()))?;

    // Register user
    let status = state
        .session_repository
        .register_user(session_id, user.id)
        .await
        .map_err(|e| match e {
            RegistrationError::SessionNotFound => {
                (StatusCode::NOT_FOUND, "Session not found".to_string())
            }
            RegistrationError::UserNotApproved => {
                (StatusCode::FORBIDDEN, "User not approved".to_string())
            }
            RegistrationError::AlreadyRegistered => {
                (StatusCode::CONFLICT, "Already registered".to_string())
            }
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Registration failed".to_string(),
            ),
        })?;

    let message = match status {
        RegistrationStatus::Confirmed => "Successfully registered!".to_string(),
        RegistrationStatus::Substitute => "Added to substitute list".to_string(),
    };

    Ok(Json(RegisterResponse { status, message }))
}

#[derive(Deserialize)]
pub struct UnregisterRequest {
    pub user_id: Uuid,
}

pub async fn unregister_from_session(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
    Json(payload): Json<UnregisterRequest>,
) -> Result<StatusCode, StatusCode> {
    match state
        .session_repository
        .unregister_user(session_id, payload.user_id)
        .await
    {
        Ok(()) => Ok(StatusCode::NO_CONTENT),
        Err(RegistrationError::NotRegistered) => Err(StatusCode::NOT_FOUND),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

#[derive(Serialize)]
pub struct SessionDetails {
    #[serde(flatten)]
    pub session: Session,
    pub confirmed_count: usize,
    pub substitute_count: usize,
}

pub async fn get_session_details(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Result<Json<SessionDetails>, StatusCode> {
    let session = state
        .session_repository
        .get(session_id)
        .await
        .ok_or(StatusCode::NOT_FOUND)?;

    let registrations = state.session_repository.get_registrations(session_id).await;

    let confirmed_count = registrations
        .iter()
        .filter(|r| r.status == RegistrationStatus::Confirmed)
        .count();

    let substitute_count = registrations
        .iter()
        .filter(|r| r.status == RegistrationStatus::Substitute)
        .count();

    Ok(Json(SessionDetails {
        session,
        confirmed_count,
        substitute_count,
    }))
}

pub async fn get_session_registrations(
    State(state): State<AppState>,
    Path(session_id): Path<Uuid>,
) -> Json<Vec<Registration>> {
    let registrations = state.session_repository.get_registrations(session_id).await;
    Json(registrations)
}
