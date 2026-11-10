import { expect, test } from "vitest"
import { isSoundHat, nextSound, soundHat } from "./sound-hats"

test("matches listed assets", () => {
  expect(isSoundHat(33070696)).toBe(true)
  expect(isSoundHat(305888394)).toBe(true)
})

test("skips other assets and bundles with the same id", () => {
  expect(isSoundHat(1)).toBe(false)
  expect(isSoundHat(24112667, true)).toBe(false)
})

test("follows each hat's script", () => {
  expect(soundHat(33070696)?.sounds).toEqual(["Harm1", "Harm2", "Harm3", "Harm4"])
  expect(soundHat(24114402)?.sounds).toHaveLength(3)
  expect(soundHat(305888394)?.wait).toEqual([5, 15])
})

test("waits a whole number of seconds within the script's range", () => {
  const hat = soundHat(24112667)
  if (!hat) throw new Error("missing hat")
  expect(nextSound(hat, () => 0)).toEqual({ wait: 5, sound: "Quack1" })
  expect(nextSound(hat, () => 0.9999)).toEqual({ wait: 20, sound: "Quack4" })
  expect(nextSound(hat, () => 0.5)).toEqual({ wait: 13, sound: "Quack3" })
})
