import { useNavigate, useParams } from "@solidjs/router"
import { Switch as Branch, createMemo, createSignal, For, type JSX, Match, onCleanup, Show } from "solid-js"
import { filterEntries } from "~/lib/commands/filter"
import { groupOf } from "~/lib/commands/groups"
import { commandList, menuItem, run } from "~/lib/commands/registry"
import { bytes, countdown, date, formatLocales } from "~/lib/format"
import {
  type AccountInfoView,
  type AppearanceView,
  type AppSettings,
  type CommandInfo,
  commands,
  type FontCatalogView,
  type MyServerRow,
  type Person,
  type PinView,
  type PrivacyRow,
  type SessionsView,
  type ThemeView,
} from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { ServerControls } from "~/lib/private-server"
import { arrange, nudgeScreen, screenKey, screens } from "~/lib/screens"
import { settingsSections } from "~/lib/sections"
import { shell, useLive } from "~/lib/store"
import {
  FONTS,
  type FontSeed,
  fontChoices,
  fontLabel,
  googleFamily,
  MONOS,
  type Mode,
  parseTheme,
  resolve,
  shownBase,
} from "~/lib/theme"
import { thumbUrl } from "~/lib/thumb"
import {
  Avatar,
  Badge,
  Button,
  Card,
  type Column,
  Combobox,
  ContextMenu,
  DataTable,
  Icon,
  IconButton,
  Kbd,
  ListItem,
  NavItem,
  Pane,
  RadioGroup,
  Robux,
  SectionHeader,
  SegmentedControl,
  Select,
  Skeleton,
  Split,
  Switch,
  TextField,
  ThemeCard,
  usePrompt,
} from "~/lib/ui"
import { privacyOptions, shownPrivacy, valueLabel } from "./privacy"

const sections = settingsSections

/** Settings, one section at a time. `nav.settings` takes a section name, so the command bar can
 * open any of them directly. */
export default function Settings() {
  const params = useParams()
  const navigate = useNavigate()
  const section = () => params.section ?? "account"

  return (
    <Split>
      <Pane side as="nav" class="w-64 gap-0.5" aria-label="Settings sections">
        <SectionHeader title="Settings" size="page" class="mb-2" />
        <For each={sections}>
          {(s) => (
            <NavItem label={s.label} active={section() === s.value} onClick={() => navigate(`/settings/${s.value}`)} />
          )}
        </For>
      </Pane>
      <Pane>
        <div class="flex max-w-2xl flex-col gap-4">
          <Branch>
            <Match when={section() === "account"}>
              <AccountSection />
            </Match>
            <Match when={section() === "accounts"}>
              <AccountsSection />
            </Match>
            <Match when={section() === "privacy"}>
              <PrivacySection />
            </Match>
            <Match when={section() === "servers"}>
              <ServersSection />
            </Match>
            <Match when={section() === "notifications"}>
              <NotificationsSection />
            </Match>
            <Match when={section() === "appearance"}>
              <AppearanceSection />
            </Match>
            <Match when={section() === "shortcuts"}>
              <ShortcutsSection />
            </Match>
            <Match when={section() === "advanced"}>
              <AdvancedSection />
            </Match>
          </Branch>
        </div>
      </Pane>
    </Split>
  )
}

/** Card holding settings rows, one line between each. */
const rows = "flex flex-col divide-y divide-line px-4"

/** A settings row: label and description on the left, the value or control on the right. */
function Row(props: { label: string; description?: string; children: JSX.Element }) {
  return (
    <div class="flex min-h-12 items-center justify-between gap-4 py-2.5">
      <div class="flex min-w-0 flex-col">
        <span>{props.label}</span>
        <Show when={props.description}>
          <span class="text-sm text-fg-muted">{props.description}</span>
        </Show>
      </div>
      <div class="flex shrink-0 items-center gap-1.5">{props.children}</div>
    </div>
  )
}

