use rallybot_core::{SessionRepository, UserRepository, VenueRepository};
use std::sync::Arc;

#[derive(Clone)]
pub struct AppState {
    pub session_repository: Arc<dyn SessionRepository>,
    pub user_repository: Arc<dyn UserRepository>,
    pub venue_repository: Arc<dyn VenueRepository>,
}