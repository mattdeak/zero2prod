use axum;
use tokio;
use zero2prod::get_router;

#[tokio::main]
async fn main() {
    let app = get_router();

    axum::Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
