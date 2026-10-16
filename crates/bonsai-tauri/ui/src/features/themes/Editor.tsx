import { useLocation, useParams } from "@solidjs/router"
import { createEffect, createMemo, createResource, createSignal, For, on, onCleanup, onMount, Show } from "solid-js"
import { useSubTabs } from "~/lib/commands/hooks"
import { menuItem, run } from "~/lib/commands/registry"
import { FontPicker } from "~/lib/font-picker"
import { commands, type FontView } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { setTabTitle } from "~/lib/tabs"
import {
  type Base,
  type Check,
  FONTS,
  type FontSeed,
  fontLabel,
  googleFamily,
  MONOS,
  parseTheme,
  registerFonts,
  resolve,
  runChecks,
  type Seeds,
  TOKEN,
  TOKENS,
  type Token,
  type Tokens,
  tokenValue,
} from "~/lib/theme"
import {
  Alert,
  Badge,
  Button,
  ColorPicker,
  EmptyState,
  Icon,
  IconButton,
  Menu,
  Pane,
  SectionHeader,
  SegmentedControl,
  Select,
  Skeleton,
  Slider,
  Split,
  Switch,
  TabPanel,
  Tabs,
  TextField,
  ToggleButton,
} from "~/lib/ui"
import {
  changed,
  type Draft,
  drafts,
  dropDraft,
  edit,
  endGroup,
  inspecting,
  liveDraft,
  pin,
  putDraft,
  setActiveDraft,
  setInfo,
  setInspecting,
  setSeed,
  startDraft,
  unpin,
  unsaved,
  update,
} from "./draft"
import { Preview } from "./Preview"

type Section = "seeds" | "tokens" | "checks"

/** The theme editor, one theme per tab. Edits go to a draft (see `draft.ts`) until Save. */
export default function Editor() {
  const params = useParams()
  const location = useLocation()
  const id = () => params.id ?? ""
  // What the id names on disk, looked up once per id. A draft needs no lookup.
  const [source] = createResource(id, async (x) => {
    if (x in drafts) return null
    return (await commands.themes()).find((t) => t.id === x) ?? null
  })
  createEffect(() => {
    const v = source()
    if (!v || v.builtin || v.id in drafts) return
    const theme = parseTheme(v.doc)
    if (theme) putDraft(startDraft(theme, { onDisk: true }))
  })
  createEffect(
    on(id, (x, prev) => {
      if (prev) leave(prev)
      setActiveDraft(x)
    }),
  )
  const leave = (x: string) => {
    setActiveDraft(undefined)
    setInspecting(false)
    // An untouched draft goes; one with changes stays, listed on the Themes screen as unsaved.
    const d = drafts[x]
    if (d && !changed(d)) dropDraft(x)
  }
  onCleanup(() => leave(id()))
  createEffect(() => {
    const d = drafts[id()]
    if (d) setTabTitle(location.pathname, d.theme.name)
  })

  return (
    <Show
      when={drafts[id()]}
      fallback={
        <Show when={!source.loading} fallback={<Skeleton class="m-5 h-40" />}>
          <Show
            when={source()?.builtin}
            fallback={<EmptyState class="m-5" icon="palette" title="No theme with this id" />}
          >
            <EmptyState
              class="m-5"
              icon="palette"
              title="Built-in themes can't be changed"
              description="Edit a copy instead. The copy is yours to change and share."
            >
              <Button variant="primary" icon="edit" onClick={() => run("theme.edit", id())}>
                Edit a copy
              </Button>
            </EmptyState>
          </Show>
        </Show>
      }
    >
      {(d) => <Workbench id={id()} draft={d()} />}
    </Show>
  )
}

