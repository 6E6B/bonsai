import { createSignal, For, onCleanup, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { countdown, date } from "~/lib/format"
import { type AccountInfoView, commands, type PinView, type SessionsView } from "~/lib/ipc/bindings"
import { shell, useLive } from "~/lib/store"
import { Badge, Button, SettingRow, SettingsGroup, Skeleton, usePrompt } from "~/lib/ui"
import { Verified, WebButton } from "./shared"

const activeAccount = () => shell.accounts.find((a) => a.id === shell.active)

/** Who the active account is: names, contact details, birthday, Premium. */
export function ProfileSection() {
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
  const prompt = usePrompt()
  const rename = async () => {
    const name = await prompt({
      title: "Change display name",
      label: "Display name",
      initial: activeAccount()?.displayName,
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
  const value = (v: () => string | null, empty = "None") => (
    <Show when={!info.loading} fallback={<Skeleton class="h-4 w-32 rounded-sm" />}>
      <span class={v() ? undefined : "text-fg-muted"}>{v() || empty}</span>
    </Show>
  )
  return (
    <>
      <Show when={info.error}>{(e) => <span class="text-sm text-danger">{e()}</span>}</Show>
      <SettingsGroup
        title="Names"
        description="Saved on Roblox for this account."
        actions={<WebButton label="Open on website" />}
      >
        <SettingRow label="Display name" description="You can change it once every 7 days.">
          <span>{activeAccount()?.displayName}</span>
          <Button size="sm" icon="edit" disabled={!activeAccount()} onClick={() => void rename()}>
            Change
          </Button>
        </SettingRow>
        <SettingRow label="Username" description="Changing it costs Robux and needs your password.">
          <span>@{activeAccount()?.name}</span>
          <WebButton label="Change on website" />
        </SettingRow>
      </SettingsGroup>
      <SettingsGroup title="Contact and age">
        <SettingRow label="Email">
          {value(() => info.email)}
          <Show when={!info.loading && info.email}>
            <Verified ok={info.emailVerified} />
          </Show>
        </SettingRow>
        <SettingRow label="Phone">
          {value(() => info.phone)}
          <Show when={!info.loading && info.phone}>
            <Verified ok={info.phoneVerified} />
          </Show>
        </SettingRow>
        <SettingRow label="Birthday">
          {value(birthday, "Unknown")}
          <Show when={!info.loading && info.ageVerified !== null}>
            <Show when={info.ageVerified} fallback={<Badge>Age not verified</Badge>}>
              <Badge variant="success">Age verified</Badge>
            </Show>
          </Show>
        </SettingRow>
        <Show when={info.voice}>
          {(v) => (
            <SettingRow label="Voice and camera" description="Set on the website. Shown here for reference.">
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
            </SettingRow>
          )}
        </Show>
      </SettingsGroup>
      <SettingsGroup title="Membership">
        <SettingRow label="Premium">
          <Show when={info.premium !== null} fallback={value(() => null, "Unknown")}>
            <Show when={info.premium} fallback={<span class="text-fg-muted">No</span>}>
              <Badge variant="accent">{info.premiumPlan ?? "Premium"}</Badge>
              <Show when={day(info.premiumRenews)}>{(d) => <span class="text-fg-muted">Renews {d()}</span>}</Show>
              <Show when={!info.premiumRenews && day(info.premiumExpires)}>
                {(d) => <span class="text-fg-muted">Ends {d()}</span>}
              </Show>
            </Show>
          </Show>
        </SettingRow>
      </SettingsGroup>
    </>
  )
}

/** Sign-in protection for the active account: password, 2-step, PIN and sessions. */
export function SecuritySection() {
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
  const seen = (secs: number | null) => (secs ? date(secs * 1000) : "")
  return (
    <>
      <SettingsGroup title="Signing in" description="Saved on Roblox for this account.">
        <SettingRow label="Password">
          <WebButton label="Change on website" />
        </SettingRow>
        <SettingRow
          label="Two-step verification"
          description="Ask for a code from your authenticator app when signing in."
        >
          <WebButton />
        </SettingRow>
        <SettingRow label="Account PIN" description={pinText()}>
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
          <WebButton />
        </SettingRow>
        <SettingRow label="Quick sign-in" description="Sign in another device with the code it shows.">
          <Button size="sm" icon="devices" disabled={!activeAccount()} onClick={() => run("account.approve_code")}>
            Enter code
          </Button>
        </SettingRow>
      </SettingsGroup>
      <SettingsGroup
        title="Signed-in sessions"
        meta={sessions.sessions.length || undefined}
        actions={
          <Button size="sm" variant="danger" disabled={!activeAccount()} onClick={() => run("account.sign_out_others")}>
            Sign out others
          </Button>
        }
        description="Signing out others ends every other browser, device and app. This app stays signed in."
      >
        <Show
          when={sessions.sessions.length > 0}
          fallback={
            <SettingRow label={sessions.loading ? "Loading sessions…" : (sessions.error ?? "No other sessions")}>
              <WebButton />
            </SettingRow>
          }
        >
          <For each={sessions.sessions}>
            {(x) => (
              <SettingRow label={x.device || "Unknown device"} description={x.location}>
                <Show when={x.current}>
                  <Badge variant="accent">This app</Badge>
                </Show>
                <span class="text-sm text-fg-muted">{seen(x.lastActive)}</span>
              </SettingRow>
            )}
          </For>
        </Show>
      </SettingsGroup>
    </>
  )
}