/** A switch laid out as a settings row. */
const SwitchRow = (props: {
  label: string
  description?: string
  checked?: boolean
  defaultChecked?: boolean
  onChange?: () => void
}) => (
  <Switch
    layout="row"
    class="min-h-12 py-2.5"
    description={props.description}
    checked={props.checked}
    defaultChecked={props.defaultChecked}
    onChange={props.onChange}
  >
    {props.label}
  </Switch>
)

const webButton = (label = "Manage on website") => (
  <Button size="sm" icon="external" onClick={() => run("entity.open_website", "section:settings")}>
    {label}
  </Button>
)

/** "Verified" or "Not verified" next to a value Roblox has on file. */
function Verified(props: { ok: boolean }) {
  return (
    <Show when={props.ok} fallback={<Badge>Not verified</Badge>}>
      <Badge variant="success">Verified</Badge>
    </Show>
  )
}

function AccountSection() {
  const active = () => shell.accounts.find((a) => a.id === shell.active)
  const info = useLive(
    () => shell.active,
    () => commands.accountInfo(),
    (): AccountInfoView => ({
      loading: true,
      error: null,
      email: "",
      emailVerified: false,
      phone: "",
      phoneVerified: false,
      birthdate: null,
      premium: null,
      premiumPlan: null,
      premiumRenews: null,
      premiumExpires: null,
      ageVerified: null,
      voice: null,
    }),
  )
  const sessions = useLive(
    () => shell.active,
    () => commands.accountSessions(),
    (): SessionsView => ({ loading: true, error: null, sessions: [] }),
  )
  const pin = useLive(
    () => shell.active,
    () => commands.pinStatus(),
    (): PinView => ({ loading: true, error: null, enabled: false, unlockedUntil: 0 }),
  )
  const [now, setNow] = createSignal(Date.now() / 1000)
  const tick = setInterval(() => setNow(Date.now() / 1000), 1000)
  onCleanup(() => clearInterval(tick))
  const unlocked = () => (pin.unlockedUntil ?? 0) > now()
  const pinText = () => {
    if (pin.loading) return "Checking…"
    if (pin.error) return pin.error
    if (!pin.enabled) return "No PIN set."
    if (unlocked()) return `Unlocked for ${countdown((pin.unlockedUntil ?? 0) - now())}.`
    return "Locked. Settings changes need the PIN."
  }
  const prompt = usePrompt()
  const rename = async () => {
    const name = await prompt({
      title: "Change display name",
      label: "Display name",
      initial: active()?.displayName,
    })
    if (name) void run("account.display_name", name)
  }
  const birthday = () => {
    const t = Date.parse(`${info.birthdate}T12:00:00`)
    return Number.isNaN(t) ? null : date(t)
  }
  const day = (iso: string | null) => {
    const t = Date.parse(iso ?? "")
    return Number.isNaN(t) ? null : date(t)
  }
  const seen = (secs: number | null) => (secs ? date(secs * 1000) : "")
  const value = (v: () => string | null, empty = "None") => (
    <Show when={!info.loading} fallback={<Skeleton class="h-4 w-32 rounded-sm" />}>
      <span class={v() ? undefined : "text-fg-muted"}>{v() || empty}</span>
    </Show>
  )
  return (
    <>
      <SectionHeader title="Account" actions={webButton("Open on website")} />
      <Show when={info.error}>{(e) => <span class="text-sm text-danger">{e()}</span>}</Show>
      <Card class={rows}>
        <Row label="Display name" description="You can change it once every 7 days.">
          <span>{active()?.displayName}</span>
          <Button size="sm" icon="edit" disabled={!active()} onClick={() => void rename()}>
            Change
          </Button>
        </Row>
        <Row label="Username" description="Changing it costs Robux and needs your password.">
          <span>@{active()?.name}</span>
          {webButton("Change on website")}
        </Row>
        <Row label="Email">
          {value(() => info.email)}
          <Show when={!info.loading && info.email}>
            <Verified ok={info.emailVerified} />
          </Show>
        </Row>
        <Row label="Phone">
          {value(() => info.phone)}
          <Show when={!info.loading && info.phone}>
            <Verified ok={info.phoneVerified} />
          </Show>
        </Row>
        <Row label="Birthday">
          {value(birthday, "Unknown")}
          <Show when={!info.loading && info.ageVerified !== null}>
            <Show when={info.ageVerified} fallback={<Badge>Age not verified</Badge>}>
              <Badge variant="success">Age verified</Badge>
            </Show>
          </Show>
        </Row>
        <Row label="Premium">
          <Show when={info.premium !== null} fallback={value(() => null, "Unknown")}>
            <Show when={info.premium} fallback={<span class="text-fg-muted">No</span>}>
              <Badge variant="accent">{info.premiumPlan ?? "Premium"}</Badge>
              <Show when={day(info.premiumRenews)}>{(d) => <span class="text-fg-muted">Renews {d()}</span>}</Show>
              <Show when={!info.premiumRenews && day(info.premiumExpires)}>
                {(d) => <span class="text-fg-muted">Ends {d()}</span>}
              </Show>
            </Show>
          </Show>
        </Row>
        <Show when={info.voice}>
          {(v) => (
            <Row label="Voice and camera" description="Set on the website. Shown here for reference.">
              <Badge variant={v().voiceEnabled ? "success" : undefined}>
                {v().banned
                  ? "Voice banned"
                  : v().voiceEnabled
                    ? "Voice on"
                    : v().voiceEligible
                      ? "Voice off"
                      : "Voice unavailable"}
              </Badge>
              <Badge variant={v().cameraEnabled ? "success" : undefined}>
                {v().cameraEnabled ? "Camera on" : v().cameraEligible ? "Camera off" : "Camera unavailable"}
              </Badge>
            </Row>
          )}
        </Show>
      </Card>
      <SectionHeader title="Security" />
      <Card class={rows}>
        <Row label="Password">{webButton("Change on website")}</Row>
        <Row label="Two-step verification" description="Ask for a code from your authenticator app when signing in.">
          {webButton()}
        </Row>
        <Row label="Account PIN" description={pinText()}>
          <Show when={pin.enabled}>
            <Show
              when={unlocked()}
              fallback={
                <Button size="sm" icon="lock" onClick={() => run("account.unlock_pin")}>
                  Unlock
                </Button>
              }
            >
              <Button size="sm" icon="lock" onClick={() => run("account.lock_pin")}>
                Lock
              </Button>
            </Show>
          </Show>
          {webButton()}
        </Row>
        <Row
          label="Signed-in sessions"
          description="Sign out every other browser, device and app. This app stays signed in."
        >
          <Button size="sm" variant="danger" disabled={!active()} onClick={() => run("account.sign_out_others")}>
            Sign out others
          </Button>
          {webButton()}
        </Row>
        <Show when={sessions.sessions.length > 0}>
          <div class="flex flex-col gap-1 px-3 py-2 text-sm">
            <For each={sessions.sessions}>
              {(x) => (
                <div class="flex items-center gap-2">
                  <span>{x.device || "Unknown device"}</span>
                  <span class="text-fg-muted">{x.location}</span>
                  <span class="ml-auto text-fg-muted">{seen(x.lastActive)}</span>
                  <Show when={x.current}>
                    <Badge variant="accent">This app</Badge>
                  </Show>
                </div>
              )}
            </For>
          </div>
        </Show>
        <Row label="Quick sign-in" description="Sign in another device with the code it shows.">
          <Button size="sm" icon="devices" disabled={!active()} onClick={() => run("account.approve_code")}>
            Enter code
          </Button>
        </Row>
      </Card>
    </>
  )
}

