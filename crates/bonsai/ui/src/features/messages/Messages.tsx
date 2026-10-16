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
  Checkbox,
  Divider,
  EmptyState,
  ListItem,
  Pane,
  SectionHeader,
  Split,
  showToast,
  Tabs,
  TextField,
  VerifiedBadge,
} from "~/lib/ui"

const ms = (iso: string) => {
  const t = Date.parse(iso)
  return Number.isNaN(t) ? 0 : t
}

/** Private messages (the website's inbox, not chat). Triage with J / K, X to select, E to
 * archive, U to mark read or unread, Shift+R to reply; read on the right. Roblox no longer
 * takes new private messages, so Reply starts a chat. */
export default function Messages() {
  const [box, setBox] = createSignal("inbox")
  const [cursor, setCursor] = createSignal(0)
  const [query, setQuery] = createSignal("")
  const [selecting, setSelecting] = createSignal(false)
  const [picked, setPicked] = createSignal<ReadonlySet<number>>(new Set())
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

  // Ticked messages that are still on screen (a search can hide some).
  const ticked = createMemo(() => shown().filter((m) => picked().has(m.id)))
  const allTicked = () => shown().length > 0 && ticked().length === shown().length
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
  const pick = (v: string) => {
    setBox(v)
    setCursor(0)
    stopSelecting()
  }
  useSubTabs(sections, box, pick)
  useCommandContext(() => `messages:${box()}`)
  useListCursor({
    count: () => shown().length,
    index: cursor,
    setIndex: setCursor,
    onSelect: (i) => {
      const m = shown()[i]
      if (!m) return
      setSelecting(true)
      toggle(m.id, !picked().has(m.id))
    },
  })

  // Keys and buttons act on the ticked messages, or on the one under the cursor when none are.
  const targets = () => {
    const t = ticked()
    return t.length > 0 ? t : open() ? [open() as MessageRow] : []
  }
  const act = async (id: string, arg: string | null, done?: () => void) => {
    const ids =
      arg ??
      targets()
        .map((m) => m.id)
        .join(",")
    if (!ids) {
      showToast("Pick a message first.")
      return
    }
    if (await runBackend(id, ids)) done?.()
  }
  onCleanup(handle("message.archive", (arg) => act("message.archive", arg, stopSelecting)))
  onCleanup(handle("message.toggle_read", (arg) => act("message.toggle_read", arg)))
  onCleanup(
    handle("message.reply", (arg) => {
      const id = arg ?? (open() ? String(open()?.id) : null)
      if (id) void runBackend("message.reply", id)
      else showToast("Pick a message first.")
    }),
  )
  onCleanup(
    handle("message.select_all", () => {
      setSelecting(true)
      setPicked(allTicked() ? new Set<number>() : new Set(shown().map((m) => m.id)))
    }),
  )

  return (
    <Split>
      <Pane side class="w-88">
        <SectionHeader
          title="Messages"
          size="page"
          actions={
            <>
              <Button size="sm" onClick={() => (selecting() ? stopSelecting() : setSelecting(true))}>
                {selecting() ? "Done" : "Select"}
              </Button>
              <Button size="sm" onClick={() => run("message.mark_all_read")}>
                Mark all read
              </Button>
            </>
          }
        />
        <Tabs aria-label="Mailbox" items={sections()} value={box()} onChange={pick} />
        <TextField
          icon="search"
          aria-label="Search messages"
          placeholder="Search loaded messages"
          value={query()}
          onChange={setQuery}
        />
        <Show when={selecting()}>
          <div class="flex flex-wrap items-center gap-2 px-2 py-1">
            <Checkbox checked={allTicked()} onChange={() => run("message.select_all")}>
              {ticked().length > 0 ? `${ticked().length} selected` : "Select all"}
            </Checkbox>
            <div class="ml-auto flex gap-1.5">
              <Show when={movable()}>
                <Button size="sm" disabled={ticked().length === 0} onClick={() => run("message.archive")}>
                  {box() === "archive" ? "Move to inbox" : "Archive"}
                </Button>
              </Show>
              <Button size="sm" disabled={ticked().length === 0} onClick={() => run("message.toggle_read")}>
                {ticked().some((m) => !m.read) ? "Mark read" : "Mark unread"}
              </Button>
            </div>
          </div>
        </Show>
        <ListState list={res.list} icon="inbox" empty="Nothing here" more={res.moreKey}>
          <div data-cursor-list class="flex flex-col">
            <For each={shown()} fallback={<EmptyState icon="search" title="No messages match" />}>
              {(m, i) => (
                <ListItem
                  data-index={i()}
                  selected={i() === cursor()}
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
                  actions={
                    <Show when={selecting()}>
                      <Checkbox hideLabel checked={picked().has(m.id)} onChange={(on) => toggle(m.id, on)}>
                        Select {m.subject}
                      </Checkbox>
                    </Show>
                  }
                  onClick={() => setCursor(i())}
                />
              )}
            </For>
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
                        <span class="text-verified">{m().sender}</span>
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
                <p class="whitespace-pre-line break-words">
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
