const phases = [
  {
    name: 'Phase 0',
    status: '进行中',
    summary: '基础骨架、插件宿主、控制面与 Worker 最小链路'
  },
  {
    name: 'Phase 1',
    status: '待开始',
    summary: '来源中心、采集器、解析器与节点归一化'
  },
  {
    name: 'Phase 2',
    status: '待开始',
    summary: '探测评分、风险值、标签与规则引擎'
  },
  {
    name: 'Phase 3',
    status: '待开始',
    summary: '绑定、分享与单 Worker runtime'
  },
  {
    name: 'Phase 4',
    status: '待开始',
    summary: '分布式 Worker、调度、租约与恢复'
  },
  {
    name: 'Phase 5',
    status: '待开始',
    summary: '审计、可观测性与产品化 UI'
  }
];

const pluginTypes = [
  'fetcher',
  'parser',
  'probe_provider',
  'risk_provider',
  'exporter',
  'notifier',
  'worker_extension'
];

export default function App() {
  return (
    <main className="min-h-screen bg-[radial-gradient(circle_at_top,_rgba(56,189,248,0.18),_transparent_35%),linear-gradient(180deg,#020617_0%,#0f172a_45%,#111827_100%)] text-slate-100">
      <section className="mx-auto max-w-6xl px-6 py-16">
        <div className="mb-12 flex flex-col gap-5">
          <span className="w-fit rounded-full border border-cyan-400/30 bg-cyan-500/10 px-4 py-1 text-xs font-semibold uppercase tracking-[0.3em] text-cyan-200">
            ProxyRust / Phase 0 Bootstrap
          </span>
          <h1 className="max-w-4xl text-4xl font-black leading-tight text-white md:text-6xl">
            高性能代理控制面骨架已经落盘，下一步按分期计划逐段实施。
          </h1>
          <p className="max-w-3xl text-sm leading-7 text-slate-300 md:text-base">
            当前页面用于承接 Phase 0 的前端外壳。它展示项目阶段、插件类型和性能优先约束，作为后续运营台的起点。
          </p>
        </div>

        <div className="grid gap-5 md:grid-cols-3">
          <InfoCard
            title="性能优先"
            value="Hot Path 内置"
            description="节点筛选、端口命中、Worker 调度与 runtime 生成全部内置实现，不走外部插件同步调用。"
          />
          <InfoCard
            title="平台化边界"
            value="Cold / Warm Path 插件化"
            description="fetcher、parser、exporter、notifier 等能力通过统一插件契约扩展。"
          />
          <InfoCard
            title="生产基线"
            value="PostgreSQL + Redis Streams"
            description="默认分布式优先，开发环境允许 SQLite + 本地替代队列。"
          />
        </div>

        <section className="mt-12 grid gap-5 lg:grid-cols-[1.4fr_1fr]">
          <article className="rounded-3xl border border-white/10 bg-white/5 p-6 shadow-2xl shadow-slate-950/30 backdrop-blur">
            <div className="mb-6 flex items-center justify-between">
              <div>
                <h2 className="text-2xl font-bold text-white">阶段路线</h2>
                <p className="mt-2 text-sm text-slate-400">总 plan 与分阶段文档已经定义为共享只读输入。</p>
              </div>
              <span className="rounded-full border border-emerald-400/30 bg-emerald-500/10 px-3 py-1 text-xs font-semibold text-emerald-200">
                docs-first
              </span>
            </div>
            <div className="space-y-4">
              {phases.map((phase) => (
                <div
                  key={phase.name}
                  className="rounded-2xl border border-white/10 bg-slate-950/40 p-4 transition hover:border-cyan-400/30 hover:bg-slate-900/60"
                >
                  <div className="flex items-center justify-between gap-3">
                    <h3 className="text-lg font-semibold text-white">{phase.name}</h3>
                    <span className="rounded-full bg-slate-800 px-3 py-1 text-xs text-slate-200">{phase.status}</span>
                  </div>
                  <p className="mt-2 text-sm leading-6 text-slate-300">{phase.summary}</p>
                </div>
              ))}
            </div>
          </article>

          <aside className="rounded-3xl border border-white/10 bg-white/5 p-6 shadow-2xl shadow-slate-950/30 backdrop-blur">
            <h2 className="text-2xl font-bold text-white">插件类型</h2>
            <p className="mt-2 text-sm leading-6 text-slate-400">
              第一版采用性能优先的平台化：实时热路径内置，异步能力和边界能力通过插件扩展。
            </p>
            <div className="mt-5 flex flex-wrap gap-3">
              {pluginTypes.map((item) => (
                <span
                  key={item}
                  className="rounded-full border border-cyan-400/20 bg-cyan-500/10 px-3 py-1 text-xs font-medium tracking-wide text-cyan-100"
                >
                  {item}
                </span>
              ))}
            </div>
            <div className="mt-8 rounded-2xl border border-amber-300/20 bg-amber-500/10 p-4 text-sm leading-6 text-amber-50">
              当前环境里尚未检测到 Rust 工具链，因此本轮完成的是工程骨架与计划落盘；Rust 编译验证将在工具链可用后补上。
            </div>
          </aside>
        </section>
      </section>
    </main>
  );
}

type InfoCardProps = {
  title: string;
  value: string;
  description: string;
};

function InfoCard({ title, value, description }: InfoCardProps) {
  return (
    <article className="rounded-3xl border border-white/10 bg-white/5 p-5 shadow-xl shadow-slate-950/20 backdrop-blur">
      <p className="text-xs font-semibold uppercase tracking-[0.25em] text-slate-400">{title}</p>
      <h2 className="mt-3 text-2xl font-bold text-white">{value}</h2>
      <p className="mt-3 text-sm leading-6 text-slate-300">{description}</p>
    </article>
  );
}

