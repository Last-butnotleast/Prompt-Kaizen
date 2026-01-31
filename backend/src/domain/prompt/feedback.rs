use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Feedback {
    id: String,
    version_id: String,
    rating: u8,
    comment: Option<String>,
    created_at: DateTime<Utc>,
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

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn version_id(&self) -> &str {
        &self.version_id
    }

    pub fn rating(&self) -> u8 {
        self.rating
    }

    pub fn comment(&self) -> Option<&str> {
        self.comment.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }
}