import { createMemo, createSignal, For, Show } from "solid-js"
import { commands } from "~/lib/ipc/bindings"
import { friends } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Button, Checkbox, ListItem, showToast, TextField } from "~/lib/ui"

/** Pick friends to chat with; more than one makes a group, which can have a name. Opened by
 * `chat.new_group`. */
export function NewChat(props: { onDone: (started: boolean) => void }) {
  const [query, setQuery] = createSignal("")
  const [picked, setPicked] = createSignal<number[]>([])
  const [name, setName] = createSignal("")
  const [busy, setBusy] = createSignal(false)
  const shown = createMemo(() => {
    const q = query().trim().toLowerCase()
    return friends.friends
      .filter((f) => !q || f.displayName.toLowerCase().includes(q) || f.name.toLowerCase().includes(q))
      .slice(0, 100)
  })
  const toggle = (id: number) => setPicked((p) => (p.includes(id) ? p.filter((x) => x !== id) : [...p, id]))
  const start = async () => {
    setBusy(true)
    const r = await commands.startChat(picked(), picked().length > 1 && name().trim() ? name().trim() : null)
    setBusy(false)
    if (r.status === "error") return showToast(r.error)
    props.onDone(true)
  }
  return (
    <form
      class="flex flex-col gap-3 p-4"
      onSubmit={(e) => {
        e.preventDefault()
        if (picked().length > 0) void start()
      }}
    >
      <TextField
        icon="search"
        aria-label="Find friends"
        placeholder="Find friends"
        value={query()}
        onChange={setQuery}
        autofocus
      />
      <div class="flex max-h-80 flex-col overflow-y-auto">
        <For each={shown()} fallback={<span class="p-2 text-sm text-fg-muted">No friends match.</span>}>
          {(f) => (
            <ListItem
              leading={
                <span class="flex items-center gap-3">
                  <Checkbox hideLabel checked={picked().includes(f.id)} onChange={() => toggle(f.id)}>
                    Pick {f.displayName}
                  </Checkbox>
                  <Avatar name={f.displayName} presence={f.presence} src={thumbUrl("headshot", f.id)} />
                </span>
              }
              title={f.displayName}
              subtitle={`@${f.name}`}
              onClick={() => toggle(f.id)}
            />
          )}
        </For>
      </div>
      <Show when={picked().length > 1}>
        <TextField label="Group name (optional)" value={name()} onChange={setName} />
      </Show>
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onDone(false)}>
          Cancel
        </Button>
        <Button variant="primary" type="submit" disabled={busy() || picked().length === 0}>
          <Show when={picked().length > 1} fallback="Chat">
            Start group of {picked().length + 1}
          </Show>
        </Button>
      </div>
    </form>
  )
}
