import { cleanup, render } from "@solidjs/testing-library"
import { createSignal } from "solid-js"
import { afterEach, expect, it, vi } from "vitest"
import type { BackgroundView } from "~/lib/ipc/bindings"
import { BACKGROUND_DEFAULTS } from "~/lib/theme/theme"
import { Backdrop } from "./Backdrop"

vi.mock("~/lib/thumb", () => ({
  backgroundUrl: (url: string) => (url ? `bg://image?url=${encodeURIComponent(url)}` : undefined),
}))
afterEach(cleanup)

it("keeps one image node and source through effect edits and store refreshes", () => {
  const original = { ...BACKGROUND_DEFAULTS, file: "https://example.com/test.gif" }
  const [background, setBackground] = createSignal<BackgroundView | null>(original)
  const { container } = render(() => <Backdrop background={background()} />)
  const image = container.querySelector("img")
  if (!image) throw new Error("Background image did not mount")
  const source = image.getAttribute("src")
  setBackground({ ...original })
  expect(container.querySelector("img")).toBe(image)
  setBackground({ ...original, blur: 8, dim: 25 })
  expect(container.querySelector("img")).toBe(image)
  expect(image.getAttribute("src")).toBe(source)
  expect(image.style.filter).toContain("blur(8px)")
  setBackground({ ...original, file: "https://example.com/other.gif" })
  expect(container.querySelector("img")?.getAttribute("src")).toContain("other.gif")
  setBackground(null)
  expect(container.querySelector("img")).toBeNull()
})
