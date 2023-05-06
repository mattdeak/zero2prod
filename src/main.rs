use axum;
use dotenvy::dotenv;
use tokio;
use zero2prod::get_router;

fn load_postgres_url() -> zero2prod::types::PostgresUrl {
    dotenv().ok();
    zero2prod::types::PostgresUrl {
        host: std::env::var("POSTGRES_HOST").expect("POSTGRES_HOST must be set"),
        port: std::env::var("POSTGRES_PORT")
            .expect("POSTGRES_PORT must be set")
            .parse::<u16>()
            .expect("POSTGRES_PORT must be a number"),
        username: std::env::var("POSTGRES_USER").expect("POSTGRES_USER must be set"),
        password: std::env::var("POSTGRES_PASSWORD").expect("POSTGRES_PASSWORD must be set"),
        database: std::env::var("POSTGRES_DB").expect("POSTGRES_DB must be set"),
    }
}

#[tokio::main]
async fn main() {
    let url = load_postgres_url();
    let app = get_router(&url)
        .await
        .unwrap_or_else(|e| panic!("Failed to connect to Postgres: {}", e.to_string().as_str()));

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
