# Phase 1：来源中心与采集解析

## 阶段目标
- 实现来源管理、采集器、解析器与节点归一化的第一条业务主链。

## 本期范围
- 实现来源 CRUD、来源启停、手动刷新、调度刷新。
- 导入当前 `config.csv` 并映射为来源中心种子数据。
- 实现 `fetcher` / `parser` 插件接口与首批内置实现。
- 建立归一化节点模型与去重逻辑。

## 非本期范围
- 不实现风险值、规则引擎、绑定与分享。
- 不实现完整分布式任务调度。

## 核心模块与接口
- `SourceService`
- `FetcherPlugin`
- `ParserPlugin`
- `NodeNormalizer`
- `NodeDeduper`
- 来源 API、节点列表 API、导入任务 API

## 数据模型变化
- 新增 `Source`
- 新增 `NodeRecord`
- 新增 `SourceRun` / `ImportRun`

## 前后端改动点
- 后端新增来源中心与节点归一化流水线。
- 前端新增来源列表、来源表单、导入结果页、节点列表页。

## 插件扩展点
- `fetcher` 插件：支持 GitHub Raw、HTTP 文本、JSON、YAML 等来源。
- `parser` 插件：支持 URI 列表、Base64、Clash YAML、公开代理文本等。

## 多 Agent 拆分方式
- Agent A：来源模型、存储、导入器。
- Agent B：采集器插件、解析器插件。
- Agent C：来源管理前端与节点列表。
- Agent D：导入质量校验与文档同步。

## 测试计划
- 来源 CRUD 测试
- 导入 `config.csv` 测试
- 协议识别测试
- 去重测试
- 节点列表分页与筛选测试

## 阶段验收标准
- 可从 UI 或 API 创建来源并触发刷新。
- `config.csv` 可导入且保留来源追踪信息。
- 归一化节点至少覆盖 `vmess`、`vless`、`trojan`、`ss`、`hy2`、`socks5`、`http/https`。
- 同一节点不会因多来源重复落库。

## 风险与回滚方式
- 风险：来源格式高度异构，首批解析器容易边界不全。
- 回滚：对未识别来源降级为原始记录保留，不阻塞来源中心主体上线。

