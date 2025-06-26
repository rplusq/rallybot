mod helpers;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use serde_json::{json, Value};

#[tokio::test]
async fn create_session_returns_201() {
    let app = helpers::TestApp::new().await;
    
    // First create a venue
    let venue_id = app.create_test_venue().await;
    
    let body = json!({
        "session_type": "S",
        "datetime": "2024-12-31T18:00:00Z",
        "duration_minutes": 90,
        "venue_id": venue_id
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/sessions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    
    let (status, body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::CREATED);
    
    let response: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(response["session_type"], "S");
    assert_eq!(response["duration_minutes"], 90);
    assert_eq!(response["venue_id"], venue_id.to_string());
    assert!(response["id"].is_string());
}

#[tokio::test]
async fn create_session_with_invalid_venue_returns_400() {
    let app = helpers::TestApp::new().await;
    
    let body = json!({
        "session_type": "S",
        "datetime": "2024-12-31T18:00:00Z",
        "duration_minutes": 90,
        "venue_id": "00000000-0000-0000-0000-000000000000"
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/sessions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    
    let (status, _body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn create_session_with_invalid_duration_returns_400() {
    let app = helpers::TestApp::new().await;
    
    // Create a venue first
    let venue_id = app.create_test_venue().await;
    
    let body = json!({
        "session_type": "S",
        "datetime": "2024-12-31T18:00:00Z",
        "duration_minutes": 45, // Invalid: not in 30 min increments
        "venue_id": venue_id
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/sessions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    
    let (status, _body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::BAD_REQUEST);
}

#[tokio::test]
async fn list_sessions_returns_empty_array() {
    let app = helpers::TestApp::new().await;
    
    let request = Request::builder()
        .uri("/sessions")
        .body(Body::empty())
        .unwrap();
    
    let (status, body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::OK);
    assert_eq!(body, "[]");
}

#[tokio::test]
async fn list_sessions_filters_by_type() {
    let app = helpers::TestApp::new().await;
    
    // Create venues
    let venue1 = app.create_test_venue().await;
    let venue2 = app.create_test_venue().await;
    
    // Create coaching session
    let coaching = json!({
        "session_type": "C",
        "datetime": "2024-12-30T10:00:00Z",
        "duration_minutes": 90,
        "venue_id": venue1
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/sessions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&coaching).unwrap()))
        .unwrap();
    
    app.call(request).await;
    
    // Create social session
    let social = json!({
        "session_type": "S",
        "datetime": "2024-12-31T18:00:00Z",
        "duration_minutes": 60,
        "venue_id": venue2
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/sessions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&social).unwrap()))
        .unwrap();
    
    app.call(request).await;
    
    // List only social sessions
    let request = Request::builder()
        .uri("/sessions?type=S")
        .body(Body::empty())
        .unwrap();
    
    let (status, body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::OK);
    
    let sessions: Vec<Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(sessions.len(), 1);
    assert_eq!(sessions[0]["session_type"], "S");
}