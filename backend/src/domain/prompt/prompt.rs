use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Prompt {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl Prompt {
    pub fn new(id: String, name: String, description: Option<String>) -> Self {
        let now = Utc::now();
        Self {
            id,
            name,
            description,
            created_at: now,
            updated_at: now,
        }
    }
}