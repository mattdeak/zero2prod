use axum::routing::{get, post, Router};
use sea_orm::Database;
use types::PostgresUrl;
pub mod entity;
mod routes;
pub mod types;
use std::sync::Arc;

async fn greet(name: String) -> String {
    format!("Hello {}!", &name)
}

async fn health_check() {}

pub async fn get_router(db_url: &PostgresUrl) -> Result<Router, Box<dyn std::error::Error>> {
    let url: String = db_url.to_string();
    let db = Database::connect(url).await?;

    Ok(Router::new()
        .route("/healthz", get(health_check))
        .route("/:name", get(greet))
        .route("/subscriptions", post(&routes::subscriptions::subscribe))
        .with_state(Arc::new(db)))
}
