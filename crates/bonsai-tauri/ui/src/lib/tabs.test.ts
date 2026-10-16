import { describe, expect, it } from "vitest"
import {
  activate,
  activeTab,
  close,
  closeOthers,
  closeRight,
  duplicate,
  goTo,
  initialTabs,
  isEntityPath,
  move,
  open,
  reopen,
  step,
  type TabsState,
  togglePin,
  visit,
} from "./tabs"

const paths = (s: TabsState) => s.tabs.map((t) => t.path)
const at = (s: TabsState, path: string) => s.tabs.find((t) => t.path === path)?.key ?? ""
const three = () => visit(visit(visit(initialTabs("/"), "/games/1"), "/games/2"), "/games/3")

describe("isEntityPath", () => {
  it("gives items, bundles and passes their own tabs", () => {
    expect(isEntityPath("/catalog/5")).toBe(true)
    expect(isEntityPath("/bundles/5")).toBe(true)
    expect(isEntityPath("/passes/9")).toBe(true)
    expect(isEntityPath("/inventory/5")).toBe(false)
  })
})

describe("visit", () => {
  it("loads screens in the current tab", () => {
    const s = visit(initialTabs("/"), "/friends")
    expect(paths(s)).toEqual(["/friends"])
  })

  it("opens entities in a new tab after the active one", () => {
    let s = visit(initialTabs("/"), "/games/1")
    s = visit(activate(s, s.tabs[0].key), "/users/2")
    expect(paths(s)).toEqual(["/", "/users/2", "/games/1"])
    expect(activeTab(s)?.path).toBe("/users/2")
  })

  it("focuses the tab already showing an entity", () => {
    let s = visit(initialTabs("/"), "/games/1")
    s = visit(activate(s, s.tabs[0].key), "/games/1")
    expect(s.tabs).toHaveLength(2)
    expect(activeTab(s)?.path).toBe("/games/1")
  })

  it("keeps a query change on an entity in its tab", () => {
    let s = visit(initialTabs("/"), "/games/1")
    s = visit(s, "/games/1?tab=servers")
    expect(paths(s)).toEqual(["/", "/games/1?tab=servers"])
  })

  it("does nothing when already there", () => {
    const s = initialTabs("/")
    expect(visit(s, "/")).toBe(s)
  })

  it("opens another screen from a pinned tab in a new tab", () => {
    let s = togglePin(initialTabs("/trades"), "t0")
    s = visit(s, "/friends")
    expect(paths(s)).toEqual(["/trades", "/friends"])
    s = visit(activate(s, "t0"), "/trades?q=x")
    expect(paths(s)).toEqual(["/trades?q=x", "/friends"])
  })
})

describe("open", () => {
  it("opens background tabs in order after the active one", () => {
    let s = open(initialTabs("/"), "/games/1", { background: true })
    s = open(s, "/games/2", { background: true })
    expect(paths(s)).toEqual(["/", "/games/1", "/games/2"])
    expect(activeTab(s)?.path).toBe("/")
  })

  it("skips a background entity already open", () => {
    const s = open(initialTabs("/games/1"), "/games/1", { background: true })
    expect(s.tabs).toHaveLength(1)
  })

  it("never lands among pinned tabs", () => {
    let s = togglePin(visit(initialTabs("/"), "/games/1"), "t0")
    s = togglePin(s, at(s, "/games/1"))
    s = open(activate(s, "t0"), "/friends")
    expect(paths(s)).toEqual(["/", "/games/1", "/friends"])
  })
})

