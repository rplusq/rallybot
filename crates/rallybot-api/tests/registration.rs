mod helpers;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use serde_json::{json, Value};

#[tokio::test]
async fn register_for_session_returns_confirmed() {
    let app = helpers::TestApp::new().await;
    
    // Create venue, user, and session
    let venue_id = app.create_test_venue().await;
    app.create_test_user("+351912345678", true).await;
    
    let session_body = json!({
        "session_type": "S",
        "datetime": "2024-12-31T18:00:00Z",
        "duration_minutes": 90,
        "venue_id": venue_id
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/sessions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&session_body).unwrap()))
        .unwrap();
    
    let (_status, body) = app.call(request).await;
    let session: Value = serde_json::from_str(&body).unwrap();
    let session_id = session["id"].as_str().unwrap();
    
    // Register for session
    let register_body = json!({
        "phone_number": "+351912345678"
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/sessions/{}/register", session_id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body).unwrap()))
        .unwrap();
    
    let (status, body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::OK);
    
    let response: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(response["status"], "confirmed");
    assert_eq!(response["message"], "Successfully registered!");
}

#[tokio::test]
async fn register_with_unapproved_user_returns_403() {
    let app = helpers::TestApp::new().await;
    
    // Create venue, unapproved user, and session
    let venue_id = app.create_test_venue().await;
    app.create_test_user("+351987654321", false).await; // Not approved
    
    let session_body = json!({
        "session_type": "S",
        "datetime": "2024-12-31T18:00:00Z",
        "duration_minutes": 90,
        "venue_id": venue_id
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/sessions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&session_body).unwrap()))
        .unwrap();
    
    let (_status, body) = app.call(request).await;
    let session: Value = serde_json::from_str(&body).unwrap();
    let session_id = session["id"].as_str().unwrap();
    
    // Try to register
    let register_body = json!({
        "phone_number": "+351987654321"
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/sessions/{}/register", session_id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body).unwrap()))
        .unwrap();
    
    let (status, body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::FORBIDDEN);
    assert_eq!(body, "User not approved");
}

#[tokio::test]
async fn fifth_registration_becomes_substitute() {
    let app = helpers::TestApp::new().await;
    
    // Create venue and session
    let venue_id = app.create_test_venue().await;
    
    let session_body = json!({
        "session_type": "S",
        "datetime": "2024-12-31T18:00:00Z",
        "duration_minutes": 90,
        "venue_id": venue_id
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/sessions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&session_body).unwrap()))
        .unwrap();
    
    let (_status, body) = app.call(request).await;
    let session: Value = serde_json::from_str(&body).unwrap();
    let session_id = session["id"].as_str().unwrap();
    
    // Create and register 4 users (should all be confirmed)
    for i in 1..=4 {
        let phone = format!("+35191234567{}", i);
        app.create_test_user(&phone, true).await;
        
        let register_body = json!({ "phone_number": phone });
        
        let request = Request::builder()
            .method(Method::POST)
            .uri(&format!("/sessions/{}/register", session_id))
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_string(&register_body).unwrap()))
            .unwrap();
        
        let (status, body) = app.call(request).await;
        assert_eq!(status, StatusCode::OK);
        
        let response: Value = serde_json::from_str(&body).unwrap();
        assert_eq!(response["status"], "confirmed");
    }
    
    // Register 5th user (should be substitute)
    let phone5 = "+351912345675";
    app.create_test_user(phone5, true).await;
    
    let register_body = json!({ "phone_number": phone5 });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/sessions/{}/register", session_id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body).unwrap()))
        .unwrap();
    
    let (status, body) = app.call(request).await;
    assert_eq!(status, StatusCode::OK);
    
    let response: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(response["status"], "substitute");
    assert_eq!(response["message"], "Added to substitute list");
}

#[tokio::test]
async fn double_registration_returns_conflict() {
    let app = helpers::TestApp::new().await;
    
    // Create venue, user, and session
    let venue_id = app.create_test_venue().await;
    app.create_test_user("+351912345678", true).await;
    
    let session_body = json!({
        "session_type": "S",
        "datetime": "2024-12-31T18:00:00Z",
        "duration_minutes": 90,
        "venue_id": venue_id
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/sessions")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&session_body).unwrap()))
        .unwrap();
    
    let (_status, body) = app.call(request).await;
    let session: Value = serde_json::from_str(&body).unwrap();
    let session_id = session["id"].as_str().unwrap();
    
    let register_body = json!({ "phone_number": "+351912345678" });
    
    // First registration
    let request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/sessions/{}/register", session_id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body).unwrap()))
        .unwrap();
    
    app.call(request).await;
    
    // Second registration (should fail)
    let request = Request::builder()
        .method(Method::POST)
        .uri(&format!("/sessions/{}/register", session_id))
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&register_body).unwrap()))
        .unwrap();
    
    let (status, body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::CONFLICT);
    assert_eq!(body, "Already registered");
}