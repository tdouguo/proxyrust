# Phase 3：绑定、分享与运行时

## 阶段目标
- 将规则结果转化为可用端口、分享输出和单 Worker runtime。

## 本期范围
- 实现 `SelectionProfile + PortBinding` 双层模型。
- 支持 `http`、`socks5`、`mixed` 三类入站。
- 实现分享 token、使用列表与客户端输出。
- 实现单 Worker 下发 sing-box 期望状态与最小热更新能力。

## 非本期范围
- 不实现多 Worker 分布式调度与复杂故障迁移。

## 核心模块与接口
- `BindingService`
- `RuntimeSpecBuilder`
- `ShareTokenService`
- `ClientExportService`

## 数据模型变化
- 新增 `PortBinding`
- 新增 `RuntimeAssignment`
- 新增 `ShareToken`
- 新增 `UsageView`

## 前后端改动点
- 后端新增绑定 API、分享 API、导出 API。
- 前端新增绑定管理页、分享页、客户端输出页、端口状态页。

## 插件扩展点
- `exporter` 插件负责不同客户端与输出格式。

## 多 Agent 拆分方式
- Agent A：Binding 与 runtime spec 模型。
- Agent B：Worker runtime 下发与状态回报。
- Agent C：绑定、分享和客户端导出界面。
- Agent D：导出插件和绑定验收检查。

## 测试计划
- 绑定创建测试
- profile 命中测试
- runtime spec 生成测试
- 分享 token 访问测试
- 客户端导出测试

## 阶段验收标准
- 创建绑定后，Worker 能获得期望状态。
- 端口命中的节点必须符合 SelectionProfile 条件。
- 分享链接与客户端输出能返回正确结果。
- 绑定变更不需要重启控制面。

## 风险与回滚方式
- 风险：runtime 生成与热更新失败会影响端口可用性。
- 回滚：保留上一个稳定 runtime spec，失败时回退到最近一次成功版本。

