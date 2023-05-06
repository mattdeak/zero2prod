use tracing::debug;

#[tracing::instrument]
pub async fn health_check() {
    debug!("health check");
}
