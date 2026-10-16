import { makeEventListener } from "@solid-primitives/event-listener"
import { createMemo, createSignal, Show } from "solid-js"
import { clashes, type Recorded, record, spec, toPress } from "~/lib/commands/keys"
import { commandList, run } from "~/lib/commands/registry"
import type { CommandInfo } from "~/lib/ipc/bindings"
import { Alert, Button, Kbd, useDialog } from "~/lib/ui"

const keysOf = (s: Recorded) => (s.kind === "sequence" ? [s.first, s.second] : spec(s).split("+"))

/** Records a new shortcut for one command. Escape cancels and Tab moves focus, so neither can be
 * bound here; everything else is captured before the app's own shortcuts see it. */
export function ShortcutCapture(props: { command: CommandInfo }) {
  const dialog = useDialog()
  const [last, setLast] = createSignal<{ shortcut: Recorded; at: number }>()
  makeEventListener(
    window,
    "keydown",
    (e) => {
      const plain = !e.ctrlKey && !e.metaKey && !e.altKey
      if ((e.key === "Escape" || e.key === "Tab") && plain) return
      // Enter on a focused button presses it.
      if (e.key === "Enter" && plain && e.target instanceof HTMLButtonElement) return
      const p = toPress(e)
      e.preventDefault()
      e.stopImmediatePropagation()
      if (!p) return
      setLast((prev) => ({ shortcut: record(prev, p, e.timeStamp), at: e.timeStamp }))
    },
    { capture: true },
  )
  const taken = createMemo(() => {
    const s = last()?.shortcut
    if (!s) return []
    return commandList().filter((c) => c.id !== props.command.id && c.shortcuts.some((k) => clashes(s, k)))
  })
  const save = (keys: string) => {
    dialog.close()
    void run("shortcuts.set", `${props.command.id}${keys ? ` ${keys}` : ""}`)
  }
  return (
    <div class="flex flex-col gap-4 p-4">
      <div class="flex min-h-16 items-center justify-center rounded-md bg-field">
        <Show
          when={last()}
          fallback={<span class="text-fg-muted">Press the keys. Two plain keys in a row make a sequence.</span>}
        >
          {(l) => <Kbd keys={keysOf(l().shortcut)} />}
        </Show>
      </div>
      <Show when={taken().length > 0}>
        <Alert
          variant="warning"
          title={`Used by ${taken()
            .map((c) => `“${c.title}”`)
            .join(", ")}`}
        >
          Saving takes it from {taken().length > 1 ? "them" : "it"}.
        </Alert>
      </Show>
      <div class="flex items-center gap-2">
        <Button size="sm" variant="ghost" disabled={props.command.shortcuts.length === 0} onClick={() => save("")}>
          No shortcut
        </Button>
        <Show when={props.command.rebound}>
          <Button
            size="sm"
            variant="ghost"
            onClick={() => {
              dialog.close()
              void run("shortcuts.reset", props.command.id)
            }}
          >
            Use default
          </Button>
        </Show>
        <span class="flex-1" />
        <Button size="sm" onClick={() => dialog.close()}>
          Cancel
        </Button>
        <Button
          size="sm"
          variant="primary"
          disabled={!last()}
          onClick={() => {
            const l = last()
            if (l) save(spec(l.shortcut))
          }}
        >
          Save
        </Button>
      </div>
    </div>
  )
}
