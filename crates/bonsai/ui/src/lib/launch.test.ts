import { describe, expect, it } from "vitest"
import type { LaunchView } from "~/lib/ipc/bindings"
import { joinConfirm, launchElapsed, launchFor, launchStatus, onTheWay, showLaunch } from "./launch"

const at = 1_700_000_000_000

function launch(over: Partial<LaunchView> = {}): LaunchView {
  return {
    account: 1,
    phase: "inGame",
    live: true,
    placeId: 920587237,
    universeId: 340227283,
    jobId: null,
    game: "Adopt",
    code: null,
    reason: null,
    startedAt: at,
    phaseAt: at,
    joinedAt: at + 10_000,
    endedAt: null,
    ...over,
  }
}

describe("launch status", () => {
  it("counts from the first join", () => {
    expect(launchElapsed(launch(), at + 10_000 + 65_000)).toBe("1:05")
    expect(launchElapsed(launch({ joinedAt: null }), at + 3_000)).toBe("0:03")
    expect(launchElapsed(launch(), at + 10_000 + 3_725_000)).toBe("1:02:05")
  })
  it("stops counting when it ends", () => {
    const l = launch({ live: false, phase: "closed", endedAt: at + 70_000 })
    expect(launchElapsed(l, at + 999_000)).toBe("1:00")
  })
  it("names the reason", () => {
    expect(launchStatus(launch({ phase: "disconnected", reason: "Kicked by the experience" }))).toBe(
      "Disconnected: Kicked by the experience",
    )
    expect(launchStatus(launch({ phase: "joining" }))).toBe("Joining")
  })
  it("keeps bad ends for a while and drops normal ones", () => {
    const crashed = launch({ live: false, phase: "crashed", endedAt: at })
    expect(showLaunch(crashed, at + 60_000)).toBe(true)
    expect(showLaunch(crashed, at + 10 * 60_000)).toBe(false)
    expect(showLaunch(launch({ live: false, phase: "closed", endedAt: at }), at)).toBe(false)
    expect(showLaunch(launch({ phase: "untracked" }), at)).toBe(false)
    expect(showLaunch(null, at)).toBe(false)
  })
  it("spins only on the way in", () => {
    expect(onTheWay(launch({ phase: "joining" }))).toBe(true)
    expect(onTheWay(launch())).toBe(false)
    expect(onTheWay(launch({ phase: "joining", live: false }))).toBe(false)
  })
  it("matches the place", () => {
    expect(launchFor(launch(), 920587237)).not.toBeNull()
    expect(launchFor(launch(), 1)).toBeNull()
    expect(launchFor(launch({ live: false }), 920587237)).toBeNull()
  })
})

describe("join confirmation", () => {
  it("asks only while Roblox is open", () => {
    expect(joinConfirm(null, false)).toBeNull()
    expect(joinConfirm(launch({ live: false, phase: "closed" }), false)).toBeNull()
    expect(joinConfirm(null, true)?.confirm).toBe("Close and join")
  })
  it("names the game it's in", () => {
    expect(joinConfirm(launch(), true)?.description).toBe("Roblox is open in Adopt. Joining closes it first.")
    expect(joinConfirm(launch({ phase: "joining" }), false)?.description).toBe(
      "Roblox is open. Joining closes it first.",
    )
  })
  it("doesn't ask while Bonsai is still getting a ticket", () => {
    expect(joinConfirm(launch({ phase: "preparing" }), false)).toBeNull()
  })
})
