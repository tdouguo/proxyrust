# Phase 0：基础骨架与插件宿主

## 阶段目标
- 搭建 Rust workspace、控制面、Worker、前端外壳和插件宿主基础能力。
- 建立后续多 Agent 可共享读取的计划文档和工程骨架。

## 本期范围
- 创建 `apps/control-plane`、`apps/worker`、`apps/web`、`crates/platform-core`、`crates/plugin-host`。
- 实现控制面健康检查、系统信息、插件清单、Worker 注册与心跳。
- 创建前端基础页，展示阶段、架构约束和内置插件类型。
- 确立插件清单模型、内置插件注册表和基础工程约束。

## 非本期范围
- 不实现真实来源抓取、解析、去重、探测、评分、绑定、分享和分布式调度。
- 不实现真实 Redis 队列、真实数据库迁移和 sing-box runtime 热更新。

## 核心模块与接口
- `crates/platform-core`：共享模型、控制面与 Worker 的基础协议对象。
- `crates/plugin-host`：插件清单、内置插件注册表。
- `apps/control-plane`：`/healthz`、`/api/v1/system/info`、`/api/v1/plugins`、`/api/v1/workers/register`、`/api/v1/workers/heartbeat`、`/api/v1/workers`。
- `apps/worker`：启动时注册、定时心跳、基础日志。
- `apps/web`：显示系统骨架与规划总览。

## 数据模型变化
- 新增 `PluginManifest`、`PluginKind`、`PluginExecutionMode`。
- 新增 `SystemInfo`。
- 新增 `WorkerRegistrationRequest`、`WorkerRegistrationResponse`、`WorkerHeartbeatRequest`、`WorkerSummary`。

## 前后端改动点
- 后端新增基础 API 与内存态 Worker 注册表。
- 前端新增 React + Vite + TypeScript 外壳页面。

## 插件扩展点
- 定义 `fetcher`、`parser`、`probe_provider`、`risk_provider`、`exporter`、`notifier`、`worker_extension` 七类插件位。
- 第一阶段仅实现插件元数据注册，不执行真实插件任务。

## 多 Agent 拆分方式
- Agent A：`platform-core` 与控制面接口模型。
- Agent B：Worker 启动、注册、心跳链路。
- Agent C：前端外壳与状态展示。
- Agent D：插件清单与计划文档结构审查。

## 测试计划
- 验证 `.ai/plans` 文档和相对路径存在。
- 验证前端 `npm run build` 可以通过。
- 待 Rust 工具链可用后，验证 workspace 编译、控制面启动、Worker 注册与心跳。

## 阶段验收标准
- 总计划与各 Phase 文件全部存在且结构完整。
- 控制面代码中存在插件清单和 Worker 注册心跳链路。
- 前端骨架可以构建并展示当前阶段与内置插件类型。
- 插件清单能够区分 `built-in` 与 `external` 执行模式。

## 风险与回滚方式
- 风险：当前环境缺失 Rust 工具链，可能无法完成编译验证。
- 回滚：仅保留文档和工程骨架，不进入业务实现。

