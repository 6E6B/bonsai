import { fireEvent, render } from "@solidjs/testing-library"
import { createStore } from "solid-js/store"
import { expect, test } from "vitest"
import { TabStrip } from "./tab-strip"

// jsdom has no layout.
Element.prototype.scrollIntoView = () => {}

const tick = (ms = 20) => new Promise((r) => setTimeout(r, ms))

function setup() {
  const [s, set] = createStore({
    tabs: [
      { key: "t0", title: "Home" },
      { key: "t1", title: "Friends" },
    ],
    active: "t1",
  })
  const selected: string[] = []
  const r = render(() => (
    <TabStrip tabs={s.tabs} active={s.active} onSelect={(k) => selected.push(k)} onClose={() => {}} />
  ))
  return { s, set, selected, r }
}

test("opening a tab doesn't select another one", async () => {
  const { s, set, selected } = setup()
  await tick()
  set({ tabs: [...s.tabs, { key: "t2", title: "Profile" }], active: "t2" })
  await tick(50)
  expect(selected).toEqual([])
})

test("clicking a tab selects it", async () => {
  const { selected, r } = setup()
  await tick()
  const trigger = r.getByRole("tab", { name: "Home" })
  fireEvent.pointerDown(trigger, { button: 0, pointerType: "mouse" })
  fireEvent.pointerUp(trigger, { button: 0, pointerType: "mouse" })
  fireEvent.click(trigger, { button: 0 })
  expect(selected).toEqual(["t0"])
})
