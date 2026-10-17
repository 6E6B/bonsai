import { For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { dateTime, full } from "~/lib/format"
import { type ActivityView, commands, type Id, type PluginBudgetView, type PluginView } from "~/lib/ipc/bindings"
import { byTier, pluginStates, tiers } from "~/lib/plugins"
import { shell, useLive } from "~/lib/store"
import {
  Alert,
  Badge,
  Button,
  type Column,
  DataTable,
  EmptyState,
  SettingRow,
  SettingSwitch,
  SettingsGroup,
} from "~/lib/ui"
import { PrefControl } from "./prefs"

/** "every 5 min", "every 2 h", "every 90 s". */
export function every(seconds: number): string {
  if (seconds % 3600 === 0) return `every ${seconds / 3600} h`
  if (seconds % 60 === 0) return `every ${seconds / 60} min`
  return `every ${seconds} s`
}

/** Installed plugins: on or off, which accounts each may use, automation, caps, reload and
 * remove, and what they did. */
export function PluginsSection() {
  const plugins = useLive(
    () => "plugins",
    () => commands.plugins(),
    (): PluginView[] => [],
  )
  const automation = useLive(
    () => "plugin-automation",
    async () => ({ on: await commands.pluginAutomation() }),
    () => ({ on: [] as Id[] }),
  )
  return (
    <>
      <SettingsGroup
        title="Plugins"
        meta={plugins.length}
        description="Plugins run in a sandbox with no access to your cookies, files or the network."
        actions={
          <>
            <Button size="sm" icon="plus" onClick={() => run("plugin.install")}>
              Install from folder
            </Button>
            <Button size="sm" onClick={() => run("plugin.load_unpacked")}>
              Load unpacked
            </Button>
          </>
        }
      >
        <Show when={plugins.length === 0}>
          <EmptyState icon="plugin" title="No plugins" description="Install one from a folder with a plugin.toml." />
        </Show>
      </SettingsGroup>
      <Show when={plugins.length > 0}>
        <SettingsGroup
          title="Automation"
          description="Lets plugins run social actions, like friend requests, without asking each time, and spend within the budgets you set. Roblox bans accounts for bots and automated trading."
          actions={
            <Button size="sm" variant="danger" onClick={() => run("plugins.stop_all")}>
              Stop all plugins
            </Button>
          }
        >
          <For each={shell.accounts}>
            {(acc) => (
              <SettingSwitch
                label={`Allow automation on ${acc.label ?? acc.displayName}`}
                description={`@${acc.name}`}
                checked={automation.on.includes(acc.id)}
                onChange={(on) => void run(on ? "plugins.allow_automation" : "plugins.block_automation", null, acc.id)}
              />
            )}
          </For>
        </SettingsGroup>
      </Show>
      <For each={plugins}>{(p) => <PluginGroup plugin={p} automation={automation.on} />}</For>
      <ActivityGroup />
    </>
  )
}

const activityColumns: Column<ActivityView>[] = [
  { key: "at", label: "Time", width: "150px", cell: (a) => dateTime(a.at * 1000) },
  { key: "plugin", label: "Plugin", width: "minmax(80px, 1fr)", cell: (a) => a.pluginName },
  { key: "account", label: "Account", width: "minmax(80px, 1fr)", cell: (a) => a.accountName },
  {
    key: "action",
    label: "Action",
    width: "minmax(140px, 2fr)",
    cell: (a) => (
      <span class="truncate" title={a.arg ? `${a.title} ${a.arg}` : a.title}>
        {a.title}
        <Show when={a.arg}>{(arg) => <span class="font-mono text-fg-muted">{` ${arg()}`}</span>}</Show>
      </span>
    ),
  },
  {
    key: "how",
    label: "How",
    width: "90px",
    cell: (a) => <span class="text-fg-muted">{a.unattended ? "By itself" : "You ran it"}</span>,
  },
  {
    key: "result",
    label: "Result",
    width: "minmax(100px, 2fr)",
    cell: (a) => (
      <Show when={a.error} fallback={<Badge variant="success">Ran</Badge>}>
        {(e) => (
          <span class="truncate text-danger" title={e()}>
            {e()}
          </span>
        )}
      </Show>
    ),
  },
]

/** Every plugin write, newest first. `plugins.activity` scrolls here. */
function ActivityGroup() {
  const activity = useLive(
    () => "plugin-activity",
    async () => ({ rows: await commands.pluginActivity() }),
    () => ({ rows: [] as ActivityView[] }),
    { keep: true },
  )
  return (
    <div data-setting="activity">
      <SettingsGroup title="Activity" meta={activity.rows.length} description="What plugins did as your accounts.">
        <DataTable
          aria-label="Plugin activity"
          columns={activityColumns}
          rows={activity.rows}
          rowKey={(a) => `${a.at}-${a.pluginName}-${a.account}-${a.command}-${a.arg}-${a.error}`}
          empty={<EmptyState icon="plugin" title="Nothing yet" description="Plugin writes show up here." />}
        />
      </SettingsGroup>
    </div>
  )
}

function accountName(id: Id): string {
  const a = shell.accounts.find((x) => x.id === id)
  return a ? (a.label ?? a.displayName) : String(id)
}

/** "R$ 500 a day, R$ 120 spent. 2 trades a day, 1 accepted." or what asks each time. */
export function budgetText(b: PluginBudgetView): string {
  const parts: string[] = []
  if (b.hasRobux)
    parts.push(
      b.robux > 0 ? `R$ ${full(b.robux)} a day, R$ ${full(b.spentRobux)} spent.` : "Asks before spending Robux.",
    )
  if (b.hasTrades)
    parts.push(
      b.trades > 0
        ? `${b.trades === 1 ? "1 trade" : `${b.trades} trades`} a day, ${b.spentTrades} accepted.`
        : "Asks before accepting trades.",
    )
  return parts.join(" ")
}

function PluginGroup(props: { plugin: PluginView; automation: Id[] }) {
  /** None of the accounts it may use has automation on. */
  const needsAutomation = () => !props.plugin.accounts.some((a) => props.automation.includes(a))
  const p = () => props.plugin
  const subtitle = () =>
    [p().version && `Version ${p().version}`, p().author && `by ${p().author}`].filter(Boolean).join(", ")
  return (
    <SettingsGroup
      title={p().name}
      meta={<Badge variant={pluginStates[p().state].variant}>{pluginStates[p().state].label}</Badge>}
      description={p().path ? `Unpacked from ${p().path}` : subtitle()}
      actions={
        <>
          <Show when={p().state === "review"}>
            <Button size="sm" variant="primary" onClick={() => run("plugin.review", p().id)}>
              Review
            </Button>
          </Show>
          <Button size="sm" icon="refresh" onClick={() => run("plugin.reload", p().id)}>
            Reload
          </Button>
          <Button size="sm" variant="danger" onClick={() => run("plugin.remove", p().id)}>
            Remove
          </Button>
        </>
      }
    >
      <SettingSwitch
        label="On"
        checked={p().enabled}
        onChange={(on) => void run(on ? "plugin.enable" : "plugin.disable", p().id)}
      />
      <Show when={p().error}>
        {(e) => (
          <div class="py-2">
            <Alert variant="danger" title="It didn't load">
              <span class="font-mono text-sm">{e()}</span>
            </Alert>
          </div>
        )}
      </Show>
      <Show when={p().review.length > 0}>
        <div class="py-2">
          <Alert variant="warning" title="It now asks for more than you agreed to">
            <span class="font-mono text-sm">{p().review.join(", ")}</span>
          </Alert>
        </div>
      </Show>
      <Show when={p().commands.length > 0}>
        <SettingRow label="Commands" description={p().commands.join(", ")} />
      </Show>
      <Show when={p().listens.length > 0 || p().timers.length > 0}>
        <SettingRow
          label="Runs by itself"
          description={[...p().listens.map((e) => `on ${e.replace("_", " ")}`), ...p().timers.map(every)].join(", ")}
        />
      </Show>
      <For each={p().settings}>
        {(s) => (
          <SettingRow
            label={s.title}
            description={s.description || undefined}
            layout={s.kind === "range" ? "stacked" : "inline"}
            changed={s.value !== s.default}
            onReset={() => run("plugin.set", `${p().id} ${s.key}`)}
          >
            <PrefControl pref={s} onSet={(v) => void run("plugin.set", `${p().id} ${s.key} ${v}`)} />
          </SettingRow>
        )}
      </For>
      <Show when={p().asks}>
        {(a) => (
          <>
            <Show when={a().permissions.length > 0}>
              <SettingRow
                label="Permissions"
                description={a()
                  .permissions.map((x) => x.label)
                  .join(", ")}
              />
            </Show>
            <For each={byTier(a().actions)}>
              {(g) => (
                <SettingRow
                  label={`Tier ${g.tier} actions, ${tiers[g.tier].label.toLowerCase()}`}
                  description={g.actions.map((x) => x.title).join(", ")}
                >
                  <Show when={g.tier >= 2 && needsAutomation()}>
                    <Badge variant="neutral">Needs automation</Badge>
                  </Show>
                </SettingRow>
              )}
            </For>
            <For each={p().caps}>
              {(c) => (
                <SettingRow
                  label={`${c.title}, most an hour`}
                  description="Without asking, per account. You can lower it, not raise it."
                  layout="stacked"
                  changed={c.value !== c.max}
                  onReset={() => run("plugin.cap", `${p().id} ${c.key} ${c.max}`)}
                >
                  <PrefControl
                    pref={{
                      key: c.key,
                      title: `${c.title}, most an hour`,
                      kind: "range",
                      options: [],
                      min: 0,
                      max: c.max,
                      step: 1,
                      placeholder: "",
                      value: String(c.value),
                    }}
                    onSet={(v) => void run("plugin.cap", `${p().id} ${c.key} ${v}`)}
                  />
                </SettingRow>
              )}
            </For>
            <For each={p().budgets}>
              {(b) => (
                <SettingRow
                  label={`Budget on ${accountName(b.account)}`}
                  description={budgetText(b)}
                  changed={b.robux > 0 || b.trades > 0}
                >
                  <Button size="sm" onClick={() => run("plugin.budget", p().id, b.account)}>
                    Change
                  </Button>
                </SettingRow>
              )}
            </For>
            <Show when={a().needsAccounts}>
              <For each={shell.accounts}>
                {(acc) => (
                  <SettingSwitch
                    label={`Use ${acc.label ?? acc.displayName}`}
                    description={`@${acc.name}`}
                    checked={p().accounts.includes(acc.id)}
                    onChange={(on) => void run(on ? "plugin.grant" : "plugin.revoke", p().id, acc.id)}
                  />
                )}
              </For>
            </Show>
          </>
        )}
      </Show>
    </SettingsGroup>
  )
}
