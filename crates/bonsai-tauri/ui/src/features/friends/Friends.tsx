import { createMemo, createSignal, Match, Show, Switch } from "solid-js"
import { useCommandContext, useListCursor, useSubTabs } from "~/lib/commands/hooks"
import { run } from "~/lib/commands/registry"
import { commands, type Person, type Presence } from "~/lib/ipc/bindings"
import { emptyList } from "~/lib/list-state"
import { joinUser, presenceText, userMenu } from "~/lib/menus"
import { friends, shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Avatar,
  Button,
  ContextMenu,
  EmptyState,
  IconButton,
  ListItem,
  Page,
  TabPanel,
  Tabs,
  TextField,
  Tooltip,
  usePrompt,
  VirtualGrid,
} from "~/lib/ui"

interface Row {
  id: number
  name: string
  displayName: string
  presence: Presence
  /** Experience name when in game. */
  place?: string
  placeId?: number
  /** Second line for people lists: mutual friends. */
  note?: string
  /** What the active account calls them. Shown as the title, and matched by the filter. */
  nickname?: string
  /** Whether the active account follows them, on Followers. */
  following?: boolean
}

const fromPerson = (p: Person): Row => ({
  id: p.id,
  name: p.name,
  displayName: p.displayName || p.name || `User ${p.id}`,
  presence: "offline",
  note: p.mutuals.length > 0 ? `${p.mutuals.length} mutual: ${p.mutuals.slice(0, 3).join(", ")}` : undefined,
  following: p.following ?? undefined,
})

/** Friends, requests, followers, following and blocked users, with a filter and keyboard
 * cursor (J / K / Enter). */
export default function Friends() {
  const [section, setSection] = createSignal<string>("friends")
  const [query, setQuery] = createSignal("")
  const [cursor, setCursor] = createSignal(0)
  const prompt = usePrompt()
  useCommandContext(() => (shell.active === null ? "" : `user:${shell.active}`))
  const people = useLive(
    () => (section() === "friends" ? "requests" : section()),
    (which) => commands.people(which),
    emptyList<Person>,
  )
  const requests = useLive(() => "requests", commands.people, emptyList<Person>)

  const sections = () => [
    { value: "friends", label: "Friends" },
    { value: "requests", label: "Requests", count: requests.items.length || undefined },
    { value: "followers", label: "Followers" },
    { value: "following", label: "Following" },
    { value: "blocked", label: "Blocked" },
  ]
  useSubTabs(sections, section, (v) => {
    setSection(v)
    setCursor(0)
  })

  const shown = createMemo(() => {
    const q = query().trim().toLowerCase()
    const list: Row[] =
      section() === "friends"
        ? friends.friends.map((f) => ({
            ...f,
            place: f.presence === "game" ? f.status : undefined,
            placeId: f.placeId ?? undefined,
            nickname: f.nickname ?? undefined,
          }))
        : people.items.map(fromPerson)
    return q ? list.filter((f) => `${f.name} ${f.displayName} ${f.nickname ?? ""}`.toLowerCase().includes(q)) : list
  })

  useListCursor({
    count: () => shown().length,
    index: cursor,
    setIndex: setCursor,
    onOpen: (i) => {
      const f = shown()[i]
      if (f) void run("user.open", String(f.id))
    },
  })

  const actions = (f: Row) => {
    const id = String(f.id)
    return (
      <Switch>
        <Match when={section() === "requests"}>
          <Button size="sm" onClick={() => run("user.accept_friend", id)}>
            Accept
          </Button>
          <Button size="sm" onClick={() => run("user.decline_friend", id)}>
            Decline
          </Button>
        </Match>
        <Match when={section() === "followers" && f.following !== undefined}>
          <Button
            size="sm"
            variant={f.following ? "ghost" : "secondary"}
            onClick={() => run(f.following ? "user.unfollow" : "user.follow", id)}
          >
            {f.following ? "Following" : "Follow back"}
          </Button>
        </Match>
        <Match when={section() === "blocked"}>
          <Button size="sm" onClick={() => run("user.unblock", id)}>
            Unblock
          </Button>
        </Match>
        <Match when={section() === "friends"}>
          <Tooltip content="Message">
            <IconButton size="sm" icon="chat" label="Message" onClick={() => run("user.message", id)} />
          </Tooltip>
          <Show when={f.presence === "game" && f.placeId}>
            {(placeId) => (
              <Tooltip content="Open experience">
                <IconButton
                  size="sm"
                  icon="game"
                  label="Open experience"
                  onClick={() => run("game.open", String(placeId()))}
                />
              </Tooltip>
            )}
          </Show>
          <Show when={f.presence === "game"}>
            <Button size="sm" icon="play" onClick={() => run("game.join", joinUser(f.id))}>
              Join
            </Button>
          </Show>
        </Match>
      </Switch>
    )
  }

  return (
    <Page
      title="Friends"
      width="narrow"
      actions={
        <Button
          icon="user-plus"
          onClick={async () => {
            const user = await prompt({ title: "Add friend", label: "Username or ID", submit: "Send request" })
            if (user) void run("user.add_friend", user)
          }}
        >
          Add friend
        </Button>
      }
    >
      <Tabs aria-label="Friends sections" items={sections()} value={section()} onChange={setSection}>
        <div class="flex items-center gap-2 pt-3">
          <TextField
            class="flex-1"
            icon="search"
            aria-label="Filter"
            placeholder="Filter by name"
            value={query()}
            onChange={setQuery}
          />
          <Switch>
            <Match when={section() === "requests" && requests.items.length > 0}>
              <Button onClick={() => run("friends.decline_all")}>Ignore all</Button>
            </Match>
          </Switch>
        </div>
        <TabPanel value={section()}>
          <div data-cursor-list>
            <VirtualGrid
              items={shown()}
              estimateRowHeight={44}
              gap={0}
              empty={
                <Show
                  when={section() === "friends" || !people.loading}
                  fallback={<EmptyState icon="friends" title="Loading…" />}
                >
                  <Show
                    when={section() === "friends" || !people.error}
                    fallback={<EmptyState icon="friends" title="Couldn't load this" description={people.error ?? ""} />}
                  >
                    <EmptyState icon="friends" title={query() ? "No one matches" : "Nobody here yet"} />
                  </Show>
                </Show>
              }
            >
              {(f, i) => (
                <ContextMenu
                  items={userMenu(f.id, {
                    friend: section() === "friends",
                    inGame: f.presence === "game",
                    placeId: f.placeId,
                    blocked: section() === "blocked",
                    following: f.following,
                  })}
                >
                  <ListItem
                    data-index={i}
                    selected={i === cursor()}
                    leading={<Avatar name={f.displayName} presence={f.presence} src={thumbUrl("headshot", f.id)} />}
                    title={f.nickname || f.displayName}
                    subtitle={[
                      f.nickname && f.displayName,
                      f.name && `@${f.name}`,
                      section() === "friends" ? presenceText(f) : f.note,
                    ]
                      .filter(Boolean)
                      .join(" · ")}
                    dim={f.presence === "offline" && section() === "friends"}
                    onClick={() => {
                      setCursor(i)
                      void run("user.open", String(f.id))
                    }}
                    actions={actions(f)}
                  />
                </ContextMenu>
              )}
            </VirtualGrid>
          </div>
          <Show when={section() !== "friends" && people.more}>
            <Button class="mt-2" variant="ghost" disabled={people.loading} onClick={() => run("list.more", section())}>
              Load more
            </Button>
          </Show>
        </TabPanel>
      </Tabs>
    </Page>
  )
}
