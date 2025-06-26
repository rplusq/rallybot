pub mod handlers;
pub mod state;

use axum::{routing::{get, post}, Router};
use rallybot_core::{InMemoryStorage, Repository, Storage};
use state::AppState;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub fn create_app() -> Router {
    let storage = Arc::new(InMemoryStorage::new());
    let repository = Arc::new(Repository::new(storage));
    create_app_with_repository(repository)
}

pub fn create_app_with_repository<S: Storage + 'static>(
    repository: Arc<Repository<S>>,
) -> Router {
    let state = AppState {
        session_repository: repository.clone() as Arc<dyn rallybot_core::SessionRepository>,
        user_repository: repository.clone() as Arc<dyn rallybot_core::UserRepository>,
        venue_repository: repository as Arc<dyn rallybot_core::VenueRepository>,
    };

    Router::new()
        .route("/sessions", get(handlers::sessions::list_sessions).post(handlers::sessions::create_session))
        .route("/sessions/:id/register", post(handlers::sessions::register_for_session))
        .route("/sessions/:id/registrations", get(handlers::sessions::get_session_registrations))
        .route("/users/:phone/sessions", get(handlers::users::get_user_sessions))
        .route("/venues", get(handlers::venues::list_venues).post(handlers::venues::create_venue))
        .route("/health", get(handlers::health_check))
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}