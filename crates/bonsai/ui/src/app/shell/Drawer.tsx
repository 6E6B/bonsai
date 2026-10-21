import { createMemo, createSignal, For, Match, Show, Switch } from "solid-js"
import { run } from "~/lib/commands/registry"
import { ago } from "~/lib/format"
import { type ConversationRow, commands, type Friend, type Paged } from "~/lib/ipc/bindings"
import { joinUser, userMenu } from "~/lib/menus"
import { groupByPresence, type PresenceRow } from "~/lib/presence"
import { friends, shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Avatar,
  ContextMenu,
  EmptyState,
  IconButton,
  ListItem,
  SectionHeader,
  Skeleton,
  TabPanel,
  Tabs,
  Tooltip,
  VirtualList,
} from "~/lib/ui"

/** The right-hand panel: friends by presence, and recent chats. Ctrl+\ toggles it. */
export function Drawer() {
  const [tab, setTab] = createSignal("friends")
  const online = () => friends.friends.filter((f) => f.presence !== "offline").length
  const rows = createMemo(() => groupByPresence(friends.friends))

  return (
    <aside
      aria-label="Friends and chat"
      data-scroller
      class="flex w-72 shrink-0 flex-col border-l border-line bg-canvas"
    >
      <Tabs
        class="px-3 pt-1"
        aria-label="Drawer"
        value={tab()}
        onChange={setTab}
        items={[
          { value: "friends", label: "Friends" },
          { value: "chat", label: "Chat", count: shell.unread.chat },
        ]}
      >
        <TabPanel value="friends" class="flex flex-col gap-1">
          <SectionHeader
            title={`${online()} of ${friends.friends.length} online`}
            size="group"
            actions={<IconButton size="sm" icon="refresh" label="Refresh" onClick={() => run("view.refresh")} />}
          />
          <Switch>
            <Match when={friends.friends.length === 0 && friends.loading}>
              <For each={[0, 1, 2, 3, 4]}>{() => <Skeleton class="my-1.5 h-6" />}</For>
            </Match>
            <Match when={friends.friends.length === 0 && friends.error}>
              {(e) => <EmptyState icon="friends" title="Couldn't load friends" description={e()} />}
            </Match>
            <Match when={friends.friends.length === 0}>
              <EmptyState icon="friends" title="No friends yet" />
            </Match>
            <Match when={true}>
              <VirtualList class="-mx-2" items={rows()} estimateRowHeight={36} gap={0}>
                {(row) => <FriendRow row={row} />}
              </VirtualList>
            </Match>
          </Switch>
        </TabPanel>
        <TabPanel value="chat" class="flex flex-col gap-1">
          <SectionHeader
            title="Recent"
            size="group"
            actions={<IconButton size="sm" icon="plus" label="New chat" onClick={() => run("chat.new_group")} />}
          />
          <Show when={tab() === "chat"}>
            <div class="-mx-2 flex flex-col">
              <RecentChats />
            </div>
          </Show>
        </TabPanel>
      </Tabs>
    </aside>
  )
}

function RecentChats() {
  const res = useLive(
    () => shell.active,
    commands.chatList,
    (): Paged<ConversationRow> => ({
      list: { loading: true, error: null, items: [], more: false },
      moreKey: "",
    }),
  )
  return (
    <For each={res.list.items.slice(0, 20)} fallback={<EmptyState icon="chat" title="No chats yet" />}>
      {(c) => {
        const t = Date.parse(c.updated)
        return (
          <ListItem
            leading={
              <Avatar size="md" name={c.title} src={c.partnerId ? thumbUrl("headshot", c.partnerId) : undefined} />
            }
            title={c.title}
            subtitle={c.last}
            meta={Number.isNaN(t) ? undefined : ago(t)}
            unread={c.unread > 0}
            onClick={() => run("nav.chat")}
          />
        )
      }}
    </For>
  )
}

function FriendRow(props: { row: PresenceRow<Friend> }) {
  return (
    <Switch>
      <Match when={props.row.kind === "header" && props.row}>
        {(h) => <SectionHeader class="mt-2 px-2" title={h().label} meta={h().count} size="group" />}
      </Match>
      <Match when={props.row.kind === "user" && props.row.user}>
        {(f) => (
          <ContextMenu
            items={userMenu(f().id, { friend: true, inGame: f().presence === "game", placeId: f().placeId })}
          >
            <ListItem
              leading={<Avatar name={f().displayName} presence={f().presence} src={thumbUrl("headshot", f().id)} />}
              title={f().displayName}
              // The group header already says online or offline; only say where they are.
              subtitle={f().presence === "game" || f().presence === "studio" ? f().status : undefined}
              dim={f().presence === "offline"}
              onClick={() => run("user.open", String(f().id))}
              actionsOnHover
              actions={
                f().presence === "game" ? (
                  <>
                    <Show when={f().placeId}>
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
                    <Tooltip content="Join">
                      <IconButton
                        size="sm"
                        icon="play"
                        label="Join"
                        onClick={() => run("game.join", joinUser(f().id))}
                      />
                    </Tooltip>
                  </>
                ) : undefined
              }
            />
          </ContextMenu>
        )}
      </Match>
    </Switch>
  )
}
