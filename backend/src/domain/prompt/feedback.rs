use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Feedback {
    pub id: String,
    pub version_id: String,
    pub rating: u8,
    pub comment: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl Feedback {
    pub fn new(
        id: String,
        version_id: String,
        rating: u8,
        comment: Option<String>,
    ) -> Result<Self, String> {
        if !(1..=5).contains(&rating) {
            return Err("Rating must be between 1 and 5".to_string());
        }

        Ok(Self {
            id,
            version_id,
            rating,
            comment,
            created_at: Utc::now(),
        })
    }
}