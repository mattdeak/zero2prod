use dotenvy::dotenv;
use tracing::Level;
use zero2prod::make_router;
use zero2prod::types::PostgresUrl;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let subscriber = tracing_subscriber::fmt()
        .with_max_level(Level::DEBUG)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to set global subscriber");

    let url = PostgresUrl::from_env()
        .unwrap_or_else(|e| panic!("Failed to parse Postgres URL: {}", e.to_string().as_str()));

    let app = make_router(&url)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to Postgres: {}", e.to_string().as_str()));

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
