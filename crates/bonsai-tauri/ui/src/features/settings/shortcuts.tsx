import { createMemo, createSignal, For } from "solid-js"
import { filterEntries } from "~/lib/commands/filter"
import { groupOf } from "~/lib/commands/groups"
import { commandList } from "~/lib/commands/registry"
import type { CommandInfo } from "~/lib/ipc/bindings"
import { Button, type Column, DataTable, Kbd, TextField } from "~/lib/ui"

/** Every command with its shortcut, filterable. */
export function ShortcutsSection() {
  const [query, setQuery] = createSignal("")
  const rows = createMemo(() =>
    filterEntries(
      commandList().map((c) => ({ ...c, words: c.keywords.join(" ") })),
      query(),
    ),
  )
  const columns: Column<CommandInfo>[] = [
    { key: "title", label: "Command", width: "minmax(12rem, 2fr)", cell: (c) => c.title },
    { key: "group", label: "Section", width: "9rem", cell: (c) => <span class="text-fg-muted">{groupOf(c.id)}</span> },
    {
      key: "keys",
      label: "Shortcut",
      width: "9rem",
      cell: (c) => (
        <For each={c.shortcuts} fallback={<span class="text-fg-faint">—</span>}>
          {(s) => <Kbd keys={s.label.split(/(?<=\S)\+(?=\S)| /)} />}
        </For>
      ),
    },
    {
      key: "edit",
      label: "",
      width: "5rem",
      align: "end",
      cell: () => (
        <Button size="sm" variant="ghost" disabled>
          Change
        </Button>
      ),
    },
  ]
  return (
    <>
      <TextField
        icon="search"
        aria-label="Filter commands"
        placeholder={`Filter ${commandList().length} commands`}
        value={query()}
        onChange={setQuery}
      />
      <DataTable aria-label="Commands" columns={columns} rows={rows()} rowKey={(c) => c.id} />
    </>
  )
}
