mod health_check;
mod subscriptions;
use axum::routing::{get, post, Router};
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

pub async fn get_router(db_conn: DatabaseConnection) -> Result<Router, Box<dyn std::error::Error>> {
    Ok(Router::new()
        .route("/healthz", get(health_check::health_check))
        .route("/subscriptions", post(subscriptions::subscribe))
        .with_state(Arc::new(db_conn))
        .layer(TraceLayer::new_for_http()))
}