function AccountsSection() {
  return (
    <>
      <SectionHeader
        title="Accounts on this computer"
        meta={shell.accounts.length}
        actions={
          <Button icon="plus" onClick={() => run("account.add")}>
            Add account
          </Button>
        }
      />
      <Card class="flex flex-col p-1">
        <For each={shell.accounts}>
          {(a) => (
            <ListItem
              leading={<Avatar name={a.displayName} src={thumbUrl("headshot", a.id)} />}
              title={a.label ?? a.displayName}
              subtitle={`@${a.name}`}
              meta={a.id === shell.active ? <Badge variant="accent">Active</Badge> : undefined}
              actions={
                <>
                  <Button size="sm" onClick={() => run("account.replace_cookie", null, a.id)}>
                    New cookie
                  </Button>
                  <Button size="sm" variant="danger" onClick={() => run("account.remove", String(a.id))}>
                    Remove
                  </Button>
                </>
              }
            />
          )}
        </For>
      </Card>
    </>
  )
}

function PrivacySection() {
  const privacy = useLive(
    () => shell.active,
    () => commands.privacySettings(),
    emptyList<PrivacyRow>,
  )
  const blocked = useLive(
    () => shell.active,
    () => commands.people("blocked"),
    emptyList<Person>,
  )
  const shown = createMemo(() => shownPrivacy(privacy.items))
  return (
    <>
      <SectionHeader title="Privacy" actions={webButton("More on website")} />
      <ListState list={privacy} icon="lock" empty="No privacy settings">
        <Card class={rows}>
          <For each={shown()}>
            {(s) => (
              <Row
                label={s.label}
                description={s.row.options.some((o) => !o.allowed) ? "Some choices need age verification." : undefined}
              >
                <Show
                  when={s.row.options.length > 0}
                  fallback={<span class="text-fg-muted">{valueLabel(s.row.value)}</span>}
                >
                  <Select
                    class="w-56"
                    aria-label={s.label}
                    options={privacyOptions(s.row)}
                    value={s.row.value}
                    onChange={(v) => v !== s.row.value && run("privacy.set", `${s.row.key} ${v}`)}
                  />
                </Show>
              </Row>
            )}
          </For>
        </Card>
      </ListState>
      <SectionHeader title="Blocked users" meta={blocked.items.length || undefined} />
      <ListState list={blocked} icon="block" empty="Nobody blocked">
        <Card class="flex flex-col p-1">
          <For each={blocked.items}>
            {(p) => (
              <ListItem
                leading={<Avatar name={p.displayName || "?"} src={thumbUrl("headshot", p.id)} />}
                title={p.displayName || String(p.id)}
                subtitle={p.name ? `@${p.name}` : undefined}
                onClick={() => run("user.open", String(p.id))}
                actions={
                  <Button size="sm" onClick={() => run("user.unblock", String(p.id))}>
                    Unblock
                  </Button>
                }
              />
            )}
          </For>
        </Card>
      </ListState>
    </>
  )
}

