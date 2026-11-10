import { createRoot } from "solid-js"
import { afterEach, describe, expect, it, vi } from "vitest"
import type { UpdateStatus } from "~/lib/ipc/bindings"
import { refreshUpdates, updateBusy, updateMessage, updates, useUpdates } from "./updates"

const updateStatus = vi.hoisted(() => vi.fn())
vi.mock("@tauri-apps/api/core", () => ({ isTauri: () => true }))
vi.mock("~/lib/ipc/bindings", () => ({ commands: { updateStatus } }))

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

describe("polling", () => {
  afterEach(() => vi.useRealTimers())

  it("reads the status at once when asked, not at the next poll", async () => {
    vi.useFakeTimers()
    updateStatus.mockResolvedValue(status())
    const dispose = createRoot((dispose) => {
      useUpdates()
      return dispose
    })
    await vi.advanceTimersByTimeAsync(0)
    expect(updateStatus).toHaveBeenCalledTimes(1)

    updateStatus.mockResolvedValue(status({ phase: "checking" }))
    refreshUpdates()
    await vi.advanceTimersByTimeAsync(0)
    expect(updateStatus).toHaveBeenCalledTimes(2)
    expect(updates.phase).toBe("checking")

    // Busy phases poll fast, and the early read didn't start a second timer chain.
    await vi.advanceTimersByTimeAsync(500)
    expect(updateStatus).toHaveBeenCalledTimes(3)

    dispose()
    refreshUpdates()
    await vi.advanceTimersByTimeAsync(5000)
    expect(updateStatus).toHaveBeenCalledTimes(3)
  })
})
