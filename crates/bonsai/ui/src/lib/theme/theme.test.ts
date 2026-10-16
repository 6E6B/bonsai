import { describe, expect, it } from "vitest"
import { builtin } from "./builtins"
import { DEFAULT_SEEDS, derive } from "./derive"
import { readTheme, resolve, SCHEMA, tokenValue } from "./theme"

const minimal = { schema: 1, id: "nord", base: "dark" }

const DARK = builtin("dark")

describe("readTheme", () => {
  it("fills a minimal theme from the base defaults", () => {
    const r = readTheme(minimal)
    expect(r.ok).toBe(true)
    if (!r.ok) return
    expect(r.theme.seeds).toEqual(DEFAULT_SEEDS.dark)
    expect(r.theme.name).toBe("Untitled")
    expect(r.problems).toEqual([])
  })

  it("refuses what it can't use", () => {
    expect(readTheme(null).ok).toBe(false)
    expect(readTheme([]).ok).toBe(false)
    expect(readTheme({ ...minimal, schema: SCHEMA + 1 })).toEqual({
      ok: false,
      error: "Made with a newer version of the app.",
    })
    expect(readTheme({ ...minimal, id: "../etc" }).ok).toBe(false)
    expect(readTheme({ ...minimal, base: "sepia" }).ok).toBe(false)
  })

  it("drops bad values and unknown tokens, and says so", () => {
    const r = readTheme({
      ...minimal,
      seeds: { accent: "hotpink", contrast: "lots", radius: 99, font: "comic" },
      tokens: {
        "color-accent": "#ff00aa",
        "color-canvas": "url(https://example.com/x.png)",
        "color-from-the-future": "#000",
      },
    })
    expect(r.ok).toBe(true)
    if (!r.ok) return
    expect(r.theme.seeds.accent).toBe(DEFAULT_SEEDS.dark.accent)
    expect(r.theme.seeds.contrast).toBe(1)
    expect(r.theme.seeds.radius).toBe(4)
    expect(r.theme.seeds.font).toBe("inter")
    expect(r.theme.tokens).toEqual({ "color-accent": "#ff00aa" })
    expect(r.problems).toEqual([
      "color-canvas: bad value",
      "color-from-the-future: unknown token",
      "seeds.accent: not a hex color",
      "seeds.contrast: not a number",
      'seeds.font: unknown font "comic"',
    ])
  })

  it("trims and caps text", () => {
    const r = readTheme({ ...minimal, name: `  ${"x".repeat(100)}  `, author: 5 })
    expect(r.ok && r.theme.name.length).toBe(64)
    expect(r.ok && r.theme.author).toBe("")
  })

  it("reads back what it wrote", () => {
    const r = readTheme(JSON.parse(JSON.stringify(DARK)))
    expect(r).toEqual({ ok: true, theme: DARK, problems: [] })
  })
})

describe("tokenValue", () => {
  it("accepts each kind's grammar", () => {
    expect(tokenValue("color-fg", "#ABC")).toBe("#abc")
    expect(tokenValue("color-presence-game", "var(--color-accent)")).toBe("var(--color-accent)")
    expect(tokenValue("radius-md", "0")).toBe("0")
    expect(tokenValue("radius-md", "6.5px")).toBe("6.5px")
    expect(tokenValue("shadow-key", "inset 0 -1px 0 #00000059,0 0 0 1px #ffffff14")).toBe(
      "inset 0 -1px 0 #00000059, 0 0 0 1px #ffffff14",
    )
    expect(tokenValue("font-sans", "system")).toBe("system-ui, sans-serif")
    expect(tokenValue("font-mono", "jetbrains")).toContain("JetBrainsMono")
  })

  it("rejects anything that could reach outside or inject CSS", () => {
    for (const [name, v] of [
      ["color-fg", "red"],
      ["color-fg", "#fff; background: url(x)"],
      ["color-fg", "var(--color-fg)"],
      ["color-fg", "var(--radius-md)"],
      ["color-fg", "var(--color-accent, url(x))"],
      ["radius-md", "1em"],
      ["radius-md", "calc(1px + 1px)"],
      ["shadow-key", "0 0 0 1px red"],
      ["shadow-key", "0 0 url(x) #000"],
      ["shadow-key", Array(7).fill("0 1px #000").join(", ")],
      ["font-sans", '"Comic Sans MS"'],
      ["font-sans", "jetbrains"],
      ["not-a-token", "#fff"],
    ]) {
      expect(tokenValue(name, v), `${name}: ${v}`).toBeUndefined()
    }
    expect(tokenValue("color-fg", 5)).toBeUndefined()
  })
})

