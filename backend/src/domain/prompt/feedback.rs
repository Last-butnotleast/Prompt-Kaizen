use chrono::{DateTime, Utc};
use uuid::Uuid;
use super::TestScenario;

#[derive(Debug, Clone)]
pub struct Feedback {
    id: Uuid,
    version_id: Uuid,
    rating: u8,
    comment: Option<String>,
    test_scenario: Option<TestScenario>,
    created_at: DateTime<Utc>,
}

impl Feedback {
    pub fn new(
        id: Uuid,
        version_id: Uuid,
        rating: u8,
        comment: Option<String>,
        test_scenario: Option<TestScenario>,
    ) -> Result<Self, String> {
        if !(1..=5).contains(&rating) {
            return Err("Rating must be between 1 and 5".to_string());
        }

        Ok(Self {
            id,
            version_id,
            rating,
            comment,
            test_scenario,
            created_at: Utc::now(),
        })
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn version_id(&self) -> Uuid {
        self.version_id
    }

    pub fn rating(&self) -> u8 {
        self.rating
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    pub fn test_scenario(&self) -> Option<&TestScenario> {
        self.test_scenario.as_ref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn update_rating(&mut self, rating: u8) -> Result<(), String> {
        if !(1..=5).contains(&rating) {
            return Err("Rating must be between 1 and 5".to_string());
        }
        self.rating = rating;
        Ok(())
    }

    pub fn update_comment(&mut self, comment: Option<String>) {
        self.comment = comment;
    }

    pub fn update_test_scenario(&mut self, test_scenario: Option<TestScenario>) {
        self.test_scenario = test_scenario;
    }
}