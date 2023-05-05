use axum::routing::{get, post, Router};
use sea_orm::DatabaseConnection;
mod config;
pub mod entity;
mod routes;
use std::sync::Arc;

async fn greet(name: String) -> String {
    format!("Hello {}!", &name)
}

async fn health_check() {}

pub fn get_router() -> Router {
    Router::new()
        .route("/healthz", get(health_check))
        .route("/:name", get(greet))
        .route("/subscriptions", post(&routes::subscriptions::subscribe))
        .with_state(Arc::new(db))
}
