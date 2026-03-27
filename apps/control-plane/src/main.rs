use std::{collections::HashMap, net::SocketAddr, sync::Arc};

use axum::{
    extract::State,
    http::StatusCode,
    routing::{get, post},
    Json, Router,
};
use platform_core::{
    now_unix_ms, PluginManifest, SystemInfo, WorkerHeartbeatRequest, WorkerRegistrationRequest,
    WorkerRegistrationResponse, WorkerSummary,
};
use tokio::sync::RwLock;
use tracing::info;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    started_at_unix_ms: u64,
    worker_shared_token: String,
    plugins: Arc<Vec<PluginManifest>>,
    workers: Arc<RwLock<HashMap<String, WorkerLease>>>,
}

#[derive(Debug, Clone)]
struct WorkerLease {
    worker_id: String,
    labels: Vec<String>,
    capabilities: Vec<String>,
    runtime_version: String,
    last_seen_unix_ms: u64,
    lease_expires_unix_ms: u64,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_env_filter(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "control_plane=info".to_string()),
        )
        .init();

    let worker_shared_token =
        std::env::var("PROXYRUST_WORKER_SHARED_TOKEN").unwrap_or_else(|_| "dev-worker-token".to_string());
    let bind = std::env::var("PROXYRUST_CONTROL_PLANE_ADDR")
        .unwrap_or_else(|_| "127.0.0.1:8080".to_string());

    let state = AppState {
        started_at_unix_ms: now_unix_ms(),
        worker_shared_token,
        plugins: Arc::new(plugin_host::built_in_plugins()),
        workers: Arc::new(RwLock::new(HashMap::new())),
    };

    let app = Router::new()
        .route("/", get(root))
        .route("/healthz", get(healthz))
        .route("/api/v1/system/info", get(system_info))
        .route("/api/v1/plugins", get(list_plugins))
        .route("/api/v1/workers", get(list_workers))
        .route("/api/v1/workers/register", post(register_worker))
        .route("/api/v1/workers/heartbeat", post(worker_heartbeat))
        .with_state(state);

    let address: SocketAddr = bind.parse().expect("invalid PROXYRUST_CONTROL_PLANE_ADDR");
    info!("control-plane listening on {}", address);

    let listener = tokio::net::TcpListener::bind(address)
        .await
        .expect("failed to bind control-plane");

    axum::serve(listener, app)
        .await
        .expect("control-plane server error");
}

async fn root() -> &'static str {
    "ProxyRust control-plane is running"
}

async fn healthz() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "ok",
        "service": "control-plane",
    }))
}

async fn system_info(State(state): State<AppState>) -> Json<SystemInfo> {
    let uptime_ms = now_unix_ms().saturating_sub(state.started_at_unix_ms);
    Json(SystemInfo {
        project: "ProxyRust".to_string(),
        phase: format!("phase-0-bootstrap (uptime={}ms)", uptime_ms),
        architecture: "Rust control-plane + sing-box data-plane".to_string(),
        plugin_strategy: "performance-first mixed plugin model".to_string(),
        production_baseline: vec![
            "PostgreSQL".to_string(),
            "Redis".to_string(),
            "Redis Streams".to_string(),
            "Multiple workers".to_string(),
        ],
    })
}

async fn list_plugins(State(state): State<AppState>) -> Json<Vec<PluginManifest>> {
    Json(state.plugins.as_ref().clone())
}

async fn register_worker(
    State(state): State<AppState>,
    Json(payload): Json<WorkerRegistrationRequest>,
) -> Result<Json<WorkerRegistrationResponse>, StatusCode> {
    if payload.token != state.worker_shared_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let lease_ttl_seconds = 30;
    let worker_id = payload
        .worker_id
        .unwrap_or_else(|| format!("worker-{}", Uuid::new_v4()));
    let now = now_unix_ms();

    let lease = WorkerLease {
        worker_id: worker_id.clone(),
        labels: payload.labels,
        capabilities: payload.capabilities,
        runtime_version: payload.runtime_version,
        last_seen_unix_ms: now,
        lease_expires_unix_ms: now + lease_ttl_seconds * 1_000,
    };

    state.workers.write().await.insert(worker_id.clone(), lease);
    info!("worker registered: {}", worker_id);

    Ok(Json(WorkerRegistrationResponse {
        worker_id,
        lease_ttl_seconds,
        accepted: true,
    }))
}

async fn worker_heartbeat(
    State(state): State<AppState>,
    Json(payload): Json<WorkerHeartbeatRequest>,
) -> Result<Json<WorkerSummary>, StatusCode> {
    if payload.token != state.worker_shared_token {
        return Err(StatusCode::UNAUTHORIZED);
    }

    let mut workers = state.workers.write().await;
    let Some(existing) = workers.get_mut(&payload.worker_id) else {
        return Err(StatusCode::NOT_FOUND);
    };

    let now = now_unix_ms();
    existing.last_seen_unix_ms = now;
    existing.lease_expires_unix_ms = now + 30_000;

    Ok(Json(to_summary(existing)))
}

async fn list_workers(State(state): State<AppState>) -> Json<Vec<WorkerSummary>> {
    let workers = state.workers.read().await;
    Json(workers.values().map(to_summary).collect())
}

fn to_summary(lease: &WorkerLease) -> WorkerSummary {
    let now = now_unix_ms();
    let status = if lease.lease_expires_unix_ms >= now {
        "online"
    } else {
        "expired"
    };

    WorkerSummary {
        worker_id: lease.worker_id.clone(),
        labels: lease.labels.clone(),
        capabilities: lease.capabilities.clone(),
        runtime_version: lease.runtime_version.clone(),
        last_seen_unix_ms: lease.last_seen_unix_ms,
        lease_expires_unix_ms: lease.lease_expires_unix_ms,
        status: status.to_string(),
    }
}