function Workbench(props: { id: string; draft: Draft }) {
  const tokens = createMemo(() => resolve(props.draft.theme))
  const checks = createMemo(() => runChecks(tokens()))
  const failing = () => checks().filter((c) => !c.ok).length
  const [section, setSection] = createSignal<Section>("seeds")
  const [view, setView] = createSignal<"app" | "parts">("app")
  /** The token whose editor is open in the Tokens list. */
  const [open, setOpen] = createSignal<string>()
  /** Tokens picked with Inspect; the Tokens list shows only these while set. */
  const [picked, setPicked] = createSignal<string[]>()
  const items = () => [
    { value: "seeds", label: "Seeds" },
    { value: "tokens", label: "Tokens", count: Object.keys(props.draft.theme.tokens).length },
    { value: "checks", label: "Checks", count: failing() },
  ]
  useSubTabs(items, section, (v) => setSection(v as Section))
  const edit = (f: (d: Draft) => Draft) => update(props.id, f)
  const showToken = (name: string) => {
    setPicked(undefined)
    setOpen(name)
    setSection("tokens")
  }

  // Google fonts in the draft load as they download, before the theme is saved.
  const families = createMemo(() =>
    [googleFamily(props.draft.theme.seeds.font), googleFamily(props.draft.theme.seeds.mono)]
      .filter((f): f is string => !!f)
      .join("\n"),
  )
  const fonts = useLive(
    families,
    (f): Promise<{ list: FontView[] }> =>
      f ? commands.fontFiles(f.split("\n")).then((list) => ({ list })) : Promise.resolve({ list: [] }),
    () => ({ list: [] }),
  )
  createEffect(() => registerFonts(fonts.list))
  const fontStatus = (seed: FontSeed<string>) => {
    const f = fonts.list.find((x) => x.family === googleFamily(seed))
    if (f?.error) return f.error
    if (f && f.files.length === 0) return `Downloading ${f.family}…`
    return undefined
  }

  let root: HTMLDivElement | undefined
  onMount(() => root?.addEventListener("keydown", onKeyDown))
  onCleanup(() => root?.removeEventListener("keydown", onKeyDown))
  const onKeyDown = (e: KeyboardEvent) => {
    const typing = e.target instanceof HTMLElement && ["INPUT", "TEXTAREA"].includes(e.target.tagName)
    const mod = e.ctrlKey || e.metaKey
    const key = e.key.toLowerCase()
    let id: string | undefined
    if (mod && key === "s") id = "theme.save"
    else if (mod && !typing && key === "z") id = e.shiftKey ? "theme.redo" : "theme.undo"
    else if (mod && !typing && key === "y") id = "theme.redo"
    else if (key === "escape" && inspecting()) id = "theme.inspect"
    if (!id) return
    e.preventDefault()
    e.stopPropagation()
    void run(id)
  }

  return (
    <div ref={root} class="flex min-h-0 flex-1 flex-col">
      <header class="flex flex-wrap items-center gap-2 border-line border-b px-5 py-2">
        <h1 class="min-w-0 truncate font-semibold text-lg">{props.draft.theme.name}</h1>
        <Badge>{props.draft.theme.base === "dark" ? "Dark" : "Light"}</Badge>
        <Show when={unsaved(props.draft)}>
          <Badge variant="warning">{props.draft.onDisk ? "Unsaved changes" : "Not saved"}</Badge>
        </Show>
        <div class="flex-1" />
        <IconButton
          size="sm"
          variant="ghost"
          icon="undo"
          label="Undo (Ctrl+Z)"
          disabled={props.draft.past.length === 0}
          onClick={() => run("theme.undo")}
        />
        <IconButton
          size="sm"
          variant="ghost"
          icon="redo"
          label="Redo (Ctrl+Shift+Z)"
          disabled={props.draft.future.length === 0}
          onClick={() => run("theme.redo")}
        />
        <ToggleButton
          size="sm"
          icon="inspect"
          pressed={inspecting()}
          onChange={() => run("theme.inspect")}
          label="Inspect"
        >
          Inspect
        </ToggleButton>
        <Switch checked={liveDraft() === props.id} onChange={() => run("theme.live")}>
          Show in app
        </Switch>
        <Button size="sm" variant="ghost" disabled={!changed(props.draft)} onClick={() => run("theme.revert")}>
          Revert
        </Button>
        <Button size="sm" variant="primary" disabled={!unsaved(props.draft)} onClick={() => run("theme.save")}>
          Save
        </Button>
        <Menu
          size="sm"
          variant="ghost"
          icon="more"
          label="More"
          placement="bottom-end"
          items={[
            menuItem("theme.rename", { arg: props.id, icon: "edit" }),
            menuItem("theme.apply", { arg: props.id, icon: "check", disabled: !props.draft.onDisk }),
            menuItem("theme.duplicate", { arg: props.id, icon: "copy", disabled: !props.draft.onDisk }),
            menuItem("theme.fix_contrast", { icon: "circle-check" }),
            menuItem("theme.delete", { arg: props.id, icon: "trash", danger: true, disabled: !props.draft.onDisk }),
          ]}
        />
      </header>
      <Show when={props.draft.from}>
        {(from) => (
          <Alert class="mx-5 mt-3" title={`A copy of ${from()}`}>
            Built-in themes can't change, so this is a new theme. Save to keep it.
          </Alert>
        )}
      </Show>
      <Split>
        <Pane side class="w-80 gap-3">
          <Tabs aria-label="Editor" items={items()} value={section()} onChange={(v) => setSection(v as Section)}>
            <TabPanel value="seeds">
              <SeedsPanel draft={props.draft} edit={edit} fontStatus={fontStatus} />
            </TabPanel>
            <TabPanel value="tokens">
              <TokensPanel
                draft={props.draft}
                tokens={tokens()}
                edit={edit}
                open={open()}
                setOpen={setOpen}
                picked={picked()}
                clearPicked={() => setPicked(undefined)}
              />
            </TabPanel>
            <TabPanel value="checks">
              <ChecksPanel checks={checks()} tokens={tokens()} onShow={showToken} />
            </TabPanel>
          </Tabs>
        </Pane>
        <Pane class="gap-3">
          <div class="flex items-center gap-2">
            <SegmentedControl
              size="sm"
              aria-label="Preview"
              value={view()}
              onChange={(v) => setView(v as "app" | "parts")}
              options={[
                { value: "app", label: "App" },
                { value: "parts", label: "Popovers and badges" },
              ]}
            />
            <Show when={inspecting()}>
              <span class="text-fg-muted text-sm">Click part of the preview to see its tokens. Esc stops.</span>
            </Show>
          </div>
          <Preview
            tokens={tokens()}
            base={props.draft.theme.base}
            view={view()}
            inspecting={inspecting()}
            onPick={(names) => {
              setPicked(names)
              setOpen(names[0])
              setSection("tokens")
              setInspecting(false)
            }}
          />
        </Pane>
      </Split>
    </div>
  )
}

