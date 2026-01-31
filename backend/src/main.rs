mod domain;
mod application;
mod infrastructure;
mod interface;

use std::sync::Arc;
use infrastructure::repositories::{InMemoryPromptRepository, PostgresPromptRepository};
use application::{use_cases::*, PromptRepository};
use interface::web::{create_router, handlers::AppState};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let repository: Arc<dyn PromptRepository> = if let Ok(database_url) = std::env::var("DATABASE_URL") {
        println!("🔗 Connecting to PostgreSQL...");
        let pool = sqlx::PgPool::connect(&database_url)
            .await
            .expect("Failed to connect to database");

        sqlx::migrate!("./migrations")
            .run(&pool)
            .await
            .expect("Failed to run migrations");

        println!("✅ PostgreSQL connected");
        Arc::new(PostgresPromptRepository::new(pool))
    } else {
        println!("📦 Using in-memory storage");
        Arc::new(InMemoryPromptRepository::new())
    };

    let create_prompt = Arc::new(CreatePrompt::new(repository.clone()));
    let update_prompt = Arc::new(UpdatePrompt::new(repository.clone()));
    let get_prompt = Arc::new(GetPrompt::new(repository.clone()));
    let list_prompts = Arc::new(ListPrompts::new(repository.clone()));
    let delete_prompt = Arc::new(DeletePrompt::new(repository.clone()));

    let create_version = Arc::new(CreateVersion::new(repository.clone()));
    let get_version = Arc::new(GetVersion::new(repository.clone()));
    let delete_version = Arc::new(DeleteVersion::new(repository.clone()));

    let create_tag = Arc::new(CreateTag::new(repository.clone()));
    let delete_tag = Arc::new(DeleteTag::new(repository.clone()));
    let get_version_by_tag = Arc::new(GetVersionByTag::new(repository.clone()));

    let submit_feedback = Arc::new(SubmitFeedback::new(repository.clone()));
    let update_feedback = Arc::new(UpdateFeedback::new(repository.clone()));
    let delete_feedback = Arc::new(DeleteFeedback::new(repository.clone()));

    let app_state = Arc::new(AppState {
        create_prompt,
        update_prompt,
        get_prompt,
        list_prompts,
        delete_prompt,
        create_version,
        get_version,
        delete_version,
        create_tag,
        delete_tag,
        get_version_by_tag,
        submit_feedback,
        update_feedback,
        delete_feedback,
    });

    let app = create_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Prompt Kaizen running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}