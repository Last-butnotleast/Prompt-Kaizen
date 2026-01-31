use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct Tag {
    id: Uuid,
    prompt_id: Uuid,
    version_id: Uuid,
    name: String,
    updated_at: DateTime<Utc>,
}

impl Tag {
    pub fn new(
        id: Uuid,
        prompt_id: Uuid,
        version_id: Uuid,
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

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn prompt_id(&self) -> Uuid {
        self.prompt_id
    }

    pub fn version_id(&self) -> Uuid {
        self.version_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub(crate) fn move_to_version(&mut self, new_version_id: Uuid) {
        self.version_id = new_version_id;
        self.updated_at = Utc::now();
    }
}