import { isTauri } from "@tauri-apps/api/core"
import { getCurrentWebview } from "@tauri-apps/api/webview"
import { createEffect, createMemo, createSignal, For, on, onCleanup, Show } from "solid-js"
import { run, runBackend } from "~/lib/commands/registry"
import { commands } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"
import { Alert, Button, Dialog, Fact, Facts, SegmentedControl, showToast, TextField, ThemeCard } from "~/lib/ui"
import { setThemeUi, themeUi } from "./commands"
import { isThemeFile, summarize } from "./import"

/**
 * The theme a code or file brought in, before it is installed: a preview card, what was skipped,
 * a contrast summary and, when the id is taken, replace or keep both. It opens by itself from
 * `shell.themeImport`, which `theme.import` sets.
 */
export function ImportDialog() {
  const [code, setCode] = createSignal("")
  const [reading, setReading] = createSignal(false)
  const [readError, setReadError] = createSignal("")
  const read = async (input: string) => {
    if (reading()) return
    setReading(true)
    setReadError("")
    try {
      if (await runBackend("theme.import", input)) {
        setThemeUi("importing", false)
        setCode("")
        showToast("Reading theme. Review it before adding it to your library.")
      }
    } catch {
      setReadError("Could not read this theme. Your pasted code is still here. Try again.")
    } finally {
      setReading(false)
    }
  }
  const pending = () => shell.themeImport
  const summary = createMemo(() => {
    const p = pending()
    return p ? summarize(p.doc) : undefined
  })
  const good = () => {
    const s = summary()
    return s?.ok ? s : undefined
  }
  const unreadable = () => {
    const s = summary()
    return s && !s.ok ? s.error : ""
  }
  const [keepBoth, setKeepBoth] = createSignal(false)
  const [busy, setBusy] = createSignal(false)
  const [error, setError] = createSignal("")
  createEffect(
    on(pending, (p) => {
      setKeepBoth(p?.clashBuiltin ?? false)
      setBusy(false)
      setError("")
    }),
  )
  const install = async (apply: boolean) => {
    if (busy()) return
    setBusy(true)
    try {
      const r = await commands.themeImportInstall(keepBoth(), apply)
      if (r.status === "error") setError(r.error)
      else showToast(apply ? "Theme added and applied." : "Theme added to your library.")
    } catch {
      setError("Could not add this theme. Try again.")
    } finally {
      setBusy(false)
    }
  }
  return (
    <>
      <Dialog
        open={themeUi.importing}
        onOpenChange={(open) => {
          if (!reading()) setThemeUi("importing", open)
        }}
        title="Import theme"
        size="md"
      >
        <div class="flex flex-col gap-4 p-4">
          <p class="text-sm text-fg-muted">
            Add a theme someone shared with you. You can review it before saving or using it.
          </p>
          <Button icon="download" loading={reading()} onClick={() => read("")}>
            Choose a theme file…
          </Button>
          <TextField
            label="Or paste a theme code"
            multiline
            placeholder="Paste a shared theme code or theme JSON"
            value={code()}
            onChange={setCode}
          />
          <Show when={readError()}>
            <Alert variant="danger" title="Could not import theme">
              {readError()}
            </Alert>
          </Show>
          <div class="flex justify-end gap-2">
            <Button disabled={reading()} onClick={() => setThemeUi("importing", false)}>
              Cancel
            </Button>
            <Button variant="primary" loading={reading()} disabled={!code().trim()} onClick={() => read(code().trim())}>
              Review theme
            </Button>
          </div>
        </div>
      </Dialog>
      <Dialog
        open={pending() !== null}
        onOpenChange={(open) => {
          if (!open && !busy()) void commands.themeImportCancel()
        }}
        title="Import theme"
        size="md"
      >
        <Show when={unreadable()}>
          <div class="p-4">
            <Alert variant="danger" title={unreadable()} />
          </div>
        </Show>
        <Show when={good()}>
          {(ok) => (
            <div class="flex flex-col gap-4 p-4">
              <div class="flex gap-4">
                <ThemeCard class="w-40 shrink-0" name={ok().theme.name} tokens={ok().tokens} />
                <Facts class="min-w-0 flex-1">
                  <Fact label="Author">{ok().theme.author || "Not given"}</Fact>
                  <Fact label="Version">{ok().theme.version}</Fact>
                  <Fact label="Base">{ok().theme.base === "dark" ? "Dark" : "Light"}</Fact>
                  <Fact label="From">{pending()?.source || "Pasted code"}</Fact>
                  <Fact label="Contrast">
                    {ok().failing.length === 0
                      ? "All text is readable"
                      : `${ok().failing.length} of ${ok().checked} pairs too low`}
                  </Fact>
                </Facts>
              </div>
              <Show when={ok().failing.length > 0}>
                <Alert variant="warning" title="Some text may be hard to read">
                  <ul class="text-sm">
                    <For each={ok().failing}>
                      {(c) => (
                        <li>
                          {c.label}: {c.ratio.toFixed(1)}:1, needs {c.min}:1
                        </li>
                      )}
                    </For>
                  </ul>
                </Alert>
              </Show>
              <Show when={ok().skipped.length > 0}>
                <Alert variant="info" title={`Skipped ${ok().skipped.length} values this version can't use`}>
                  <ul class="font-mono text-sm">
                    <For each={ok().skipped}>{(p) => <li>{p}</li>}</For>
                  </ul>
                </Alert>
              </Show>
              <Show when={pending()?.clash}>
                {(name) => (
                  <div class="flex items-center justify-between gap-3">
                    <span class="text-sm text-fg-muted">
                      <Show when={pending()?.clashBuiltin} fallback={`You have this theme already, as ${name()}.`}>
                        {`It has the same id as the built-in ${name()}, so it installs as a copy.`}
                      </Show>
                    </span>
                    <SegmentedControl
                      size="sm"
                      aria-label="Same id"
                      value={keepBoth() ? "both" : "replace"}
                      onChange={(v) => setKeepBoth(v === "both")}
                      options={[
                        { value: "replace", label: "Replace", disabled: pending()?.clashBuiltin ?? false },
                        { value: "both", label: "Keep both" },
                      ]}
                    />
                  </div>
                )}
              </Show>
              <Show when={error()}>
                <Alert variant="danger" title={error()} />
              </Show>
              <div class="flex justify-end gap-2">
                <Button onClick={() => void commands.themeImportCancel()} disabled={busy()}>
                  Cancel
                </Button>
                <Button onClick={() => void install(false)} disabled={busy()}>
                  Add to library
                </Button>
                <Button variant="primary" onClick={() => void install(true)} loading={busy()}>
                  Add and use theme
                </Button>
              </div>
            </div>
          )}
        </Show>
      </Dialog>
    </>
  )
}

/** Dropping a `.bonsaitheme` file on the window imports it. */
export function useThemeDrop() {
  if (!isTauri()) return
  let stop: (() => void) | undefined
  let gone = false
  void getCurrentWebview()
    .onDragDropEvent((e) => {
      if (e.payload.type !== "drop") return
      const file = e.payload.paths.find(isThemeFile)
      if (file) void run("theme.import", file)
    })
    .then((f) => {
      if (gone) f()
      else stop = f
    })
  onCleanup(() => {
    gone = true
    stop?.()
  })
}