const serverTabs = [
  { value: "owned", label: "Yours" },
  { value: "invited", label: "Invited to" },
]

/** Private servers across every experience: yours (with their settings) and ones you were let
 * into. */
function ServersSection() {
  const [tab, setTab] = createSignal("owned")
  const list = useLive(
    () => `${shell.active}:${tab()}`,
    () => commands.myPrivateServers(tab()),
    emptyList<MyServerRow>,
  )
  const [open, setOpen] = createSignal<number>()
  const when = (iso: string) => {
    const t = Date.parse(iso)
    return Number.isNaN(t) ? null : date(t)
  }
  const subtitle = (s: MyServerRow) =>
    [
      s.name || "Private server",
      tab() === "invited" ? `by ${s.ownerName || "…"}` : null,
      s.active ? null : "Off",
      s.expires && when(s.expires) ? `${s.renews ? "Renews" : "Ends"} ${when(s.expires)}` : null,
    ]
      .filter(Boolean)
      .join(" · ")
  return (
    <>
      <SectionHeader
        title="Private servers"
        actions={
          <SegmentedControl aria-label="Which servers" size="sm" value={tab()} onChange={setTab} options={serverTabs} />
        }
      />
      <ListState
        list={list}
        icon="server"
        empty={tab() === "owned" ? "You have no private servers" : "No one let you into theirs"}
        more={`my_servers:${tab()}`}
      >
        <Card class="flex flex-col p-1">
          <For each={list.items}>
            {(s) => (
              <div class="flex flex-col">
                <ListItem
                  leading={<Avatar name={s.game || "?"} src={thumbUrl("game_icon", s.universeId)} />}
                  title={s.game || "Experience"}
                  subtitle={subtitle(s)}
                  onClick={() => run("game.open", String(s.placeId))}
                  meta={
                    <Show when={s.price} fallback={<span class="text-sm text-fg-muted">Free</span>}>
                      {(p) => (
                        <span class="text-sm">
                          <Robux amount={p()} />
                          /mo
                        </span>
                      )}
                    </Show>
                  }
                  actions={
                    <>
                      <Show when={tab() === "owned"}>
                        <Button
                          size="sm"
                          variant="ghost"
                          icon="sliders"
                          aria-expanded={open() === s.id}
                          onClick={() => setOpen(open() === s.id ? undefined : s.id)}
                        >
                          Settings
                        </Button>
                      </Show>
                      <Button
                        size="sm"
                        disabled={!s.active}
                        onClick={() => run("game.join_private_server", `${s.id} ${s.placeId}`)}
                      >
                        Join
                      </Button>
                    </>
                  }
                />
                <Show when={open() === s.id}>
                  <ServerControls serverId={s.id} class="px-3 pb-3" />
                </Show>
              </div>
            )}
          </For>
        </Card>
      </ListState>
    </>
  )
}