type Edit = (f: (d: Draft) => Draft) => void

const SEED_COLORS: [keyof Seeds, string][] = [
  ["background", "Background"],
  ["foreground", "Text"],
  ["accent", "Accent"],
  ["success", "Success"],
  ["warning", "Warning"],
  ["danger", "Danger"],
]

const fontSeed = <Id extends string>(label: string, options: Record<Id, { label: string }>): FontSeed<Id> => {
  const id = (Object.keys(options) as Id[]).find((k) => options[k].label === label)
  return id ?? { google: label }
}

/** A dozen values that make the whole theme. */
function SeedsPanel(props: { draft: Draft; edit: Edit; fontStatus: (s: FontSeed<string>) => string | undefined }) {
  const seeds = () => props.draft.theme.seeds
  const [open, setOpen] = createSignal<keyof Seeds>()
  const num = (key: "contrast" | "radius", label: string, max: number) => (
    <Slider
      label={label}
      value={seeds()[key]}
      min={0}
      max={max}
      step={0.05}
      format={(v) => `${Math.round(v * 100)}%`}
      onChange={(v) => props.edit((d) => setSeed(d, key, v, key))}
      onChangeEnd={() => props.edit(endGroup)}
    />
  )
  const shadow = (key: "depth" | "ring" | "highlight", label: string) => (
    <Slider
      label={label}
      value={seeds().shadow[key]}
      min={0}
      max={3}
      step={0.05}
      format={(v) => `${Math.round(v * 100)}%`}
      onChange={(v) => props.edit((d) => setSeed(d, "shadow", { ...d.theme.seeds.shadow, [key]: v }, `shadow.${key}`))}
      onChangeEnd={() => props.edit(endGroup)}
    />
  )
  return (
    <div class="flex flex-col gap-3">
      <TextField
        size="sm"
        label="Name"
        value={props.draft.theme.name}
        maxLength={64}
        onChange={(v) => props.edit((d) => setInfo(d, "name", v, "name"))}
        onBlur={() => props.edit(endGroup)}
      />
      <TextField
        size="sm"
        label="Author"
        value={props.draft.theme.author}
        maxLength={64}
        onChange={(v) => props.edit((d) => setInfo(d, "author", v, "author"))}
        onBlur={() => props.edit(endGroup)}
      />
      <SegmentedControl
        size="sm"
        aria-label="Base"
        value={props.draft.theme.base}
        onChange={(v) => props.edit((d) => edit(endGroup(d), { ...d.theme, base: v as Base }))}
        options={[
          { value: "dark", label: "Dark" },
          { value: "light", label: "Light" },
        ]}
      />

      <SectionHeader title="Colors" size="group" />
      <div class="flex flex-col">
        <For each={SEED_COLORS}>
          {([key, label]) => (
            <>
              <button
                type="button"
                class="flex items-center gap-2 rounded-sm px-1 py-1 text-left hover:bg-hover"
                aria-expanded={open() === key}
                onClick={() => setOpen(open() === key ? undefined : key)}
              >
                <ColorChip color={seeds()[key] as string} />
                <span class="flex-1 text-sm">{label}</span>
                <span class="font-mono text-fg-muted text-xs">{seeds()[key] as string}</span>
              </button>
              <Show when={open() === key}>
                <ColorPicker
                  class="px-1 pt-1 pb-3"
                  label={label}
                  value={seeds()[key] as string}
                  onChange={(v) => props.edit((d) => setSeed(d, key, v, key))}
                  onChangeEnd={() => props.edit(endGroup)}
                />
              </Show>
            </>
          )}
        </For>
      </div>

      <SectionHeader title="Shape" size="group" />
      {num("contrast", "Surface contrast", 3)}
      {num("radius", "Corner radius", 4)}
      {shadow("depth", "Shadow depth")}
      {shadow("ring", "Outlines")}
      {shadow("highlight", "Top highlight")}

      <SectionHeader title="Fonts" size="group" />
      <FontPicker
        class="w-full"
        label="Font"
        mono={false}
        value={fontLabel(seeds().font, FONTS)}
        onChange={(v) => props.edit((d) => setSeed(d, "font", fontSeed(v, FONTS)))}
      />
      <Show when={props.fontStatus(seeds().font)}>{(s) => <span class="text-fg-muted text-xs">{s()}</span>}</Show>
      <FontPicker
        class="w-full"
        label="Code font"
        mono
        value={fontLabel(seeds().mono, MONOS)}
        onChange={(v) => props.edit((d) => setSeed(d, "mono", fontSeed(v, MONOS)))}
      />
      <Show when={props.fontStatus(seeds().mono)}>{(s) => <span class="text-fg-muted text-xs">{s()}</span>}</Show>
    </div>
  )
}

