import { For, type JSX, type ParentProps, Show } from "solid-js"
import { Button, Divider, Switch, TextField } from "~/lib/ui"

// Static replica of OpenCode's console workspace page (packages/console/app/src/routes/workspace/[id],
// removed upstream in b9ca4d21b), rebuilt from lib/ui components. Visual test only: buttons are
// disabled because no registry commands exist for them, and the model switches hold no state.

const features = [
  {
    title: "Tested & Verified Models",
    body: "We've benchmarked and tested models specifically for coding agents to ensure the best performance.",
  },
  {
    title: "Highest Quality",
    body: "Access models configured for optimal performance - no downgrades or routing to cheaper providers.",
  },
  {
    title: "No Lock-in",
    body: "Use Zen with any coding agent, and continue using other providers with opencode whenever you want.",
  },
]

const models = [
  { name: "Claude Sonnet 4.5", lab: "Anthropic", enabled: true },
  { name: "Claude Haiku 4.5", lab: "Anthropic", enabled: true },
  { name: "GPT 5", lab: "OpenAI", enabled: true },
  { name: "Gemini 3 Pro", lab: "Google", enabled: true },
  { name: "Kimi K2", lab: "Moonshot AI", enabled: false },
  { name: "GLM 4.6", lab: "Z.ai", enabled: true },
  { name: "Qwen3 Coder", lab: "Alibaba", enabled: false },
  { name: "Grok Code Fast 1", lab: "xAI", enabled: true },
]

const providers = [
  { name: "OpenAI", key: "sk-proj-...a81fQ2xT" },
  { name: "Anthropic", key: null },
  { name: "Google Gemini", key: null },
]

function SectionTitle(props: { title: string; children: JSX.Element }) {
  return (
    <div class="flex flex-col gap-1">
      <h2 class="text-lg font-medium text-fg">{props.title}</h2>
      <p class="text-fg-muted">{props.children}</p>
    </div>
  )
}

function Th(props: ParentProps<{ class?: string }>) {
  return (
    <th class={`px-4 py-3 text-left text-sm font-medium uppercase text-fg-muted ${props.class ?? ""}`}>
      {props.children}
    </th>
  )
}

function Td(props: ParentProps<{ class?: string }>) {
  return <td class={`px-4 py-3 font-mono text-fg-muted ${props.class ?? ""}`}>{props.children}</td>
}

function Code(props: ParentProps) {
  return <code class="rounded-md border border-line px-1.5 py-0.5 font-mono text-sm text-fg">{props.children}</code>
}

export default function Overview() {
  return (
    <div class="flex max-w-5xl flex-col gap-10 px-12 py-8">
      <section class="flex flex-col gap-2">
        <h1 class="text-xl font-medium uppercase text-fg">Zen</h1>
        <div class="flex items-center justify-between gap-4 text-fg">
          <span>
            Reliable optimized models for coding agents. <span class="text-fg-muted underline">Learn more</span>.
          </span>
          <span class="shrink-0 text-fg-muted">
            Current balance <b class="font-medium text-fg">$0.00</b>
          </span>
        </div>
      </section>

      <Divider />

      <div class="flex flex-col gap-8 rounded-md border border-dashed border-line-strong bg-surface p-6">
        <div class="grid grid-cols-[repeat(auto-fit,minmax(14rem,1fr))] gap-6">
          <For each={features}>
            {(f) => (
              <div class="flex flex-col gap-2 rounded-md border border-line p-4">
                <h3 class="font-medium uppercase text-fg">{f.title}</h3>
                <p class="text-fg-muted">{f.body}</p>
              </div>
            )}
          </For>
        </div>

        <div class="flex items-center gap-3">
          <TextField
            class="min-w-0 flex-1"
            size="lg"
            aria-label="API key"
            value="sk-zen-2f***********************9c1e"
            readOnly
          />
          <Button size="lg" icon="copy" disabled>
            Copy Key
          </Button>
        </div>

        <ol class="flex list-inside list-decimal flex-col gap-2 text-fg">
          <li>Enable billing</li>
          <li>
            Run <Code>opencode auth login</Code> and select opencode
          </li>
          <li>Paste your API key</li>
          <li>
            Start opencode and run <Code>/models</Code> to select a model
          </li>
        </ol>
      </div>

      <section class="flex flex-col gap-8">
        <SectionTitle title="Models">
          Manage which models workspace members can access. <span class="underline">Learn more</span>.
        </SectionTitle>
        <table class="w-full border-collapse">
          <thead class="border-b border-line-strong">
            <tr>
              <Th>Model</Th>
              <Th />
              <Th class="w-24">Enabled</Th>
            </tr>
          </thead>
          <tbody>
            <For each={models}>
              {(m) => (
                <tr class="border-b border-line last:border-b-0">
                  <Td class={m.enabled ? "text-fg" : ""}>{m.name}</Td>
                  <Td>{m.lab}</Td>
                  <Td>
                    <Switch hideLabel defaultChecked={m.enabled}>
                      {m.name}
                    </Switch>
                  </Td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </section>

      <Divider />

      <section class="flex flex-col gap-8">
        <SectionTitle title="Bring Your Own Key">Configure your own API keys from AI providers.</SectionTitle>
        <table class="w-full border-collapse">
          <thead class="border-b border-line-strong">
            <tr>
              <Th class="w-44">Provider</Th>
              <Th>API Key</Th>
              <Th class="w-48" />
            </tr>
          </thead>
          <tbody>
            <For each={providers}>
              {(p) => (
                <tr class="border-b border-line last:border-b-0">
                  <Td class="text-fg">{p.name}</Td>
                  <Td>{p.key ?? "-"}</Td>
                  <Td>
                    <div class="flex gap-1">
                      <Show
                        when={p.key}
                        fallback={
                          <Button variant="ghost" size="sm" disabled>
                            Configure
                          </Button>
                        }
                      >
                        <Button variant="ghost" size="sm" disabled>
                          Edit
                        </Button>
                        <Button variant="ghost" size="sm" disabled>
                          Delete
                        </Button>
                      </Show>
                    </div>
                  </Td>
                </tr>
              )}
            </For>
          </tbody>
        </table>
      </section>
    </div>
  )
}
