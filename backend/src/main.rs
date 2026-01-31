mod domain;
mod application;
mod infrastructure;
mod interface;

use std::sync::Arc;
use infrastructure::repositories::InMemoryPromptRepository;
use application::use_cases::{
    CreatePromptUseCase, CreateVersionUseCase, ManageTagsUseCase, SubmitFeedbackUseCase,
};
use interface::web::{create_router, handlers::prompt_handlers::AppState};

#[tokio::main]
async fn main() {
    let repository = Arc::new(InMemoryPromptRepository::new());

    let create_prompt = Arc::new(CreatePromptUseCase::new(repository.clone()));
    let create_version = Arc::new(CreateVersionUseCase::new(repository.clone()));
    let manage_tags = Arc::new(ManageTagsUseCase::new(repository.clone()));
    let submit_feedback = Arc::new(SubmitFeedbackUseCase::new(repository.clone()));

    let app_state = Arc::new(AppState {
        create_prompt,
        create_version,
        manage_tags,
        submit_feedback,
    });

    let app = create_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Prompt Kaizen running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}