/** Every token by group. Pinned ones are marked; open one to change or reset it. */
function TokensPanel(props: {
  draft: Draft
  tokens: Tokens
  edit: Edit
  open: string | undefined
  setOpen: (name: string | undefined) => void
  picked: string[] | undefined
  clearPicked: () => void
}) {
  const [query, setQuery] = createSignal("")
  const shown = createMemo(() => {
    const q = query().trim().toLowerCase()
    const picked = props.picked
    return TOKENS.filter(
      (t) =>
        (!picked || picked.includes(t.name)) &&
        (!q || t.label.toLowerCase().includes(q) || t.name.includes(q) || t.group.toLowerCase().includes(q)),
    )
  })
  const groups = createMemo(() => {
    const out: [string, Token[]][] = []
    for (const t of shown()) {
      const last = out.at(-1)
      if (last && last[0] === t.group) last[1].push(t)
      else out.push([t.group, [t]])
    }
    return out
  })
  return (
    <div class="flex flex-col gap-2">
      <TextField
        size="sm"
        icon="search"
        aria-label="Search tokens"
        placeholder="Search"
        value={query()}
        onChange={setQuery}
      />
      <Show when={props.picked}>
        {(p) => (
          <div class="flex items-center gap-2 text-fg-muted text-sm">
            <span class="flex-1">Picked from the preview: {p().length}</span>
            <Button size="sm" variant="ghost" onClick={props.clearPicked}>
              Show all
            </Button>
          </div>
        )}
      </Show>
      <Show when={groups().length > 0} fallback={<span class="text-fg-muted text-sm">No tokens match.</span>}>
        <For each={groups()}>
          {([group, list]) => (
            <div class="flex flex-col">
              <SectionHeader title={group} size="group" />
              <For each={list}>
                {(t) => (
                  <TokenRow
                    token={t}
                    draft={props.draft}
                    tokens={props.tokens}
                    edit={props.edit}
                    open={props.open === t.name}
                    onToggle={() => props.setOpen(props.open === t.name ? undefined : t.name)}
                    onShow={(name) => props.setOpen(name)}
                  />
                )}
              </For>
            </div>
          )}
        </For>
      </Show>
    </div>
  )
}

