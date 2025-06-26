use crate::state::AppState;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use rallybot_core::Session;

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