function NotificationsSection() {
  const app = useLive(
    () => "app",
    () => commands.appSettings(),
    initialApp,
  )
  const toggle = (kind: string) => () => run("app.notify", kind)
  return (
    <>
      <SectionHeader title="Notifications" />
      <Card class={rows}>
        <SwitchRow
          label="Friends joining games"
          description="A desktop notification when a friend joins an experience."
          checked={app.notify.friendJoins}
          onChange={toggle("friend_joins")}
        />
        <SwitchRow label="Trade offers" checked={app.notify.trades} onChange={toggle("trades")} />
        <SwitchRow label="Chat messages" checked={app.notify.chats} onChange={toggle("chats")} />
        <SwitchRow label="Friend requests" checked={app.notify.friendRequests} onChange={toggle("friend_requests")} />
        <SwitchRow
          label="Experience updates"
          description="From experiences you turned Notify on for."
          checked={app.notify.experienceUpdates}
          onChange={toggle("experience_updates")}
        />
        <SwitchRow
          label="Sales"
          description="When someone buys something you sell. Checked every few minutes."
          checked={app.notify.sales}
          onChange={toggle("sales")}
        />
      </Card>
    </>
  )
}

const initialApp = (): AppSettings => ({
  mode: "system",
  darkTheme: "dark",
  lightTheme: "light",
  density: "comfortable",
  playerCommand: "",
  formatLocale: "",
  notify: {
    friendJoins: true,
    trades: true,
    chats: true,
    friendRequests: false,
    experienceUpdates: false,
    sales: true,
  },
  cacheData: null,
  cacheImages: null,
})

