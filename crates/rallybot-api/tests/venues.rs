mod helpers;

use axum::{
    body::Body,
    http::{Method, Request, StatusCode},
};
use serde_json::{json, Value};

#[tokio::test]
async fn create_venue_returns_201() {
    let app = helpers::TestApp::new().await;
    
    let body = json!({
        "name": "Test Sports Center",
        "address": "123 Test Street, Test City"
    });
    
    let request = Request::builder()
        .method(Method::POST)
        .uri("/venues")
        .header("content-type", "application/json")
        .body(Body::from(serde_json::to_string(&body).unwrap()))
        .unwrap();
    
    let (status, body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::CREATED);
    
    let response: Value = serde_json::from_str(&body).unwrap();
    assert_eq!(response["name"], "Test Sports Center");
    assert_eq!(
        response["address"],
        "123 Test Street, Test City"
    );
    assert!(response["id"].is_string());
}

#[tokio::test]
async fn list_venues_returns_all_venues() {
    let app = helpers::TestApp::new().await;
    
    // Create two venues with specific names for testing
    let venue1_name = "Alpha Sports Center";
    let venue2_name = "Beta Sports Center";
    
    app.create_test_venue_with_data(Some(venue1_name), None).await;
    app.create_test_venue_with_data(Some(venue2_name), None).await;
    
    let request = Request::builder()
        .uri("/venues")
        .body(Body::empty())
        .unwrap();
    
    let (status, body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::OK);
    
    let venues: Vec<Value> = serde_json::from_str(&body).unwrap();
    assert_eq!(venues.len(), 2);
    
    let venue_names: Vec<String> = venues
        .iter()
        .map(|v| v["name"].as_str().unwrap().to_string())
        .collect();
    
    assert!(venue_names.contains(&venue1_name.to_string()));
    assert!(venue_names.contains(&venue2_name.to_string()));
}