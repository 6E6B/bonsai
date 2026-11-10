import { expect, test } from "@playwright/test"

test("sending a trusted request changes the profile action to pending", async ({ page }) => {
  await page.goto("/users/100?rows=8&trusted=none")
  await page.getByRole("button", { name: "Add trusted friend", exact: true }).click()
  await expect(page.getByRole("button", { name: "Trusted request sent", exact: true })).toBeVisible()
  await expect(page.getByRole("button", { name: "Add trusted friend", exact: true })).toHaveCount(0)
})

test("trusted requests can be searched and accepted from Friends", async ({ page }) => {
  const errors: string[] = []
  page.on("pageerror", (e) => errors.push(e.message))
  await page.goto("/friends?rows=8")
  await page.getByRole("tab", { name: /Trusted requests/ }).click()
  await page.getByRole("textbox", { name: "Filter", exact: true }).fill("player_100")
  await expect(page.getByRole("button", { name: "Accept", exact: true })).toHaveCount(1)
  await page.getByRole("button", { name: "Accept", exact: true }).click()
  await expect(page.getByText("No one matches", { exact: true })).toBeVisible()
  await page.getByRole("textbox", { name: "Filter", exact: true }).fill("")
  await expect(page.getByRole("button", { name: "Accept", exact: true })).toHaveCount(5)
  expect(errors).toEqual([])
})

test("incoming profile request becomes trusted after acceptance", async ({ page }) => {
  await page.goto("/users/100?rows=8&trusted=received")
  await page.getByRole("button", { name: "Accept trusted friend", exact: true }).click()
  await expect(page.getByRole("button", { name: "Trusted friends", exact: true })).toBeVisible()
})

test("trusted API failures leave ordinary profile actions available", async ({ page }) => {
  await page.goto("/users/100?rows=8&trusted=error")
  await expect(page.getByRole("button", { name: "Trusted status unavailable", exact: true })).toBeVisible()
  await expect(page.getByRole("button", { name: "Message", exact: true }).first()).toBeVisible()
  await expect(page.getByRole("button", { name: "Add trusted friend", exact: true })).toHaveCount(0)
})

test("a sent request can be canceled from the existing Manage menu", async ({ page }) => {
  await page.goto("/users/100?rows=8&trusted=sent")
  await page.getByRole("button", { name: "More", exact: true }).click()
  await page.getByRole("menuitem", { name: "Manage", exact: true }).focus()
  await page.keyboard.press("ArrowRight")
  // Plain-browser fixtures have no native command catalog, so menu labels fall back to IDs.
  await page.getByRole("menuitem", { name: /^(Cancel trusted friend request|user.cancel_trusted_friend)$/ }).click()
  await expect(page.getByRole("button", { name: "Add trusted friend", exact: true })).toBeVisible()
})

test("trusted request rows keep actions visible in a narrow window", async ({ page }) => {
  await page.setViewportSize({ width: 720, height: 900 })
  await page.goto("/friends?rows=8")
  await page.getByRole("tab", { name: /Trusted requests/ }).click()
  await expect(page.getByRole("button", { name: "Accept", exact: true }).first()).toBeInViewport()
  await expect(page.getByRole("button", { name: "Decline", exact: true }).first()).toBeInViewport()
  await page.getByRole("button", { name: "Decline", exact: true }).first().click()
  await expect(page.getByRole("button", { name: "Accept", exact: true })).toHaveCount(5)
})
