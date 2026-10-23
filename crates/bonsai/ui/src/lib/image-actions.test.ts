import { describe, expect, it } from "vitest"
import { imageAt, imageKey, selectedImageArgument, selectImage } from "./image-actions"

const src = "thumb://localhost/asset.123.s"

describe("image identity", () => {
  it("accepts every thumb:// kind on Linux/macOS and mapped Windows URLs", () => {
    for (const kind of [
      "headshot",
      "avatar",
      "asset",
      "bundle",
      "outfit",
      "game_icon",
      "game_thumbnail",
      "group_icon",
      "badge_icon",
      "game_pass",
    ]) {
      for (const origin of ["thumb://localhost", "http://thumb.localhost", "https://thumb.localhost"]) {
        expect(imageKey(`${origin}/${kind}.123.l`)).toBe(`${kind}.123.l`)
      }
    }
  })

  it("rejects arbitrary URLs, paths, invalid IDs and other app protocols", () => {
    for (const url of [
      undefined,
      "",
      "/asset.123.s",
      "file:///asset.123.s",
      "https://example.com/asset.123.s",
      "https://thumb.localhost.evil/asset.123.s",
      "thumb://user@localhost/asset.123.s",
      "thumb://localhost/asset.0.s",
      "thumb://localhost/asset.9007199254740992.s",
      "thumb://localhost/asset.123.x",
      `${src}?x=1`,
      `${src}#hash`,
      `${src}.extra`,
      "bg://localhost/image",
      "thumb://localhost/%invalid",
    ]) {
      expect(imageKey(url)).toBeUndefined()
    }
  })
})

describe("image menu selection", () => {
  it("uses the clicked image in a multi-image row and a row image for keyboard actions", () => {
    const root = document.createElement("div")
    root.innerHTML = `<button><img src="${src}" alt="First"><img src="thumb://localhost/outfit.456.l" alt="Second"><span>Row</span></button>`
    const second = root.querySelectorAll("img")[1]
    expect(imageAt(second, root)).toEqual({ key: "outfit.456.l", name: "Second" })
    expect(imageAt(root.querySelector("span"), root)).toEqual({ key: "asset.123.s", name: "First" })
    expect(imageAt(document.createElement("img"), root)).toBeUndefined()
  })

  it("keeps a command-palette selection until it is cleared", () => {
    selectImage({ key: "asset.123.s", name: "Item" })
    expect(JSON.parse(selectedImageArgument() ?? "")).toEqual({ key: "asset.123.s", name: "Item" })
    selectImage(undefined)
    expect(selectedImageArgument()).toBeNull()
  })
})
