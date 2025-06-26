use crate::state::AppState;
use axum::{extract::State, http::StatusCode, response::Json};
use rallybot_core::Venue;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreateVenueRequest {
    pub name: String,
    pub address: String,
}

pub async fn list_venues(State(state): State<AppState>) -> Json<Vec<Venue>> {
    let venues = state.venue_repository.list().await;
    Json(venues)
}

pub async fn create_venue(
    State(state): State<AppState>,
    Json(payload): Json<CreateVenueRequest>,
) -> (StatusCode, Json<Venue>) {
    let venue = Venue::new(payload.name, payload.address);
    let created = state.venue_repository.create(venue).await;
    (StatusCode::CREATED, Json(created))
}