import { For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, type PluginView } from "~/lib/ipc/bindings"
import { byTier, pluginStates, tiers } from "~/lib/plugins"
import { shell, useLive } from "~/lib/store"
import { Alert, Badge, Button, EmptyState, SettingRow, SettingSwitch, SettingsGroup } from "~/lib/ui"
import { PrefControl } from "./prefs"

/** "every 5 min", "every 2 h", "every 90 s". */
export function every(seconds: number): string {
  if (seconds % 3600 === 0) return `every ${seconds / 3600} h`
  if (seconds % 60 === 0) return `every ${seconds / 60} min`
  return `every ${seconds} s`
}

/** Installed plugins: on or off, which accounts each may use, reload and remove. */
export function PluginsSection() {
  const plugins = useLive(
    () => "plugins",
    () => commands.plugins(),
    (): PluginView[] => [],
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
      <For each={plugins}>{(p) => <PluginGroup plugin={p} />}</For>
    </>
  )
}

function PluginGroup(props: { plugin: PluginView }) {
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
                />
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
