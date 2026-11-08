import { createMemo, createSignal, For, onCleanup, Show } from "solid-js"
import { useCommandContext, useListCursor, useSubTabs } from "~/lib/commands/hooks"
import { handle, run, runBackend } from "~/lib/commands/registry"
import { ExperienceCard } from "~/lib/experience-card"
import { ago, date } from "~/lib/format"
import { commands, type MessageRow, type Paged } from "~/lib/ipc/bindings"
import { LinkedText } from "~/lib/linked-text"
import { experienceLinks, externalHosts } from "~/lib/links"
import { emptyList, ListState } from "~/lib/list-state"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import {
  Alert,
  Avatar,
  Button,
  Divider,
  EmptyState,
  ListItem,
  Pane,
  Split,
  showToast,
  Tabs,
  TextField,
  VerifiedBadge,
  VirtualList,
} from "~/lib/ui"

const ms = (iso: string) => {
  const t = Date.parse(iso)
  return Number.isNaN(t) ? 0 : t
}

/** Private messages (the website's inbox, not chat). Triage with J / K, E to archive, U to
 * mark read or unread, Shift+R to reply; read on the right. Roblox no longer
 * takes new private messages, so Reply starts a chat. */
export default function Messages() {
  const [box, setBox] = createSignal("inbox")
  const [cursor, setCursor] = createSignal(0)
  const [query, setQuery] = createSignal("")
  const res = useLive(
    () => [shell.active, box()] as const,
    ([, b]) => commands.messages(b),
    (): Paged<MessageRow> => ({ list: emptyList<MessageRow>(), moreKey: "" }),
  )
  const sections = () => [
    { value: "inbox", label: "Inbox", count: shell.unread.messages },
    { value: "sent", label: "Sent" },
    { value: "news", label: "News" },
    { value: "archive", label: "Archive" },
  ]
  const shown = createMemo(() => {
    const q = query().trim().toLowerCase()
    return res.list.items.filter(
      (m) => !q || `${m.subject} ${m.sender} ${m.recipient} ${m.body}`.toLowerCase().includes(q),
    )
  })
  const open = () => shown()[cursor()]
  /** Only Inbox and Archive messages can be moved; Sent and News can't. */
  const movable = () => box() === "inbox" || box() === "archive"
  const pick = (v: string) => {
    setBox(v)
    setCursor(0)
  }
  useSubTabs(sections, box, pick)
  useCommandContext(() => `messages:${box()}`)
  useListCursor({
    count: () => shown().length,
    index: cursor,
    setIndex: setCursor,
  })

  // Keys act on the message under the cursor; buttons pass the message id.
  const act = (id: string, arg: string | null) => {
    const ids = arg ?? (open() ? String(open()?.id) : null)
    if (ids) void runBackend(id, ids)
    else showToast("Pick a message first.")
  }
  onCleanup(handle("message.archive", (arg) => act("message.archive", arg)))
  onCleanup(handle("message.toggle_read", (arg) => act("message.toggle_read", arg)))
  onCleanup(handle("message.reply", (arg) => act("message.reply", arg)))

  return (
    <Split>
      <Pane side class="w-88">
        <div class="flex items-center gap-2">
          <TextField
            class="min-w-0 flex-1"
            icon="search"
            aria-label="Search loaded messages"
            placeholder="Search messages"
            value={query()}
            onChange={setQuery}
          />
          <Button onClick={() => run("message.mark_all_read")}>Mark all read</Button>
        </div>
        <Tabs aria-label="Mailbox" items={sections()} value={box()} onChange={pick} />
        <ListState list={res.list} icon="inbox" empty="Nothing here" more={res.moreKey}>
          <div data-cursor-list class="flex flex-col">
            <VirtualList
              items={shown()}
              activeIndex={cursor()}
              estimateRowHeight={56}
              empty={<EmptyState icon="search" title="No messages match" />}
            >
              {(m, i) => (
                <ListItem
                  data-index={i}
                  selected={i === cursor()}
                  unread={!m.read}
                  leading={
                    <Avatar
                      name={box() === "sent" ? m.recipient : m.sender}
                      src={thumbUrl("headshot", box() === "sent" ? m.recipientId : m.senderId)}
                    />
                  }
                  title={m.subject}
                  subtitle={box() === "sent" ? `To ${m.recipient}` : m.sender}
                  meta={ago(ms(m.created))}
                  onClick={() => setCursor(i)}
                />
              )}
            </VirtualList>
          </div>
        </ListState>
      </Pane>
      <Pane>
        <Show when={open()} fallback={<EmptyState class="mt-16" icon="messages" title="No message open" />}>
          {(m) => {
            const hosts = () => externalHosts(m().body)
            return (
              <article class="flex max-w-3xl flex-col gap-3">
                <div class="flex flex-col gap-2">
                  <div class="flex flex-col">
                    <h1 class="text-xl font-medium">{m().subject}</h1>
                    <span class="flex items-center gap-1 text-sm text-fg-muted">
                      From{" "}
                      <Show when={m().senderVerified} fallback={m().sender}>
                        <span>{m().sender}</span>
                        <VerifiedBadge />
                      </Show>
                      <Show when={box() === "sent"}> to {m().recipient}</Show> · {date(ms(m().created))}
                    </span>
                  </div>
                  <div class="flex flex-wrap gap-1.5">
                    <Show when={!m().system && box() !== "sent"}>
                      <Button icon="chat" onClick={() => run("message.reply", String(m().id))}>
                        Reply in chat
                      </Button>
                    </Show>
                    <Button onClick={() => run("message.toggle_read", String(m().id))}>
                      {m().read ? "Mark unread" : "Mark read"}
                    </Button>
                    <Show when={movable()}>
                      <Button icon="archive" onClick={() => run("message.archive", String(m().id))}>
                        {box() === "archive" ? "Move to inbox" : "Archive"}
                      </Button>
                    </Show>
                  </div>
                </div>
                <Show when={hosts().length > 0}>
                  <Alert variant="warning" title={`This message links to another site: ${hosts().join(", ")}`}>
                    It isn't a Roblox site, so the app won't open it. Copy a link only if you trust the sender.
                  </Alert>
                </Show>
                <Divider />
                <p class="max-w-[70ch] whitespace-pre-line break-words">
                  <LinkedText text={m().body} />
                </p>
                <For each={experienceLinks(m().body)}>{(id) => <ExperienceCard placeId={id} />}</For>
              </article>
            )
          }}
        </Show>
      </Pane>
    </Split>
  )
}
