use axum;
use dotenvy::dotenv;
use tokio;
use zero2prod::make_server;
use zero2prod::types::PostgresUrl;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let url = PostgresUrl::from_env()
        .unwrap_or_else(|e| panic!("Failed to parse Postgres URL: {}", e.to_string().as_str()));

    let app = make_server(&url)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to Postgres: {}", e.to_string().as_str()));

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
