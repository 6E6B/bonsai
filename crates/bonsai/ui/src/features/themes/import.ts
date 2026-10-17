/**
 * What the import dialog says about a theme before it is installed: what was skipped and which
 * text pairs would be hard to read.
 */
import { type Check, readTheme, resolve, runChecks, type Theme, type Tokens } from "~/lib/theme"

export type ImportSummary =
  | { ok: true; theme: Theme; tokens: Tokens; skipped: string[]; failing: Check[]; checked: number }
  | { ok: false; error: string }

export function summarize(doc: string): ImportSummary {
  let raw: unknown
  try {
    raw = JSON.parse(doc)
  } catch {
    return { ok: false, error: "Not a theme." }
  }
  const r = readTheme(raw)
  if (!r.ok) return r
  const tokens = resolve(r.theme)
  const checks = runChecks(tokens)
  return {
    ok: true,
    theme: r.theme,
    tokens,
    skipped: r.problems,
    failing: checks.filter((c) => !c.ok),
    checked: checks.length,
  }
}

/** True for a pasted theme code. */
export const isThemeCode = (text: string) => text.trim().startsWith("bonsai-theme:")

/** True for a file the import takes when dropped on the window. */
export const isThemeFile = (path: string) => /\.bonsaitheme$/i.test(path)
