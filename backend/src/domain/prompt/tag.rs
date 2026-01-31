use chrono::{DateTime, Utc};

#[derive(Debug, Clone)]
pub struct Tag {
    id: String,
    prompt_id: String,
    version_id: String,
    name: String,
    updated_at: DateTime<Utc>,
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

    pub fn id(&self) -> &str {
        &self.id
    }

    pub fn prompt_id(&self) -> &str {
        &self.prompt_id
    }

    pub fn version_id(&self) -> &str {
        &self.version_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub(crate) fn move_to_version(&mut self, new_version_id: String) {
        self.version_id = new_version_id;
        self.updated_at = Utc::now();
    }
}