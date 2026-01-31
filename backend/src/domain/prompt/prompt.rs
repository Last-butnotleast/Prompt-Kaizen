use chrono::{DateTime, Utc};
use super::{PromptVersion, Tag};

#[derive(Debug, Clone)]
pub struct Prompt {
    id: String,
    name: String,
    description: Option<String>,
    created_at: DateTime<Utc>,
    updated_at: DateTime<Utc>,
    versions: Vec<PromptVersion>,
    tags: Vec<Tag>,
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
            versions: Vec::new(),
            tags: Vec::new(),
        }
    }

    pub fn id(&self) -> &str {
        &self.id
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

    pub fn tags(&self) -> &[Tag] {
        &self.tags
    }

    pub fn add_version(
        &mut self,
        version_id: String,
        version: String,
        content: String,
        changelog: Option<String>,
    ) -> Result<&PromptVersion, String> {
        if self.versions.iter().any(|v| v.version() == version) {
            return Err(format!("Version {} already exists", version));
        }

        let prompt_version = PromptVersion::new(
            version_id,
            self.id.clone(),
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

    pub fn find_version_by_id(&self, id: &str) -> Option<&PromptVersion> {
        self.versions.iter().find(|v| v.id() == id)
    }

    pub fn tag_version(
        &mut self,
        tag_id: String,
        tag_name: String,
        version_id: &str,
    ) -> Result<(), String> {
        if !self.versions.iter().any(|v| v.id() == version_id) {
            return Err("Version not found in this prompt".to_string());
        }

        if let Some(existing_tag) = self.tags.iter_mut().find(|t| t.name() == tag_name) {
            existing_tag.move_to_version(version_id.to_string());
        } else {
            let tag = Tag::new(tag_id, self.id.clone(), version_id.to_string(), tag_name);
            self.tags.push(tag);
        }

        self.updated_at = Utc::now();
        Ok(())
    }

    pub fn find_tag(&self, tag_name: &str) -> Option<&Tag> {
        self.tags.iter().find(|t| t.name() == tag_name)
    }
}