use crate::application::PromptRepository;
use crate::domain::prompt::{Prompt, PromptVersion, Tag, Feedback, TestScenario, Version, PromptType, ContentType, ImprovementSuggestion, SuggestionStatus};
use async_trait::async_trait;
use sqlx::{PgPool, Row};
use uuid::Uuid;

pub struct PostgresPromptRepository {
    pool: PgPool,
}

impl PostgresPromptRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    async fn fetch_versions(&self, prompt_id: Uuid) -> Result<Vec<PromptVersion>, String> {
        let rows = sqlx::query(
            "SELECT id, prompt_id, version, digest, content, content_type, variables, changelog, created_at
             FROM versions WHERE prompt_id = $1 ORDER BY created_at"
        )
            .bind(prompt_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to fetch versions: {}", e))?;

        let mut versions = Vec::new();
        for row in rows {
            let version_id: Uuid = row.try_get("id").map_err(|e| e.to_string())?;
            let prompt_id: Uuid = row.try_get("prompt_id").map_err(|e| e.to_string())?;

            let feedbacks = self.fetch_feedbacks(version_id).await?;
            let suggestions = self.fetch_improvement_suggestions(version_id).await?;

            let version_string: String = row.try_get("version").map_err(|e| e.to_string())?;
            let version = Version::from_str(&version_string)?;

            let content_type_str: String = row.try_get("content_type").map_err(|e| e.to_string())?;
            let content_type = match content_type_str.as_str() {
                "static" => ContentType::Static,
                "template" => ContentType::Template,
                _ => return Err("Invalid content_type".to_string()),
            };

            let variables: Option<Vec<String>> = row.try_get::<Option<sqlx::types::Json<Vec<String>>>, _>("variables")
                .map_err(|e| e.to_string())?
                .map(|j| j.0);

            let mut version = PromptVersion::new(
                version_id,
                prompt_id,
                version,
                row.try_get("content").map_err(|e| e.to_string())?,
                content_type,
                variables,
                row.try_get("changelog").map_err(|e| e.to_string())?,
            );

            for feedback in feedbacks {
                let _ = version.add_feedback(
                    feedback.id(),
                    feedback.rating(),
                    feedback.comment().map(|s| s.to_string()),
                    feedback.test_scenario().cloned(),
                );
            }

            for suggestion in suggestions {
                version.improvement_suggestions_mut().push(suggestion);
            }

            versions.push(version);
        }

        Ok(versions)
    }

