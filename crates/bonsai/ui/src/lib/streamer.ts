import { createStore } from "solid-js/store"
import type { StreamerPromptView } from "~/lib/ipc/bindings"
import { shell } from "~/lib/store"

/** How long `streamer.reveal` shows hidden content. */
export const REVEAL_MS = 10_000

/** What a recording app opening does, as Settings and the command bar list it. */
export const streamerDetectOptions = [
  { value: "ask", label: "Ask" },
  { value: "auto", label: "Turn on automatically" },
  { value: "off", label: "Off" },
] as const

const [ui, setUi] = createStore({ revealing: false, messagesShown: false, friendsShown: false })

/** UI-only streamer mode state: a reveal in progress, and Messages unhidden. */
export const streamerUi = ui

let revealTimer: ReturnType<typeof setTimeout> | undefined

/** Shows hidden content for `ms`, then hides it again. */
export function revealFor(ms = REVEAL_MS) {
  clearTimeout(revealTimer)
  setUi("revealing", true)
  revealTimer = setTimeout(() => setUi("revealing", false), ms)
}

/** Shows the Messages screen until streamer mode is turned off. */
export function showMessages() {
  setUi("messagesShown", true)
}

/** Shows friend lists until streamer mode is turned off. */
export function showFriends() {
  setUi("friendsShown", true)
}

/** Forgets reveals and unhidden screens, for when streamer mode turns off. */
export function resetStreamerUi() {
  clearTimeout(revealTimer)
  setUi({ revealing: false, messagesShown: false, friendsShown: false })
}

/** Private content is hidden right now. */
export const streamerHidden = () => shell.streamer.on && !ui.revealing

/** The Messages screen is covered right now. */
export const messagesHidden = () => streamerHidden() && shell.streamer.hideMessages && !ui.messagesShown

/** Friend lists are covered right now. */
export const friendsHidden = () => streamerHidden() && !ui.friendsShown

/** The name an account shows while streamer mode hides it: by position, not name. */
export const maskedAccountLabel = (index: number) => `Account ${index + 1}`

/** `name` for account `id`, or "Account N" while streamer mode hides private content. */
export function shownAccountLabel(id: number, name: string) {
  if (!streamerHidden()) return name
  return maskedAccountLabel(
    Math.max(
      0,
      shell.accounts.findIndex((b) => b.id === id),
    ),
  )
}

/** An account's name as the shell shows it: its label or display name, or "Account N" while
 * streamer mode hides private content. */
export function accountName(a: { id: number; label: string | null; displayName: string }) {
  return shownAccountLabel(a.id, a.label ?? a.displayName)
}

export interface PromptButton {
  label: string
  /** The command a click runs, or `null` to only close the question. */
  run: [id: string, arg: string] | null
}

/** The toast text and buttons for a recording app opening or closing. */
export function promptToast(p: StreamerPromptView): { title: string; buttons: PromptButton[] } {
  if (p.start) {
    return {
      title: `${p.app} is running. Turn on streamer mode?`,
      buttons: [
        { label: "Turn on", run: ["streamer.toggle", "on"] },
        { label: "Not now", run: null },
        { label: `Don't ask for ${p.app}`, run: ["streamer.ignore", p.app] },
      ],
    }
  }
  return {
    title: `${p.app} closed. Turn off streamer mode?`,
    buttons: [
      { label: "Turn off", run: ["streamer.toggle", "off"] },
      { label: "Keep on", run: null },
    ],
  }
}
