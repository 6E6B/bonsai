import { describe, expect, it } from "vitest"
import type { PrivacyRow } from "~/lib/ipc/bindings"
import { privacyOptions, shownPrivacy, valueLabel } from "./privacy"

const row = (key: string, value: string, options: [string, boolean][]): PrivacyRow => ({
  key,
  value,
  options: options.map(([v, allowed]) => ({ value: v, allowed })),
})

describe("privacy", () => {
  it("labels known values and splits unknown ones", () => {
    expect(valueLabel("NoOne")).toBe("No one")
    expect(valueLabel("FriendsAndFollowing")).toBe("Friends and following")
    expect(valueLabel("someNewValue")).toBe("Some New Value")
  })

  it("disables options the account can't pick and keeps the current value", () => {
    const opts = privacyOptions(row("whoCanTradeWithMe", "NoOne", [["AllUsers", false]]))
    expect(opts).toEqual([
      { value: "NoOne", label: "No one" },
      { value: "AllUsers", label: "Everyone", disabled: true },
    ])
  })

  it("shows known settings in order and skips the rest", () => {
    const rows = [
      row("whoCanTradeWithMe", "NoOne", []),
      row("someFutureSetting", "X", []),
      row("whoCanChatWithMeInApp", "Friends", []),
    ]
    expect(shownPrivacy(rows).map((r) => r.row.key)).toEqual(["whoCanChatWithMeInApp", "whoCanTradeWithMe"])
  })
})
