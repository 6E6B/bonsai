import { cleanup, fireEvent, render, screen } from "@solidjs/testing-library"
import { afterEach, expect, test, vi } from "vitest"

const runCommand = vi.fn(async () => ({ status: "ok" as const, data: null }))
vi.mock("~/lib/ipc/bindings", () => ({ commands: { runCommand, commands: async () => [] } }))
const { ContextMenu, ImageMenu } = await import("./menu")
const { Avatar } = await import("./avatar")
const { Thumbnail } = await import("./thumbnail")

afterEach(() => {
  cleanup()
  runCommand.mockClear()
})
const src = "thumb://localhost/asset.123.s"

test("adds image actions to the existing menu and sends an opaque identity", async () => {
  const action = vi.fn()
  const r = render(() => (
    <ContextMenu items={[{ label: "Open item", onSelect: action }]}>
      <ImageMenu src={src}>
        <img src={src} alt="Catalog item" />
      </ImageMenu>
    </ContextMenu>
  ))
  expect(r.container.querySelectorAll("[data-slot=context-trigger]")).toHaveLength(1)
  fireEvent.contextMenu(r.getByAltText("Catalog item"))
  expect(await screen.findByRole("menuitem", { name: "Open item" })).toBeTruthy()
  fireEvent.keyDown(screen.getByRole("menuitem", { name: "Save Image As…" }), { key: "Enter" })
  expect(runCommand).toHaveBeenCalledWith(
    expect.objectContaining({
      id: "image.save",
      arg: JSON.stringify({ key: "asset.123.s", name: "Catalog item" }),
    }),
  )
  expect(action).not.toHaveBeenCalled()
})

test("standalone images open a menu with the keyboard", async () => {
  const r = render(() => (
    <ImageMenu src={src}>
      <button type="button">
        <img src={src} alt="Outfit" />
      </button>
    </ImageMenu>
  ))
  fireEvent.keyDown(r.getByAltText("Outfit"), { key: "F10", shiftKey: true })
  expect(await screen.findByRole("menuitem", { name: "Copy Image" })).toBeTruthy()
  expect(screen.getByRole("menuitem", { name: "Copy Image URL" })).toBeTruthy()
})

test("shared images provide menus only when outside an existing menu", () => {
  const r = render(() => (
    <>
      <Avatar src={src} name="User" />
      <Thumbnail src={src} alt="Item" />
      <ContextMenu items={[]}>
        <Thumbnail src={src} alt="Nested item" />
      </ContextMenu>
      <Thumbnail alt="Fallback" />
    </>
  ))
  expect(r.container.querySelectorAll("[data-slot=context-trigger]")).toHaveLength(3)
  expect(r.container.querySelectorAll("[tabindex='0']")).toHaveLength(2)
})

test("a loading thumbnail retains its image identity", async () => {
  const r = render(() => <Thumbnail src={src} alt="Loading item" />)
  const image = r.container.querySelector("[data-component=thumbnail]")
  expect(image).toBeTruthy()
  if (!image) return
  fireEvent.contextMenu(image)
  fireEvent.keyDown(await screen.findByRole("menuitem", { name: "Copy Image URL" }), { key: "Enter" })
  expect(runCommand).toHaveBeenCalledWith(
    expect.objectContaining({
      id: "image.copy_url",
      arg: JSON.stringify({ key: "asset.123.s", name: "Loading item" }),
    }),
  )
})
