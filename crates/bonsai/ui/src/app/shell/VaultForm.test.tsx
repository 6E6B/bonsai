import { cleanup, fireEvent, render, waitFor } from "@solidjs/testing-library"
import type { JSX } from "solid-js"
import { afterEach, expect, it, vi } from "vitest"
import { VaultForm } from "./VaultForm"

const { submit, refresh } = vi.hoisted(() => ({ submit: vi.fn(), refresh: vi.fn() }))
vi.mock("~/lib/ipc/bindings", () => ({ commands: { vaultSubmit: submit } }))
vi.mock("~/lib/vault", () => ({ refreshVault: refresh, vault: { retrySeconds: 0, systemLockWarning: null } }))
vi.mock("~/lib/ui", () => ({
  Button: (props: JSX.ButtonHTMLAttributes<HTMLButtonElement>) => <button {...props} />,
  TextField: (props: { label: string; type?: string; value: string; onChange: (value: string) => void }) => (
    <label>
      {props.label}
      <input type={props.type} value={props.value} onInput={(event) => props.onChange(event.currentTarget.value)} />
    </label>
  ),
}))
afterEach(() => {
  cleanup()
  vi.clearAllMocks()
})

it("requires exact reset confirmation and submits no unlock secret", async () => {
  submit.mockResolvedValue({ status: "ok", data: null })
  const view = render(() => <VaultForm action="forget" />)
  const button = view.getByRole("button", { name: "Remove all local accounts" }) as HTMLButtonElement
  const confirmation = view.getByLabelText("Type RESET to continue")
  expect(button.disabled).toBe(true)
  fireEvent.input(confirmation, { target: { value: "reset" } })
  expect(button.disabled).toBe(true)
  fireEvent.input(confirmation, { target: { value: "RESET" } })
  expect(button.disabled).toBe(false)
  fireEvent.click(button)
  await waitFor(() => expect(submit).toHaveBeenCalledWith("forget", "", "RESET"))
  await waitFor(() => expect((confirmation as HTMLInputElement).value).toBe(""))
})

it("clears the unlock secret even when the backend rejects it", async () => {
  submit.mockResolvedValue({ status: "error", error: "Incorrect PIN" })
  const view = render(() => <VaultForm action="unlock" />)
  const pin = view.getByLabelText("PIN or passphrase") as HTMLInputElement
  fireEvent.input(pin, { target: { value: "synthetic-test-passphrase" } })
  fireEvent.click(view.getByRole("button", { name: "Unlock" }))
  await waitFor(() => expect(view.getByRole("alert").textContent).toBe("Incorrect PIN"))
  expect(pin.value).toBe("")
  expect(refresh).toHaveBeenCalledOnce()
})
