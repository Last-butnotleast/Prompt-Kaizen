mod domain;
mod application;
mod infrastructure;
mod interface;

use std::sync::Arc;
use infrastructure::repositories::{PostgresPromptRepository, PostgresApiKeyRepository};
use application::use_cases::*;
use interface::web::{create_router, handlers::AppState};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");

    println!("🔗 Connecting to Supabase PostgreSQL...");
    let pool = sqlx::PgPool::connect(&database_url)
        .await
        .expect("Failed to connect to database");

    println!("🔄 Running migrations...");
    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("Failed to run migrations");

    println!("✅ Database connected and migrated");

    // Repositories
    let prompt_repository = Arc::new(PostgresPromptRepository::new(pool.clone()));
    let api_key_repository = Arc::new(PostgresApiKeyRepository::new(pool.clone()));

    // Prompt use cases
    let create_prompt = Arc::new(CreatePrompt::new(prompt_repository.clone()));
    let update_prompt = Arc::new(UpdatePrompt::new(prompt_repository.clone()));
    let get_prompt = Arc::new(GetPrompt::new(prompt_repository.clone()));
    let list_prompts = Arc::new(ListPrompts::new(prompt_repository.clone()));
    let delete_prompt = Arc::new(DeletePrompt::new(prompt_repository.clone()));

    // Version use cases
    let create_version = Arc::new(CreateVersion::new(prompt_repository.clone()));
    let get_version = Arc::new(GetVersion::new(prompt_repository.clone()));
    let delete_version = Arc::new(DeleteVersion::new(prompt_repository.clone()));

    // Tag use cases
    let create_tag = Arc::new(CreateTag::new(prompt_repository.clone()));
    let delete_tag = Arc::new(DeleteTag::new(prompt_repository.clone()));
    let get_version_by_tag = Arc::new(GetVersionByTag::new(prompt_repository.clone()));

    // Feedback use cases
    let submit_feedback = Arc::new(SubmitFeedback::new(prompt_repository.clone()));
    let update_feedback = Arc::new(UpdateFeedback::new(prompt_repository.clone()));
    let delete_feedback = Arc::new(DeleteFeedback::new(prompt_repository.clone()));

    // API Key use cases
    let create_api_key = Arc::new(CreateApiKey::new(api_key_repository.clone()));
    let list_api_keys = Arc::new(ListApiKeys::new(api_key_repository.clone()));
    let delete_api_key = Arc::new(DeleteApiKey::new(api_key_repository.clone()));

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
        create_api_key,
        list_api_keys,
        delete_api_key,
        api_key_repository,
    });

    let app = create_router(app_state);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("🚀 Prompt Kaizen running on http://localhost:3000");

    axum::serve(listener, app).await.unwrap();
}
