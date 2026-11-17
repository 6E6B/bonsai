import { describe, expect, it } from "vitest"
import { maskedAccountLabel, promptToast, streamerDetectOptions } from "./streamer"

describe("promptToast", () => {
  it("offers to turn on, wait or stop asking when an app opens", () => {
    const t = promptToast({ seq: 1, app: "OBS", start: true })
    expect(t.title).toBe("OBS is running. Turn on streamer mode?")
    expect(t.buttons.map((b) => b.label)).toEqual(["Turn on", "Not now", "Don't ask for OBS"])
    expect(t.buttons.map((b) => b.run)).toEqual([["streamer.toggle", "on"], null, ["streamer.ignore", "OBS"]])
  })

  it("only offers to turn off when the app closes", () => {
    const t = promptToast({ seq: 2, app: "Medal", start: false })
    expect(t.title).toBe("Medal closed. Turn off streamer mode?")
    expect(t.buttons.map((b) => b.run)).toEqual([["streamer.toggle", "off"], null])
  })
})

describe("maskedAccountLabel", () => {
  it("numbers accounts from one", () => {
    expect(maskedAccountLabel(0)).toBe("Account 1")
    expect(maskedAccountLabel(4)).toBe("Account 5")
  })
})

describe("streamerDetectOptions", () => {
  it("lists the values bonsai-core accepts", () => {
    expect(streamerDetectOptions.map((o) => o.value)).toEqual(["ask", "auto", "off"])
  })
})
