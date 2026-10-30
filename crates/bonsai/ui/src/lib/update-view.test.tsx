import { cleanup, fireEvent, render, screen } from "@solidjs/testing-library"
import { afterEach, beforeEach, expect, test, vi } from "vitest"
import { UpdateDetails, UpdateNotice } from "./update-view"
import { setUpdates } from "./updates"

const run = vi.hoisted(() => vi.fn())
vi.mock("~/lib/commands/registry", () => ({ run }))

beforeEach(() => {
  setUpdates({
    phase: "idle",
    currentVersion: "1.0.0",
    version: "1.1.0",
    notes: null,
    downloaded: 0,
    total: null,
    checkedAt: null,
    error: null,
    unavailableReason: null,
  })
})
afterEach(() => {
  cleanup()
  run.mockClear()
})

test("keeps background checks quiet and announces a pending update", () => {
  const view = render(() => <UpdateNotice />)
  expect(view.container.textContent).toBe("")
  setUpdates("phase", "available")
  expect(screen.getByText("Bonsai 1.1.0 is available.")).toBeTruthy()
  expect(run).not.toHaveBeenCalled()
  fireEvent.click(screen.getByRole("button", { name: "Download update" }))
  expect(run).toHaveBeenCalledWith("app.download_update")
})

test("offers install only for a verified download and requires a click", () => {
  render(() => <UpdateDetails />)
  setUpdates("phase", "downloading")
  expect(screen.queryByRole("button", { name: "Install and restart" })).toBeNull()
  setUpdates("phase", "ready")
  expect(run).not.toHaveBeenCalled()
  fireEvent.click(screen.getByRole("button", { name: "Install and restart" }))
  expect(run).toHaveBeenCalledWith("app.install_update")
})

test("shows progress without requiring a Content-Length", () => {
  setUpdates({ phase: "downloading", downloaded: 5 * 1048576, total: null })
  render(() => <UpdateDetails />)
  expect(screen.getByText("5 MB downloaded")).toBeTruthy()
  setUpdates("total", 10 * 1048576)
  expect(screen.getByText("5 / 10 MB")).toBeTruthy()
})

test("keeps a failed download retryable and does not offer install", () => {
  setUpdates({ phase: "available", error: "Signature verification failed." })
  render(() => <UpdateDetails />)
  expect(screen.getByText("Signature verification failed.")).toBeTruthy()
  expect(screen.queryByRole("button", { name: "Install and restart" })).toBeNull()
  expect(screen.getByRole("button", { name: "Download update" })).toBeTruthy()
})

test("shows release notes as inert text", () => {
  setUpdates({ phase: "available", notes: '<script>alert("x")</script>' })
  const view = render(() => <UpdateDetails />)
  expect(view.container.querySelector("script")).toBeNull()
  expect(screen.getByText('<script>alert("x")</script>')).toBeTruthy()
})
