mod config;

use axum::{
    body::Body,
    http::{Request, StatusCode},
    Router,
};
use rallybot_api::create_app_with_repository;
use fake::{faker::*, Fake};
use rallybot_core::{
    Gender, InMemoryStorage, LookingFor, PlayFrequency, PostgresStorage, PreferredSide, 
    Repository, SkillLevel, Storage, User, Venue,
};
use rand::{seq::SliceRandom, Rng};
use std::sync::Arc;
use tower::ServiceExt;
use uuid::Uuid;

pub use config::{StorageType, TestDatabase};

pub struct TestApp {
    pub app: Router,
    pub storage: Arc<dyn Storage>,
    pub test_db: Option<TestDatabase>,
}

impl TestApp {
    pub async fn new() -> Self {
        match StorageType::from_env() {
            StorageType::InMemory => Self::with_in_memory().await,
            StorageType::Postgres => Self::with_postgres().await,
        }
    }
    
    pub async fn with_in_memory() -> Self {
        let storage = Arc::new(InMemoryStorage::new());
        let repository = Arc::new(Repository::new(storage.clone()));
        let app = create_app_with_repository(repository);
        
        Self { 
            app, 
            storage: storage as Arc<dyn Storage>,
            test_db: None,
        }
    }
    
    pub async fn with_postgres() -> Self {
        let test_db = TestDatabase::new().await;
        let pool = test_db.get_pool().await;
        let storage = Arc::new(PostgresStorage::new_with_pool(pool));
        let repository = Arc::new(Repository::new(storage.clone()));
        let app = create_app_with_repository(repository);
        
        Self { 
            app, 
            storage: storage as Arc<dyn Storage>,
            test_db: Some(test_db),
        }
    }

    #[allow(dead_code)]
    pub async fn create_test_user(&self, phone: &str, approved: bool) -> Uuid {
        let mut rng = rand::thread_rng();
        
        let user = User::new(
            name::en::FirstName().fake(),
            name::en::LastName().fake(),
            phone.to_string(),
            internet::en::FreeEmail().fake(),
            address::en::CityName().fake(),
            company::en::Profession().fake(),
            company::en::CompanyName().fake(),
            company::en::Industry().fake(),
            format!("https://linkedin.com/in/{}", internet::en::Username().fake::<String>()),
            if rng.gen_bool(0.5) { Gender::Male } else { Gender::Female },
            vec![
                *[SkillLevel::Beginner, SkillLevel::LowIntermediate, SkillLevel::Intermediate, 
                  SkillLevel::UpperIntermediate, SkillLevel::Advanced]
                    .choose(&mut rng)
                    .unwrap()
            ],
            *[PreferredSide::Right, PreferredSide::Left, PreferredSide::Flexible]
                .choose(&mut rng)
                .unwrap(),
            *[PlayFrequency::OnceWeek, PlayFrequency::SeveralTimesWeek, PlayFrequency::FewTimesMonth]
                .choose(&mut rng)
                .unwrap(),
            vec![
                *[LookingFor::SocialConnections, LookingFor::BusinessOpportunities]
                    .choose(&mut rng)
                    .unwrap()
            ],
        );
        
        let mut user = user;
        user.is_approved = approved;
        
        self.storage.create_user(user.clone()).await;
        user.id
    }

    #[allow(dead_code)]
    pub async fn create_test_venue(&self) -> Uuid {
        self.create_test_venue_with_data(None, None).await
    }

    #[allow(dead_code)]
    pub async fn create_test_venue_with_data(&self, name: Option<&str>, address: Option<&str>) -> Uuid {
        let venue = Venue::new(
            name.map(|s| s.to_string())
                .unwrap_or_else(|| format!("{} Sports Center", company::en::CompanyName().fake::<String>())),
            address.map(|s| s.to_string())
                .unwrap_or_else(|| format!("{}, {}, {}", 
                    address::en::StreetName().fake::<String>(),
                    address::en::CityName().fake::<String>(),
                    address::en::CountryName().fake::<String>()
                )),
        );
        self.storage.create_venue(venue.clone()).await;
        venue.id
    }

    pub async fn call(&self, request: Request<Body>) -> (StatusCode, String) {
        let response = self.app.clone().oneshot(request).await.unwrap();
        let status = response.status();
        let body_bytes = axum::body::to_bytes(response.into_body(), usize::MAX)
            .await
            .unwrap();
        let body = String::from_utf8(body_bytes.to_vec()).unwrap();
        
        (status, body)
    }
}

impl TestApp {
    pub async fn cleanup(mut self) {
        if let Some(test_db) = self.test_db.take() {
            test_db.cleanup().await;
        }
    }
}