describe("derive", () => {
  const d = DEFAULT_SEEDS.dark

  it("tints surfaces from the background and foreground", () => {
    const t = derive("dark", { ...d, background: "#2e3440", foreground: "#eceff4" })
    expect(t["color-canvas"]).toBe("#2e3440")
    expect(t["color-surface"]).not.toBe(t["color-canvas"])
    expect(t["color-deep"]).not.toBe(t["color-canvas"])
  })

  it("flattens surfaces at contrast 0", () => {
    const t = derive("dark", { ...d, contrast: 0 })
    expect(new Set([t["color-deep"], t["color-canvas"], t["color-surface"], t["color-raised"]]).size).toBe(1)
  })

  it("follows the accent", () => {
    const t = derive("dark", { ...d, accent: "#ff0066" })
    expect(t["color-accent"]).toBe("#ff0066")
    expect(t["color-accent-soft"]).toBe("#ff006629")
    expect(t["color-accent-hover"]).not.toBe("#ff0066")
  })

  it("puts black text on pale accents", () => {
    expect(derive("dark", { ...d, accent: "#ffd400" })["color-on-accent"]).toBe("#000000")
  })

  it("scales radii and shadows", () => {
    const t = derive("dark", { ...d, radius: 0, shadow: { depth: 0, ring: 2, highlight: 1 } })
    expect(t["radius-md"]).toBe("0px")
    expect(t["radius-full"]).toBe("9999px")
    expect(t["shadow-control"]).toBe("0 1px 2px #00000000, 0 0 0 1px #ffffff34, 0 -1px 0 #ffffff0d")
  })

  it("falls back to defaults for seeds a caller got wrong", () => {
    expect(derive("dark", { ...d, accent: "nope" })["color-accent"]).toBe(d.accent)
  })
})

describe("resolve", () => {
  it("lets pinned tokens win over seeds", () => {
    expect(resolve({ ...DARK, tokens: { "color-canvas": "#000000" } })["color-canvas"]).toBe("#000000")
  })

  it("keeps reference chains and breaks cycles", () => {
    const t = resolve({
      ...DARK,
      tokens: {
        "color-focus": "var(--color-presence-online)",
        "color-accent": "var(--color-success)",
        "color-success": "var(--color-accent)",
      },
    })
    expect(t["color-focus"]).toBe("var(--color-presence-online)")
    expect(t["color-accent"]).toBe(DARK.seeds.accent)
    expect(t["color-success"]).toBe(DARK.seeds.success)
  })

  it("breaks cycles through derived references", () => {
    const t = resolve({ ...DARK, tokens: { "color-accent": "var(--color-presence-online)" } })
    expect(t["color-accent"]).toBe(DARK.seeds.accent)
    expect(t["color-presence-online"]).toBe("var(--color-accent)")
  })
})

describe("Google font seeds", () => {
  it("reads a Google family and puts it in front of the bundled stack", () => {
    const r = readTheme({ ...minimal, seeds: { font: { google: "Space Grotesk" }, mono: { google: "Fira Code" } } })
    expect(r.ok).toBe(true)
    if (!r.ok) return
    expect(r.problems).toEqual([])
    expect(r.theme.seeds.font).toEqual({ google: "Space Grotesk" })
    const t = resolve(r.theme)
    expect(t["font-sans"]).toBe('"Space Grotesk", "Inter", system-ui, sans-serif')
    expect(t["font-mono"]).toBe('"Fira Code", "JetBrainsMono Nerd Font Mono", ui-monospace, monospace')
  })

  it("drops a family name that could break out of the stack", () => {
    const r = readTheme({ ...minimal, seeds: { font: { google: 'X", url(evil)' } } })
    expect(r.ok).toBe(true)
    if (!r.ok) return
    expect(r.theme.seeds.font).toBe("inter")
    expect(r.problems).toHaveLength(1)
  })
})
