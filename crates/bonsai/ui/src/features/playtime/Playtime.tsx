import { createMemo, createSignal, For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { ago, date, full } from "~/lib/format"
import { commands, type PlaytimeAccount, type PlaytimeExperience, type PlaytimeView } from "~/lib/ipc/bindings"
import { gameMenu } from "~/lib/menus"
import { duration, experienceName, hours, playtimeRanges, playtimeView, utcOffsetMinutes } from "~/lib/playtime"
import { shell, useLive } from "~/lib/store"
import { shownAccountLabel } from "~/lib/streamer"
import { thumbUrl } from "~/lib/thumb"
import {
  Alert,
  Button,
  Card,
  type Column,
  ContextMenu,
  DataTable,
  EmptyState,
  Page,
  SectionHeader,
  SegmentedControl,
  Select,
  Stat,
  Thumbnail,
} from "~/lib/ui"
import { barWidths, weeklyAverage } from "./shape"

const ALL = "all"

/** Your accounts' time per experience, account and week, from the local playtime log. */
export default function Playtime() {
  const [account, setAccount] = createSignal(ALL)
  const view = useLive(
    () => `${playtimeView.range}:${account()}`,
    () => commands.playtime(playtimeView.range, account() === ALL ? null : Number(account()), utcOffsetMinutes()),
    (): PlaytimeView => ({
      enabled: true,
      background: true,
      range: playtimeView.range,
      secs: 0,
      sessions: 0,
      experiences: [],
      accounts: [],
      weeks: [],
    }),
    { keep: true },
  )
  const accountOptions = createMemo(() => [
    { value: ALL, label: "All accounts" },
    ...shell.accounts.map((a) => ({
      value: String(a.id),
      label: shownAccountLabel(a.id, a.label || a.displayName || a.name),
    })),
  ])
  const average = () => weeklyAverage(view.weeks)
  return (
    <Page
      class="flex flex-col gap-4"
      toolbar={
        <>
          <SegmentedControl
            aria-label="Range"
            options={playtimeRanges}
            value={playtimeView.range}
            onChange={(r) => void run("playtime.range", r)}
          />
          <Select
            class="w-48"
            aria-label="Account"
            options={accountOptions()}
            value={account()}
            onChange={setAccount}
          />
        </>
      }
    >
      <Show when={!view.enabled}>
        <Alert
          variant="info"
          title="The playtime log is off"
          actions={
            <Button size="sm" onClick={() => void run("settings.set", "playtime_log on")}>
              Turn on
            </Button>
          }
        >
          Nothing new is recorded. What was recorded before stays below.
        </Alert>
      </Show>
      <Card class="flex flex-wrap gap-x-8 gap-y-3 p-4">
        <Stat label="Played">{duration(view.secs)}</Stat>
        <Stat label="Sessions">{full(view.sessions)}</Stat>
        <Stat label="Experiences">{full(view.experiences.length)}</Stat>
        <Show when={view.weeks.length > 1}>
          <Stat label="Per week">{duration(average())}</Stat>
        </Show>
      </Card>
      <Show when={view.weeks.length > 1}>
        <Weeks view={view} />
      </Show>
      <div class="grid gap-4 lg:grid-cols-[minmax(0,1fr)_18rem]">
        <Experiences rows={view.experiences} allAccounts={account() === ALL} />
        <Accounts rows={view.accounts} selected={account()} onSelect={setAccount} />
      </div>
      <p class="text-xs text-fg-muted">
        Recorded from each account's presence every two minutes while Bonsai runs.
        <Show when={!view.background}> Only the active account is recorded while background accounts are off.</Show>{" "}
        Kept on this computer only.
      </p>
    </Page>
  )
}

function Bar(props: { width: number }) {
  return (
    <div class="h-1.5 rounded-full bg-control">
      <div class="h-1.5 rounded-full bg-accent" style={{ width: `${props.width}%` }} />
    </div>
  )
}

function Weeks(props: { view: PlaytimeView }) {
  const widths = createMemo(() => barWidths(props.view.weeks.map((w) => w.secs)))
  return (
    <Card class="flex flex-col gap-3 p-4">
      <SectionHeader size="group" title="By week" />
      <div class="grid grid-cols-[6rem_minmax(0,1fr)_4rem] items-center gap-x-4 gap-y-2 text-sm">
        <For each={[...props.view.weeks].reverse()}>
          {(w, i) => (
            <>
              <span class="text-fg-muted">{date(w.start)}</span>
              <Bar width={widths()[props.view.weeks.length - 1 - i()] ?? 0} />
              <span class="text-end tabular-nums">{hours(w.secs)}</span>
            </>
          )}
        </For>
      </div>
    </Card>
  )
}

function Experiences(props: { rows: PlaytimeExperience[]; allAccounts: boolean }) {
  const widths = createMemo(() => barWidths(props.rows.map((r) => r.secs)))
  const index = createMemo(() => new Map(props.rows.map((r, i) => [r.universeId, i])))
  const open = (r: PlaytimeExperience) => {
    if (r.placeId) void run("game.open", String(r.placeId))
  }
  const columns = (): Column<PlaytimeExperience>[] => [
    {
      key: "name",
      label: "Experience",
      width: "minmax(12rem, 3fr)",
      cell: (r) => (
        <ContextMenu items={r.placeId ? gameMenu(r.placeId) : []}>
          <span class="flex min-w-0 items-center gap-2">
            <Thumbnail
              class="w-6 shrink-0 rounded-sm"
              alt=""
              src={thumbUrl("game_icon", r.universeId)}
              fallback="game"
            />
            <span class="truncate">{experienceName(r)}</span>
          </span>
        </ContextMenu>
      ),
    },
    {
      key: "time",
      label: "Time",
      width: "minmax(8rem, 2fr)",
      cell: (r) => (
        <span class="flex w-full items-center gap-2">
          <span class="flex-1">
            <Bar width={widths()[index().get(r.universeId) ?? 0] ?? 0} />
          </span>
          <span class="w-16 shrink-0 text-end tabular-nums">{duration(r.secs)}</span>
        </span>
      ),
    },
    { key: "sessions", label: "Sessions", width: "5rem", align: "end", cell: (r) => full(r.sessions) },
    ...(props.allAccounts
      ? [
          {
            key: "accounts",
            label: "Accounts",
            width: "minmax(6rem, 1fr)",
            cell: (r: PlaytimeExperience) => (
              <span class="truncate text-fg-muted">
                {r.accounts.map((a) => shownAccountLabel(a.account, a.label)).join(", ")}
              </span>
            ),
          },
        ]
      : []),
    {
      key: "last",
      label: "Last played",
      width: "7rem",
      align: "end",
      cell: (r) => <span class="text-fg-muted">{ago(r.lastPlayed)}</span>,
    },
  ]
  return (
    <Card class="flex min-w-0 flex-col gap-2 p-3">
      <SectionHeader class="px-1" size="group" title="By experience" />
      <DataTable
        aria-label="Playtime by experience"
        columns={columns()}
        rows={props.rows}
        rowKey={(r) => r.universeId}
        onRowClick={open}
        empty={<EmptyState icon="clock" title="No playtime in this range" />}
      />
    </Card>
  )
}

function Accounts(props: { rows: PlaytimeAccount[]; selected: string; onSelect: (id: string) => void }) {
  const widths = createMemo(() => barWidths(props.rows.map((r) => r.secs)))
  return (
    <Card class="flex h-fit flex-col gap-1 p-3">
      <SectionHeader class="px-1 pb-1" size="group" title="By account" />
      <Show when={props.rows.length > 0} fallback={<span class="px-1 text-sm text-fg-muted">No playtime yet.</span>}>
        <For each={props.rows}>
          {(a, i) => (
            <button
              type="button"
              class="flex flex-col gap-1.5 rounded-md px-1 py-1.5 text-start text-sm hover:bg-hover"
              classList={{ "bg-hover": props.selected === String(a.account) }}
              aria-pressed={props.selected === String(a.account)}
              onClick={() => props.onSelect(props.selected === String(a.account) ? ALL : String(a.account))}
            >
              <span class="flex items-center justify-between gap-2">
                <span class="truncate">{shownAccountLabel(a.account, a.label)}</span>
                <span class="shrink-0 tabular-nums">{duration(a.secs)}</span>
              </span>
              <Bar width={widths()[i()] ?? 0} />
            </button>
          )}
        </For>
      </Show>
    </Card>
  )
}
