import { describe, expect, it, vi } from "vitest"

const runCommand = vi.fn(async () => ({ status: "ok" as const, data: null }))
vi.mock("~/lib/ipc/bindings", () => ({ commands: { runCommand, commands: async () => [] } }))

const { handle, run, setCommandContext } = await import("./registry")

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

describe("image command context", () => {
  it("uses the selected image for palette actions and clears it on navigation", async () => {
    const { selectImage } = await import("~/lib/image-actions")
    setCommandContext("item:123")
    selectImage({ key: "asset.123.s", name: "Selected" })
    await run("image.copy")
    expect(runCommand).toHaveBeenLastCalledWith({
      id: "image.copy",
      arg: JSON.stringify({ key: "asset.123.s", name: "Selected" }),
      asAccount: null,
      context: "item:123",
    })
    await run("image.copy", "asset.456.l")
    expect(runCommand).toHaveBeenLastCalledWith(expect.objectContaining({ arg: "asset.456.l" }))
    setCommandContext("user:1")
    await run("image.copy")
    expect(runCommand).toHaveBeenLastCalledWith(expect.objectContaining({ arg: null, context: "user:1" }))
    setCommandContext(null)
  })
})
