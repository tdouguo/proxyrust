use platform_core::{PluginExecutionMode, PluginKind, PluginManifest};

pub fn built_in_plugins() -> Vec<PluginManifest> {
    vec![
        PluginManifest {
            plugin_id: "builtin.fetcher.http_source".to_string(),
            name: "HTTP Source Fetcher".to_string(),
            version: "0.1.0".to_string(),
            kind: PluginKind::Fetcher,
            execution_mode: PluginExecutionMode::BuiltIn,
            hot_path_allowed: false,
            description: "抓取 HTTP / GitHub Raw / 公开订阅源内容。".to_string(),
            capabilities: vec!["http_get".to_string(), "github_raw".to_string()],
        },
        PluginManifest {
            plugin_id: "builtin.parser.uri_bundle".to_string(),
            name: "URI Bundle Parser".to_string(),
            version: "0.1.0".to_string(),
            kind: PluginKind::Parser,
            execution_mode: PluginExecutionMode::BuiltIn,
            hot_path_allowed: false,
            description: "解析 URI 列表、Base64 文本与 Clash 节点集合。".to_string(),
            capabilities: vec!["uri".to_string(), "base64".to_string(), "clash_yaml".to_string()],
        },
        PluginManifest {
            plugin_id: "builtin.exporter.plain_text".to_string(),
            name: "Plain Text Exporter".to_string(),
            version: "0.1.0".to_string(),
            kind: PluginKind::Exporter,
            execution_mode: PluginExecutionMode::BuiltIn,
            hot_path_allowed: false,
            description: "导出基础文本与通用列表格式。".to_string(),
            capabilities: vec!["plain_text".to_string(), "uri_bundle".to_string()],
        },
        PluginManifest {
            plugin_id: "builtin.notifier.webhook".to_string(),
            name: "Webhook Notifier".to_string(),
            version: "0.1.0".to_string(),
            kind: PluginKind::Notifier,
            execution_mode: PluginExecutionMode::BuiltIn,
            hot_path_allowed: false,
            description: "向外部 Webhook 发送告警或任务通知。".to_string(),
            capabilities: vec!["webhook".to_string(), "task_event".to_string()],
        },
    ]
}

