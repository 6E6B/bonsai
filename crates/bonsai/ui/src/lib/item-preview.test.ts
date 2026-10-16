import { describe, expect, it } from "vitest"
import { parseItemTarget, wantsPreview } from "./item-preview"

describe("parseItemTarget", () => {
  it("reads items, bundles and bare ids", () => {
    expect(parseItemTarget("item:5")).toEqual({ id: 5, bundle: false })
    expect(parseItemTarget(" bundle:7 ")).toEqual({ id: 7, bundle: true })
    expect(parseItemTarget("1342550")).toEqual({ id: 1342550, bundle: false })
  })

  it("rejects other targets", () => {
    expect(parseItemTarget("pass:9")).toBeNull()
    expect(parseItemTarget("")).toBeNull()
    expect(parseItemTarget(null)).toBeNull()
  })
})

describe("wantsPreview", () => {
  it("previews on Ctrl or Cmd", () => {
    expect(wantsPreview(new MouseEvent("click", { ctrlKey: true }))).toBe(true)
    expect(wantsPreview(new MouseEvent("click", { metaKey: true }))).toBe(true)
    expect(wantsPreview(new MouseEvent("click"))).toBe(false)
  })
})
