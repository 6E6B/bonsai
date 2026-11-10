import { describe, expect, it, vi } from "vitest"
import type { Running } from "~/lib/ipc/bindings"

const shell = vi.hoisted(() => ({ active: 1 as number | null, running: [] as Running[] }))
vi.mock("~/lib/store", () => ({ shell }))

const { begin, isRunning, listed } = await import("./running")

describe("listed", () => {
  const list: Running[] = [{ id: "trade.accept", arg: "7", account: 1 }]

  it("matches id, argument and account", () => {
    expect(listed(list, "trade.accept", "7", 1)).toBe(true)
    expect(listed(list, "trade.accept", "8", 1)).toBe(false)
    expect(listed(list, "trade.decline", "7", 1)).toBe(false)
    expect(listed(list, "trade.accept", "7", 2)).toBe(false)
  })

  it("tells no argument from an empty one", () => {
    const none: Running[] = [{ id: "group.join", arg: null, account: 1 }]
    expect(listed(none, "group.join", null, 1)).toBe(true)
    expect(listed(none, "group.join", "", 1)).toBe(false)
  })
})

describe("isRunning", () => {
  it("is true from the click until the end function runs", () => {
    expect(isRunning("user.add_friend", "5")).toBe(false)
    const end = begin("user.add_friend", "5", null)
    expect(isRunning("user.add_friend", "5")).toBe(true)
    end()
    expect(isRunning("user.add_friend", "5")).toBe(false)
  })

  it("keeps one run going when another of the same command ends", () => {
    const a = begin("message.archive", null, null)
    const b = begin("message.archive", null, null)
    a()
    expect(isRunning("message.archive")).toBe(true)
    b()
    expect(isRunning("message.archive")).toBe(false)
  })

  it("follows what the backend lists, for the active account unless one is given", () => {
    shell.running = [{ id: "game.join", arg: "9", account: 2 }]
    expect(isRunning("game.join", "9")).toBe(false)
    expect(isRunning("game.join", "9", 2)).toBe(true)
    shell.active = 2
    expect(isRunning("game.join", "9")).toBe(true)
    shell.running = []
    shell.active = 1
  })
})
