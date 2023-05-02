use axum::routing::{get, Router};

async fn greet(name: String) -> String {
    format!("Hello {}!", &name)
}

async fn health_check() {}

pub fn get_router() -> Router {
    Router::new()
        .route("/healthz", get(health_check))
        .route("/:name", get(greet))
}
