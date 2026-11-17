import { describe, expect, it } from "vitest"
import type { Account, CopyItemView, CopyPlanView } from "~/lib/ipc/bindings"
import { copySummary, copyTargets } from "./copy"

const account = (id: number, session: Account["session"] = "ok") => ({ id, session }) as Account
const item = (id: number, owned: CopyItemView["owned"]): CopyItemView => ({
  id,
  name: `i${id}`,
  typeName: "Hat",
  owned,
})
const plan = (items: CopyItemView[]): CopyPlanView => ({
  loading: false,
  error: null,
  rig: "R15",
  items,
  ready: items.every((i) => i.owned !== "checking"),
  wear: items.filter((i) => i.owned === "yes").map((i) => i.id),
})

describe("copyTargets", () => {
  it("leaves out the source and signed-out accounts", () => {
    const all = [account(1), account(2), account(3, "expired")]
    expect(copyTargets(all, 1).map((a) => a.id)).toEqual([2])
  })
})

describe("copySummary", () => {
  it("counts progress, then what is worn and left off", () => {
    expect(copySummary(plan([item(1, "yes"), item(2, "checking")]))).toBe("Checking which items it owns… 1 of 2")
    expect(copySummary(plan([item(1, "yes"), item(2, "no"), item(3, "unknown")]))).toBe(
      "Wears 1 of 3 items, plus body type, colors and scales. 2 not owned or not checked will be left off.",
    )
    expect(copySummary(plan([item(1, "yes")]))).toBe("Wears 1 of 1 items, plus body type, colors and scales.")
    expect(copySummary({ ...plan([]), error: "Nope" })).toBe("Nope")
  })
})