function AppearanceSection() {
  const app = useLive(
    () => "app",
    () => commands.appSettings(),
    initialApp,
  )
  return (
    <>
      <SectionHeader title="Appearance" />
      <Card class={rows}>
        <Row label="Mode" description="System switches between your dark and light themes with the OS.">
          <RadioGroup
            aria-label="Mode"
            orientation="horizontal"
            value={app.mode}
            options={[
              { value: "system", label: "System" },
              { value: "dark", label: "Dark" },
              { value: "light", label: "Light" },
            ]}
            onChange={(v) => run("view.theme", v)}
          />
        </Row>
        <Row label="Density" description="Compact makes buttons and fields shorter.">
          <RadioGroup
            aria-label="Density"
            orientation="horizontal"
            value={app.density}
            options={[
              { value: "comfortable", label: "Comfortable" },
              { value: "compact", label: "Compact" },
            ]}
            onChange={(v) => run("view.density", v)}
          />
        </Row>
        <Row label="Number and date format" description="Applies to counts, prices and dates.">
          <Select
            size="sm"
            aria-label="Number and date format"
            value={app.formatLocale || "system"}
            options={[{ value: "system", label: "System" }, ...formatLocales]}
            onChange={(v) => run("app.format_locale", v === "system" ? "" : v)}
          />
        </Row>
        <Row label="Zoom">
          <Button size="sm" onClick={() => run("view.zoom_out")}>
            Smaller
          </Button>
          <Button size="sm" onClick={() => run("view.zoom_reset")}>
            Reset
          </Button>
          <Button size="sm" onClick={() => run("view.zoom_in")}>
            Larger
          </Button>
        </Row>
      </Card>
      <ThemeGrid dark={app.darkTheme} light={app.lightTheme} />
      <ThemeFonts />
      <SidebarCard />
    </>
  )
}

/** Installed themes. Picking one uses it for its base (dark or light) and switches to that base. */
function ThemeGrid(props: { dark: string; light: string }) {
  const themes = useLive(
    () => "themes",
    () => commands.themes(),
    (): ThemeView[] => [],
  )
  const detail = (t: ThemeView) => {
    if (t.id === props.dark) return "Used in dark mode"
    if (t.id === props.light) return "Used in light mode"
    return t.author || (t.builtin ? "Built in" : "")
  }
  return (
    <>
      <SectionHeader title="Themes" />
      <Card class="grid grid-cols-[repeat(auto-fill,minmax(9rem,1fr))] gap-3 p-3">
        <For each={themes}>
          {(t) => {
            const tokens = createMemo(() => {
              const theme = parseTheme(t.doc)
              return theme ? resolve(theme) : {}
            })
            return (
              <ContextMenu
                items={[
                  menuItem("theme.apply", { arg: t.id, icon: "check" }),
                  menuItem("theme.delete", { arg: t.id, icon: "trash", danger: true, disabled: t.builtin }),
                ]}
              >
                <ThemeCard
                  class="w-full"
                  name={t.name}
                  detail={detail(t)}
                  tokens={tokens()}
                  selected={t.id === props.dark || t.id === props.light}
                  onClick={() => run("theme.apply", t.id)}
                />
              </ContextMenu>
            )
          }}
        </For>
      </Card>
    </>
  )
}

/** The fonts of the theme on screen. Picking one changes that theme; a built-in one is copied
 * first. Google fonts download in the background and show once they are on disk. */
function ThemeFonts() {
  const themes = useLive(
    () => "themes",
    () => commands.themes(),
    (): ThemeView[] => [],
  )
  // Mode and documents come in one snapshot, so the theme on screen is never out of step.
  const appearance = useLive(
    () => "appearance",
    () => commands.appearance(),
    (): AppearanceView => ({ mode: "system", dark: "", light: "", fonts: [] }),
  )
  const shown = createMemo(() => {
    const base = shownBase(appearance.mode as Mode)
    return parseTheme(base === "dark" ? appearance.dark : appearance.light)
  })
  const builtin = () => themes.find((t) => t.id === shown()?.id)?.builtin ?? false
  const status = (seed: FontSeed<string> | undefined) => {
    const family = seed && googleFamily(seed)
    const f = family && appearance.fonts.find((x) => x.family === family)
    if (!f) return undefined
    if (f.error) return f.error
    if (f.files.length === 0) return `Downloading ${f.family}…`
    return undefined
  }
  const description = (seed: FontSeed<string> | undefined) => {
    const t = shown()
    if (!t) return undefined
    return (
      status(seed) ??
      (builtin() ? `${t.name} is built in, so picking a font makes a copy of it.` : `Saved in ${t.name}.`)
    )
  }
  return (
    <>
      <SectionHeader title="Fonts" class="mt-6" />
      <Card class={rows}>
        <Row label="Font" description={description(shown()?.seeds.font)}>
          <FontPicker
            label="Font"
            mono={false}
            value={(() => {
              const seed = shown()?.seeds.font
              return seed ? fontLabel(seed, FONTS) : ""
            })()}
            onChange={(v) => run("theme.font", v)}
          />
        </Row>
        <Row label="Code font" description={description(shown()?.seeds.mono)}>
          <FontPicker
            label="Code font"
            mono
            value={(() => {
              const seed = shown()?.seeds.mono
              return seed ? fontLabel(seed, MONOS) : ""
            })()}
            onChange={(v) => run("theme.mono_font", v)}
          />
        </Row>
      </Card>
    </>
  )
}

