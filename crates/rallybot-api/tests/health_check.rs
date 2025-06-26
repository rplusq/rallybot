mod helpers;

use axum::{
    body::Body,
    http::{Request, StatusCode},
};

#[tokio::test]
async fn health_check_works() {
    let app = helpers::TestApp::new().await;
    
    let request = Request::builder()
        .uri("/health")
        .body(Body::empty())
        .unwrap();
    
    let (status, _body) = app.call(request).await;
    
    assert_eq!(status, StatusCode::OK);
}