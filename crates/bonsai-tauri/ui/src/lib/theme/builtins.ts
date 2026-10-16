/**
 * For tests only: the built-in themes, read from the files rbx-core bundles, so the app and the
 * tests can't drift apart.
 */
import { readdirSync, readFileSync } from "node:fs"
import { readTheme, type Theme } from "./theme"

const dir = `${import.meta.dirname}/../../../../../rbx-core/themes`

export function builtinThemes(): Theme[] {
  return readdirSync(dir)
    .filter((f) => f.endsWith(".json"))
    .map((f) => {
      const r = readTheme(JSON.parse(readFileSync(`${dir}/${f}`, "utf8")))
      if (!r.ok) throw new Error(`${f}: ${r.error}`)
      if (r.problems.length) throw new Error(`${f}: ${r.problems.join(", ")}`)
      return r.theme
    })
}

export function builtin(id: string): Theme {
  const t = builtinThemes().find((x) => x.id === id)
  if (!t) throw new Error(`no built-in theme ${id}`)
  return t
}
