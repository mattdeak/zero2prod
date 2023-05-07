use axum::routing::Router;
use sea_orm::Database;
use types::PostgresUrl;

pub mod entity;
pub mod routes;
pub mod types;

pub async fn make_router(db_url: &PostgresUrl) -> Result<Router, Box<dyn std::error::Error>> {
    let url: String = db_url.to_string();
    let db = Database::connect(url).await?;

    Ok(routes::get_router(db).await?)
}
