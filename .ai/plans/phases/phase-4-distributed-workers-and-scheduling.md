# Phase 4：分布式 Worker 与调度

## 阶段目标
- 将系统从单 Worker 提升为多 Worker 可调度、可恢复的集群形态。

## 本期范围
- 实现 Worker 注册、租约、能力标签、失联检测、任务回收。
- 实现 binding 调度、Worker 亲和性、运行时迁移和基础故障恢复。

## 非本期范围
- 不实现多租户和复杂组织权限。

## 核心模块与接口
- `WorkerRegistry`
- `LeaseManager`
- `Scheduler`
- `AssignmentService`
- `RecoveryCoordinator`

## 数据模型变化
- 新增 `WorkerLease`
- 新增 `WorkerCapability`
- 新增 `AssignmentDecision`
- 新增 `RecoveryEvent`

## 前后端改动点
- 后端新增调度与租约 API。
- 前端新增 Worker 管理页、调度状态页、恢复事件页。

## 插件扩展点
- `worker_extension` 插件用于给特定 Worker 增加额外能力。

## 多 Agent 拆分方式
- Agent A：Worker 注册表与租约。
- Agent B：调度器与恢复逻辑。
- Agent C：Worker 管理与运维 UI。
- Agent D：扩展能力声明与兼容性检查。

## 测试计划
- 多 Worker 注册测试
- 租约失效与重分配测试
- assignment 迁移测试
- 单 Worker 宕机恢复测试

## 阶段验收标准
- 多个 Worker 可同时注册并稳定维持租约。
- Worker 离线后任务可重新分配。
- binding 能迁移或重新下发到健康 Worker。
- 调度状态与异常恢复可在管理端观察。

## 风险与回滚方式
- 风险：调度抖动会影响运行时稳定性。
- 回滚：调度器异常时冻结重新分配，只保留已有 assignment。

