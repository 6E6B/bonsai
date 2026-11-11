import { describe, expect, it } from "vitest"
import type { PaletteEntry } from "./palette"
import { enterVerb, rowIcon } from "./rowkind"

const row = (id: string, extra: Partial<PaletteEntry> = {}): PaletteEntry => ({
  key: id,
  title: id,
  words: "",
  id,
  arg: null,
  ...extra,
})

describe("rowIcon", () => {
  it("uses the screen's icon for its command", () => expect(rowIcon(row("nav.friends"))).toBe("friends"))
  it("falls back to the command's area", () => expect(rowIcon(row("trade.new"))).toBe("trades"))
  it("shows a clock for a past search", () => expect(rowIcon(row("", { fill: "tower" }))).toBe("clock"))
  it("has a default", () => expect(rowIcon(row("weird.thing"))).toBe("code"))
})

describe("enterVerb", () => {
  it("joins games", () => expect(enterVerb(row("game.join"))).toBe("Join"))
  it("goes to screens", () => expect(enterVerb(row("nav.trades"))).toBe("Go"))
  it("opens results", () => expect(enterVerb(row("user.open"))).toBe("Open"))
  it("asks for an argument", () => expect(enterVerb(row("user.message", { ask: "user" }))).toBe("Choose"))
  it("types a past search", () => expect(enterVerb(row("", { fill: "x" }))).toBe("Search"))
  it("runs the rest", () => expect(enterVerb(row("tab.new"))).toBe("Run"))
})
