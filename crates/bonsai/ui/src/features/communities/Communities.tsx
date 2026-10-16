import { useNavigate, useSearchParams } from "@solidjs/router"
import { createMemo, createSignal, For, onCleanup, Show } from "solid-js"
import { useListCursor } from "~/lib/commands/hooks"
import { handle, menuItem, run } from "~/lib/commands/registry"
import { compact } from "~/lib/format"
import { type CommunityHit, commands, type GroupCard } from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { shareMenu } from "~/lib/menus"
import { useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Badge,
  Button,
  Checkbox,
  ContextMenu,
  EmptyState,
  ListItem,
  Page,
  TextField,
  Thumbnail,
  VerifiedBadge,
} from "~/lib/ui"

/** Your communities, with search and bulk leave for cleanup. Checkboxes show only while
 * selecting, which the Select button or X on a row starts. With `?q=`, shows search results
 * across all communities instead. */
export default function Communities() {
  const [params] = useSearchParams()
  const navigate = useNavigate()
  const searched = () => (typeof params.q === "string" ? params.q : "")
  const mine = useLive(() => 0, commands.myCommunities, emptyList<GroupCard>)
  const [query, setQuery] = createSignal("")
  const [picked, setPicked] = createSignal<Set<number>>(new Set())
  const [selecting, setSelecting] = createSignal(false)
  const [cursor, setCursor] = createSignal(0)
  const shown = createMemo(() => {
    const q = query().trim().toLowerCase()
    return mine.items.filter((c) => !q || c.name.toLowerCase().includes(q))
  })
  const toggle = (id: number, on: boolean) =>
    setPicked((s) => {
      const next = new Set(s)
      if (on) next.add(id)
      else next.delete(id)
      return next
    })
  const stopSelecting = () => {
    setSelecting(false)
    setPicked(new Set<number>())
  }

  useListCursor({
    count: () => (searched() ? 0 : shown().length),
    index: cursor,
    setIndex: setCursor,
    onOpen: (i) => {
      const c = shown()[i]
      if (c) void run("group.open", String(c.id))
    },
    onSelect: (i) => {
      const c = shown()[i]
      if (!c) return
      setSelecting(true)
      toggle(c.id, !picked().has(c.id))
    },
  })

  // The bulk command leaves whatever is ticked here; the backend's `group.leave` takes the list.
  onCleanup(
    handle("group.leave_selected", async () => {
      const ids = [...picked()]
      if (ids.length === 0) return
      await run("group.leave", ids.join(","))
      stopSelecting()
    }),
  )

  const search = () => {
    const q = query().trim()
    if (q) void run("group.search", q)
  }

  return (
    <Page
      title="Communities"
      meta={mine.items.length || undefined}
      width="narrow"
      actions={
        <Show
          when={selecting()}
          fallback={
            <>
              <Button onClick={() => setSelecting(true)}>Select</Button>
              <Button icon="search" onClick={search}>
                Find communities
              </Button>
            </>
          }
        >
          <span class="text-fg-muted">{picked().size} selected</span>
          <Button variant="danger" disabled={picked().size === 0} onClick={() => run("group.leave_selected")}>
            Leave
          </Button>
          <Button onClick={stopSelecting}>Done</Button>
        </Show>
      }
    >
      <form
        onSubmit={(e) => {
          e.preventDefault()
          search()
        }}
      >
        <TextField
          icon="search"
          aria-label="Filter communities"
          placeholder="Filter, or press Enter to search all communities"
          value={query()}
          onChange={setQuery}
        />
      </form>
      <Show when={searched()} fallback={<MyList />}>
        {(q) => <Results query={q()} onClear={() => navigate("/communities")} />}
      </Show>
    </Page>
  )

  function MyList() {
    return (
      <ListState list={mine} icon="communities" empty="Not in any communities">
        <div data-cursor-list class="flex flex-col">
          <For each={shown()} fallback={<EmptyState icon="communities" title="No communities match" />}>
            {(c, i) => (
              <ContextMenu
                items={[
                  menuItem("group.open", { arg: String(c.id), icon: "communities" }),
                  menuItem("group.primary", {
                    arg: String(c.id),
                    icon: "star",
                    label: c.primary ? "Clear primary" : "Make primary",
                  }),
                  shareMenu(`group:${c.id}`),
                  { kind: "separator" },
                  menuItem("group.leave", { arg: String(c.id), icon: "sign-out", label: "Leave", danger: true }),
                ]}
              >
                <ListItem
                  data-index={i()}
                  selected={i() === cursor()}
                  leading={<Thumbnail class="w-8" alt="" src={thumbUrl("group_icon", c.id)} fallback="communities" />}
                  title={
                    <Show when={c.verified} fallback={c.name}>
                      <span class="text-verified">{c.name}</span> <VerifiedBadge />
                    </Show>
                  }
                  subtitle={`${compact(c.members)} members · ${c.role}`}
                  meta={
                    <Show when={c.primary}>
                      <Badge variant="accent">Primary</Badge>
                    </Show>
                  }
                  actions={
                    <Show when={selecting()}>
                      <Checkbox hideLabel checked={picked().has(c.id)} onChange={(on) => toggle(c.id, on)}>
                        Select {c.name}
                      </Checkbox>
                    </Show>
                  }
                  onClick={() => {
                    setCursor(i())
                    if (selecting()) toggle(c.id, !picked().has(c.id))
                    else void run("group.open", String(c.id))
                  }}
                />
              </ContextMenu>
            )}
          </For>
        </div>
      </ListState>
    )
  }
}

/** Search results across all communities. */
function Results(props: { query: string; onClear: () => void }) {
  const list = useLive(() => props.query, commands.communitySearch, emptyList<CommunityHit>)
  return (
    <div class="flex flex-col gap-2">
      <div class="flex items-center justify-between gap-2">
        <span class="text-fg-muted">Results for “{props.query}”</span>
        <Button variant="ghost" icon="x" onClick={props.onClear}>
          Your communities
        </Button>
      </div>
      <ListState list={list} icon="search" empty="No communities found">
        <div class="flex flex-col">
          <For each={list.items}>
            {(c) => (
              <ContextMenu
                items={[menuItem("group.open", { arg: String(c.id), icon: "communities" }), shareMenu(`group:${c.id}`)]}
              >
                <ListItem
                  leading={<Thumbnail class="w-8" alt="" src={thumbUrl("group_icon", c.id)} fallback="communities" />}
                  title={
                    <Show when={c.verified} fallback={c.name}>
                      <span class="text-verified">{c.name}</span> <VerifiedBadge />
                    </Show>
                  }
                  subtitle={`${compact(c.members)} members${c.publicEntry ? "" : " · Approval needed"}`}
                  onClick={() => run("group.open", String(c.id))}
                />
              </ContextMenu>
            )}
          </For>
        </div>
      </ListState>
    </div>
  )
}
