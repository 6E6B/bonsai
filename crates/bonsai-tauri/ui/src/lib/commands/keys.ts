import type { Keys } from "~/lib/ipc/bindings"

/** A key press in rbx-core's key names (`"K"`, `"1"`, `"Slash"`, `"PageDown"`). */
export interface Press {
  key: string
  ctrl: boolean
  shift: boolean
  alt: boolean
}

const NAMED: Record<string, string> = {
  "/": "Slash",
  "?": "Questionmark",
  ",": "Comma",
  ".": "Period",
  "\\": "Backslash",
  "+": "Plus",
  "=": "Equals",
  "-": "Minus",
  " ": "Space",
}

/** Maps a DOM key event to rbx-core's key names. Returns undefined for lone modifiers. */
export function toPress(
  e: Pick<KeyboardEvent, "key" | "ctrlKey" | "metaKey" | "shiftKey" | "altKey">,
): Press | undefined {
  if (["Control", "Shift", "Alt", "Meta"].includes(e.key)) return undefined
  const key = NAMED[e.key] ?? (e.key.length === 1 ? e.key.toUpperCase() : e.key)
  return { key, ctrl: e.ctrlKey || e.metaKey, shift: e.shiftKey, alt: e.altKey }
}

/** True for presses with no modifier, which must not fire while the user is typing. */
export function isPlain(p: Press) {
  return !p.ctrl && !p.alt
}

export type Match = { kind: "run" } | { kind: "pending"; first: string } | { kind: "none" }

/**
 * Checks one press against a command's shortcuts. `pending` is the first key of a sequence
 * typed just before, like `G` in `g h`.
 */
export function match(press: Press, shortcuts: Keys[], pending: string | undefined): Match {
  let result: Match = { kind: "none" }
  for (const s of shortcuts) {
    if (s.kind === "chord") {
      if (pending === undefined && chordMatches(s, press)) return { kind: "run" }
    } else if (!press.ctrl && !press.alt) {
      if (pending === s.first && press.key === s.second) return { kind: "run" }
      if (pending === undefined && press.key === s.first) result = { kind: "pending", first: s.first }
    }
  }
  return result
}

function chordMatches(s: Extract<Keys, { kind: "chord" }>, p: Press) {
  // Shift is part of the character for keys like "?" and "+", so only compare it for letters.
  const shiftMatters = /^[A-Z]$/.test(s.key)
  return s.key === p.key && s.ctrl === p.ctrl && s.alt === p.alt && (!shiftMatters || s.shift === p.shift)
}