/** Searches the bundled fonts and Google Fonts. Nothing is fetched until the list opens. */
function FontPicker(props: { label: string; mono: boolean; value: string; onChange: (v: string) => unknown }) {
  const [query, setQuery] = createSignal<string | null>(null)
  const catalog = useLive(
    () => query(),
    (q): Promise<FontCatalogView> =>
      q === null ? Promise.resolve(emptyCatalog()) : commands.fontCatalog(q, props.mono),
    emptyCatalog,
    { keep: true },
  )
  const options = createMemo(() => fontChoices(query() ?? "", props.mono, catalog.fonts))
  return (
    <Combobox
      size="sm"
      aria-label={props.label}
      class="w-56"
      searchPlaceholder="Search Google Fonts"
      value={props.value}
      options={options()}
      loading={catalog.loading}
      onSearch={setQuery}
      onChange={props.onChange}
    />
  )
}

const emptyCatalog = (): FontCatalogView => ({ fonts: [], loading: false, error: null })

/** Which screens the sidebar shows and in what order. Dragging rows in the sidebar does the
 * same as the arrows here. */
function SidebarCard() {
  const groups = () => arrange(screens, shell.sidebar).filter((g) => g.length > 0)
  return (
    <>
      <SectionHeader
        title="Sidebar"
        class="mt-6"
        actions={
          <Button size="sm" variant="ghost" onClick={() => run("view.sidebar_reset")}>
            Reset
          </Button>
        }
      />
      <Card class="flex flex-col divide-y divide-line px-4">
        <For each={groups()}>
          {(group) => (
            <div class="flex flex-col py-1.5">
              <For each={group}>
                {(s) => {
                  const key = screenKey(s)
                  const off = () => s.shown?.() === false
                  return (
                    <div class="flex h-10 items-center gap-2.5">
                      <Icon name={s.icon} />
                      <span class="flex-1">{s.label}</span>
                      <Show when={off()}>
                        <span class="text-sm text-fg-muted">Off while item values are off</span>
                      </Show>
                      <IconButton
                        icon="chevron-up"
                        label={`Move ${s.label} up`}
                        size="sm"
                        disabled={!nudgeScreen(screens, shell.sidebar, key, -1)}
                        onClick={() => run("view.sidebar_up", key)}
                      />
                      <IconButton
                        icon="chevron-down"
                        label={`Move ${s.label} down`}
                        size="sm"
                        disabled={!nudgeScreen(screens, shell.sidebar, key, 1)}
                        onClick={() => run("view.sidebar_down", key)}
                      />
                      <Switch
                        hideLabel
                        checked={!shell.sidebar.hidden.includes(key)}
                        onChange={(on) => run("view.sidebar_toggle", `${key} ${on ? "on" : "off"}`)}
                      >
                        {`Show ${s.label} in the sidebar`}
                      </Switch>
                    </div>
                  )
                }}
              </For>
            </div>
          )}
        </For>
      </Card>
    </>
  )
}

