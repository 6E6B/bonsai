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
  EFFECTS,
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
import { BACKGROUND_DEFAULTS, validBackgroundUrl } from "~/lib/theme/theme"
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
  TabPanel,
  Tabs,
  TextField,
} from "~/lib/ui"
import { saves } from "./commands"
import {
  changed,
  type Draft,
  drafts,
  dropDraft,
  edit,
  endGroup,
  pin,
  putDraft,
  setActiveDraft,
  setInfo,
  setSeed,
  startDraft,
  unpin,
  unsaved,
  update,
} from "./draft"

type Section = "seeds" | "background" | "details" | "tokens" | "checks"

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
    // Only discard a saved, unchanged draft. New themes stay in the library until saved.
    const d = drafts[x]
    if (d?.onDisk && !changed(d)) dropDraft(x)
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
  /** The token whose editor is open in the Tokens list. */
  const [open, setOpen] = createSignal<string>()
  /** Tokens picked with Inspect; the Tokens list shows only these while set. */
  const [picked, setPicked] = createSignal<string[]>()
  const items = () => [
    { value: "seeds", label: "Appearance" },
    { value: "background", label: "Background" },
    { value: "details", label: "Details" },
    { value: "tokens", label: "Advanced", count: Object.keys(props.draft.theme.tokens).length },
    { value: "checks", label: "Readability", count: failing() },
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
        <span role="status" aria-live="polite" class="text-sm text-fg-muted">
          {saves[props.id]?.busy ? "Saving…" : unsaved(props.draft) ? "Unsaved changes" : "All changes saved"}
        </span>
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
        <Button size="sm" variant="ghost" disabled={!changed(props.draft)} onClick={() => run("theme.revert")}>
          Revert
        </Button>
        <Button
          size="sm"
          variant="primary"
          loading={saves[props.id]?.busy}
          disabled={!unsaved(props.draft)}
          onClick={() => run("theme.save")}
        >
          Save theme
        </Button>
        <Button
          size="sm"
          disabled={unsaved(props.draft) || saves[props.id]?.busy}
          onClick={() => run("theme.apply", props.id)}
        >
          Use theme
        </Button>
        <Button
          size="sm"
          disabled={unsaved(props.draft) || saves[props.id]?.busy}
          onClick={() => run("hub.publish", JSON.stringify({ kind: "themes", id: props.id }))}
        >
          Publish…
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
            menuItem("theme.delete", {
              arg: props.id,
              icon: "trash",
              danger: true,
              label: props.draft.onDisk ? "Delete theme" : "Discard draft",
            }),
          ]}
        />
      </header>
      <div class="flex flex-wrap items-center gap-2 border-line border-b px-5 py-2 text-sm text-fg-muted">
        <Badge>Live preview</Badge>
        <span>Changes preview across Bonsai. Save to keep them, then Use theme to make it your default.</span>
      </div>
      <Show when={saves[props.id]?.error}>
        <Alert class="mx-5 mt-3" variant="danger" title="Save failed">
          {saves[props.id]?.error}
        </Alert>
      </Show>
      <Show when={props.draft.from}>
        {(from) => (
          <Alert class="mx-5 mt-3" title={`A copy of ${from()}`}>
            Built-in themes can't change, so this is a new theme. Save to keep it.
          </Alert>
        )}
      </Show>
      <Pane class="gap-3">
        <div class="w-full max-w-3xl">
          <Tabs aria-label="Editor" items={items()} value={section()} onChange={(v) => setSection(v as Section)}>
            <TabPanel value="seeds">
              <SeedsPanel draft={props.draft} edit={edit} fontStatus={fontStatus} />
            </TabPanel>
            <TabPanel value="background">
              <div class="flex flex-col gap-3">
                <BackgroundFields draft={props.draft} edit={edit} />
              </div>
            </TabPanel>
            <TabPanel value="details">
              <div class="flex flex-col gap-3">
                <TextField
                  label="Theme name"
                  value={props.draft.theme.name}
                  maxLength={64}
                  onChange={(v) => edit((d) => setInfo(d, "name", v, "name"))}
                  onBlur={() => edit(endGroup)}
                />
                <TextField
                  label="Author"
                  description="Shown when someone imports your theme."
                  value={props.draft.theme.author}
                  maxLength={64}
                  onChange={(v) => edit((d) => setInfo(d, "author", v, "author"))}
                  onBlur={() => edit(endGroup)}
                />
                <p class="text-sm text-fg-muted">
                  Save keeps this theme on your computer. Publish shares a saved version on the hub. Export a file or
                  copy a theme code from your theme library to share it privately.
                </p>
              </div>
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
        </div>
      </Pane>
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

/** The portable background is part of the draft, including undo/redo and live preview. */
function BackgroundFields(props: { draft: Draft; edit: Edit }) {
  const [url, setUrl] = createSignal("")
  const [error, setError] = createSignal<string>()
  let pending: ReturnType<typeof setTimeout> | undefined
  onCleanup(() => clearTimeout(pending))
  createEffect(
    on(
      () => props.draft.theme.background?.url ?? "",
      (value) => {
        setUrl(value)
        setError(undefined)
      },
    ),
  )
  const change = (value: string) => {
    setUrl(value)
    const link = value.trim()
    if (link && !validBackgroundUrl(link)) {
      setError("Use an external https image link.")
      return
    }
    setError(undefined)
    props.edit((d) =>
      edit(
        d,
        {
          ...d.theme,
          background: link ? { ...(d.theme.background ?? BACKGROUND_DEFAULTS), url: link } : undefined,
        },
        "background.url",
      ),
    )
  }
  return (
    <>
      <SectionHeader title="Background picture" size="group" />
      <TextField
        size="sm"
        label="Image URL"
        placeholder="https://example.com/background.gif"
        description="Paste a direct HTTPS link to a PNG, JPEG, WebP or GIF. Animated GIFs play automatically."
        value={url()}
        error={error()}
        onChange={(value) => {
          setUrl(value)
          clearTimeout(pending)
          pending = setTimeout(() => change(value), 400)
        }}
        onBlur={() => {
          clearTimeout(pending)
          change(url())
          props.edit(endGroup)
        }}
      />
      <Show when={props.draft.theme.background}>
        {(background) => (
          <>
            <Button size="sm" variant="ghost" onClick={() => change("")}>
              Remove picture
            </Button>
            <For
              each={[
                { key: "image" as const, label: "Picture strength", max: 100, step: 5, unit: "%" },
                { key: "panels" as const, label: "Page opacity", max: 100, step: 5, unit: "%" },
                ...EFFECTS,
              ]}
            >
              {(level) => (
                <Slider
                  label={level.label}
                  min={0}
                  max={level.max}
                  step={level.step}
                  value={background()[level.key]}
                  format={(v) => `${v}${level.unit}`}
                  onChange={(value) =>
                    props.edit((d) =>
                      edit(
                        d,
                        {
                          ...d.theme,
                          background: { ...background(), [level.key]: value },
                        },
                        `background.${level.key}`,
                      ),
                    )
                  }
                  onChangeEnd={() => props.edit(endGroup)}
                />
              )}
            </For>
          </>
        )}
      </Show>
    </>
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
