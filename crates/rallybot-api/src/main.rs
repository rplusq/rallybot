use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use rallybot_core::{InMemoryStorage, PostgresStorage, Repository};
use std::sync::Arc;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "rallybot_api=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    let app = if let Ok(database_url) = std::env::var("DATABASE_URL") {
        tracing::info!("Using PostgreSQL storage");
        let storage = PostgresStorage::new(&database_url)
            .await
            .expect("Failed to connect to PostgreSQL");
        let repository = Arc::new(Repository::new(Arc::new(storage)));
        rallybot_api::create_app_with_repository(repository)
    } else {
        tracing::info!("Using in-memory storage");
        let storage = Arc::new(InMemoryStorage::new());
        let repository = Arc::new(Repository::new(storage));
        rallybot_api::create_app_with_repository(repository)
    };
    
    let port = std::env::var("PORT").unwrap_or_else(|_| "3000".to_string());
    let bind_addr = format!("0.0.0.0:{}", port);
    
    let listener = tokio::net::TcpListener::bind(&bind_addr)
        .await
        .unwrap();
    
    tracing::info!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app).await.unwrap();
}