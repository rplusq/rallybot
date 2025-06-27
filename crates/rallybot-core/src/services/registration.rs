use crate::{
    registration::{Registration, RegistrationStatus},
    repository::RegistrationError,
    storage::Storage,
};
use uuid::Uuid;

pub struct RegistrationService<S> {
    storage: S,
}

impl<S> RegistrationService<S> {
    pub fn new(storage: S) -> Self {
        Self { storage }
    }
}

impl<S: Storage> RegistrationService<S> {
    pub async fn register_user(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<RegistrationStatus, RegistrationError> {
        // Check session exists
        if self.storage.get_session(session_id).await.is_none() {
            return Err(RegistrationError::SessionNotFound);
        }

        // Check user exists and is approved
        let user = self
            .storage
            .get_user(user_id)
            .await
            .ok_or(RegistrationError::UserNotFound)?;

        if !user.is_approved {
            return Err(RegistrationError::UserNotApproved);
        }

        // Check if already registered
        if self.storage.registration_exists(session_id, user_id).await {
            return Err(RegistrationError::AlreadyRegistered);
        }

        // Count confirmed registrations
        let registrations = self.storage.get_registrations(session_id).await;
        let confirmed_count = registrations
            .iter()
            .filter(|r| r.status == RegistrationStatus::Confirmed)
            .count();

        // Determine status based on capacity
        let status = if confirmed_count < 4 {
            RegistrationStatus::Confirmed
        } else {
            RegistrationStatus::Substitute
        };

        // Create registration
        let registration = Registration::new(user_id, session_id, status);
        self.storage.create_registration(registration).await;

        Ok(status)
    }

    pub async fn get_session_registrations(&self, session_id: Uuid) -> Vec<Registration> {
        self.storage.get_registrations(session_id).await
    }

    pub async fn get_user_sessions(&self, user_id: Uuid) -> Vec<Uuid> {
        let registrations = self.storage.get_user_registrations(user_id).await;
        registrations.into_iter().map(|r| r.session_id).collect()
    }

    pub async fn unregister_user(
        &self,
        session_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), RegistrationError> {
        // Get all registrations for this session
        let registrations = self.storage.get_registrations(session_id).await;
        
        // Find the user's registration
        let user_registration = registrations
            .iter()
            .find(|r| r.user_id == user_id)
            .ok_or(RegistrationError::NotRegistered)?;
        
        let was_confirmed = user_registration.status == RegistrationStatus::Confirmed;
        
        // Delete the registration
        if !self.storage.delete_registration(session_id, user_id).await {
            return Err(RegistrationError::NotRegistered);
        }
        
        // If user was confirmed, promote the oldest substitute
        if was_confirmed {
            if let Some(first_substitute) = registrations
                .iter()
                .filter(|r| r.status == RegistrationStatus::Substitute)
                .min_by_key(|r| r.created_at)
            {
                let mut promoted = first_substitute.clone();
                promoted.status = RegistrationStatus::Confirmed;
                self.storage.update_registration(promoted).await;
            }
        }
        
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        models::{Session, SessionType, Venue},
        storage::InMemoryStorage,
        user::{Gender, SkillLevel, PreferredSide, PlayFrequency, LookingFor, User},
    };
    use std::sync::Arc;
    use chrono::Utc;
    use fake::{faker::*, Fake};
    use rand::{seq::SliceRandom, Rng};

    async fn create_test_storage() -> Arc<InMemoryStorage> {
        let storage = Arc::new(InMemoryStorage::new());
        
        // Create a test venue
        let venue = Venue::new(
            format!("{} Padel Club", company::en::CompanyName().fake::<String>()),
            format!("{}, {}", 
                address::en::StreetName().fake::<String>(),
                address::en::CityName().fake::<String>()
            ),
        );
        storage.create_venue(venue.clone()).await;
        
        // Create a test session
        let session = Session::new(
            SessionType::Social,
            Utc::now() + chrono::Duration::days(1),
            90,
            venue.id,
        ).expect("Valid session");
        storage.create_session(session).await;
        
        storage
    }

    async fn create_test_user(storage: &Arc<InMemoryStorage>, approved: bool) -> User {
        let mut rng = rand::thread_rng();
        
        let mut user = User::new(
            name::en::FirstName().fake(),
            name::en::LastName().fake(),
            format!("+351{}", phone_number::en::PhoneNumber().fake::<String>()),
            internet::en::FreeEmail().fake(),
            address::en::CityName().fake(),
            None, // photo_url
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
        user.is_approved = approved;
        storage.create_user(user.clone()).await;
        user
    }

    #[tokio::test]
    async fn register_user_success() {
        let storage = create_test_storage().await;
        let user = create_test_user(&storage, true).await;
        
        let service = RegistrationService::new(storage.clone());
        
        // Get the session we created
        let sessions = storage.list_sessions(None).await;
        let session = &sessions[0];
        
        let result = service.register_user(session.id, user.id).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RegistrationStatus::Confirmed);
    }

    #[tokio::test]
    async fn register_unapproved_user_fails() {
        let storage = create_test_storage().await;
        let user = create_test_user(&storage, false).await; // Not approved
        
        let service = RegistrationService::new(storage.clone());
        
        let sessions = storage.list_sessions(None).await;
        let session = &sessions[0];
        
        let result = service.register_user(session.id, user.id).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RegistrationError::UserNotApproved));
    }

    #[tokio::test]
    async fn fifth_registration_becomes_substitute() {
        let storage = create_test_storage().await;
        let service = RegistrationService::new(storage.clone());
        
        let sessions = storage.list_sessions(None).await;
        let session = &sessions[0];
        
        // Register 4 users (all should be confirmed)
        for _ in 0..4 {
            let user = create_test_user(&storage, true).await;
            let result = service.register_user(session.id, user.id).await;
            assert_eq!(result.unwrap(), RegistrationStatus::Confirmed);
        }
        
        // Register 5th user (should be substitute)
        let user5 = create_test_user(&storage, true).await;
        let result = service.register_user(session.id, user5.id).await;
        
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), RegistrationStatus::Substitute);
    }

    #[tokio::test]
    async fn double_registration_fails() {
        let storage = create_test_storage().await;
        let user = create_test_user(&storage, true).await;
        
        let service = RegistrationService::new(storage.clone());
        
        let sessions = storage.list_sessions(None).await;
        let session = &sessions[0];
        
        // First registration
        let result1 = service.register_user(session.id, user.id).await;
        assert!(result1.is_ok());
        
        // Second registration (should fail)
        let result2 = service.register_user(session.id, user.id).await;
        assert!(result2.is_err());
        assert!(matches!(result2.unwrap_err(), RegistrationError::AlreadyRegistered));
    }

    #[tokio::test]
    async fn register_for_nonexistent_session_fails() {
        let storage = create_test_storage().await;
        let user = create_test_user(&storage, true).await;
        
        let service = RegistrationService::new(storage.clone());
        
        let fake_session_id = Uuid::new_v4();
        let result = service.register_user(fake_session_id, user.id).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RegistrationError::SessionNotFound));
    }

    #[tokio::test]
    async fn register_nonexistent_user_fails() {
        let storage = create_test_storage().await;
        let service = RegistrationService::new(storage.clone());
        
        let sessions = storage.list_sessions(None).await;
        let session = &sessions[0];
        
        let fake_user_id = Uuid::new_v4();
        let result = service.register_user(session.id, fake_user_id).await;
        
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RegistrationError::UserNotFound));
    }
}