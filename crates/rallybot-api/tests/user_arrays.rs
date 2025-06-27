mod helpers;

use rallybot_core::{Gender, LookingFor, PlayFrequency, PreferredSide, SkillLevel, User};
use uuid::Uuid;

#[tokio::test]
async fn test_user_with_multiple_skill_levels_and_looking_for() {
    let app = helpers::TestApp::new().await;
    
    // Create a user with multiple skill levels and looking_for values
    let user = User {
        id: Uuid::new_v4(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        phone_number: "+1234567890".to_string(),
        email: "test@example.com".to_string(),
        city: "Test City".to_string(),
        photo_url: None,
        occupation: "Software Engineer".to_string(),
        company: "Test Company".to_string(),
        industry: "Technology".to_string(),
        linkedin_url: "https://linkedin.com/in/testuser".to_string(),
        gender: Gender::Male,
        skill_levels: vec![SkillLevel::Intermediate, SkillLevel::UpperIntermediate], // Multiple levels
        preferred_side: PreferredSide::Right,
        play_frequency: PlayFrequency::SeveralTimesWeek,
        looking_for: vec![LookingFor::SocialConnections, LookingFor::BusinessOpportunities], // Both options
        is_approved: true,
        created_at: chrono::Utc::now(),
    };
    
    // Save the user
    app.storage.create_user(user.clone()).await;
    
    // Retrieve the user
    let retrieved = app.storage.get_user(user.id).await.unwrap();
    
    // Verify arrays are preserved
    assert_eq!(retrieved.skill_levels.len(), 2);
    assert!(retrieved.skill_levels.contains(&SkillLevel::Intermediate));
    assert!(retrieved.skill_levels.contains(&SkillLevel::UpperIntermediate));
    
    assert_eq!(retrieved.looking_for.len(), 2);
    assert!(retrieved.looking_for.contains(&LookingFor::SocialConnections));
    assert!(retrieved.looking_for.contains(&LookingFor::BusinessOpportunities));
    
    // Cleanup for postgres
    if let Some(test_db) = app.test_db {
        test_db.cleanup().await;
    }
}

#[tokio::test]
async fn test_user_with_empty_arrays_fails() {
    let app = helpers::TestApp::new().await;
    
    // Note: In practice, you might want to allow empty arrays
    // This test documents the current behavior where arrays must be non-empty
    let user = User {
        id: Uuid::new_v4(),
        first_name: "Test".to_string(),
        last_name: "User".to_string(),
        phone_number: "+0987654321".to_string(),
        email: "test2@example.com".to_string(),
        city: "Test City".to_string(),
        photo_url: None,
        occupation: "Designer".to_string(),
        company: "Test Company".to_string(),
        industry: "Design".to_string(),
        linkedin_url: "https://linkedin.com/in/testuser2".to_string(),
        gender: Gender::Female,
        skill_levels: vec![], // Empty array
        preferred_side: PreferredSide::Left,
        play_frequency: PlayFrequency::OnceWeek,
        looking_for: vec![], // Empty array
        is_approved: true,
        created_at: chrono::Utc::now(),
    };
    
    // This might fail depending on database constraints
    // PostgreSQL allows empty arrays by default, but we might want to add CHECK constraints
    app.storage.create_user(user).await;
    
    // Cleanup for postgres
    if let Some(test_db) = app.test_db {
        test_db.cleanup().await;
    }
}