const REF = /^var\(--([\w-]+)\)$/

/** A color token's hex, through references. */
function hexOf(tokens: Tokens, name: string): string {
  let v = tokens[name] ?? ""
  for (let i = 0; i < 8; i++) {
    const ref = REF.exec(v)
    if (!ref) break
    v = tokens[ref[1]] ?? ""
  }
  return v
}

const colorTokens = TOKENS.filter((t) => t.kind === "color")

function TokenRow(props: {
  token: Token
  draft: Draft
  tokens: Tokens
  edit: Edit
  open: boolean
  onToggle: () => void
  onShow: (name: string) => void
}) {
  const name = () => props.token.name
  const pinned = () => name() in props.draft.theme.tokens
  const value = () => props.tokens[name()] ?? ""
  const ref = () => REF.exec(value())?.[1]
  return (
    <div class="flex flex-col">
      <div class="flex items-center gap-1">
        <button
          type="button"
          class="flex min-w-0 flex-1 items-center gap-2 rounded-sm px-1 py-1 text-left hover:bg-hover"
          aria-expanded={props.open}
          onClick={() => props.onToggle()}
        >
          <Show when={props.token.kind === "color"}>
            <ColorChip color={hexOf(props.tokens, name())} />
          </Show>
          <span class="min-w-0 flex-1 truncate text-sm">{props.token.label}</span>
          <Show when={ref()}>
            {(r) => <span class="truncate text-fg-faint text-xs">→ {TOKEN.get(r())?.label}</span>}
          </Show>
        </button>
        <Show when={pinned()}>
          <IconButton
            size="sm"
            variant="ghost"
            icon="x"
            label={`Reset ${props.token.label} to follow the seeds`}
            onClick={() => props.edit((d) => unpin(d, name()))}
          />
        </Show>
      </div>
      <Show when={props.open}>
        <div class="flex flex-col gap-2 px-1 pt-1 pb-3">
          <span class="font-mono text-fg-faint text-xs">--{name()}</span>
          <Show when={props.token.kind === "color"}>
            <ColorPicker
              label={props.token.label}
              alpha
              value={hexOf(props.tokens, name())}
              onChange={(v) => props.edit((d) => pin(d, name(), v, `token:${name()}`))}
              onChangeEnd={() => props.edit(endGroup)}
            />
            <Select
              size="sm"
              aria-label="Same as"
              placeholder="Same as another token"
              value={ref() ?? ""}
              options={colorTokens
                .filter((t) => t.name !== name())
                .map((t) => ({ value: t.name, label: `Same as ${t.label}` }))}
              onChange={(v) => v && props.edit((d) => pin(d, name(), `var(--${v})`))}
            />
            <Show when={ref()}>
              {(r) => (
                <Button size="sm" variant="ghost" onClick={() => props.onShow(r())}>
                  Go to {TOKEN.get(r())?.label}
                </Button>
              )}
            </Show>
          </Show>
          <Show when={props.token.kind === "length" || props.token.kind === "shadow"}>
            <RawField token={props.token} value={value()} edit={props.edit} />
          </Show>
          <Show when={props.token.kind === "font"}>
            <Select
              size="sm"
              aria-label={props.token.label}
              value={
                Object.entries(name() === "font-mono" ? MONOS : FONTS).find(([, f]) => f.stack === value())?.[0] ?? ""
              }
              options={Object.entries(name() === "font-mono" ? MONOS : FONTS).map(([id, f]) => ({
                value: id,
                label: f.label,
              }))}
              onChange={(v) => props.edit((d) => pin(d, name(), v))}
            />
            <span class="text-fg-muted text-xs">Google fonts are set in Seeds.</span>
          </Show>
        </div>
      </Show>
    </div>
  )
}

