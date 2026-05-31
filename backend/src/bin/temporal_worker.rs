use wara_backend::{
    libs::{config::Config, telemetry},
    services::queues::QueueName,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let queue = std::env::args()
        .nth(1)
        .map(|value| value.parse::<QueueName>())
        .transpose()
        .map_err(anyhow::Error::msg)?
        .unwrap_or(QueueName::Default);
    let config = Config::from_env();
    let guard = telemetry::init(&config)?;
    tracing::info!(
        task_queue = %queue.as_task_queue(),
        temporal_address = %config.temporal_address,
        namespace = %config.temporal_namespace,
        "Wara Temporal worker scaffold started"
    );
    telemetry::shutdown(guard)?;
    Ok(())
}
