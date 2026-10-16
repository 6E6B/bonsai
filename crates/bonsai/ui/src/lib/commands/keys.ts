import type { Keys } from "~/lib/ipc/bindings"

/** A key press in bonsai-core's key names (`"K"`, `"1"`, `"Slash"`, `"PageDown"`). */
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
  "[": "OpenBracket",
  "]": "CloseBracket",
}

/** Maps a DOM key event to bonsai-core's key names. Returns undefined for lone modifiers. */
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

/** A shortcut being recorded, in the shape bonsai-core's `Shortcut::parse` reads back. */
export type Recorded =
  | { kind: "chord"; ctrl: boolean; shift: boolean; alt: boolean; key: string }
  | { kind: "sequence"; first: string; second: string }

/** How long after a plain key a second one still makes a sequence. Matches `shortcutListener`. */
export const SEQUENCE_MS = 1000

/**
 * Adds a key press to a shortcut being recorded. A press with Ctrl or Alt is a chord on its own;
 * a plain key is one too until a second plain key follows within `SEQUENCE_MS`, making `G H`.
 */
export function record(prev: { shortcut: Recorded; at: number } | undefined, p: Press, now: number): Recorded {
  // Shift is part of the character for keys like "?", so it only counts for letters.
  const shift = p.shift && /^[A-Z]$/.test(p.key)
  const chord: Recorded = { kind: "chord", ctrl: p.ctrl, shift, alt: p.alt, key: p.key }
  if (!isPlain(p) || shift) return chord
  const last = prev?.shortcut
  const plainBefore = last?.kind === "chord" && isPlain(last) && !last.shift
  if (last && plainBefore && now - (prev?.at ?? 0) < SEQUENCE_MS)
    return { kind: "sequence", first: last.key, second: p.key }
  return chord
}

/** The text `shortcuts.set` takes: `Ctrl+Shift+K`, `G H`. */
export function spec(s: Recorded): string {
  if (s.kind === "sequence") return `${s.first} ${s.second}`
  return `${s.ctrl ? "Ctrl+" : ""}${s.alt ? "Alt+" : ""}${s.shift ? "Shift+" : ""}${s.key}`
}

const plainKey = (s: Recorded | Keys) => (s.kind === "chord" && !s.ctrl && !s.alt ? s.key : undefined)
const firstKey = (s: Recorded | Keys) => (s.kind === "sequence" ? s.first : undefined)

/** Like bonsai-core's `Shortcut::clashes`: the same keys, or a plain key that starts a sequence. */
export function clashes(a: Recorded, b: Keys): boolean {
  if (a.kind === "chord" && b.kind === "chord")
    return a.key === b.key && a.ctrl === b.ctrl && a.alt === b.alt && a.shift === b.shift
  if (a.kind === "sequence" && b.kind === "sequence") return a.first === b.first && a.second === b.second
  const pa = plainKey(a)
  const fa = firstKey(a)
  return (pa !== undefined && pa === firstKey(b)) || (fa !== undefined && fa === plainKey(b))
}
