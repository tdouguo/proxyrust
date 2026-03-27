# ProxyRust Master Plan

## 项目目标
- 构建一个以 `Rust 控制面 + sing-box 数据面` 为核心的高性能代理管理平台。
- 平台默认按分布式生产形态设计，支持 `PostgreSQL + Redis Streams + 多 Worker`。
- 系统既要覆盖来源采集、解析、探测、筛选、端口绑定、分享输出，也要具备“性能优先的平台化扩展能力”。

## 文档导航
- [Phase 0：基础骨架与插件宿主](./phases/phase-0-foundation-and-plugin-host.md)
- [Phase 1：来源中心与采集解析](./phases/phase-1-source-center-and-ingestion.md)
- [Phase 2：探测评分与规则引擎](./phases/phase-2-probing-scoring-and-rule-engine.md)
- [Phase 3：绑定、分享与运行时](./phases/phase-3-bindings-share-and-runtime.md)
- [Phase 4：分布式 Worker 与调度](./phases/phase-4-distributed-workers-and-scheduling.md)
- [Phase 5：运维能力、审计与产品化 UI](./phases/phase-5-ops-observability-and-productized-ui.md)

## 总体架构
- 控制面：Rust 服务，负责来源管理、节点归一化、规则决策、任务编排、Worker 调度、插件注册与管理 API。
- 数据面：sing-box 负责复杂协议的出站与本地 `http/socks5/mixed` 入站执行。
- Worker：负责任务消费、探测执行、sing-box 配置生成、runtime 热更新与状态回报。
- 管理端：前后端分离 Web 应用，负责配置、任务、Worker、绑定、插件和审计的运营界面。
- 存储与队列：生产默认 `PostgreSQL + Redis`，开发默认 `SQLite + 本地替代队列`。

## 性能优先约束
- 热路径逻辑必须内置实现，不允许请求级同步调用外部插件。
- 核心热路径包括：节点索引、SelectionProfile 执行、PortBinding 解析、Worker 调度主逻辑、runtime 配置生成、绑定命中时的节点选择。
- 插件只允许进入异步链路、边界链路或外部集成链路。
- 任何新增扩展点都必须标记为 `hot path`、`warm path` 或 `cold path`。

## 插件体系边界
- 第一版插件模型为“混合插件模型”。
- 默认扩展方式：进程外 `HTTP / gRPC / MQ` 插件。
- 内置扩展方式：平台内部高性能插件接口，用于 fetcher、parser、exporter、notifier 等内置能力的统一抽象。
- 预留 WASM 能力位，但第一期不依赖 WASM 完成主链路。
- 第一版不支持任意原生动态库热加载。

## Phase 依赖关系
- `Phase 0` 完成前，不进入任何业务功能开发。
- `Phase 1` 依赖 `Phase 0` 的工程骨架、插件宿主、基础 API 和前端外壳。
- `Phase 2` 依赖 `Phase 1` 的来源中心、节点库与任务入口。
- `Phase 3` 依赖 `Phase 2` 的评分、标签、规则与任务流水线。
- `Phase 4` 依赖 `Phase 3` 的单 Worker 运行时与绑定模型。
- `Phase 5` 在前四期链路稳定后进行统一运营能力收敛。

## 全局验收标准
- 文档落盘完成且相对路径可达。
- 核心链路可跑通：来源创建 -> 节点入库 -> 探测评分 -> 规则筛选 -> 端口绑定 -> 本地使用 -> 分享输出。
- 插件禁用后主链路仍可运行。
- 单 Worker 故障、单来源失败、单批任务失败不会导致控制面整体不可用。
- 管理端必须支持在线调整频率、并发、黑名单、风险阈值、Worker 参数与插件启停状态。

## 风险与回滚原则
- 先做最小可运行骨架，再逐期增加能力，不跨 Phase 大跳跃。
- 每期都保留明确的回滚边界：未通过该期验收标准，不进入下一期。
- 插件扩展如与性能目标冲突，优先回退为内置实现。
- 控制面和 Worker 的接口变更必须带版本兼容策略。

## 默认假设
- 第一版数据面固定为 sing-box。
- 第一版生产主库默认是 PostgreSQL，Redis Streams 为默认队列。
- SQLite 仅用于开发和轻量模式。
- 第一版不追求“任何功能都能插件化”，而是追求“核心性能稳定、平台边界清晰、扩展点真实可用”。

