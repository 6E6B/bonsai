import { createSignal, onCleanup } from "solid-js"
import { dateTime } from "~/lib/format"
import type { Account, ModerationView, VoiceBanView } from "~/lib/ipc/bindings"

/** Seconds → "6d 4h", "3h 12m", "45m", "under a minute". */
export function duration(secs: number): string {
  const m = Math.floor(Math.max(0, secs) / 60)
  if (m < 1) return "under a minute"
  const h = Math.floor(m / 60)
  const d = Math.floor(h / 24)
  if (d > 0) return h % 24 ? `${d}d ${h % 24}h` : `${d}d`
  if (h > 0) return m % 60 ? `${h}h ${m % 60}m` : `${h}h`
  return `${m}m`
}

const banOver = (m: ModerationView, now: number) => m.kind === "banned" && m.ends !== null && now >= m.ends

/** Roblox takes a reactivation for a warning or an ended ban. Same rule as bonsai-core. */
export const canReactivate = (m: ModerationView | null | undefined, now: number) =>
  !!m && (m.kind === "warned" || banOver(m, now))

/** One or two words for the rail and the alert title. `null` is moderated without details. */
export function moderationLabel(m: ModerationView | null | undefined, now: number): string {
  if (!m) return "Moderated"
  if (banOver(m, now)) return "Ban ended"
  return { warned: "Warned", banned: "Banned", terminated: "Terminated", locked: "Locked", unknown: "Moderated" }[
    m.kind
  ]
}

/** Danger while the account is gone or banned; warning when the user can fix it. */
export function moderationTone(m: ModerationView | null | undefined, now: number): "warning" | "danger" {
  if (!m) return "warning"
  if (m.kind === "terminated") return "danger"
  return m.kind === "banned" && !banOver(m, now) ? "danger" : "warning"
}

/** What happens next, in one line. Times are unix seconds. */
export function moderationNext(m: ModerationView | null | undefined, now: number): string {
  if (!m) return "Roblox moderated this account. Open the website to see why."
  switch (m.kind) {
    case "warned":
      return "Reactivate to accept the warning and keep using this account."
    case "terminated":
      return "A terminated account can't be reactivated."
    case "locked":
      return "Roblox wants you to verify you're human. Sign in on the website to do it."
    case "unknown":
      return "Open the website to see what Roblox needs."
    case "banned":
      if (m.ends === null) return "Roblox didn't say when the ban ends."
      if (banOver(m, now)) return "The ban is over. Reactivate to keep using this account."
      return `Ends in ${duration(m.ends - now)}, ${dateTime(m.ends * 1000)}.`
  }
}

/** "Voice banned" with its end, when Roblox says. */
export function voiceBanText(v: Pick<VoiceBanView, "until">, now: number): string {
  if (v.until === null || v.until <= now) return "Voice banned"
  return `Voice banned for ${duration(v.until - now)}`
}

/** Seconds → "2d 03:04:05", "3:04:05", "4:05": a countdown that changes every second. */
export function clock(secs: number): string {
  const t = Math.max(0, Math.floor(secs))
  const d = Math.floor(t / 86400)
  const h = Math.floor(t / 3600) % 24
  const m = Math.floor(t / 60) % 60
  const pad = (n: number) => String(n).padStart(2, "0")
  const ms = `${pad(m)}:${pad(t % 60)}`
  if (d > 0) return `${d}d ${pad(h)}:${ms}`
  if (h > 0) return `${h}:${ms}`
  return `${m}:${pad(t % 60)}`
}

/** Accounts with a voice ban that hasn't ended, soonest end first. Bans with no end go last. */
export function activeVoiceBans<A extends Pick<Account, "voiceBan">>(
  accounts: readonly A[],
  now: number,
): (A & { voiceBan: VoiceBanView })[] {
  const end = (a: { voiceBan: VoiceBanView }) => a.voiceBan.until ?? Number.POSITIVE_INFINITY
  return accounts
    .filter((a): a is A & { voiceBan: VoiceBanView } => !!a.voiceBan && (a.voiceBan.until ?? Infinity) > now)
    .sort((a, b) => end(a) - end(b))
}

/** The account's problem in a few words, for the rail tooltip, or "" when it has none. */
export function accountProblem(a: Pick<Account, "session" | "moderation" | "voiceBan">, now: number): string {
  if (a.session === "moderated") return moderationLabel(a.moderation, now)
  if (a.voiceBan) return voiceBanText(a.voiceBan, now)
  return ""
}

/** Banned or terminated: the account can't be used, so its picture is grayed. */
export const accountUnusable = (a: Pick<Account, "session" | "moderation">, now: number) =>
  a.session === "moderated" && moderationTone(a.moderation, now) === "danger"

/** Unix seconds, updated every `everyMs` (30 seconds by default, enough for ban labels) while
 * the caller is mounted. Countdowns pass 1000. */
export function useNowSecs(everyMs = 30_000) {
  const [now, setNow] = createSignal(Date.now() / 1000)
  const tick = setInterval(() => setNow(Date.now() / 1000), everyMs)
  onCleanup(() => clearInterval(tick))
  return now
}
