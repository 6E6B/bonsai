import { createEffect, createMemo, createSignal, For, on, onCleanup, Show } from "solid-js"
import { useCommandContext, useListCursor } from "~/lib/commands/hooks"
import { handle, menuItem, run, runBackend } from "~/lib/commands/registry"
import { ExperienceCard } from "~/lib/experience-card"
import { ago } from "~/lib/format"
import { type ChatView, type ConversationRow, commands, type GameCard, type Paged } from "~/lib/ipc/bindings"
import { LinkedText } from "~/lib/linked-text"
import { experienceLinks, externalHosts, isOnlyExperienceLinks } from "~/lib/links"
import { emptyList, ListState } from "~/lib/list-state"
import { joinUser, presenceText } from "~/lib/menus"
import { friends, shell, useHome, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Avatar,
  Button,
  EmojiPicker,
  EmptyState,
  Icon,
  IconButton,
  ListItem,
  Menu,
  type MenuEntry,
  Pane,
  SectionHeader,
  Skeleton,
  Split,
  showToast,
  TextField,
  usePrompt,
} from "~/lib/ui"
import { dayLabel, filterConversations, startsDay } from "./timeline"

const ms = (iso: string) => {
  const t = Date.parse(iso)
  return Number.isNaN(t) ? null : t
}

/** The longest chat message Roblox takes. */
const MAX_CHARS = 400

/** Chat: conversations on the left, the open one on the right with a composer. Drafts stay
 * per conversation. */
export default function Chat() {
  const list = useLive(
    () => shell.active,
    commands.chatList,
    (): Paged<ConversationRow> => ({ list: emptyList<ConversationRow>(), moreKey: "" }),
  )
  const [openId, setOpenId] = createSignal<string | null>(null)
  const [cursor, setCursor] = createSignal(0)
  const [query, setQuery] = createSignal("")
  // The search narrows the list only; the open conversation stays open.
  const shown = createMemo(() => filterConversations(list.list.items, query()))
  createEffect(on(query, () => setCursor(0), { defer: true }))
  const focus = useLive(
    () => shell.active,
    async () => ({ id: await commands.takeChatFocus() }),
    () => ({ id: null as string | null }),
  )
  // A chat started elsewhere (Message on a profile, Reply) opens here once.
  createEffect(() => {
    if (focus.id) setOpenId(focus.id)
  })
  createEffect(
    on(
      () => shell.active,
      () => setOpenId(null),
      { defer: true },
    ),
  )
  const open = () => {
    const id = openId()
    return list.list.items.find((c) => c.id === id) ?? (id ? null : shown()[cursor()])
  }
  const current = () => openId() ?? shown()[cursor()]?.id ?? null
  useCommandContext(() => (current() ? `chat:${current()}` : null))
  useListCursor({
    count: () => shown().length,
    index: cursor,
    setIndex: (i) => {
      setCursor(i)
      setOpenId(shown()[i]?.id ?? null)
    },
  })
  // Opening a conversation marks it read.
  createEffect(
    on(current, (id) => {
      const c = list.list.items.find((x) => x.id === id)
      if (id && c && c.unread > 0) void commands.markChatRead(id)
    }),
  )

  const prompt = usePrompt()
  onCleanup(
    handle("chat.rename", async (name) => {
      const n =
        name ?? (await prompt({ title: "Rename chat", label: "Name", initial: open()?.title, submit: "Rename" }))
      if (n?.trim()) void runBackend("chat.rename", n)
    }),
  )
  onCleanup(
    handle("chat.add_people", async (who) => {
      const w =
        who ?? (await prompt({ title: "Add people", label: "Usernames or IDs, comma separated", submit: "Add" }))
      if (w?.trim()) void runBackend("chat.add_people", w)
    }),
  )

  return (
    <Split>
      <Pane side class="w-80">
        <SectionHeader
          title="Chat"
          size="page"
          actions={<IconButton icon="plus" label="New chat" onClick={() => run("chat.new_group")} />}
        />
        <TextField
          icon="search"
          aria-label="Search conversations"
          placeholder="Search by friend or group name"
          value={query()}
          onChange={setQuery}
        />
        <ListState list={list.list} icon="chat" empty="No conversations" more={list.moreKey}>
          <div data-cursor-list class="flex flex-col">
            <For each={shown()} fallback={<EmptyState icon="search" title="No conversations match" />}>
              {(c, i) => (
                <ListItem
                  data-index={i()}
                  selected={c.id === current()}
                  unread={c.unread > 0}
                  leading={<ConvAvatar c={c} />}
                  title={
                    <span class="flex items-center gap-1">
                      <Show when={c.pinned}>
                        <Icon name="pin-fill" size={12} class="text-fg-faint" />
                      </Show>
                      {c.title}
                    </span>
                  }
                  subtitle={c.last}
                  meta={ms(c.updated) ? ago(ms(c.updated) ?? 0) : undefined}
                  onClick={() => {
                    setCursor(i())
                    setOpenId(c.id)
                  }}
                />
              )}
            </For>
          </div>
        </ListState>
      </Pane>
      <Show when={current()} fallback={<EmptyState class="m-auto" icon="chat" title="No conversation open" />}>
        {(id) => <Conversation id={id()} row={open() ?? null} />}
      </Show>
    </Split>
  )
}

