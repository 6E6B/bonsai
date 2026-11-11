import { expect, test } from "@playwright/test"

const drawerName = "Friends, chat and bookmarks"

test("bookmarks group, filter and collapse in the drawer", async ({ page }) => {
  await page.goto("/marketplace?rows=8&bookmarks=1")
  const drawer = page.getByRole("complementary", { name: drawerName })
  await expect(drawer.getByRole("tab", { name: "Bookmarks" })).toHaveAttribute("aria-selected", "true")
  await expect(drawer.getByRole("button", { name: "Industrial Outpost 1,240 playing" })).toBeVisible()
  await drawer.getByRole("button", { name: "Experiences · 2" }).click()
  await expect(drawer.getByText("Industrial Outpost")).toHaveCount(0)
  await drawer.getByRole("textbox", { name: "Filter bookmarks" }).fill("outpost")
  await expect(drawer.getByText("Industrial Outpost")).toBeVisible()
  await expect(drawer.getByText("Builder", { exact: true })).toHaveCount(0)
  await drawer.getByRole("textbox", { name: "Filter bookmarks" }).fill("missing")
  await expect(drawer.getByText("No matching bookmarks")).toBeVisible()
})

test("Ctrl+click opens a bookmark in a background tab", async ({ page }) => {
  await page.goto("/marketplace?rows=8&bookmarks=1")
  const before = page.url()
  await page
    .getByRole("complementary", { name: drawerName })
    .getByRole("button", { name: "Industrial Outpost 1,240 playing" })
    .click({ modifiers: ["Control"] })
  await expect(page.locator('[data-component="tab-strip"] [data-slot="tab"]')).toHaveCount(2)
  expect(page.url()).toBe(before)
})

test("Enter restores a saved view query", async ({ page }) => {
  await page.goto("/marketplace?rows=8&bookmarks=1")
  const row = page
    .getByRole("complementary", { name: drawerName })
    .getByRole("button", { name: "Limited accessories", exact: true })
  await row.focus()
  await row.press("Enter")
  await expect(page).toHaveURL(/\/marketplace\?sales=limited$/)
})

test("row menus have one remove action and normal entity actions", async ({ page }) => {
  await page.goto("/marketplace?rows=8&bookmarks=1")
  await page.getByRole("complementary", { name: drawerName }).getByText("Industrial Outpost").click({ button: "right" })
  await expect(page.getByRole("menuitem", { name: "Remove bookmark", exact: true })).toHaveCount(1)
  await expect(page.getByRole("menuitem", { name: "Rename…", exact: true })).toBeVisible()
  await expect(page.getByRole("menuitem", { name: "Share", exact: true })).toBeVisible()
})

test("Ctrl+D adds with Undo and Rename, and saved rows can be renamed or removed", async ({ page }) => {
  await page.goto("/marketplace?rows=8&bookmarks=1")
  const drawer = page.getByRole("complementary", { name: drawerName })
  await page.keyboard.press("Control+d")
  const toast = page.locator('[data-component="toast"]').filter({ hasText: "Bookmarked" })
  await expect(toast).toBeVisible()
  await expect(toast.getByRole("button", { name: "Undo", exact: true })).toBeVisible()
  await toast.getByRole("button", { name: "Rename", exact: true }).click()
  const dialog = page.getByRole("dialog")
  await dialog.getByRole("textbox").fill("My marketplace")
  await dialog.getByRole("button", { name: "Rename", exact: true }).click()
  await expect(drawer.getByRole("button", { name: "My marketplace", exact: true })).toBeVisible()
  await page.keyboard.press("Control+d")
  await expect(drawer.getByRole("button", { name: "My marketplace", exact: true })).toHaveCount(0)
})

test("b: lists bookmarks without remote search", async ({ page }) => {
  await page.goto("/marketplace?rows=8&bookmarks=1")
  await page.locator('[data-component="search-button"]').click()
  const dialog = page.getByRole("dialog")
  await dialog.getByRole("textbox", { name: "Search", exact: true }).fill("b: outpost")
  await expect(dialog.getByText("Industrial Outpost", { exact: false })).toBeVisible()
  await expect(dialog.getByText("Night Shift", { exact: false })).toHaveCount(0)
  const count = await page.evaluate(() => window.__bonsaiPerf.calls.remoteSearch ?? 0)
  expect(count).toBe(0)
})
