import { describe, expect, it } from "vitest"
import type { UpdateStatus } from "~/lib/ipc/bindings"
import { updateBusy, updateMessage } from "./updates"

const status = (patch: Partial<UpdateStatus> = {}): UpdateStatus => ({
  phase: "idle",
  currentVersion: "1.0.0",
  version: "1.1.0",
  notes: null,
  downloaded: 0,
  total: null,
  checkedAt: null,
  error: null,
  unavailableReason: null,
  ...patch,
})

describe("update status", () => {
  it("does not claim the app is current before a successful check", () => {
    expect(updateMessage(status())).toContain("Check for")
    expect(updateMessage(status({ checkedAt: 1000 }))).toContain("up to date")
  })
  it("keeps checks, downloads, and installation busy while allowing a pending download to wait", () => {
    for (const phase of ["checking", "downloading", "verifying", "installing"] as const)
      expect(updateBusy(phase)).toBe(true)
    for (const phase of ["idle", "available", "ready", "unavailable"] as const) expect(updateBusy(phase)).toBe(false)
  })
  it("names the pending version and explains installation restarts the app", () => {
    expect(updateMessage(status({ phase: "available" }))).toContain("1.1.0")
    expect(updateMessage(status({ phase: "ready" }))).toContain("ready to install")
    expect(updateMessage(status({ phase: "installing" }))).toContain("restart")
  })
  it("explains package-manager and development build restrictions", () => {
    expect(updateMessage(status({ phase: "unavailable", unavailableReason: "Use your package manager." }))).toBe(
      "Use your package manager.",
    )
  })
})
