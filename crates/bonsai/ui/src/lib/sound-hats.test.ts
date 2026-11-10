import { expect, test } from "vitest"
import { isSoundHat } from "./sound-hats"

test("matches listed assets", () => {
  expect(isSoundHat(33070696)).toBe(true)
  expect(isSoundHat(305888394)).toBe(true)
})

test("skips other assets and bundles with the same id", () => {
  expect(isSoundHat(1)).toBe(false)
  expect(isSoundHat(24112667, true)).toBe(false)
})