function ShortcutsSection() {
  const [query, setQuery] = createSignal("")
  const rows = createMemo(() =>
    filterEntries(
      commandList().map((c) => ({ ...c, words: c.keywords.join(" ") })),
      query(),
    ),
  )
  const columns: Column<CommandInfo>[] = [
    { key: "title", label: "Command", width: "minmax(12rem, 2fr)", cell: (c) => c.title },
    { key: "group", label: "Section", width: "9rem", cell: (c) => <span class="text-fg-muted">{groupOf(c.id)}</span> },
    {
      key: "keys",
      label: "Shortcut",
      width: "9rem",
      cell: (c) => (
        <For each={c.shortcuts} fallback={<span class="text-fg-faint">—</span>}>
          {(s) => <Kbd keys={s.label.split(/(?<=\S)\+(?=\S)| /)} />}
        </For>
      ),
    },
    {
      key: "edit",
      label: "",
      width: "5rem",
      align: "end",
      cell: () => (
        <Button size="sm" variant="ghost" disabled>
          Change
        </Button>
      ),
    },
  ]
  return (
    <>
      <SectionHeader title="Shortcuts" meta={commandList().length} />
      <TextField
        icon="search"
        aria-label="Filter commands"
        placeholder="Filter commands"
        value={query()}
        onChange={setQuery}
      />
      <DataTable aria-label="Commands" columns={columns} rows={rows()} rowKey={(c) => c.id} />
    </>
  )
}

/** Launch command presets. `{uri}` is the `roblox-player:` link; without it the link goes last. */
const SOBER = "flatpak run org.vinegarhq.Sober {uri}"
const launchModes = [
  { value: "default", label: "System default" },
  { value: "sober", label: "Sober (Flatpak)" },
  { value: "custom", label: "Custom command" },
]

function AdvancedSection() {
  const app = useLive(
    () => "app",
    () => commands.appSettings(),
    initialApp,
  )
  const [custom, setCustom] = createSignal<string>()
  const mode = () => {
    if (custom() !== undefined) return "custom"
    if (!app.playerCommand) return "default"
    return app.playerCommand === SOBER ? "sober" : "custom"
  }
  const pick = (m: string) => {
    if (m === "custom") return setCustom(app.playerCommand === SOBER ? "" : app.playerCommand)
    setCustom(undefined)
    void run("app.player_command", m === "sober" ? SOBER : "")
  }
  const usage = () =>
    app.cacheData === null || app.cacheImages === null
      ? "Measuring…"
      : `${bytes(app.cacheData)} of data, ${bytes(app.cacheImages)} of images`
  return (
    <>
      <SectionHeader title="Advanced" />
      <Card class={rows}>
        <Row label="Launch Roblox with" description="How Join opens the game.">
          <Select class="w-48" aria-label="Launch Roblox with" options={launchModes} value={mode()} onChange={pick} />
        </Row>
        <Show when={mode() === "custom"}>
          <form
            class="flex items-end gap-2 py-2.5"
            onSubmit={(e) => {
              e.preventDefault()
              void run("app.player_command", custom() ?? app.playerCommand)
              setCustom(undefined)
            }}
          >
            <TextField
              class="flex-1"
              label="Command"
              placeholder="wine /path/to/RobloxPlayerLauncher.exe {uri}"
              value={custom() ?? app.playerCommand}
              onChange={setCustom}
            />
            <Button type="submit" variant="primary">
              Save
            </Button>
          </form>
        </Show>
        <Row label="Cache" description={usage()}>
          <Button size="sm" icon="trash" onClick={() => run("app.clear_cache")}>
            Clear
          </Button>
        </Row>
        <SwitchRow
          label="Item values"
          description="Show Rolimons values, demand and trends for limiteds, the Values screen, and trading stats and last online on profiles. Refreshed every two minutes while a screen shows them."
          checked={shell.itemValues}
          onChange={() => run("market.values")}
        />
        <Row label="Component gallery">
          <Button size="sm" onClick={() => run("nav.components")}>
            Open
          </Button>
        </Row>
      </Card>
    </>
  )
}
