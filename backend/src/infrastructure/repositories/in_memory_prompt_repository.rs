use crate::application::PromptRepository;
use crate::domain::prompt::Prompt;
use async_trait::async_trait;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub struct InMemoryPromptRepository {
    store: RwLock<HashMap<String, Prompt>>,
}

impl InMemoryPromptRepository {
    pub fn new() -> Self {
        Self {
            store: RwLock::new(HashMap::new()),
        }
    }
}

#[async_trait]
impl PromptRepository for InMemoryPromptRepository {
    async fn save(&self, prompt: &Prompt) -> Result<(), String> {
        let mut store = self.store.write().await;
        store.insert(prompt.id().to_string(), prompt.clone());
        Ok(())
    }

    async fn find_by_id(&self, id: &str) -> Result<Option<Prompt>, String> {
        let store = self.store.read().await;
        Ok(store.get(id).cloned())
    }

    async fn find_all(&self) -> Result<Vec<Prompt>, String> {
        let store = self.store.read().await;
        Ok(store.values().cloned().collect())
    }

    async fn find_by_tag(&self, tag_name: &str) -> Result<Vec<Prompt>, String> {
        let store = self.store.read().await;
        Ok(store
            .values()
            .filter(|p| p.tags().iter().any(|t| t.name() == tag_name))
            .cloned()
            .collect())
    }

    async fn delete(&self, id: &str) -> Result<(), String> {
        let mut store = self.store.write().await;
        store.remove(id)
            .ok_or_else(|| "Prompt not found".to_string())?;
        Ok(())
    }
}