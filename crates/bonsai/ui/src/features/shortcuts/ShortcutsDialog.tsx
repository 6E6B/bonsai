import { createMemo, createSignal, For, onMount, Show } from "solid-js"
import { filterEntries } from "~/lib/commands/filter"
import { groupOf, groupOrder } from "~/lib/commands/groups"
import { commandList, run } from "~/lib/commands/registry"
import { Kbd, SectionHeader, TextField, TextLink, useDialog } from "~/lib/ui"

/** Every command that has a shortcut, grouped and searchable. Opened with `?`. Rebinding
 * lives in Settings → Shortcuts. */
export function ShortcutsDialog() {
  const dialog = useDialog()
  const [query, setQuery] = createSignal("")
  let input: HTMLInputElement | undefined
  onMount(() => input?.focus())

  const bound = createMemo(() =>
    commandList()
      .filter((c) => c.shortcuts.length > 0)
      .map((c) => ({ ...c, words: c.keywords.join(" ") })),
  )
  const sections = createMemo(() => {
    const shown = filterEntries(bound(), query())
    return groupOrder
      .map((g) => ({ name: g, commands: shown.filter((c) => groupOf(c.id) === g) }))
      .filter((s) => s.commands.length > 0)
  })

  return (
    <div class="flex max-h-[70vh] flex-col gap-3 p-4">
      <TextField
        ref={input}
        icon="search"
        aria-label="Filter shortcuts"
        placeholder="Filter"
        value={query()}
        onChange={setQuery}
      />
      <div class="grid min-h-0 grid-cols-2 gap-x-6 gap-y-4 overflow-y-auto">
        <For each={sections()}>
          {(s) => (
            <section class="flex flex-col">
              <SectionHeader title={s.name} size="group" />
              <For each={s.commands}>
                {(c) => (
                  <div class="flex h-7 items-center justify-between gap-3">
                    <span class="truncate">{c.title}</span>
                    <Kbd keys={c.shortcuts[0].label.split(/(?<=\S)\+(?=\S)| /)} />
                  </div>
                )}
              </For>
            </section>
          )}
        </For>
      </div>
      <Show when={sections().length === 0}>
        <p class="text-fg-faint">No shortcuts match.</p>
      </Show>
      <p class="text-sm text-fg-muted">
        Change them in{" "}
        <TextLink
          onClick={() => {
            dialog.close()
            void run("nav.settings", "shortcuts")
          }}
        >
          Settings → Shortcuts
        </TextLink>
        .
      </p>
    </div>
  )
}
