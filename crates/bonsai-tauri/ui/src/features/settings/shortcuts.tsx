import { createMemo, createSignal, For, Show } from "solid-js"
import { filterEntries } from "~/lib/commands/filter"
import { groupOf } from "~/lib/commands/groups"
import { commandList, run } from "~/lib/commands/registry"
import type { CommandInfo } from "~/lib/ipc/bindings"
import { Button, type Column, DataTable, IconButton, Kbd, TextField, useDialog } from "~/lib/ui"
import { ShortcutCapture } from "./shortcut-capture"

/** Every command with its shortcut, filterable and rebindable. */
export function ShortcutsSection() {
  const dialog = useDialog()
  const [query, setQuery] = createSignal("")
  const rows = createMemo(() =>
    filterEntries(
      commandList().map((c) => ({ ...c, words: c.keywords.join(" ") })),
      query(),
    ),
  )
  const rebound = () => commandList().filter((c) => c.rebound).length
  const change = (c: CommandInfo) =>
    dialog.show(() => <ShortcutCapture command={c} />, { title: `Shortcut for “${c.title}”`, size: "sm" })
  const columns: Column<CommandInfo>[] = [
    { key: "title", label: "Command", width: "minmax(12rem, 2fr)", cell: (c) => c.title },
    { key: "group", label: "Section", width: "9rem", cell: (c) => <span class="text-fg-muted">{groupOf(c.id)}</span> },
    {
      key: "keys",
      label: "Shortcut",
      width: "10rem",
      cell: (c) => (
        <span class="flex items-center gap-1" data-setting={c.id}>
          <For each={c.shortcuts} fallback={<span class="text-fg-faint">—</span>}>
            {(s) => <Kbd keys={s.label.split(/(?<=\S)\+(?=\S)| /)} />}
          </For>
        </span>
      ),
    },
    {
      key: "edit",
      label: "",
      width: "7rem",
      align: "end",
      cell: (c) => (
        <span class="flex items-center gap-1">
          <Show when={c.rebound}>
            <IconButton
              size="sm"
              icon="undo"
              label="Reset to default"
              onClick={() => void run("shortcuts.reset", c.id)}
            />
          </Show>
          <Button size="sm" variant="ghost" onClick={() => change(c)}>
            Change
          </Button>
        </span>
      ),
    },
  ]
  return (
    <>
      <div class="flex items-center gap-2">
        <TextField
          class="flex-1"
          icon="search"
          aria-label="Filter commands"
          placeholder={`Filter ${commandList().length} commands`}
          value={query()}
          onChange={setQuery}
        />
        <Button size="sm" disabled={rebound() === 0} onClick={() => void run("shortcuts.reset")}>
          Reset all
        </Button>
      </div>
      <DataTable aria-label="Commands" columns={columns} rows={rows()} rowKey={(c) => c.id} />
    </>
  )
}
