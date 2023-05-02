use reqwest;
use tokio;
use zero2prod;

const TEST_ADDRESS: &'static str = "127.0.0.1:8000";

#[tokio::test]
async fn health_check_works() {
    spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get("http://127.0.0.1:8000/healthz")
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}

fn spawn_app() {
    let router = zero2prod::get_router();

    let _ = tokio::spawn(
        axum::Server::bind(&TEST_ADDRESS.parse().unwrap()).serve(router.into_make_service()),
    );
}
