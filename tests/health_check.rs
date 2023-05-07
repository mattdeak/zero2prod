use reqwest;
use tokio;
mod utils;

#[tokio::test]
async fn health_check_works() {
    let url = utils::spawn_app();

    let client = reqwest::Client::new();

    let response = client
        .get(format!("{}/healthz", url))
        .send()
        .await
        .expect("Failed to execute request.");

    assert!(response.status().is_success());
    assert_eq!(Some(0), response.content_length());
}
