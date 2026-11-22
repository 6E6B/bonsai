import type { LaunchPhase, LaunchView } from "~/lib/ipc/bindings"
import type { ConfirmOptions } from "~/lib/ui"

/** Short words for where a launch is, for the status bar and join buttons. */
export const phaseLabel: Record<LaunchPhase, string> = {
  preparing: "Getting ready",
  opening: "Opening Roblox",
  starting: "Starting",
  joining: "Joining",
  inGame: "In game",
  teleporting: "Teleporting",
  menu: "In the Roblox menu",
  disconnected: "Disconnected",
  closed: "Closed",
  crashed: "Crashed",
  alreadyRunning: "Already open",
  didNotStart: "Didn't start",
  commandFailed: "Launch command failed",
  replaced: "Replaced",
  failed: "Couldn't launch",
  untracked: "Opened",
}

/** Phases on the way into a server, where a join button shows a spinner. */
const ON_THE_WAY: ReadonlySet<LaunchPhase> = new Set(["preparing", "opening", "starting", "joining", "teleporting"])

/** Ends worth keeping in the status bar for a while: the user may want to rejoin. */
const WORTH_SHOWING: ReadonlySet<LaunchPhase> = new Set(["disconnected", "crashed"])

/** How long a bad end stays in the status bar. */
export const ENDED_SHOWN_MS = 5 * 60_000

export function onTheWay(l: LaunchView | null | undefined) {
  return !!l?.live && ON_THE_WAY.has(l.phase)
}

/** The launch for this place, while it runs. */
export function launchFor(l: LaunchView | null | undefined, placeId: number) {
  return l?.live && l.placeId === placeId ? l : null
}

/** Whether the status bar shows the launch at `now` (unix ms). */
export function showLaunch(l: LaunchView | null | undefined, now: number) {
  if (!l) return false
  if (l.live) return l.phase !== "untracked"
  return WORTH_SHOWING.has(l.phase) && now - (l.endedAt ?? 0) < ENDED_SHOWN_MS
}

/** "Disconnected: Kicked by the experience", the phase with any reason. */
export function launchStatus(l: LaunchView) {
  const label = phaseLabel[l.phase]
  return l.reason ? `${label}: ${l.reason}` : label
}

/** "12:04" in a server, from the first join (or the launch) until it ended or `now`. */
export function launchElapsed(l: LaunchView, now: number) {
  const from = l.joinedAt ?? l.startedAt ?? now
  const to = l.endedAt ?? now
  const t = Math.max(0, Math.floor((to - from) / 1000))
  const h = Math.floor(t / 3600)
  const m = Math.floor(t / 60) % 60
  const s = String(t % 60).padStart(2, "0")
  return h > 0 ? `${h}:${String(m).padStart(2, "0")}:${s}` : `${m}:${s}`
}

/** Commands that launch Roblox into a server. Bonsai closes a running player first. */
export const JOIN_COMMANDS = [
  "game.join",
  "game.join_as",
  "game.join_server",
  "game.join_best",
  "game.join_smallest",
  "game.join_random",
  "game.join_job",
  "game.join_private_server",
  "game.rejoin",
] as const

/** Asks before a join closes the running player, or null when nothing is open. */
export function joinConfirm(l: LaunchView | null | undefined, playerOpen: boolean): ConfirmOptions | null {
  const live = !!l?.live && l.phase !== "preparing"
  if (!live && !playerOpen) return null
  const where = live && l?.game && (l.phase === "inGame" || l.phase === "teleporting") ? ` in ${l.game}` : ""
  return {
    title: "Close Roblox and join?",
    description: `Roblox is open${where}. Joining closes it first.`,
    confirm: "Close and join",
  }
}
