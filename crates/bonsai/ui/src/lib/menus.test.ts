import { describe, expect, it } from "vitest"
import type { MenuEntry } from "~/lib/ui"
import { itemMenu, templateItems, userMenu } from "./menus"

const labels = (rows: MenuEntry[]) => rows.map((r) => ("label" in r ? r.label : r.kind))
const sub = (rows: MenuEntry[], label: string) => {
  const s = rows.find((r) => r.kind === "submenu" && r.label === label)
  return s?.kind === "submenu" ? s.items : undefined
}

describe("templateItems", () => {
  it("offers both downloads for shirts and pants", () => {
    expect(labels(templateItems(1, 11))).toEqual(["market.template", "market.template_masked"])
    expect(labels(templateItems(1, 12))).toHaveLength(2)
  })

  it("offers only the plain download for T-shirts", () => {
    expect(labels(templateItems(1, 2))).toEqual(["market.template"])
  })

  it("offers nothing for other items or unknown types", () => {
    expect(templateItems(1, 8)).toEqual([])
    expect(templateItems(1, undefined)).toEqual([])
  })
})

describe("itemMenu", () => {
  it("groups both template downloads for shirts and pants", () => {
    expect(sub(itemMenu(1, false, 11), "Download template")).toHaveLength(2)
  })

  it("shows a single template row for T-shirts", () => {
    expect(labels(itemMenu(1, false, 2))).toContain("market.template")
  })

  it("leaves bundles alone", () => {
    expect(itemMenu(1, true, 11)).toHaveLength(itemMenu(1, false, 8).length)
  })

  it("puts screen actions under Open and destructive rows last", () => {
    const act: MenuEntry = { label: "Inspect", onSelect: () => {} }
    const del: MenuEntry = { label: "Delete", onSelect: () => {}, danger: true }
    const rows = itemMenu(1, false, 8, { actions: [act], danger: [del] })
    expect(rows[1]).toBe(act)
    expect(rows.at(-1)).toBe(del)
  })

  it("keeps copy and website rows in Share", () => {
    expect(labels(sub(itemMenu(1), "Share") ?? [])).toHaveLength(3)
  })
})

describe("userMenu", () => {
  it("keeps the top level short", () => {
    const top = userMenu(1, { friend: true, inGame: true, placeId: 2, following: true })
    expect(top.filter((r) => r.kind !== "separator").length).toBeLessThanOrEqual(10)
  })

  it("puts remove friend, block and report under Manage", () => {
    const manage = labels(sub(userMenu(1, { friend: true }), "Manage") ?? [])
    expect(manage).toEqual(expect.arrayContaining(["user.unfriend", "user.block", "Report"]))
  })

  it("leaves out Open profile on the profile itself", () => {
    expect(labels(userMenu(1, { onProfile: true }))).not.toContain("user.open")
  })

  it("only offers Find server when the game isn't known", () => {
    expect(labels(userMenu(1, { inGame: true }))).not.toContain("Find server")
    expect(labels(userMenu(1))).toContain("Find server")
  })
})