function ConvAvatar(props: { c: ConversationRow; size?: "sm" | "md" }) {
  // One-to-one chats with a friend show whether they are around.
  const friend = () => (props.c.partnerId ? friends.friends.find((f) => f.id === props.c.partnerId) : undefined)
  return (
    <Avatar
      size={props.size}
      presence={friend()?.presence}
      name={props.c.title}
      src={props.c.partnerId ? thumbUrl("headshot", props.c.partnerId) : undefined}
    />
  )
}

/** Drafts persist across restarts, per account and conversation. */
const draftKey = (chat: string) => `draft:${shell.active}:${chat}`
const drafts = {
  get: (chat: string) => localStorage.getItem(draftKey(chat)) ?? undefined,
  set: (chat: string, text: string) =>
    text ? localStorage.setItem(draftKey(chat), text) : localStorage.removeItem(draftKey(chat)),
}

function Conversation(props: { id: string; row: ConversationRow | null }) {
  const view = useLive(
    () => props.id,
    commands.chatView,
    (): ChatView => ({
      loading: true,
      error: null,
      info: null,
      lines: [],
      more: false,
      people: [],
      owner: false,
      typing: [],
      blocked: "",
      maxTitle: 150,
    }),
  )
  const [draft, setDraft] = createSignal("")
  createEffect(
    on(
      () => props.id,
      (id) => setDraft(drafts.get(id) ?? ""),
    ),
  )
  const edit = (v: string) => {
    setDraft(v)
    drafts.set(props.id, v)
    if (v.trim()) void commands.chatTyping(props.id)
  }
  const send = async () => {
    const text = draft().trim()
    if (!text) return
    edit("")
    await run("chat.send", text)
  }
  const info = () => view.info ?? props.row
  const group = () => info()?.group ?? false
  // Presence of the person in a one-to-one chat, when they are a friend.
  const friend = () => {
    const id = info()?.partnerId
    return id ? friends.friends.find((f) => f.id === id) : undefined
  }

  const [emojiOpen, setEmojiOpen] = createSignal(false)
  let input: HTMLInputElement | undefined
  /** Puts an emoji where the caret is, then returns focus to the composer. */
  const insert = (emoji: string) => {
    const v = draft()
    const from = input?.selectionStart ?? v.length
    const to = input?.selectionEnd ?? v.length
    const next = v.slice(0, from) + emoji + v.slice(to)
    if (next.length > MAX_CHARS) {
      showToast("That message is too long for an emoji.")
      return
    }
    edit(next)
    const caret = from + emoji.length
    requestAnimationFrame(() => {
      input?.focus()
      input?.setSelectionRange(caret, caret)
    })
  }
  const prompt = usePrompt()
  const writable = () => {
    if (view.blocked) showToast(view.blocked)
    return !view.blocked
  }
  onCleanup(handle("chat.emoji", () => writable() && setEmojiOpen(true)))
  onCleanup(
    handle("chat.share_experience", async (arg) => {
      if (!writable()) return
      const what =
        arg ?? (await prompt({ title: "Share an experience", label: "Experience link or ID", submit: "Share" }))
      if (what?.trim()) void runBackend("chat.share_experience", what)
    }),
  )

  // Recent and favorite experiences to share in one click.
  const home = useHome()
  const shareItems = (): MenuEntry[] => {
    const seen = new Set<number>()
    const rows = (games: GameCard[]) =>
      games
        .filter((g) => !seen.has(g.placeId) && seen.add(g.placeId))
        .slice(0, 6)
        .map((g) => menuItem("chat.share_experience", { arg: String(g.placeId), label: g.name, icon: "game" }))
    const recent = rows(home.recent)
    const favorites = rows(home.favorites)
    return [
      ...(recent.length > 0 ? [{ kind: "heading" as const, label: "Recent" }, ...recent] : []),
      ...(favorites.length > 0 ? [{ kind: "heading" as const, label: "Favorites" }, ...favorites] : []),
      ...(recent.length + favorites.length > 0 ? [{ kind: "separator" as const }] : []),
      menuItem("chat.share_experience", { icon: "link", label: "Link or ID…" }),
    ]
  }

  const items = (): MenuEntry[] => [
    menuItem("chat.pin", { icon: "pin", label: info()?.pinned ? "Unpin" : "Pin" }),
    ...(group()
      ? [
          menuItem("chat.rename", { icon: "edit" }),
          menuItem("chat.add_people", { icon: "user-plus" }),
          ...(view.owner && view.people.length > 0
            ? [
                {
                  kind: "submenu" as const,
                  label: "Remove someone",
                  icon: "user-minus" as const,
                  items: view.people.map((p) => menuItem("chat.remove_person", { arg: String(p.id), label: p.name })),
                },
              ]
            : []),
          { kind: "separator" as const },
          menuItem("chat.leave", { icon: "sign-out", danger: true }),
        ]
      : [
          ...view.people.map((p) =>
            menuItem("user.open", { arg: String(p.id), icon: "user", label: `Open ${p.name}` }),
          ),
        ]),
  ]

  return (
    <div class="flex min-w-0 flex-1 flex-col">
      <header class="flex h-12 shrink-0 items-center gap-3 border-b border-line px-5">
        <Show when={info()}>{(c) => <ConvAvatar c={c()} />}</Show>
        <div class="flex min-w-0 flex-col">
          <span class="truncate font-medium">{info()?.title ?? "…"}</span>
          <span class="truncate text-xs text-fg-muted">
            <Show
              when={group()}
              fallback={
                <Show
                  when={view.typing.length > 0}
                  fallback={
                    <Show when={friend()}>
                      {(f) =>
                        presenceText({
                          presence: f().presence,
                          place: f().presence === "game" ? f().status : undefined,
                        })
                      }
                    </Show>
                  }
                >
                  typing…
                </Show>
              }
            >
              {view.people.length + 1} people
            </Show>
          </span>
        </div>
        <div class="ml-auto flex items-center gap-1">
          <Show when={friend()?.presence === "game" && friend()}>
            {(f) => (
              <Button size="sm" variant="primary" icon="play" onClick={() => run("game.join", joinUser(f().id))}>
                Join
              </Button>
            )}
          </Show>
          <Menu icon="more" label="Conversation" placement="bottom-end" items={items()} />
        </div>
      </header>
      <div class="flex min-h-0 flex-1 flex-col-reverse overflow-y-auto px-5 py-4">
        <div class="flex flex-col gap-0.5">
          <Show when={view.more}>
            <Button
              class="self-center"
              variant="ghost"
              size="sm"
              disabled={view.loading}
              onClick={() => run("list.more", `chat:${props.id}`)}
            >
              Load older
            </Button>
          </Show>
          <Show when={view.lines.length > 0 || !view.loading} fallback={<Skeleton class="h-32 w-full rounded-md" />}>
            <Show when={view.error}>{(e) => <span class="text-fg-muted">{e()}</span>}</Show>
            <For each={view.lines}>
              {(l, i) => {
                const at = (j: number) => ms(view.lines[j]?.created ?? "")
                const newDay = () => startsDay(i() > 0 ? at(i() - 1) : null, at(i()))
                // Runs of lines from one sender share an avatar and a time, on the last line.
                const last = () => {
                  const next = view.lines[i() + 1]
                  return !next || next.senderId !== l.senderId || startsDay(at(i()), at(i() + 1))
                }
                const places = () => (l.moderated ? [] : experienceLinks(l.text))
                const hosts = () => (l.moderated ? [] : externalHosts(l.text))
                // A line that is only experience links shows just their cards.
                const bare = () => !l.moderated && isOnlyExperienceLinks(l.text)
                return (
                  <>
                    <Show when={newDay()}>
                      <h3 class="my-3 flex items-center gap-3 text-xs font-normal text-fg-muted">
                        <span aria-hidden="true" class="h-px flex-1 bg-line" />
                        {dayLabel(at(i()) ?? 0)}
                        <span aria-hidden="true" class="h-px flex-1 bg-line" />
                      </h3>
                    </Show>
                    <Show
                      when={!l.system}
                      fallback={<span class="my-1 self-center text-xs text-fg-muted">{l.text}</span>}
                    >
                      <div class="flex flex-col" classList={{ "items-end": l.mine, "mb-2": last() }}>
                        <div class="flex max-w-[70%] items-end gap-2">
                          <Show when={!l.mine}>
                            <span class="w-5 shrink-0">
                              <Show when={last()}>
                                <Avatar size="sm" name={l.sender} src={thumbUrl("headshot", l.senderId)} />
                              </Show>
                            </span>
                          </Show>
                          <div class="flex min-w-0 flex-col gap-1" classList={{ "items-end": l.mine }}>
                            <Show when={!bare()}>
                              <div
                                class="whitespace-pre-wrap break-words rounded-lg px-3 py-1.5"
                                classList={{
                                  "bg-accent text-on-accent": l.mine,
                                  "bg-surface": !l.mine,
                                  "opacity-60": l.pending,
                                  "italic text-fg-muted": l.moderated,
                                }}
                              >
                                <Show when={!l.moderated} fallback="Hidden by Roblox">
                                  <LinkedText text={l.text} tone={l.mine ? "inherit" : "accent"} />
                                </Show>
                              </div>
                            </Show>
                            <For each={places()}>{(id) => <ExperienceCard placeId={id} />}</For>
                            <Show when={hosts().length > 0}>
                              <span class="flex items-center gap-1 text-xs text-warning">
                                <Icon name="alert" size={12} />
                                Links to another site ({hosts().join(", ")}). It won't open from here.
                              </span>
                            </Show>
                          </div>
                        </div>
                        <Show when={last()}>
                          <span class="mt-0.5 text-xs text-fg-faint" classList={{ "pl-7": !l.mine }}>
                            <Show when={group() && !l.mine}>{l.sender} · </Show>
                            {l.pending ? "Sending…" : ms(l.created) ? ago(ms(l.created) ?? 0) : ""}
                          </span>
                        </Show>
                      </div>
                    </Show>
                  </>
                )
              }}
            </For>
          </Show>
          <Show when={view.typing.length > 0}>
            <span class="text-xs text-fg-muted">{view.typing.join(", ")} typing…</span>
          </Show>
        </div>
      </div>
      <Show
        when={!view.blocked}
        fallback={<p class="border-t border-line px-5 py-3 text-sm text-fg-muted">{view.blocked}</p>}
      >
        <form
          class="flex items-end gap-2 border-t border-line px-5 py-3"
          onSubmit={(e) => {
            e.preventDefault()
            void send()
          }}
        >
          <EmojiPicker label="Emoji" open={emojiOpen()} onOpenChange={setEmojiOpen} onPick={insert} />
          <Menu icon="share" label="Share experience" placement="top-start" items={shareItems()} />
          <TextField
            ref={(el: HTMLInputElement) => {
              input = el
            }}
            multiline
            class="flex-1"
            onKeyDown={(e) => {
              if (e.key === "Enter" && !e.shiftKey && !e.isComposing) {
                e.preventDefault()
                void send()
              }
            }}
            aria-label="Message"
            placeholder={`Message ${info()?.title ?? ""}`}
            maxLength={MAX_CHARS}
            value={draft()}
            onChange={edit}
          />
          <Button type="submit" variant="primary" icon="send" disabled={!draft().trim()}>
            Send
          </Button>
        </form>
      </Show>
    </div>
  )
}