    async fn fetch_tags(&self, prompt_id: Uuid) -> Result<Vec<Tag>, String> {
        let rows = sqlx::query(
            "SELECT id, prompt_id, version_id, name, updated_at
             FROM tags WHERE prompt_id = $1"
        )
            .bind(prompt_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to fetch tags: {}", e))?;

        rows.iter()
            .map(|row| {
                let id: Uuid = row.try_get("id").map_err(|e| e.to_string())?;
                let prompt_id: Uuid = row.try_get("prompt_id").map_err(|e| e.to_string())?;
                let version_id: Uuid = row.try_get("version_id").map_err(|e| e.to_string())?;

                Ok(Tag::new(
                    id,
                    prompt_id,
                    version_id,
                    row.try_get("name").map_err(|e| e.to_string())?,
                ))
            })
            .collect()
    }

    async fn fetch_feedbacks(&self, version_id: Uuid) -> Result<Vec<Feedback>, String> {
        let rows = sqlx::query(
            "SELECT id, version_id, rating, comment, test_input, test_actual_output, test_expected_output, created_at
             FROM feedbacks WHERE version_id = $1 ORDER BY created_at"
        )
            .bind(version_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to fetch feedbacks: {}", e))?;

        rows.iter()
            .map(|row| {
                let id: Uuid = row.try_get("id").map_err(|e| e.to_string())?;
                let version_id: Uuid = row.try_get("version_id").map_err(|e| e.to_string())?;

                let test_scenario = match (
                    row.try_get::<Option<String>, _>("test_input").map_err(|e| e.to_string())?,
                    row.try_get::<Option<String>, _>("test_actual_output").map_err(|e| e.to_string())?,
                ) {
                    (Some(input), Some(actual_output)) => {
                        let expected_output = row.try_get("test_expected_output").map_err(|e| e.to_string())?;
                        Some(TestScenario::new(input, actual_output, expected_output)?)
                    }
                    _ => None,
                };

                Feedback::new(
                    id,
                    version_id,
                    row.try_get::<i16, _>("rating").map_err(|e| e.to_string())? as u8,
                    row.try_get("comment").map_err(|e| e.to_string())?,
                    test_scenario,
                )
            })
            .collect()
    }

    async fn fetch_improvement_suggestions(&self, version_id: Uuid) -> Result<Vec<ImprovementSuggestion>, String> {
        let rows = sqlx::query(
            "SELECT id, source_version_id, suggested_content, ai_rationale, status, decline_reason, created_at, resolved_at, resulting_version_id
         FROM improvement_suggestions WHERE source_version_id = $1 ORDER BY created_at"
        )
            .bind(version_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to fetch improvement suggestions: {}", e))?;

        rows.iter()
            .map(|row| {
                let id: Uuid = row.try_get("id").map_err(|e| e.to_string())?;
                let source_version_id: Uuid = row.try_get("source_version_id").map_err(|e| e.to_string())?;
                let suggested_content: String = row.try_get("suggested_content").map_err(|e| e.to_string())?;
                let ai_rationale: String = row.try_get("ai_rationale").map_err(|e| e.to_string())?;

                let status_str: String = row.try_get("status").map_err(|e| e.to_string())?;
                let status = match status_str.as_str() {
                    "pending" => SuggestionStatus::Pending,
                    "accepted" => SuggestionStatus::Accepted,
                    "declined" => SuggestionStatus::Declined,
                    _ => return Err("Invalid suggestion status".to_string()),
                };

                let mut suggestion = ImprovementSuggestion::new(
                    id,
                    source_version_id,
                    suggested_content,
                    ai_rationale,
                );

                if status == SuggestionStatus::Accepted {
                    let resulting_version_id: Option<Uuid> = row.try_get("resulting_version_id")
                        .map_err(|e| e.to_string())?;
                    let resulting_version_id = resulting_version_id
                        .ok_or("Accepted suggestion missing resulting_version_id")?;
                    suggestion.accept(resulting_version_id)?;
                } else if status == SuggestionStatus::Declined {
                    let reason: Option<String> = row.try_get("decline_reason")
                        .map_err(|e| e.to_string())?;
                    let reason = reason.ok_or("Declined suggestion missing reason")?;
                    suggestion.decline(reason)?;
                }

                Ok(suggestion)
            })
            .collect()
    }

    async fn save_versions(&self, prompt_id: Uuid, versions: &[PromptVersion]) -> Result<(), String> {
        sqlx::query("DELETE FROM versions WHERE prompt_id = $1")
            .bind(prompt_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete versions: {}", e))?;

        for version in versions {
            let content_type_str = match version.content_type() {
                ContentType::Static => "static",
                ContentType::Template => "template",
            };

            let variables_json = version.variables().map(|v| sqlx::types::Json(v.to_vec()));

            sqlx::query(
                "INSERT INTO versions (id, prompt_id, version, digest, content, content_type, variables, changelog, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
            )
                .bind(version.id())
                .bind(version.prompt_id())
                .bind(version.version_string())
                .bind(version.digest())
                .bind(version.content())
                .bind(content_type_str)
                .bind(variables_json)
                .bind(version.changelog())
                .bind(version.created_at())
                .execute(&self.pool)
                .await
                .map_err(|e| format!("Failed to save version: {}", e))?;

            self.save_feedbacks(version.id(), version.feedbacks()).await?;
            self.save_improvement_suggestions(version.id(), version.improvement_suggestions()).await?;
        }
        Ok(())
    }

    async fn save_tags(&self, prompt_id: Uuid, tags: &[Tag]) -> Result<(), String> {
        sqlx::query("DELETE FROM tags WHERE prompt_id = $1")
            .bind(prompt_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete tags: {}", e))?;

        for tag in tags {
            sqlx::query(
                "INSERT INTO tags (id, prompt_id, version_id, name, updated_at)
             VALUES ($1, $2, $3, $4, $5)"
            )
                .bind(tag.id())
                .bind(tag.prompt_id())
                .bind(tag.version_id())
                .bind(tag.name())
                .bind(tag.updated_at())
                .execute(&self.pool)
                .await
                .map_err(|e| format!("Failed to save tag: {}", e))?;
        }
        Ok(())
    }

    async fn save_feedbacks(&self, version_id: Uuid, feedbacks: &[Feedback]) -> Result<(), String> {
        sqlx::query("DELETE FROM feedbacks WHERE version_id = $1")
            .bind(version_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete feedbacks: {}", e))?;

        for feedback in feedbacks {
            let (test_input, test_actual_output, test_expected_output) =
                if let Some(scenario) = feedback.test_scenario() {
                    (
                        Some(scenario.input()),
                        Some(scenario.actual_output()),
                        scenario.expected_output(),
                    )
                } else {
                    (None, None, None)
                };

            sqlx::query(
                "INSERT INTO feedbacks (id, version_id, rating, comment, test_input, test_actual_output, test_expected_output, created_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)"
            )
                .bind(feedback.id())
                .bind(version_id)
                .bind(feedback.rating() as i16)
                .bind(feedback.comment())
                .bind(test_input)
                .bind(test_actual_output)
                .bind(test_expected_output)
                .bind(feedback.created_at())
                .execute(&self.pool)
                .await
                .map_err(|e| format!("Failed to save feedback: {}", e))?;
        }
        Ok(())
    }

    async fn save_improvement_suggestions(&self, version_id: Uuid, suggestions: &[ImprovementSuggestion]) -> Result<(), String> {
        sqlx::query("DELETE FROM improvement_suggestions WHERE source_version_id = $1")
            .bind(version_id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete improvement suggestions: {}", e))?;

        for suggestion in suggestions {
            let status_str = match suggestion.status() {
                SuggestionStatus::Pending => "pending",
                SuggestionStatus::Accepted => "accepted",
                SuggestionStatus::Declined => "declined",
            };

            sqlx::query(
                "INSERT INTO improvement_suggestions (id, source_version_id, suggested_content, ai_rationale, status, decline_reason, created_at, resolved_at, resulting_version_id)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)"
            )
                .bind(suggestion.id())
                .bind(suggestion.source_version_id())
                .bind(suggestion.suggested_content())
                .bind(suggestion.ai_rationale())
                .bind(status_str)
                .bind(suggestion.decline_reason())
                .bind(suggestion.created_at())
                .bind(suggestion.resolved_at())
                .bind(suggestion.resulting_version_id())
                .execute(&self.pool)
                .await
                .map_err(|e| format!("Failed to save improvement suggestion: {}", e))?;
        }
        Ok(())
    }

    async fn build_prompt(&self, row: &sqlx::postgres::PgRow) -> Result<Prompt, String> {
        let prompt_id: Uuid = row.try_get("id").map_err(|e| e.to_string())?;
        let user_id: Uuid = row.try_get("user_id").map_err(|e| e.to_string())?;

        let prompt_type_str: String = row.try_get("prompt_type").map_err(|e| e.to_string())?;
        let prompt_type = match prompt_type_str.as_str() {
            "system" => PromptType::System,
            "user" => PromptType::User,
            _ => return Err("Invalid prompt_type".to_string()),
        };

        let versions = self.fetch_versions(prompt_id).await?;
        let tags = self.fetch_tags(prompt_id).await?;

        let mut prompt = Prompt::new(
            prompt_id,
            user_id,
            row.try_get("name").map_err(|e| e.to_string())?,
            row.try_get("description").map_err(|e| e.to_string())?,
            prompt_type,
        );

        for version in versions {
            prompt.versions_mut().push(version);
        }

        for tag in tags {
            prompt.tags_mut().push(tag);
        }

        Ok(prompt)
    }
}

#[async_trait]
impl PromptRepository for PostgresPromptRepository {
    async fn save(&self, prompt: &Prompt) -> Result<(), String> {
        let prompt_type_str = match prompt.prompt_type() {
            PromptType::System => "system",
            PromptType::User => "user",
        };

        sqlx::query(
            "INSERT INTO prompts (id, user_id, name, description, prompt_type, created_at, updated_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7)
         ON CONFLICT (id) DO UPDATE SET
         name = EXCLUDED.name,
         description = EXCLUDED.description,
         prompt_type = EXCLUDED.prompt_type,
         updated_at = EXCLUDED.updated_at"
        )
            .bind(prompt.id())
            .bind(prompt.user_id())
            .bind(prompt.name())
            .bind(prompt.description())
            .bind(prompt_type_str)
            .bind(prompt.created_at())
            .bind(prompt.updated_at())
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to save prompt: {}", e))?;

        self.save_versions(prompt.id(), prompt.versions()).await?;
        self.save_tags(prompt.id(), prompt.tags()).await?;

        Ok(())
    }

    async fn find_by_id(&self, id: Uuid) -> Result<Option<Prompt>, String> {
        let row = sqlx::query(
            "SELECT id, user_id, name, description, prompt_type, created_at, updated_at
             FROM prompts WHERE id = $1"
        )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Failed to find prompt: {}", e))?;

        match row {
            Some(row) => Ok(Some(self.build_prompt(&row).await?)),
            None => Ok(None),
        }
    }

    async fn find_by_id_and_user(&self, id: Uuid, user_id: Uuid) -> Result<Option<Prompt>, String> {
        let row = sqlx::query(
            "SELECT id, user_id, name, description, prompt_type, created_at, updated_at
             FROM prompts WHERE id = $1 AND user_id = $2"
        )
            .bind(id)
            .bind(user_id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|e| format!("Failed to find prompt: {}", e))?;

        match row {
            Some(row) => Ok(Some(self.build_prompt(&row).await?)),
            None => Ok(None),
        }
    }

    async fn find_all(&self) -> Result<Vec<Prompt>, String> {
        let rows = sqlx::query(
            "SELECT id, user_id, name, description, prompt_type, created_at, updated_at
             FROM prompts ORDER BY created_at DESC"
        )
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to fetch prompts: {}", e))?;

        let mut prompts = Vec::new();
        for row in rows {
            prompts.push(self.build_prompt(&row).await?);
        }

        Ok(prompts)
    }

    async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<Prompt>, String> {
        let rows = sqlx::query(
            "SELECT id, user_id, name, description, prompt_type, created_at, updated_at
             FROM prompts WHERE user_id = $1 ORDER BY created_at DESC"
        )
            .bind(user_id)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to fetch prompts: {}", e))?;

        let mut prompts = Vec::new();
        for row in rows {
            prompts.push(self.build_prompt(&row).await?);
        }

        Ok(prompts)
    }

    async fn find_by_tag(&self, tag_name: &str) -> Result<Vec<Prompt>, String> {
        let rows = sqlx::query(
            "SELECT DISTINCT p.id, p.user_id, p.name, p.description, p.prompt_type, p.created_at, p.updated_at
             FROM prompts p
             INNER JOIN tags t ON p.id = t.prompt_id
             WHERE t.name = $1
             ORDER BY p.created_at DESC"
        )
            .bind(tag_name)
            .fetch_all(&self.pool)
            .await
            .map_err(|e| format!("Failed to fetch prompts by tag: {}", e))?;

        let mut prompts = Vec::new();
        for row in rows {
            prompts.push(self.build_prompt(&row).await?);
        }

        Ok(prompts)
    }

    async fn delete(&self, id: Uuid) -> Result<(), String> {
        let result = sqlx::query("DELETE FROM prompts WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|e| format!("Failed to delete prompt: {}", e))?;

        if result.rows_affected() == 0 {
            return Err("Prompt not found".to_string());
        }

        Ok(())
    }
}