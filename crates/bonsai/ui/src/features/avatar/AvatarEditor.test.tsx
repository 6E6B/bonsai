import { cleanup, fireEvent, render, waitFor } from "@solidjs/testing-library"
import { For } from "solid-js"
import { afterEach, expect, test, vi } from "vitest"
import AvatarEditor from "./AvatarEditor"

const mocks = vi.hoisted(() => ({ run: vi.fn() }))
vi.mock("~/lib/avatar3d/viewer", () => ({ AvatarViewer: () => <div>Avatar preview</div> }))
vi.mock("~/lib/commands/hooks", () => ({ useSubTabs: vi.fn() }))
vi.mock("~/lib/commands/registry", async (original) => ({ ...(await original<object>()), run: mocks.run }))
vi.mock("~/lib/ui", async (original) => ({
  ...(await original<object>()),
  usePrompt: () => vi.fn(),
  Slider: (props: { label: string; disabled: boolean }) => (
    <input type="range" aria-label={props.label} disabled={props.disabled} />
  ),
  ColorPicker: (props: { label: string; value: string; onChange: (v: string) => void }) => (
    <input aria-label={props.label} value={props.value} onInput={(e) => props.onChange(e.currentTarget.value)} />
  ),
  VirtualGrid: (props: { items: unknown[]; children: (item: unknown) => unknown }) => (
    <For each={props.items}>{props.children as never}</For>
  ),
}))
vi.mock("~/lib/store", async () => {
  const { commands } = await import("~/lib/ipc/bindings")
  const list = (items: unknown[]) => ({ loading: false, error: null, items, more: false })
  return {
    shell: { active: 1 },
    useLive: (_source: unknown, fetch: unknown) => {
      if (fetch === commands.avatar)
        return {
          loading: false,
          error: null,
          rig: "R6",
          rev: 1,
          wearing: [],
          emotes: [],
          colors: [
            { part: "head", label: "Head", color: 1, hex: "#123456" },
            { part: "torso", label: "Torso", color: 2, hex: "#654321" },
          ],
          scales: [{ name: "height", label: "Height", value: 1, min: 0.9, max: 1.05, step: 0.01 }],
        }
      if (fetch === commands.outfits)
        return { list: list([{ id: "1", name: "Explorer", editable: true, rev: 1 }]), moreKey: "" }
      if (fetch === commands.wardrobe)
        return list([
          {
            name: "Accessories",
            slots: [
              { value: "a8", label: "Hats" },
              { value: "a41", label: "Hair" },
            ],
          },
        ])
      if (fetch === commands.recentItems)
        return list([{ id: "1", name: "Blue Hat", outfit: false, typeId: 8, typeName: "Hat" }])
      if (fetch === commands.bodyPalette) return [{ id: 1, name: "Custom swatch", hex: "#123456", basic: true }]
      const source = (_source as () => unknown)()
      if (Array.isArray(source) && source[1] === "wardrobe")
        return {
          list: list([
            { id: 90001, name: "Red Hat", assetType: 8, copies: 1, limited: false, bundle: false },
            { id: 90004, name: "Red Shirt", assetType: 11, copies: 1, limited: false, bundle: false },
          ]),
          moreKey: "inventory:1:wardrobe:false",
        }
      return { list: list([]), moreKey: "" }
    },
  }
})

afterEach(() => {
  cleanup()
  mocks.run.mockReset()
})

test("organizes navigation and exposes an empty filtered state", async () => {
  const r = render(() => <AvatarEditor />)
  expect(r.getByRole("tab", { name: "Wardrobe", selected: true })).toBeTruthy()
  fireEvent.input(r.getByRole("textbox", { name: "Search recent items" }), { target: { value: "missing" } })
  expect(r.getByText("No matches")).toBeTruthy()
  fireEvent.click(r.getByRole("button", { name: "Clear filters" }))
  expect(r.queryByText("No matches")).toBeNull()
  fireEvent.click(r.getByRole("tab", { name: "Accessories" }))
  expect(r.getByRole("tab", { name: "Hats", selected: true })).toBeTruthy()
  expect(r.getByRole("tab", { name: "Hair" })).toBeTruthy()
  expect(r.getByLabelText("Sort wardrobe")).toBeTruthy()
})

test("separates color controls from R15-only scale controls", async () => {
  const r = render(() => <AvatarEditor />)
  fireEvent.click(r.getByRole("tab", { name: "Body" }))
  expect(r.getByText("Mixed colors")).toBeTruthy()
  fireEvent.click(r.getByRole("button", { name: /Head/ }))
  expect(r.queryByText("Mixed colors")).toBeNull()
  fireEvent.click(r.getByRole("button", { name: "Custom swatch" }))
  expect(mocks.run).toHaveBeenCalledWith("avatar.color", "head:1")
  fireEvent.click(r.getByRole("tab", { name: "Scale" }))
  await waitFor(() => expect(r.getByText("R15 only")).toBeTruthy())
  const slider = r.getByRole("slider", { name: "Height" })
  expect((slider as HTMLInputElement).disabled).toBe(true)
})

test("custom colors are applied explicitly to the selected body part", async () => {
  const r = render(() => <AvatarEditor />)
  fireEvent.click(r.getByRole("tab", { name: "Body" }))
  fireEvent.click(r.getByRole("button", { name: /Head/ }))
  fireEvent.click(r.getByRole("button", { name: "Custom" }))
  await waitFor(() => expect(r.getByLabelText("Custom body color")).toBeTruthy())
  fireEvent.input(r.getByLabelText("Custom body color"), { target: { value: "#AABBCC" } })
  expect(mocks.run).not.toHaveBeenCalled()
  fireEvent.click(r.getByRole("button", { name: "Apply to 1 selected part" }))
  expect(mocks.run).toHaveBeenCalledWith("avatar.color", "head:#AABBCC")
})

test("searches across wearable types and wears the matched item's type", () => {
  const r = render(() => <AvatarEditor />)
  fireEvent.click(r.getByRole("tab", { name: "All items" }))
  expect(r.getByRole("button", { name: /Red Hat/ })).toBeTruthy()
  expect(r.getByRole("button", { name: /Red Shirt/ })).toBeTruthy()
  fireEvent.input(r.getByRole("textbox", { name: "Search wardrobe" }), { target: { value: "shirt red" } })
  expect(r.queryByRole("button", { name: /Red Hat/ })).toBeNull()
  fireEvent.click(r.getByRole("button", { name: /Red Shirt/ }))
  expect(mocks.run).toHaveBeenCalledWith("avatar.wear", "90004:11")
})
