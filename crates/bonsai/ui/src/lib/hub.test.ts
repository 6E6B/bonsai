import { describe, expect, it } from "vitest"
import { publishProblem, target } from "./hub"
import type { HubEntry } from "./ipc/bindings"

const valid = { hubId: "night-sky", name: "Night sky", version: "1.2.0", description: "A theme", tags: "dark, blue" }
describe("hub publishing", () => {
  it("accepts valid releases and prereleases", () => {
    expect(publishProblem(valid)).toBeUndefined()
    expect(publishProblem({ ...valid, version: "2.0.0-beta.1" })).toBeUndefined()
  })
  it("rejects unsafe IDs, ambiguous versions and invalid tags", () => {
    for (const hubId of ["../theme", "Theme", "-theme", "", "a".repeat(65)])
      expect(publishProblem({ ...valid, hubId })).toBeTruthy()
    for (const version of ["latest", "1", "01.0.0", "1.2.3/../x"])
      expect(publishProblem({ ...valid, version })).toBeTruthy()
    expect(publishProblem({ ...valid, tags: "dark, two words" })).toBeTruthy()
    expect(publishProblem({ ...valid, description: "x".repeat(2001) })).toBeTruthy()
  })
  it("passes only identity to installation", () => {
    const entry = {
      id: "theme",
      kind: "themes",
      version: "1.0.0",
      document: "untrusted",
      ownerId: "publisher",
    } as HubEntry
    expect(JSON.parse(target(entry))).toEqual({ id: "theme", kind: "themes", version: "1.0.0" })
  })
})
