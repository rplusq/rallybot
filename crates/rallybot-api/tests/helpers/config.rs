use sqlx::postgres::PgPoolOptions;
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestDatabase {
    pub db_name: String,
    pub connection_string: String,
}

impl TestDatabase {
    pub async fn new() -> Self {
        dotenvy::from_filename(".env.test").ok();
        
        let db_name = format!("rallybot_test_{}", Uuid::new_v4().simple());
        let base_url = std::env::var("TEST_DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://rallybot:rallybot@localhost:5432/postgres".to_string());
        
        // Connect to postgres database to create our test database
        let mut conn = PgConnection::connect(&base_url).await
            .expect("Failed to connect to Postgres");
        
        conn.execute(&*format!(r#"CREATE DATABASE "{}""#, db_name))
            .await
            .expect("Failed to create test database");
        
        let connection_string = base_url.replace("/postgres", &format!("/{}", db_name));
        
        Self {
            db_name,
            connection_string,
        }
    }
    
    pub async fn get_pool(&self) -> PgPool {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&self.connection_string)
            .await
            .expect("Failed to connect to test database");
        
        // Run migrations
        sqlx::migrate!("../../migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");
        
        pool
    }
    
    pub async fn cleanup(self) {
        // Connect to postgres database to drop our test database
        let base_url = self.connection_string
            .replace(&format!("/{}", self.db_name), "/postgres");
        
        let mut conn = PgConnection::connect(&base_url).await
            .expect("Failed to connect to Postgres");
        
        // Terminate existing connections
        let query = format!(
            r#"
            SELECT pg_terminate_backend(pg_stat_activity.pid)
            FROM pg_stat_activity
            WHERE pg_stat_activity.datname = '{}'
              AND pid <> pg_backend_pid()
            "#,
            self.db_name
        );
        let _ = conn.execute(&*query).await;
        
        // Drop the database
        conn.execute(&*format!(r#"DROP DATABASE "{}""#, self.db_name))
            .await
            .expect("Failed to drop test database");
    }
}

pub enum StorageType {
    InMemory,
    Postgres,
}

impl StorageType {
    pub fn from_env() -> Self {
        match std::env::var("TEST_STORAGE").as_deref() {
            Ok("postgres") => StorageType::Postgres,
            _ => StorageType::InMemory,
        }
    }
}