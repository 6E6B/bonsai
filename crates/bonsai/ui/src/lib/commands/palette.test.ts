import { describe, expect, it } from "vitest"
import type { CommandInfo } from "~/lib/ipc/bindings"
import {
  argEntries,
  entityActions,
  hasActions,
  type PaletteEntry,
  paletteEntries,
  paletteSections,
  parseScope,
  remoteEntries,
  remoteQuery,
} from "./palette"

const cmd = (id: string, arg: CommandInfo["arg"]): CommandInfo => ({
  id,
  title: id,
  keywords: [],
  arg,
  shortcuts: [],
  rebound: false,
  context: null,
})

const people = [
  { id: 1, name: "builderman", displayName: "Builder", inGame: true },
  { id: 2, name: "stickmasterluke", displayName: "Luke" },
]

describe("paletteEntries", () => {
  it("lists plain commands, one row per fixed choice, and asking rows for typed args", () => {
    const rows = paletteEntries([cmd("nav.home", "none"), cmd("view.theme", "mode"), cmd("user.open", "user")], [])
    expect(rows.map((r) => [r.key, r.arg, r.ask ?? null])).toEqual([
      ["nav.home", null, null],
      ["view.theme:system", "system", null],
      ["view.theme:light", "light", null],
      ["view.theme:dark", "dark", null],
      ["user.open", null, "user"],
    ])
  })

  it("adds a row without the argument for commands that take none too", () => {
    const rows = paletteEntries([cmd("theme.import", "text")], [])
    expect(rows.map((r) => [r.key, r.arg, r.ask ?? null])).toEqual([
      ["theme.import:bare", null, null],
      ["theme.import", null, "text"],
    ])
  })

  it("lists installed themes, and only user themes to delete", () => {
    const themes = [
      { id: "dark", name: "Dark", builtin: true },
      { id: "mine", name: "Mine", builtin: false },
    ]
    const rows = paletteEntries([cmd("theme.apply", "theme"), cmd("theme.delete", "theme")], [], [], [], themes)
    expect(rows.map((r) => [r.key, r.title])).toEqual([
      ["theme.apply:dark", "theme.apply: Dark"],
      ["theme.apply:mine", "theme.apply: Mine"],
      ["theme.delete:mine", "theme.delete: Mine"],
    ])
  })

  it("lists plugins, skipping ones already on or off", () => {
    const plugins = [
      { id: "on", name: "On one", enabled: true },
      { id: "off", name: "Off one", enabled: false },
    ]
    const rows = paletteEntries(
      [cmd("plugin.enable", "plugin"), cmd("plugin.disable", "plugin"), cmd("plugin.reload", "plugin")],
      [],
      [],
      [],
      [],
      [],
      plugins,
    )
    expect(rows.map((r) => r.key)).toEqual([
      "plugin.enable:off",
      "plugin.disable:on",
      "plugin.reload:on",
      "plugin.reload:off",
    ])
  })

  it("lists playlists for playlist arguments", () => {
    const rows = paletteEntries([cmd("playlist.open", "playlist")], [], [], [], [], [], [], [{ id: 3, name: "Obbies" }])
    expect(rows.map((r) => [r.key, r.title, r.arg])).toEqual([["playlist.open:3", "playlist.open: Obbies", "3"]])
  })

  it("lists accounts for account arguments", () => {
    const rows = paletteEntries(
      [cmd("account.remove", "account")],
      [{ id: 7, name: "alt", displayName: "Alt", label: "Trading" }],
    )
    expect(rows).toHaveLength(1)
    expect(rows[0].title).toBe("account.remove: Trading")
    expect(rows[0].arg).toBe("7")
  })

  it("lists open tabs for tab arguments", () => {
    const rows = paletteEntries([cmd("tab.switch", "tab")], [], [{ key: "t3", title: "Jailbreak" }])
    expect(rows.map((r) => [r.title, r.arg])).toEqual([["tab.switch: Jailbreak", "t3"]])
  })

  it("lists one row per sidebar screen", () => {
    const rows = paletteEntries([cmd("view.sidebar_toggle", "screen")], [], [], [{ key: "trades", title: "Trades" }])
    expect(rows.map((r) => [r.title, r.arg])).toEqual([["view.sidebar_toggle: Trades", "trades"]])
  })

  it("lists on and off for switches and each option for choices", () => {
    const prefs = [
      { key: "notify.sales", section: "notifications", title: "Sales", keywords: [], kind: "bool", options: [] },
      {
        key: "density",
        section: "appearance",
        title: "Density",
        keywords: [],
        kind: "choice",
        options: [{ value: "compact", label: "Compact" }],
      },
      { key: "format_locale", section: "general", title: "Format", keywords: [], kind: "text", options: [] },
    ]
    const rows = paletteEntries([cmd("settings.set", "setting")], [], [], [], [], prefs)
    expect(rows.map((r) => [r.title, r.arg])).toEqual([
      ["Sales: On", "notify.sales on"],
      ["Sales: Off", "notify.sales off"],
      ["Density: Compact", "density compact"],
    ])
    const resets = paletteEntries([cmd("settings.reset", "setting")], [], [], [], [], prefs)
    expect(resets.map((r) => r.arg)).toEqual(["all", "general", "appearance", "notifications"])
  })

  it("keeps the bare settings command next to its sections", () => {
    const rows = paletteEntries([cmd("nav.settings", "settingsSection")], [])
    expect(rows[0].arg).toBeNull()
    expect(rows.some((r) => r.arg === "privacy")).toBe(true)
  })
})

