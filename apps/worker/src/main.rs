use std::time::Duration;

use platform_core::{
    WorkerHeartbeatRequest, WorkerRegistrationRequest, WorkerRegistrationResponse, WorkerSummary,
};
use tracing::{error, info};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(std::env::var("RUST_LOG").unwrap_or_else(|_| "worker=info".to_string()))
        .init();

    let control_plane_url =
        std::env::var("PROXYRUST_CONTROL_PLANE_URL").unwrap_or_else(|_| "http://127.0.0.1:8080".to_string());
    let worker_token =
        std::env::var("PROXYRUST_WORKER_SHARED_TOKEN").unwrap_or_else(|_| "dev-worker-token".to_string());
    let worker_id = std::env::var("PROXYRUST_WORKER_ID").ok();

    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(5))
        .build()
        .expect("failed to build reqwest client");

    let registration = WorkerRegistrationRequest {
        worker_id,
        token: worker_token.clone(),
        labels: vec!["phase-0".to_string(), "runtime".to_string()],
        capabilities: vec!["heartbeat".to_string(), "plugin_read".to_string()],
        runtime_version: "phase-0-skeleton".to_string(),
    };

    let response = register_worker(&client, &control_plane_url, &registration).await;
    let mut registered = match response {
        Ok(data) => data,
        Err(error) => {
            error!("failed to register worker: {error}");
            return;
        }
    };

    info!("worker registered as {}", registered.worker_id);

    let heartbeat_every = (registered.lease_ttl_seconds / 2).max(5);
    let mut interval = tokio::time::interval(Duration::from_secs(heartbeat_every));

    loop {
        interval.tick().await;
        let heartbeat = WorkerHeartbeatRequest {
            worker_id: registered.worker_id.clone(),
            token: worker_token.clone(),
        };

        match send_heartbeat(&client, &control_plane_url, &heartbeat).await {
            Ok(summary) => info!("heartbeat ok for {} ({})", summary.worker_id, summary.status),
            Err(error) => {
                error!("heartbeat failed: {error}, attempting re-registration");

                let retry_payload = WorkerRegistrationRequest {
                    worker_id: Some(registered.worker_id.clone()),
                    token: worker_token.clone(),
                    labels: vec!["phase-0".to_string(), "runtime".to_string()],
                    capabilities: vec!["heartbeat".to_string(), "plugin_read".to_string()],
                    runtime_version: "phase-0-skeleton".to_string(),
                };

                match register_worker(&client, &control_plane_url, &retry_payload).await {
                    Ok(new_registration) => {
                        info!("worker re-registered as {}", new_registration.worker_id);
                        registered = new_registration;
                    }
                    Err(register_error) => {
                        error!("worker re-registration failed: {register_error}");
                    }
                }
            }
        }
    }
}

async fn register_worker(
    client: &reqwest::Client,
    control_plane_url: &str,
    payload: &WorkerRegistrationRequest,
) -> Result<WorkerRegistrationResponse, reqwest::Error> {
    client
        .post(format!("{control_plane_url}/api/v1/workers/register"))
        .json(payload)
        .send()
        .await?
        .error_for_status()?
        .json::<WorkerRegistrationResponse>()
        .await
}

async fn send_heartbeat(
    client: &reqwest::Client,
    control_plane_url: &str,
    payload: &WorkerHeartbeatRequest,
) -> Result<WorkerSummary, reqwest::Error> {
    client
        .post(format!("{control_plane_url}/api/v1/workers/heartbeat"))
        .json(payload)
        .send()
        .await?
        .error_for_status()?
        .json::<WorkerSummary>()
        .await
}
