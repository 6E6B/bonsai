import { cleanup, fireEvent, render, waitFor } from "@solidjs/testing-library"
import { For } from "solid-js"
import { afterEach, beforeEach, expect, test, vi } from "vitest"
import { NewChat } from "./NewChat"

const mocks = vi.hoisted(() => ({
  startChat: vi.fn(),
  options: { loading: false, error: null, enabled: true, groupEnabled: true, maxTitle: 12 },
}))
vi.mock("~/lib/ipc/bindings", () => ({ commands: { startChat: mocks.startChat, chatOptions: vi.fn() } }))
vi.mock("~/lib/store", () => ({
  shell: { active: 1 },
  friends: {
    friends: [
      { id: 2, name: "ann", displayName: "Ann", presence: "offline" },
      { id: 3, name: "bo", displayName: "Bo", presence: "offline" },
    ],
  },
  useLive: () => mocks.options,
}))

// Selection rules run independently of browser layout; windowing is covered in Playwright.
vi.mock("~/lib/ui/virtual-list", () => ({
  VirtualList: (props: { items: unknown[]; children: (item: unknown) => unknown }) => (
    <For each={props.items}>{props.children as never}</For>
  ),
}))

beforeEach(() => {
  mocks.startChat.mockReset().mockResolvedValue({ status: "ok", data: null })
  mocks.options.enabled = true
  mocks.options.groupEnabled = true
})
afterEach(cleanup)

test("selecting friends creates a named group, once per person", async () => {
  const onDone = vi.fn()
  const r = render(() => <NewChat onDone={onDone} />)
  fireEvent.click(r.getByRole("checkbox", { name: "Ann" }))
  fireEvent.click(r.getByRole("checkbox", { name: "Bo" }))
  const name = r.getByLabelText("Group name (optional)")
  expect(name.getAttribute("maxlength")).toBe("12")
  fireEvent.input(name, { target: { value: "  Squad  " } })
  fireEvent.click(r.getByRole("button", { name: "Start group of 3" }))
  await waitFor(() => expect(onDone).toHaveBeenCalledWith(true))
  expect(mocks.startChat).toHaveBeenCalledExactlyOnceWith([2, 3], "Squad")
})

test("group privacy blocks multiple selections while allowing a direct chat", () => {
  mocks.options.groupEnabled = false
  const r = render(() => <NewChat onDone={() => {}} />)
  fireEvent.click(r.getByRole("checkbox", { name: "Ann" }))
  expect((r.getByRole("checkbox", { name: "Bo" }) as HTMLInputElement).disabled).toBe(true)
  fireEvent.click(r.getByRole("button", { name: "Chat" }))
  expect(mocks.startChat).toHaveBeenCalledExactlyOnceWith([2], null)
})

test("disabled chat cannot create a conversation", () => {
  mocks.options.enabled = false
  const r = render(() => <NewChat onDone={() => {}} />)
  expect((r.getByRole("checkbox", { name: "Ann" }) as HTMLInputElement).disabled).toBe(true)
  expect((r.getByRole("button", { name: "Chat" }) as HTMLButtonElement).disabled).toBe(true)
  expect(mocks.startChat).not.toHaveBeenCalled()
})
