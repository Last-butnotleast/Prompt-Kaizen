use chrono::{DateTime, Utc};
use uuid::Uuid;
use super::SuggestionStatus;

#[derive(Debug, Clone)]
pub struct ImprovementSuggestion {
    id: Uuid,
    source_version_id: Uuid,
    suggested_content: String,
    ai_rationale: String,
    status: SuggestionStatus,
    decline_reason: Option<String>,
    created_at: DateTime<Utc>,
    resolved_at: Option<DateTime<Utc>>,
    resulting_version_id: Option<Uuid>,
}

impl ImprovementSuggestion {
    pub fn new(
        id: Uuid,
        source_version_id: Uuid,
        suggested_content: String,
        ai_rationale: String,
    ) -> Self {
        Self {
            id,
            source_version_id,
            suggested_content,
            ai_rationale,
            status: SuggestionStatus::Pending,
            decline_reason: None,
            created_at: Utc::now(),
            resolved_at: None,
            resulting_version_id: None,
        }
    }

    pub fn id(&self) -> Uuid {
        self.id
    }

    pub fn source_version_id(&self) -> Uuid {
        self.source_version_id
    }

    pub fn suggested_content(&self) -> &str {
        &self.suggested_content
    }

    pub fn ai_rationale(&self) -> &str {
        &self.ai_rationale
    }

    pub fn status(&self) -> SuggestionStatus {
        self.status
    }

    pub fn decline_reason(&self) -> Option<&str> {
        self.decline_reason.as_deref()
    }

    pub fn created_at(&self) -> DateTime<Utc> {
        self.created_at
    }

    pub fn resolved_at(&self) -> Option<DateTime<Utc>> {
        self.resolved_at
    }

    pub fn resulting_version_id(&self) -> Option<Uuid> {
        self.resulting_version_id
    }

    pub fn accept(&mut self, resulting_version_id: Uuid) -> Result<(), String> {
        if self.status != SuggestionStatus::Pending {
            return Err("Can only accept pending suggestions".to_string());
        }
        self.status = SuggestionStatus::Accepted;
        self.resulting_version_id = Some(resulting_version_id);
        self.resolved_at = Some(Utc::now());
        Ok(())
    }

    pub fn decline(&mut self, reason: String) -> Result<(), String> {
        if self.status != SuggestionStatus::Pending {
            return Err("Can only decline pending suggestions".to_string());
        }
        self.status = SuggestionStatus::Declined;
        self.decline_reason = Some(reason);
        self.resolved_at = Some(Utc::now());
        Ok(())
    }
}