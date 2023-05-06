mod health_check;
mod subscriptions;
use axum::routing::{get, post, Router};
use sea_orm::DatabaseConnection;
use std::sync::Arc;

pub async fn get_router(db_conn: DatabaseConnection) -> Result<Router, Box<dyn std::error::Error>> {
    Ok(Router::new()
        .route("/healthz", get(&health_check::health_check))
        .route("/subscriptions", post(&subscriptions::subscribe))
        .with_state(Arc::new(db_conn)))
}
