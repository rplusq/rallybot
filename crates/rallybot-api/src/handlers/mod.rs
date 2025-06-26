pub mod sessions;
pub mod users;
pub mod venues;

use axum::http::StatusCode;

pub async fn health_check() -> StatusCode {
    StatusCode::OK
}