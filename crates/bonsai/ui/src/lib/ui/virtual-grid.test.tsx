import { cleanup, render } from "@solidjs/testing-library"
import { afterEach, expect, test, vi } from "vitest"
import { VirtualGrid } from "./virtual-grid"

const observers: { callback: ResizeObserverCallback; targets: Element[] }[] = []
vi.stubGlobal(
  "ResizeObserver",
  class {
    targets: Element[] = []
    constructor(callback: ResizeObserverCallback) {
      observers.push({ callback, targets: this.targets })
    }
    observe(el: Element) {
      this.targets.push(el)
    }
    unobserve() {}
    disconnect() {}
  },
)

// jsdom has no layout. Give the scroller a viewport so rows render.
for (const prop of ["offsetHeight", "offsetWidth", "clientHeight", "clientWidth"])
  Object.defineProperty(HTMLElement.prototype, prop, { configurable: true, get: () => 400 })

afterEach(() => {
  cleanup()
  observers.length = 0
})

const frame = () => new Promise((resolve) => requestAnimationFrame(() => setTimeout(resolve, 0)))

function resize(el: Element, width: number) {
  for (const o of observers.filter((o) => o.targets.includes(el)))
    o.callback([{ target: el, contentRect: { width } } as unknown as ResizeObserverEntry], {} as ResizeObserver)
}

test("keeps tiles when the column count changes", async () => {
  const items = ["a", "b", "c", "d", "e", "f"]
  const made = vi.fn()
  const r = render(() => (
    <div style={{ "overflow-y": "auto", height: "400px" }}>
      <VirtualGrid items={items} minItemWidth={100} gap={0} estimateRowHeight={10}>
        {(item) => {
          made(item)
          return <span data-tile={item}>{item}</span>
        }}
      </VirtualGrid>
    </div>
  ))
  const grid = r.container.querySelector("[data-component=virtual-grid]") as HTMLElement
  resize(grid, 300)
  await frame()
  const row = (tile: string) =>
    r.container.querySelector(`[data-tile=${tile}]`)?.closest("[data-slot=row]")?.getAttribute("data-index")
  const before = r.container.querySelector("[data-tile=c]")
  expect(before).toBeTruthy()
  expect(row("c")).toBe("0")
  const count = made.mock.calls.length

  resize(grid, 200)
  await frame()
  // "c" moved from row 0 to row 1 as the same node.
  expect(row("c")).toBe("1")
  expect(r.container.querySelector("[data-tile=c]")).toBe(before)
  expect(before?.closest("[data-slot=item]")?.getAttribute("data-virtual-item")).toBe("2")
  expect(made.mock.calls.length).toBe(count)
})
