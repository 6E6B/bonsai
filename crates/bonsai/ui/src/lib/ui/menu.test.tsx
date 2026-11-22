import { cleanup, fireEvent, render, screen } from "@solidjs/testing-library"
import { afterEach, describe, expect, test, vi } from "vitest"
import type { MenuEntry } from "./menu"

const runCommand = vi.fn(async () => ({ status: "ok" as const, data: null }))
vi.mock("~/lib/ipc/bindings", () => ({ commands: { runCommand, commands: async () => [] } }))
const { ContextMenu, ImageMenu, withImageRows } = await import("./menu")
const { Avatar } = await import("./avatar")
const { Thumbnail } = await import("./thumbnail")

afterEach(() => {
  cleanup()
  runCommand.mockClear()
})
const src = "thumb://localhost/asset.123.s"

test("adds image actions to the existing menu under one row", async () => {
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
  expect(screen.getByRole("menuitem", { name: "Image" })).toBeTruthy()
  expect(screen.queryByRole("menuitem", { name: "Save image as…" })).toBeNull()
  expect(action).not.toHaveBeenCalled()
})

describe("withImageRows", () => {
  const row = (label: string, danger = false): MenuEntry => ({ label, onSelect: () => {}, danger })
  const img = [row("Save image as…")]
  const labels = (rows: MenuEntry[]) => rows.map((r) => ("label" in r ? r.label : r.kind))

  test("shows image rows flat when there is nothing else", () => {
    expect(withImageRows([], img)).toBe(img)
  })

  test("adds them to Share when the menu has one", () => {
    const share: MenuEntry = { kind: "submenu", id: "share", label: "Share", items: [row("Copy link")] }
    const rows = withImageRows([row("Open"), share], img)
    expect(labels(rows)).toEqual(["Open", "Share"])
    const s = rows[1]
    expect(s.kind === "submenu" && labels(s.items)).toEqual(["Copy link", "separator", "Save image as…"])
    expect(share.kind === "submenu" && share.items).toHaveLength(1)
  })

  test("puts an Image submenu above trailing destructive rows", () => {
    const rows = withImageRows([row("Open"), { kind: "separator" }, row("Delete", true)], img)
    expect(labels(rows)).toEqual(["Open", "Image", "separator", "Delete"])
  })

  test("puts an Image submenu last otherwise", () => {
    expect(labels(withImageRows([row("Open"), { kind: "separator" }, row("Edit")], img))).toEqual([
      "Open",
      "separator",
      "Edit",
      "Image",
    ])
  })
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
  expect(await screen.findByRole("menuitem", { name: "Copy image" })).toBeTruthy()
  expect(screen.getByRole("menuitem", { name: "Copy image URL" })).toBeTruthy()
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
  fireEvent.keyDown(await screen.findByRole("menuitem", { name: "Copy image URL" }), { key: "Enter" })
  expect(runCommand).toHaveBeenCalledWith(
    expect.objectContaining({
      id: "image.copy_url",
      arg: JSON.stringify({ key: "asset.123.s", name: "Loading item" }),
    }),
  )
})