/** A text field for a px length or shadow layers. Bad values show an error and aren't kept. */
function RawField(props: { token: Token; value: string; edit: Edit }) {
  const [text, setText] = createSignal(props.value)
  const [error, setError] = createSignal<string>()
  createEffect(
    on(
      () => props.value,
      (v) => setText(v),
    ),
  )
  return (
    <TextField
      size="sm"
      aria-label={props.token.label}
      class="font-mono"
      value={text()}
      error={error()}
      onChange={(v) => {
        setText(v)
        if (tokenValue(props.token.name, v) === undefined) {
          setError(
            props.token.kind === "length" ? "A length in px, like 6px." : "Shadow layers: x y blur spread #color.",
          )
          return
        }
        setError(undefined)
        props.edit((d) => pin(d, props.token.name, v.trim(), `token:${props.token.name}`))
      }}
      onBlur={() => {
        setError(undefined)
        setText(props.value)
        props.edit(endGroup)
      }}
    />
  )
}

/** Text and fill pairs with their contrast. Click one to edit the token its fix changes. */
function ChecksPanel(props: { checks: Check[]; tokens: Tokens; onShow: (name: string) => void }) {
  const failing = () => props.checks.filter((c) => !c.ok).length
  return (
    <div class="flex flex-col gap-2">
      <div class="flex items-center gap-2">
        <span class="flex-1 text-fg-muted text-sm">
          {failing() === 0 ? "Everything is readable." : `${failing()} below the WCAG minimum.`}
        </span>
        <Button size="sm" disabled={failing() === 0} onClick={() => run("theme.fix_contrast")}>
          Fix all
        </Button>
      </div>
      <For each={props.checks}>
        {(c) => (
          <button
            type="button"
            class="flex items-center gap-2 rounded-sm px-1 py-1 text-left hover:bg-hover"
            onClick={() => props.onShow(c.fix)}
          >
            <span
              class="flex size-8 flex-none items-center justify-center rounded-sm border border-line font-semibold text-sm"
              style={{ background: hexOf(props.tokens, c.bg), color: hexOf(props.tokens, c.fg) }}
              aria-hidden="true"
            >
              Aa
            </span>
            <span class="flex min-w-0 flex-1 flex-col">
              <span class="truncate text-sm">{c.label}</span>
              <span class="text-fg-muted text-xs">
                {c.ratio.toFixed(1)}:1, needs {c.min}:1
              </span>
            </span>
            <Icon name={c.ok ? "circle-check" : "circle-x"} class={c.ok ? "text-success" : "text-danger"} />
          </button>
        )}
      </For>
    </div>
  )
}

/** A small square of a color from the draft. */
function ColorChip(props: { color: string }) {
  return (
    <span
      class="size-4 flex-none rounded-xs border border-line-strong"
      style={{ background: props.color }}
      aria-hidden="true"
    />
  )
}
