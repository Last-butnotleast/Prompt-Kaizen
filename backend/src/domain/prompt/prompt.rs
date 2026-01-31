use chrono::{DateTime, Utc};
use uuid::Uuid;
use super::{PromptVersion, Tag};

#[derive(Debug, Clone)]
pub struct Prompt {
    id: Uuid,
    user_id: Uuid,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    versions: Vec<PromptVersion>,
    tags: Vec<Tag>,
}

impl Prompt {
    pub fn new(
        id: Uuid,
        user_id: Uuid,
        name: String,
        description: Option<String>,
    ) -> Self {
        let now = Utc::now();
        Self {
            id,
            user_id,
            name,
            description,
            created_at: now,
            updated_at: now,
            versions: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn user_id(&self) -> Uuid {
        self.user_id
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn description(&self) -> Option<&str> {
        self.description.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn updated_at(&self) -> DateTime<Utc> {
        self.updated_at
    }

    pub fn versions(&self) -> &[PromptVersion] {
        &self.versions
    }

    pub fn versions_mut(&mut self) -> &mut Vec<PromptVersion> {
        &mut self.versions
    }

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn tags_mut(&mut self) -> &mut Vec<Tag> {
        &mut self.tags
    }

    pub fn add_version(
        &mut self,
        version_id: Uuid,
        version: String,
        content: String,
        changelog: Option<String>,
    ) -> Result<&PromptVersion, String> {
        if self.versions.iter().any(|v| v.version() == version) {
            return Err(format!("Version {} already exists", version));
        }

        let prompt_version = PromptVersion::new(
            version_id,
            self.id,
            version,
            content,
            changelog,
        );

        self.versions.push(prompt_version);
        self.updated_at = Utc::now();

        Ok(self.versions.last().unwrap())
    }

    pub fn find_version(&self, version: &str) -> Option<&PromptVersion> {
        self.versions.iter().find(|v| v.version() == version)
    }

    pub fn find_version_by_id(&self, id: Uuid) -> Option<&PromptVersion> {
        self.versions.iter().find(|v| v.id() == id)
    }

    pub fn tag_version(
        &mut self,
        tag_id: Uuid,
        tag_name: String,
        version_id: Uuid,
    ) -> Result<(), String> {
        if !self.versions.iter().any(|v| v.id() == version_id) {
            return Err("Version not found in this prompt".to_string());
        }

        if let Some(existing_tag) = self.tags.iter_mut().find(|t| t.name() == tag_name) {
            existing_tag.move_to_version(version_id);
        } else {
            let tag = Tag::new(tag_id, self.id, version_id, tag_name);
            self.tags.push(tag);
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn find_tag(&self, tag_name: &str) -> Option<&Tag> {
        self.tags.iter().find(|t| t.name() == tag_name)
    }

    // Update operations
    pub fn update_name(&mut self, name: String) -> Result<(), String> {
        if name.trim().is_empty() {
            return Err("Name cannot be empty".to_string());
        }
        self.name = name;
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn update_description(&mut self, description: Option<String>) {
        self.description = description;
        self.updated_at = Utc::now();
    }

    // Delete operations
    pub fn delete_version(&mut self, version_id: Uuid) -> Result<(), String> {
        let initial_len = self.versions.len();
        self.versions.retain(|v| v.id() != version_id);

        if self.versions.len() == initial_len {
            return Err("Version not found".to_string());
        }

        // Remove tags pointing to this version
        self.tags.retain(|t| t.version_id() != version_id);
        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn remove_tag(&mut self, tag_name: &str) -> Result<(), String> {
        let initial_len = self.tags.len();
        self.tags.retain(|t| t.name() != tag_name);

        if self.tags.len() == initial_len {
            return Err("Tag not found".to_string());
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    // Access feedback through aggregate
    pub fn update_feedback(
        &mut self,
        version_id: Uuid,
        feedback_id: Uuid,
        rating: Option<u8>,
        comment: Option<Option<String>>,
    ) -> Result<(), String> {
        let version = self.versions.iter_mut()
            .find(|v| v.id() == version_id)
            .ok_or("Version not found")?;

        version.update_feedback(feedback_id, rating, comment)
    }

    pub fn delete_feedback(&mut self, version_id: Uuid, feedback_id: Uuid) -> Result<(), String> {
        let version = self.versions.iter_mut()
            .find(|v| v.id() == version_id)
            .ok_or("Version not found")?;

        version.delete_feedback(feedback_id)
    }
}