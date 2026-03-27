# Phase 2：探测评分与规则引擎

## 阶段目标
- 建立异步探测流水线、质量评分、风险值、标签和规则筛选能力。

## 本期范围
- 实现延迟、速度、地区、风险、解锁等任务模型。
- 实现标签系统、自动标签规则、SelectionProfile。
- 支持在线调整探测并发、频率、超时、阈值。

## 非本期范围
- 不实现端口绑定与 sing-box runtime 实际下发。
- 不实现分布式 Worker 调度策略。

## 核心模块与接口
- `ProbeService`
- `RiskScoringService`
- `TagRuleService`
- `SelectionProfileService`
- `TaskCenter`

## 数据模型变化
- 新增 `NodeProbeResult`
- 新增 `RiskScore`
- 新增 `Tag` / `TagRule`
- 新增 `SelectionProfile`
- 新增 `Task`

## 前后端改动点
- 后端新增任务中心、评分与规则引擎 API。
- 前端新增任务页、标签页、规则配置页、SelectionProfile 预览页。

## 插件扩展点
- `probe_provider`
- `risk_provider`
- `selector_rule`

## 多 Agent 拆分方式
- Agent A：探测结果与评分模型。
- Agent B：Worker 任务执行器。
- Agent C：规则配置与任务中心 UI。
- Agent D：风险提供方与规则插件接口审查。

## 测试计划
- 探测任务入队与消费测试
- 风险值计算测试
- 标签规则命中测试
- SelectionProfile 预览测试
- 在线调参测试

## 阶段验收标准
- 节点探测结果能够入库并回显。
- 风险值和标签可以基于任务结果自动更新。
- SelectionProfile 预览结果稳定、可追溯。
- 探测并发和频率可在线更新并即时影响新任务。

## 风险与回滚方式
- 风险：评分口径不稳定会导致后续绑定命中结果波动。
- 回滚：保留原始探测字段，风险值只作为附加字段，不直接影响核心筛选。
