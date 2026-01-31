use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Tag {
    pub id: String,
    pub prompt_id: String,
    pub version_id: String,
    pub name: String,
    pub updated_at: DateTime<Utc>,
}

impl Tag {
    pub fn new(
        id: String,
        prompt_id: String,
        version_id: String,
        name: String,
    ) -> Self {
        Self {
            id,
            prompt_id,
            version_id,
            name,
            updated_at: Utc::now(),
        }
    }

    pub fn move_to_version(&mut self, new_version_id: String) {
        self.version_id = new_version_id;
        self.updated_at = Utc::now();
    }
}