describe("argEntries", () => {
  const ask = (a: CommandInfo["arg"]) => paletteEntries([cmd("x", a)], [])[0]

  it("suggests friends, then Roblox people, then the typed text for user arguments", () => {
    const rows = argEntries(ask("user"), "luk", people, [user(2, "Luke"), user(9, "Luke Two")])
    expect(rows.map((r) => r.arg)).toEqual(["2", "9", "luk"])
    expect(rows.every((r) => r.id === "x")).toBe(true)
  })

  it("only suggests friends in a game for joins, as user targets", () => {
    const rows = argEntries(ask("gameOrFriend"), "b", people)
    expect(rows.map((r) => r.arg)).toEqual(["user:1", "b"])
  })

  it("offers nothing for an empty query", () => {
    expect(argEntries(ask("text"), "  ", people)).toEqual([])
  })
})

describe("remote search", () => {
  it("searches any query of three letters, and ids and links at any length", () => {
    expect(remoteQuery("all", "adopt me")).toBe("adopt me")
    expect(remoteQuery("all", "ad")).toBe("")
    expect(remoteQuery("all", "1")).toBe("1")
    expect(remoteQuery("people", " bob ")).toBe("@bob")
    expect(remoteQuery("experiences", "adopt")).toBe("g:adopt")
    expect(remoteQuery("commands", "adopt")).toBe("")
  })

  it("splits a typed prefix off as a scope", () => {
    expect(parseScope("@ builderman")).toEqual({ scope: "people", text: "builderman" })
    expect(parseScope("g:adopt")).toEqual({ scope: "experiences", text: "adopt" })
    expect(parseScope(">theme")).toEqual({ scope: "commands", text: "theme" })
    expect(parseScope("adopt")).toBeNull()
  })

  it("turns results into rows that run their command, with a picture", () => {
    const rows = remoteEntries([
      {
        group: "Experiences",
        title: "Adopt Me!",
        subtitle: "1K playing",
        command: "game.open",
        arg: "920587237",
        kind: "experience",
        id: 920587237,
        universeId: 383310974,
      },
    ])
    expect(rows[0]).toMatchObject({
      title: "Adopt Me!",
      subtitle: "1K playing",
      group: "Experiences",
      id: "game.open",
      arg: "920587237",
      thumb: { kind: "game_icon", id: 383310974 },
      entity: { kind: "experience", id: 920587237 },
    })
  })
})

const user = (id: number, title: string): PaletteEntry => ({
  ...remoteEntries([
    { group: "People", title, subtitle: "", command: "user.open", arg: String(id), kind: "user", id, universeId: null },
  ])[0],
})

describe("paletteSections", () => {
  const commands = paletteEntries([cmd("Go to Friends", "none"), cmd("Zoom in", "none")], [])
  const input = {
    scope: "all" as const,
    commands,
    friends: people,
    remote: [user(2, "Luke"), user(9, "Luke Two")],
  }

  it("shows only commands for an empty query", () => {
    const s = paletteSections({ ...input, text: "" })
    expect(s.map((x) => x.title)).toEqual(["Commands"])
  })

  it("puts title matches first, then friends, then Roblox results without repeating friends", () => {
    const s = paletteSections({ ...input, text: "lu" })
    expect(s.map((x) => x.title)).toEqual(["Friends", "People"])
    expect(s[1].rows.map((r) => r.entity?.id)).toEqual([9])
    const f = paletteSections({ ...input, text: "friends" })
    expect(f[0].title).toBe("Commands")
  })

  it("shows one kind of result in a scope", () => {
    const s = paletteSections({ ...input, scope: "people", text: "zoom" })
    expect(s.map((x) => x.title)).toEqual(["People"])
  })
})

describe("entityActions", () => {
  const title = (id: string) => `T ${id}`

  it("lists user actions, joining only friends in a game", () => {
    const ids = entityActions({ kind: "user", id: 5 }, title).map((a) => a.id)
    expect(ids[0]).toBe("user.open")
    expect(ids).toContain("trade.new")
    expect(ids).not.toContain("game.join")
    const join = entityActions({ kind: "user", id: 5, inGame: true }, title).find((a) => a.id === "game.join")
    expect(join?.arg).toBe("user:5")
  })

  it("has no action list for items", () => {
    expect(hasActions(user(1, "x"))).toBe(true)
    expect(hasActions({ ...user(1, "x"), entity: { kind: "item", id: 1 } })).toBe(false)
  })
})
