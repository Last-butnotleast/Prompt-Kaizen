use std::sync::Arc;
use crate::application::use_cases::*;
use crate::application::api_key_repository::ApiKeyRepository;

pub struct AppState {
    pub create_prompt: Arc<CreatePrompt>,
    pub update_prompt: Arc<UpdatePrompt>,
    pub get_prompt: Arc<GetPrompt>,
    pub list_prompts: Arc<ListPrompts>,
    pub delete_prompt: Arc<DeletePrompt>,

    pub create_version: Arc<CreateVersion>,
    pub get_version: Arc<GetVersion>,
    pub delete_version: Arc<DeleteVersion>,
    pub render_version: Arc<RenderVersion>,
    pub render_version_by_tag: Arc<RenderVersionByTag>,

    pub create_tag: Arc<CreateTag>,
    pub delete_tag: Arc<DeleteTag>,
    pub get_version_by_tag: Arc<GetVersionByTag>,

    pub submit_feedback: Arc<SubmitFeedback>,
    pub update_feedback: Arc<UpdateFeedback>,
    pub delete_feedback: Arc<DeleteFeedback>,

    pub create_api_key: Arc<CreateApiKey>,
    pub list_api_keys: Arc<ListApiKeys>,
    pub delete_api_key: Arc<DeleteApiKey>,

    pub api_key_repository: Arc<dyn ApiKeyRepository>,
}