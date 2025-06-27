mod helpers;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};
use rallybot_core::SessionType;
use serde_json::json;

/// Test that both storage implementations work correctly
#[tokio::test]
async fn test_in_memory_storage() {
    let app = helpers::TestApp::with_in_memory().await;
    run_basic_flow(app).await;
}

#[tokio::test]
#[serial_test::serial]
async fn test_postgres_storage() {
    let app = helpers::TestApp::with_postgres().await;
    run_basic_flow(app).await;
    // Cleanup happens automatically when app is dropped
}

async fn run_basic_flow(app: helpers::TestApp) {
    // Create a venue
    let venue_id = app.create_test_venue().await;
    
    // Create a session
    let session_datetime = chrono::Utc::now() + chrono::Duration::days(1);
    let create_session_request = Request::builder()
        .method("POST")
        .uri("/sessions")
        .header("content-type", "application/json")
        .body(Body::from(json!({
            "session_type": SessionType::Social,
            "datetime": session_datetime,
            "duration_minutes": 90,
            "venue_id": venue_id,
            "skill_level": "C"
        }).to_string()))
        .unwrap();
    
    let (status, body) = app.call(create_session_request).await;
    assert_eq!(status, StatusCode::CREATED);
    
    let session: serde_json::Value = serde_json::from_str(&body).unwrap();
    let session_id = session["id"].as_str().unwrap();
    
    // List sessions
    let list_request = Request::builder()
        .uri("/sessions")
        .body(Body::empty())
        .unwrap();
    
    let (status, body) = app.call(list_request).await;
    assert_eq!(status, StatusCode::OK);
    
    let sessions: Vec<serde_json::Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0]["id"].as_str().unwrap(), session_id);
    
    // Test with filter
    let filtered_request = Request::builder()
        .uri("/sessions?type=S")
        .body(Body::empty())
        .unwrap();
    
    let (status, body) = app.call(filtered_request).await;
    assert_eq!(status, StatusCode::OK);
    
    let sessions: Vec<serde_json::Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(sessions.len(), 1);
    
    // Cleanup for postgres happens automatically
    if let Some(test_db) = app.test_db {
        test_db.cleanup().await;
    }
}