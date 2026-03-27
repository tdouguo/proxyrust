use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PluginKind {
    Fetcher,
    Parser,
    ProbeProvider,
    RiskProvider,
    Exporter,
    Notifier,
    WorkerExtension,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum PluginExecutionMode {
    BuiltIn,
    ExternalHttp,
    ExternalGrpc,
    ExternalMq,
    ReservedWasm,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct PluginManifest {
    pub plugin_id: String,
    pub name: String,
    pub version: String,
    pub kind: PluginKind,
    pub execution_mode: PluginExecutionMode,
    pub hot_path_allowed: bool,
    pub description: String,
    pub capabilities: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SystemInfo {
    pub project: String,
    pub phase: String,
    pub architecture: String,
    pub plugin_strategy: String,
    pub production_baseline: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkerRegistrationRequest {
    pub worker_id: Option<String>,
    pub token: String,
    pub labels: Vec<String>,
    pub capabilities: Vec<String>,
    pub runtime_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkerRegistrationResponse {
    pub worker_id: String,
    pub lease_ttl_seconds: u64,
    pub accepted: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkerHeartbeatRequest {
    pub worker_id: String,
    pub token: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct WorkerSummary {
    pub worker_id: String,
    pub labels: Vec<String>,
    pub capabilities: Vec<String>,
    pub runtime_version: String,
    pub last_seen_unix_ms: u64,
    pub lease_expires_unix_ms: u64,
    pub status: String,
}

pub fn now_unix_ms() -> u64 {
    use std::time::{SystemTime, UNIX_EPOCH};

    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_millis() as u64)
        .unwrap_or_default()
}

