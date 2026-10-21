import { useParams } from "@solidjs/router"
import { createEffect, createMemo, createSignal, For, on, onCleanup, Show } from "solid-js"
import { useCommandContext, useSubTabs } from "~/lib/commands/hooks"
import { handle, menuItem, run, runBackend } from "~/lib/commands/registry"
import { ago, compact } from "~/lib/format"
import {
  type AffiliatesView,
  type AuditRow,
  type CommunityView,
  commands,
  type ForumCategoryRow,
  type ForumCommentRow,
  type ForumPostRow,
  type GameCard,
  type JoinRequestRow,
  type MemberRow,
  type RoleRow,
  type SettingRow,
} from "~/lib/ipc/bindings"
import { Description, LinkedText } from "~/lib/linked-text"
import { emptyList, ListState } from "~/lib/list-state"
import { entityMenu } from "~/lib/menus"
import { useLive } from "~/lib/store"
import { setTabTitle } from "~/lib/tabs"
import { thumbUrl } from "~/lib/thumb"
import { GameTile } from "~/lib/tiles"
import {
  Avatar,
  Badge,
  Button,
  Card,
  Checkbox,
  Clamp,
  type Column,
  DataTable,
  Icon,
  IconButton,
  ListItem,
  Menu,
  Page,
  Select,
  Skeleton,
  Stat,
  Switch,
  showToast,
  TabPanel,
  Tabs,
  TextField,
  TextLink,
  Thumbnail,
  useDialog,
  usePrompt,
  VerifiedBadge,
  VirtualGrid,
  VirtualList,
} from "~/lib/ui"
import { Affiliates } from "./Affiliates"
import { PayoutForm, Revenue, SplitForm } from "./Revenue"
import { Store } from "./Store"

const ms = (iso: string) => {
  const t = Date.parse(iso)
  return Number.isNaN(t) ? Date.now() : t
}

const noAffiliates = (): AffiliatesView => ({ loading: true, error: null, allies: [], enemies: [] })

/** The forum post open in the Forum tab. */
interface OpenPost {
  category: string
  id: string
  title: string
}

/** A community: header, shout, then about, games, store, forum, members, allies and enemies
 * and, with the right permissions, join requests, audit log, revenue and settings. */
