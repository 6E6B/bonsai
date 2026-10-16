import { readFileSync } from "node:fs"
import { describe, expect, it } from "vitest"
import { builtin } from "./builtins"
import { resolve } from "./theme"
import { TOKEN, TOKENS, UNTHEMED } from "./tokens"

// Vitest stubs CSS imports, `?raw` included, so read the file itself.
const css = readFileSync(`${import.meta.dirname}/../ui/styles/theme.css`, "utf8")

/** The custom properties declared in each block of theme.css, comments stripped. */
function blocks() {
  const src = css.replace(/\/\*[\s\S]*?\*\//g, "")
  const props = (body: string) => {
    const out: Record<string, string> = {}
    for (const m of body.matchAll(/--([\w-]+)\s*:\s*([^;]+);/g)) {
      if (m[1] !== "color-*") out[m[1]] = m[2].trim().replace(/\s+/g, " ")
    }
    return out
  }
  const body = (start: RegExp) => {
    const i = src.search(start)
    const open = src.indexOf("{", i)
    return src.slice(open + 1, src.indexOf("}", open))
  }
  const dark = { ...props(body(/@theme static/)), ...props(body(/:root\s*\{/)) }
  const light = { ...dark, ...props(body(/:root\[data-theme="light"\]/)) }
  return { dark, light }
}

describe("token registry", () => {
  const { dark, light } = blocks()

  it("lists every property in theme.css, or says why not", () => {
    const names = new Set([...Object.keys(dark), ...Object.keys(light)])
    const missing = [...names].filter((n) => !TOKEN.has(n) && !(n in UNTHEMED))
    expect(missing).toEqual([])
  })

  it("lists nothing that theme.css lacks", () => {
    expect(TOKENS.filter((t) => !(t.name in dark)).map((t) => t.name)).toEqual([])
    expect(Object.keys(UNTHEMED).filter((n) => !(n in dark))).toEqual([])
  })

  it("has no duplicates", () => {
    expect(TOKEN.size).toBe(TOKENS.length)
  })
})

describe("built-in themes", () => {
  const { dark, light } = blocks()
  const diff = (got: Record<string, string>, want: Record<string, string>) =>
    TOKENS.map((t) => t.name)
      .filter((n) => got[n] !== want[n])
      .map((n) => `${n}: got ${got[n]}, theme.css has ${want[n]}`)

  it("Dark matches theme.css", () => {
    expect(diff(resolve(builtin("dark")), dark)).toEqual([])
  })

  it("Light matches theme.css", () => {
    expect(diff(resolve(builtin("light")), light)).toEqual([])
  })
})