describe("close", () => {
  it("activates the right neighbor, else the left", () => {
    let s = activate(three(), at(three(), "/games/2"))
    expect(activeTab(close(s, s.active))?.path).toBe("/games/3")
    s = activate(three(), at(three(), "/games/1"))
    s = activate(s, at(s, "/games/3"))
    expect(activeTab(close(s, s.active))?.path).toBe("/games/2")
  })

  it("goes back to the opener while the user stayed on the new tab", () => {
    let s = visit(visit(initialTabs("/"), "/friends"), "/games/1")
    s = visit(activate(s, at(s, "/friends")), "/games/2")
    // games/2 opened from friends, before games/1; closing it returns to friends
    expect(paths(s)).toEqual(["/friends", "/games/2", "/games/1"])
    expect(activeTab(close(s, s.active))?.path).toBe("/friends")
  })

  it("keeps the active tab when closing another", () => {
    const s = three()
    expect(activeTab(close(s, s.tabs[0].key))?.path).toBe("/games/3")
  })

  it("leaves a Home tab when the last one closes", () => {
    const s = visit(initialTabs("/"), "/friends")
    expect(paths(close(s, s.active))).toEqual(["/"])
  })

  it("closes others and to the right, sparing pinned tabs", () => {
    let s = togglePin(three(), "t0")
    s = closeOthers(s, at(s, "/games/2"))
    expect(paths(s)).toEqual(["/", "/games/2"])
    expect(activeTab(s)?.path).toBe("/games/2")
    s = closeRight(three(), at(three(), "/games/1"))
    expect(paths(s)).toEqual(["/", "/games/1"])
  })

  it("forgets titles nothing uses", () => {
    let s = three()
    s = { ...s, titles: { "/games/1": "One" } }
    for (let i = 0; i < 30; i++) s = close(visit(s, `/users/${i}`), `t${s.next}`)
    s = close(s, at(s, "/games/1"))
    expect(s.titles["/games/1"]).toBe("One")
    for (let i = 0; i < 30; i++) s = close(visit(s, `/users/${i}`), `t${s.next}`)
    expect(s.titles["/games/1"]).toBeUndefined()
  })
})

describe("reopen", () => {
  it("reopens what was closed where it was, most recent first", () => {
    let s = three()
    s = close(s, s.tabs[1].key)
    s = close(s, s.tabs[1].key)
    s = reopen(s)
    expect(activeTab(s)?.path).toBe("/games/2")
    s = reopen(s)
    expect(paths(s)).toEqual(["/", "/games/1", "/games/2", "/games/3"])
    expect(reopen(s)).toBe(s)
  })

  it("restores a pinned tab as pinned", () => {
    let s = togglePin(three(), at(three(), "/games/2"))
    s = reopen(close(s, at(s, "/games/2")))
    expect(s.tabs[0]).toMatchObject({ path: "/games/2", pinned: true })
  })
})

describe("move and pin", () => {
  it("moves a tab", () => {
    const s = move(three(), "t0", 2)
    expect(paths(s)).toEqual(["/games/1", "/games/2", "/", "/games/3"])
  })

  it("keeps pinned tabs first", () => {
    let s = togglePin(three(), at(three(), "/games/3"))
    expect(paths(s)).toEqual(["/games/3", "/", "/games/1", "/games/2"])
    expect(paths(move(s, at(s, "/games/1"), 0))).toEqual(["/games/3", "/games/1", "/", "/games/2"])
    expect(move(s, at(s, "/games/3"), 3)).toBe(s)
    s = togglePin(s, at(s, "/games/3"))
    expect(s.tabs[0].pinned).toBeUndefined()
  })
})

describe("step, goTo and duplicate", () => {
  it("wraps both ways", () => {
    let s = visit(visit(initialTabs("/"), "/games/1"), "/games/2")
    s = step(s, 1)
    expect(activeTab(s)?.path).toBe("/")
    s = step(s, -1)
    expect(activeTab(s)?.path).toBe("/games/2")
  })

  it("goes to a numbered or the last tab", () => {
    const s = three()
    expect(activeTab(goTo(s, 1))?.path).toBe("/")
    expect(activeTab(goTo(s, Number.POSITIVE_INFINITY))?.path).toBe("/games/3")
    expect(goTo(s, 9)).toBe(s)
  })

  it("duplicates after the original", () => {
    const s = duplicate(three(), "t0")
    expect(paths(s)).toEqual(["/", "/", "/games/1", "/games/2", "/games/3"])
    expect(s.active).toBe(s.tabs[1].key)
  })
})
