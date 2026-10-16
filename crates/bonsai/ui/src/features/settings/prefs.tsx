import { createContext, createSignal, type JSX, Match, type ParentProps, Show, Switch, useContext } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, type PrefView } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { RadioGroup, Select, SettingRow, Slider, TextField, Switch as Toggle } from "~/lib/ui"

const PrefsContext = createContext<PrefView[]>([])

/** Every setting `settings.set` takes, live. Settings creates it once for all its sections. */
export const usePrefsSource = () =>
  useLive(
    () => "prefs",
    () => commands.prefs(),
    (): PrefView[] => [],
  )

export function PrefsProvider(props: ParentProps<{ prefs: PrefView[] }>) {
  return <PrefsContext.Provider value={props.prefs}>{props.children}</PrefsContext.Provider>
}

/** The setting named `key`, once loaded. */
export const usePref = (key: string) => {
  const prefs = useContext(PrefsContext)
  return () => prefs.find((p) => p.key === key)
}

const set = (key: string, value: string) => run("settings.set", `${key} ${value}`)

/** One setting from bonsai-core's list: its title, description, a control for its kind, and a
 * reset button while it differs from the default. Pass children to use another control. */
export function PrefRow(props: { name: string; description?: JSX.Element; children?: JSX.Element }) {
  const pref = usePref(props.name)
  return (
    <Show when={pref()}>
      {(p) => (
        <SettingRow
          setting={p().key}
          label={p().title}
          description={props.description ?? (p().description || undefined)}
          layout={p().kind === "range" ? "stacked" : "inline"}
          changed={p().value !== p().default}
          onReset={() => run("settings.reset", p().key)}
        >
          <Show when={props.children} fallback={<PrefControl pref={p()} />}>
            {props.children}
          </Show>
        </SettingRow>
      )}
    </Show>
  )
}

function PrefControl(props: { pref: PrefView }) {
  const p = () => props.pref
  /** Text being typed; saved on Enter or when the field loses focus. */
  const [draft, setDraft] = createSignal<string>()
  const commit = () => {
    const v = draft()
    setDraft(undefined)
    if (v !== undefined && v !== p().value) set(p().key, v)
  }
  return (
    <Switch>
      <Match when={p().kind === "bool"}>
        <Toggle hideLabel checked={p().value === "on"} onChange={(on) => set(p().key, on ? "on" : "off")}>
          {p().title}
        </Toggle>
      </Match>
      <Match when={p().kind === "choice" && p().options.length <= 3}>
        <RadioGroup
          aria-label={p().title}
          orientation="horizontal"
          value={p().value}
          options={p().options}
          onChange={(v) => set(p().key, v)}
        />
      </Match>
      <Match when={p().kind === "choice"}>
        <Select
          size="sm"
          class="w-48"
          aria-label={p().title}
          value={p().value}
          options={p().options}
          onChange={(v) => set(p().key, v)}
        />
      </Match>
      <Match when={p().kind === "range"}>
        <Slider
          label={p().title}
          value={Number(p().value)}
          min={p().min}
          max={p().max}
          step={p().step}
          onChangeEnd={(v) => set(p().key, String(v))}
        />
      </Match>
      <Match when={p().kind === "text"}>
        <TextField
          size="sm"
          class="w-56"
          aria-label={p().title}
          placeholder={p().placeholder}
          value={draft() ?? p().value}
          onChange={setDraft}
          onBlur={commit}
          onKeyDown={(e) => {
            if (e.key === "Enter") commit()
            if (e.key === "Escape") setDraft(undefined)
          }}
        />
      </Match>
    </Switch>
  )
}
