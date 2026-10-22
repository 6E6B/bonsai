import { describe, expect, it, vi } from "vitest"

const runCommand = vi.fn(async () => ({ status: "ok" as const, data: null }))
vi.mock("~/lib/ipc/bindings", () => ({ commands: { runCommand, commands: async () => [] } }))

const { handle, run } = await import("./registry")

describe("handle", () => {
  it("keeps a newer handler when an older one is removed", async () => {
    const first = vi.fn()
    const second = vi.fn()
    const offFirst = handle("x.rename", first)
    handle("x.rename", second)
    offFirst()
    await run("x.rename", "3")
    expect(second).toHaveBeenCalledWith("3", null)
    expect(first).not.toHaveBeenCalled()
    expect(runCommand).not.toHaveBeenCalled()
  })
})