export default function Community() {
  const params = useParams()
  const groupId = () => Number(params.id)
  const view = useLive(
    groupId,
    commands.community,
    (): CommunityView => ({ loading: true, error: null, info: null, me: null, alts: [] }),
  )
  const [post, setPost] = createSignal<OpenPost | null>(null)
  createEffect(on(groupId, () => setPost(null), { defer: true }))
  useCommandContext(() => {
    const p = post()
    return p ? `group:${groupId()}/forum/${p.category}/${p.id}` : `group:${groupId()}`
  })
  createEffect(() => {
    const name = view.info?.name
    if (name) setTabTitle(`/communities/${params.id}`, name)
  })

  const me = () => view.me
  const member = () => Boolean(me()?.role)
  const [section, setSection] = createSignal("about")
  const sections = () => [
    { value: "about", label: "About" },
    { value: "games", label: "Games" },
    { value: "store", label: "Store" },
    { value: "forum", label: "Forum", disabled: !me()?.viewForums },
    { value: "members", label: "Members", disabled: me() !== null && !me()?.canViewMembers },
    { value: "allies", label: "Allies" },
    { value: "enemies", label: "Enemies" },
    { value: "requests", label: "Join requests", disabled: !me()?.manageRequests },
    { value: "audit", label: "Audit log", disabled: !me()?.viewAuditLog },
    { value: "revenue", label: "Revenue", disabled: !me()?.viewPayouts && !me()?.spendFunds },
    { value: "settings", label: "Settings", disabled: !me()?.manageSettings },
  ]
  useSubTabs(sections, section, setSection)
  const affiliates = useLive(
    () => (section() === "allies" || section() === "enemies" ? groupId() : 0),
    (g) => (g ? commands.communityAffiliates(g) : Promise.resolve(noAffiliates())),
    noAffiliates,
  )

  const prompt = usePrompt()
  const dialog = useDialog()
  const id = () => String(groupId())
  const newShout = async () => {
    const text = await prompt({
      title: "Post a shout",
      label: "Shout",
      placeholder: "Leave empty to clear it",
      submit: "Post",
    })
    if (text !== null) void run("group.shout", text)
  }

  // With an argument these run at once (after confirming); without one they ask first.
  onCleanup(
    handle("group.payout", (arg) => {
      if (arg?.trim()) return void runBackend("group.payout", arg)
      if (!me()?.spendFunds) return showToast("You can't spend this community's funds.")
      dialog.show(() => <PayoutForm onDone={dialog.close} />, { title: "Pay out Robux", size: "sm" })
    }),
  )
  onCleanup(
    handle("group.recurring_payouts", async (arg) => {
      if (arg?.trim()) return void runBackend("group.recurring_payouts", arg)
      if (!me()?.spendFunds) return showToast("You can't change this community's payouts.")
      const current = (await commands.communityRevenue(groupId())).split.items
      dialog.show(() => <SplitForm current={current} onDone={dialog.close} />, {
        title: "Recurring payouts",
        description: "Shares of the community's income, paid out as it comes in.",
        size: "md",
      })
    }),
  )
  for (const [cmd, title, submit] of [
    ["group.ally", "Send an ally request", "Send"],
    ["group.enemy", "Declare an enemy", "Declare"],
  ] as const) {
    onCleanup(
      handle(cmd, async (arg) => {
        if (arg?.trim()) return void runBackend(cmd, arg)
        const other = await prompt({ title, label: "Community ID or link", submit })
        if (other) void run(cmd, other)
      }),
    )
  }

  onCleanup(
    handle("group.new_post", async (category) => {
      if (!me()?.createPosts) return showToast("You can't post in this community's forum.")
      const cats = (await commands.forumCategories(groupId())).items
      if (cats.length === 0) return showToast("This forum has no categories.")
      const initial = cats.find((c) => c.id === category)?.id ?? cats[0].id
      dialog.show(() => <NewPostForm groupId={groupId()} categories={cats} initial={initial} onDone={dialog.close} />, {
        title: "New forum post",
        size: "md",
      })
    }),
  )

  return (
    <Page>
      <Show
        when={view.info}
        fallback={
          <Show when={view.error} fallback={<Skeleton class="h-24 w-full rounded-md" />}>
            {(e) => <span class="text-fg-muted">{e()}</span>}
          </Show>
        }
      >
        {(g) => (
          <header class="flex flex-wrap items-center gap-4">
            <Thumbnail class="w-20" alt={g().name} src={thumbUrl("group_icon", groupId())} fallback="communities" />
            <div class="flex min-w-0 flex-1 flex-col gap-1">
              <h1 class="flex items-center gap-2 truncate text-2xl font-medium">
                <span class={g().verified ? "truncate text-verified" : "truncate"}>{g().name}</span>
                <Show when={g().verified}>
                  <VerifiedBadge size={18} />
                </Show>
                <Show when={g().locked}>
                  <Badge variant="danger">Locked</Badge>
                </Show>
              </h1>
              <span class="text-fg-muted">
                <Show when={g().ownerId} fallback="No owner">
                  by{" "}
                  <TextLink
                    tone={g().ownerVerified ? "verified" : "name"}
                    onClick={() => run("user.open", String(g().ownerId))}
                  >
                    {g().owner}
                  </TextLink>
                  <Show when={g().ownerVerified}>
                    {" "}
                    <VerifiedBadge />
                  </Show>
                </Show>{" "}
                · {compact(g().members)} members
                <Show when={me()?.role}>{(r) => <> · You're {r()}</>}</Show>
                <Show when={view.alts.length > 0}> · {view.alts.join(", ")}</Show>
              </span>
              <div class="mt-1 flex flex-wrap gap-1.5">
                <Show
                  when={member()}
                  fallback={
                    <Show
                      when={!me()?.pending}
                      fallback={
                        <Button disabled icon="clock">
                          Requested
                        </Button>
                      }
                    >
                      <Button variant="primary" disabled={!me()} onClick={() => run("group.join", id())}>
                        {g().publicEntry ? "Join" : "Ask to join"}
                      </Button>
                    </Show>
                  }
                >
                  <Button onClick={() => run("group.primary", id())} icon={me()?.primary ? "star-fill" : "star"}>
                    {me()?.primary ? "Primary" : "Make primary"}
                  </Button>
                </Show>
                <Menu
                  icon="more"
                  label="More"
                  items={[
                    ...entityMenu(`group:${id()}`),
                    ...(member() && !me()?.owner
                      ? [
                          { kind: "separator" as const },
                          menuItem("group.leave", { arg: id(), icon: "sign-out", danger: true }),
                        ]
                      : []),
                  ]}
                />
              </div>
            </div>
          </header>
        )}
      </Show>

      <Show when={view.info?.shout || me()?.postShout}>
        <Card class="flex items-start gap-3 p-3">
          <Show
            when={view.info?.shout}
            fallback={
              <>
                <Icon name="megaphone" class="mt-0.5 text-fg-muted" />
                <p class="flex-1 text-fg-muted">No shout</p>
              </>
            }
          >
            {(s) => (
              <>
                <Avatar size="sm" name={s().poster} src={thumbUrl("headshot", s().posterId)} />
                <div class="flex min-w-0 flex-1 flex-col gap-0.5">
                  <div class="flex flex-wrap items-center gap-1.5 text-sm text-fg-muted">
                    <TextLink
                      tone={s().posterVerified ? "verified" : "name"}
                      onClick={() => run("user.open", String(s().posterId))}
                    >
                      {s().poster}
                    </TextLink>
                    <Show when={s().posterVerified}>
                      <VerifiedBadge />
                    </Show>
                    <Show when={s().posterUsername}>
                      <span>@{s().posterUsername}</span>
                    </Show>
                    <span>· {ago(ms(s().updated))}</span>
                  </div>
                  <p class="whitespace-pre-wrap break-words">
                    <LinkedText text={s().text} />
                  </p>
                </div>
              </>
            )}
          </Show>
          <Show when={me()?.postShout}>
            <Button size="sm" onClick={newShout}>
              New shout
            </Button>
          </Show>
        </Card>
      </Show>

      <Tabs aria-label="Community sections" items={sections()} value={section()} onChange={setSection}>
        <TabPanel value="about" class="flex flex-col gap-3">
          <Show when={view.info?.description}>
            {(d) => (
              <div class="max-w-prose">
                <Clamp lines={4}>
                  <Description text={d()} />
                </Clamp>
              </div>
            )}
          </Show>
          <Card class="grid grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-4 p-3">
            <Stat label="Members">{compact(view.info?.members ?? 0)}</Stat>
            <Stat label="Owner">{view.info?.owner || "–"}</Stat>
            <Stat label="Entry">{view.info?.publicEntry ? "Open" : "Approval"}</Stat>
            <Stat label="ID">{params.id}</Stat>
          </Card>
        </TabPanel>
        <TabPanel value="games">
          <Games groupId={groupId()} />
        </TabPanel>
        <TabPanel value="store">
          <Store groupId={groupId()} />
        </TabPanel>
        <TabPanel value="forum">
          <Show
            when={post()}
            fallback={
              <Forum
                groupId={groupId()}
                canPost={Boolean(me()?.createPosts)}
                canDelete={Boolean(me()?.removePosts)}
                onOpen={setPost}
              />
            }
          >
            {(p) => (
              <Thread
                groupId={groupId()}
                post={p()}
                canReply={Boolean(me()?.createComments)}
                canDelete={Boolean(me()?.removeComments)}
                onBack={() => setPost(null)}
              />
            )}
          </Show>
        </TabPanel>
        <TabPanel value="members">
          <Members
            groupId={groupId()}
            myRank={me()?.rank ?? 0}
            canRank={Boolean(me()?.changeRank)}
            canKick={Boolean(me()?.removeMembers)}
          />
        </TabPanel>
        <TabPanel value="allies">
          <Affiliates view={affiliates} kind="allies" canManage={Boolean(me()?.manageRelationships)} />
        </TabPanel>
        <TabPanel value="enemies">
          <Affiliates view={affiliates} kind="enemies" canManage={Boolean(me()?.manageRelationships)} />
        </TabPanel>
        <TabPanel value="requests">
          <Requests groupId={groupId()} />
        </TabPanel>
        <TabPanel value="audit">
          <Audit groupId={groupId()} />
        </TabPanel>
        <TabPanel value="revenue">
          <Revenue groupId={groupId()} canSpend={Boolean(me()?.spendFunds)} />
        </TabPanel>
        <TabPanel value="settings">
          <Settings groupId={groupId()} />
        </TabPanel>
      </Tabs>
    </Page>
  )
}

function Games(props: { groupId: number }) {
  const list = useLive(() => props.groupId, commands.communityGames, emptyList<GameCard>)
  return (
    <ListState list={list} icon="game" empty="No experiences">
      <VirtualGrid items={list.items} minItemWidth={128} estimateRowHeight={240} gap={12}>
        {(g) => <GameTile game={g} />}
      </VirtualGrid>
    </ListState>
  )
}

/** Members of one role at a time, the way Roblox pages them. */
function Members(props: { groupId: number; myRank: number; canRank: boolean; canKick: boolean }) {
  const roles = useLive(() => props.groupId, commands.communityRoles, emptyList<RoleRow>)
  const [picked, setPicked] = createSignal<number | null>(null)
  // Guests (rank 0) have no member list; start on the lowest real role.
  const role = createMemo(() => {
    const p = picked()
    const real = roles.items.filter((r) => r.rank > 0)
    return real.find((r) => r.id === p) ?? real[0] ?? null
  })
  const members = useLive(
    () => [props.groupId, role()?.id ?? 0] as const,
    ([g, r]) => (r ? commands.communityMembers(g, r) : Promise.resolve(emptyList<MemberRow>())),
    emptyList<MemberRow>,
  )
  const [filter, setFilter] = createSignal("")
  const shown = createMemo(() => {
    const q = filter().trim().toLowerCase()
    return members.items.filter(
      (m) => !q || m.name.toLowerCase().includes(q) || m.displayName.toLowerCase().includes(q),
    )
  })
  // You can move people below your own rank, to roles below your own rank.
  const ranked = () => roles.items.filter((r) => r.rank > 0)
  const canTouch = () => (role()?.rank ?? 255) < props.myRank

  const columns: Column<MemberRow>[] = [
    {
      key: "name",
      label: "Member",
      width: "minmax(10rem, 2fr)",
      cell: (m) => (
        <>
          <Avatar size="sm" name={m.displayName} src={thumbUrl("headshot", m.id)} />
          <TextLink tone={m.verified ? "verified" : "name"} onClick={() => run("user.open", String(m.id))}>
            {m.displayName}
          </TextLink>
          <Show when={m.verified}>
            <VerifiedBadge />
          </Show>
          <span class="truncate text-fg-muted">@{m.name}</span>
        </>
      ),
    },
    {
      key: "role",
      label: "Role",
      width: "12rem",
      cell: (m) => (
        <Show when={props.canRank && canTouch()} fallback={role()?.name}>
          <Select
            size="sm"
            aria-label={`Role of ${m.displayName}`}
            options={ranked().map((r) => ({ value: String(r.id), label: r.name, disabled: r.rank >= props.myRank }))}
            value={String(role()?.id)}
            onChange={(r) => run("group.change_rank", `${m.id}:${r}`)}
          />
        </Show>
      ),
    },
    {
      key: "actions",
      label: "",
      width: "3rem",
      align: "end",
      cell: (m) => (
        <Show when={props.canKick && canTouch()}>
          <IconButton
            size="sm"
            icon="user-minus"
            label={`Remove ${m.displayName}`}
            onClick={() => run("group.kick", String(m.id))}
          />
        </Show>
      ),
    },
  ]

  return (
    <div class="flex flex-col gap-2">
      <div class="flex flex-wrap gap-2">
        <Select
          aria-label="Role"
          options={roles.items
            .filter((r) => r.rank > 0)
            .map((r) => ({ value: String(r.id), label: `${r.name} (${compact(r.members)})` }))}
          value={role() ? String(role()?.id) : undefined}
          onChange={(v) => setPicked(Number(v))}
        />
        <TextField
          class="flex-1"
          icon="search"
          aria-label="Filter members"
          placeholder="Filter loaded members"
          value={filter()}
          onChange={setFilter}
        />
        <Button icon="copy" onClick={() => run("group.export_members")}>
          Copy CSV
        </Button>
      </div>
      <ListState list={members} icon="friends" empty="No members in this role" more={`members:${role()?.id}`}>
        <DataTable aria-label="Members" columns={columns} rows={shown()} rowKey={(m) => m.id} />
      </ListState>
    </div>
  )
}

function Requests(props: { groupId: number }) {
  const list = useLive(() => props.groupId, commands.communityRequests, emptyList<JoinRequestRow>)
  const [picked, setPicked] = createSignal<Set<number>>(new Set())
  const toggle = (id: number, on: boolean) =>
    setPicked((s) => {
      const next = new Set(s)
      if (on) next.add(id)
      else next.delete(id)
      return next
    })
  const answer = async (accept: boolean) => {
    const ids = [...picked()]
    if (ids.length === 0) return showToast("Select requests first.")
    await run(accept ? "group.accept_request" : "group.decline_request", ids.join(","))
    setPicked(new Set<number>())
  }
  onCleanup(handle("group.accept_selected", () => void answer(true)))
  onCleanup(handle("group.decline_selected", () => void answer(false)))
  const all = () => list.items.length > 0 && list.items.every((u) => picked().has(u.id))

  return (
    <div class="flex flex-col gap-2">
      <div class="flex items-center justify-between gap-1.5">
        <Checkbox
          checked={all()}
          onChange={(on) => setPicked(new Set(on ? list.items.map((u) => u.id) : []))}
          disabled={list.items.length === 0}
        >
          Select all loaded
        </Checkbox>
        <div class="flex gap-1.5">
          <Button disabled={picked().size === 0} onClick={() => run("group.decline_selected")}>
            Decline selected
          </Button>
          <Button variant="primary" disabled={picked().size === 0} onClick={() => run("group.accept_selected")}>
            Accept selected
          </Button>
        </div>
      </div>
      <ListState list={list} icon="user-plus" empty="No pending requests" more="requests">
        <div class="flex flex-col">
          <VirtualList items={list.items} estimateRowHeight={56}>
            {(u) => (
              <ListItem
                leading={
                  <span class="flex items-center gap-3">
                    <Checkbox hideLabel checked={picked().has(u.id)} onChange={(on) => toggle(u.id, on)}>
                      Select {u.displayName}
                    </Checkbox>
                    <Avatar name={u.displayName} src={thumbUrl("headshot", u.id)} />
                  </span>
                }
                title={u.displayName}
                subtitle={`@${u.name} · ${ago(ms(u.created))}`}
                actions={
                  <>
                    <Button size="sm" onClick={() => run("group.decline_request", String(u.id))}>
                      Decline
                    </Button>
                    <Button size="sm" onClick={() => run("group.accept_request", String(u.id))}>
                      Accept
                    </Button>
                  </>
                }
                onClick={() => run("user.open", String(u.id))}
              />
            )}
          </VirtualList>
        </div>
      </ListState>
    </div>
  )
}

const AUDIT_ACTIONS: { value: string; label: string; matches: string[] }[] = [
  { value: "all", label: "All actions", matches: [] },
  { value: "ChangeRank", label: "Rank change", matches: ["ChangeRank"] },
  { value: "RemoveMember", label: "Member kicked", matches: ["RemoveMember"] },
  { value: "BanMember", label: "Member banned", matches: ["BanMember", "UnbanMember"] },
  {
    value: "JoinRequest",
    label: "Join requests",
    matches: ["AcceptJoinRequest", "DeclineJoinRequest"],
  },
  { value: "PostStatus", label: "Shout posted", matches: ["PostStatus"] },
  { value: "DeletePost", label: "Post deleted", matches: ["DeletePost"] },
  {
    value: "Settings",
    label: "Settings updated",
    matches: [
      "UpdateGroupDescription",
      "ChangeDescription",
      "Rename",
      "Lock",
      "Unlock",
      "ChangeOwner",
      "UpdateRolesetRank",
      "UpdateRolesetData",
      "UpdateRoleSetPosition",
    ],
  },
  {
    value: "SpendGroupFunds",
    label: "Funds spent",
    matches: ["SpendGroupFunds", "AdjustCurrencyAmounts"],
  },
  {
    value: "Relationships",
    label: "Affiliates updated",
    matches: ["SendAllyRequest", "AcceptAllyRequest", "DeclineAllyRequest", "DeleteAlly", "CreateEnemy", "DeleteEnemy"],
  },
]

function Audit(props: { groupId: number }) {
  const list = useLive(() => props.groupId, commands.communityAudit, emptyList<AuditRow>)
  const [q, setQ] = createSignal("")
  const [action, setAction] = createSignal("all")
  onCleanup(handle("group.audit_search", (a) => setQ(a ?? "")))
  const shown = createMemo(() => {
    const t = q().trim().toLowerCase()
    const picked = AUDIT_ACTIONS.find((a) => a.value === action())
    return list.items.filter((a) => {
      if (picked && picked.value !== "all" && !picked.matches.includes(a.action)) return false
      if (!t) return true
      return a.actor.toLowerCase().includes(t) || a.text.toLowerCase().includes(t)
    })
  })
  return (
    <ListState list={list} icon="list" empty="Nothing logged" more="audit">
      <div class="mb-2 flex flex-wrap items-center gap-2">
        <Select
          aria-label="Action type"
          options={AUDIT_ACTIONS.map((a) => ({ value: a.value, label: a.label }))}
          value={action()}
          onChange={setAction}
        />
        <TextField
          class="min-w-48 flex-1"
          icon="search"
          aria-label="Search the log"
          placeholder="Search loaded entries by person or action"
          value={q()}
          onChange={setQ}
        />
      </div>
      <div class="flex flex-col">
        <VirtualList items={shown()} estimateRowHeight={56}>
          {(a) => (
            <ListItem
              leading={<Avatar size="sm" name={a.actor} src={thumbUrl("headshot", a.actorId)} />}
              title={a.text}
              subtitle={`${a.actor} · ${a.actorRole} · ${ago(ms(a.created))}`}
              onClick={() => run("user.open", String(a.actorId))}
            />
          )}
        </VirtualList>
      </div>
    </ListState>
  )
}

function Settings(props: { groupId: number }) {
  const list = useLive(() => props.groupId, commands.communitySettings, emptyList<SettingRow>)
  return (
    <ListState list={list} icon="settings" empty="No settings">
      <div class="flex max-w-xl flex-col gap-3">
        <For each={list.items}>
          {(s) => (
            <Switch checked={s.on} onChange={() => run("group.setting", s.key)}>
              {s.label}
            </Switch>
          )}
        </For>
        <span class="text-sm text-fg-muted">Other settings are on the website.</span>
      </div>
    </ListState>
  )
}

/** Categories down the side, posts of the picked one. */
function Forum(props: { groupId: number; canPost: boolean; canDelete: boolean; onOpen: (p: OpenPost) => void }) {
  const cats = useLive(() => props.groupId, commands.forumCategories, emptyList<ForumCategoryRow>)
  const [picked, setPicked] = createSignal<string | null>(null)
  const category = () => picked() ?? cats.items[0]?.id ?? ""
  const posts = useLive(
    () => [props.groupId, category()] as const,
    ([g, c]) => (c ? commands.forumPosts(g, c) : Promise.resolve(emptyList<ForumPostRow>())),
    emptyList<ForumPostRow>,
  )
  return (
    <ListState list={cats} icon="chat" empty="This community has no forum">
      <div class="flex flex-col gap-2">
        <div class="flex flex-wrap items-center gap-2">
          <Select
            aria-label="Category"
            options={cats.items.map((c) => ({ value: c.id, label: c.name }))}
            value={category()}
            onChange={setPicked}
          />
          <span class="flex-1 truncate text-sm text-fg-muted">
            {cats.items.find((c) => c.id === category())?.description}
          </span>
          <Show when={props.canPost}>
            <Button icon="plus" onClick={() => run("group.new_post", category())}>
              New post
            </Button>
          </Show>
        </div>
        <ListState list={posts} icon="chat" empty="No posts yet" more={`posts:${category()}`}>
          <div class="flex flex-col">
            <VirtualList items={posts.items} estimateRowHeight={56}>
              {(p) => {
                const open = () => props.onOpen({ category: category(), id: p.id, title: p.title })
                const repliesLabel = () => `${compact(p.replies)} ${p.replies === 1 ? "reply" : "replies"}`
                const activeLabel = () => ago(ms(p.updated || p.created))
                const rolePart = () => (p.authorRole ? ` · ${p.authorRole}` : "")
                return (
                  <ListItem
                    leading={<Avatar size="sm" name={p.author} src={thumbUrl("headshot", p.authorId)} />}
                    title={p.title}
                    subtitle={`${p.author}${rolePart()} · ${repliesLabel()} · active ${activeLabel()}`}
                    unread={p.unread}
                    meta={
                      <Show when={p.pinned || p.locked}>
                        <span class="flex items-center gap-1">
                          <Show when={p.pinned}>
                            <Badge>Pinned</Badge>
                          </Show>
                          <Show when={p.locked}>
                            <Badge>Locked</Badge>
                          </Show>
                        </span>
                      </Show>
                    }
                    actions={
                      <Show when={props.canDelete}>
                        <IconButton
                          size="sm"
                          icon="trash"
                          label={`Delete ${p.title}`}
                          onClick={(e: MouseEvent) => {
                            e.stopPropagation()
                            void run("group.delete_post", `${category()}/${p.id}`)
                          }}
                        />
                      </Show>
                    }
                    onClick={open}
                  />
                )
              }}
            </VirtualList>
          </div>
        </ListState>
      </div>
    </ListState>
  )
}

/** One post and its replies. The page context names it, so `group.reply` answers it. */
function Thread(props: { groupId: number; post: OpenPost; canReply: boolean; canDelete: boolean; onBack: () => void }) {
  const list = useLive(
    () => [props.groupId, props.post.category, props.post.id] as const,
    ([g, c, p]) => commands.forumComments(g, c, p),
    emptyList<ForumCommentRow>,
  )
  const [reply, setReply] = createSignal("")
  const send = async () => {
    const text = reply().trim()
    if (!text) return
    await run("group.reply", text)
    setReply("")
  }
  return (
    <div class="flex flex-col gap-2">
      <div class="flex items-center gap-2">
        <IconButton icon="chevron-left" label="Back to posts" onClick={() => props.onBack()} />
        <h2 class="truncate text-lg font-medium">{props.post.title}</h2>
      </div>
      <ListState list={list} icon="chat" empty="No replies yet" more="comments">
        <div class="flex flex-col gap-3">
          <VirtualList items={list.items} estimateRowHeight={90} gap={12}>
            {(c) => (
              <div class="flex items-start gap-3">
                <Avatar size="sm" name={c.author} src={thumbUrl("headshot", c.authorId)} />
                <div class="flex min-w-0 flex-1 flex-col">
                  <span class="text-sm text-fg-muted">
                    <TextLink tone="name" onClick={() => run("user.open", String(c.authorId))}>
                      {c.author}
                    </TextLink>{" "}
                    · {c.authorRole} · {ago(ms(c.created))}
                  </span>
                  <p class="whitespace-pre-wrap break-words">
                    <LinkedText text={c.text} />
                  </p>
                </div>
                <Show when={props.canDelete}>
                  <IconButton
                    size="sm"
                    icon="trash"
                    label="Delete reply"
                    onClick={() => run("group.delete_reply", `${props.post.category}/${props.post.id}/${c.id}`)}
                  />
                </Show>
              </div>
            )}
          </VirtualList>
        </div>
      </ListState>
      <Show when={props.canReply}>
        <form
          class="flex gap-2"
          onSubmit={(e) => {
            e.preventDefault()
            void send()
          }}
        >
          <TextField
            class="flex-1"
            aria-label="Reply"
            placeholder="Write a reply"
            value={reply()}
            onChange={setReply}
          />
          <Button type="submit" icon="send" disabled={reply().trim() === ""}>
            Reply
          </Button>
        </form>
      </Show>
    </div>
  )
}

function NewPostForm(props: { groupId: number; categories: ForumCategoryRow[]; initial: string; onDone: () => void }) {
  const [category, setCategory] = createSignal(props.initial)
  const [title, setTitle] = createSignal("")
  const [text, setText] = createSignal("")
  const [busy, setBusy] = createSignal(false)
  const post = async () => {
    setBusy(true)
    const r = await commands.createForumPost(props.groupId, category(), title().trim(), text().trim())
    setBusy(false)
    if (r.status === "error") return showToast(r.error)
    props.onDone()
  }
  return (
    <form
      class="flex flex-col gap-3 p-4"
      onSubmit={(e) => {
        e.preventDefault()
        void post()
      }}
    >
      <Select
        label="Category"
        options={props.categories.map((c) => ({ value: c.id, label: c.name }))}
        value={category()}
        onChange={setCategory}
      />
      <TextField label="Title" value={title()} onChange={setTitle} autofocus />
      <TextField label="Text" value={text()} onChange={setText} />
      <div class="flex justify-end gap-2">
        <Button variant="ghost" type="button" onClick={() => props.onDone()}>
          Cancel
        </Button>
        <Button variant="primary" type="submit" disabled={busy() || title().trim() === "" || text().trim() === ""}>
          Post
        </Button>
      </div>
    </form>
  )
}
