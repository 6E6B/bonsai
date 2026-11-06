import { expect, type Page, test } from "@playwright/test"

const row = (page: Page, name: string) => page.getByRole("treeitem", { name, exact: true })

async function settle(page: Page) {
  await page.waitForFunction(() => window.__bonsaiPerf?.pending === 0)
}

test("marketplace keeps its category and filters while another tab shows", async ({ page }) => {
  await page.goto("/marketplace?rows=60")
  await row(page, "Clothing").click()
  await row(page, "Shirts").click()
  await page.getByRole("textbox", { name: "Minimum price" }).fill("25")
  await expect(page).toHaveURL(/min=25/)
  await expect(row(page, "Shirts")).toHaveAttribute("aria-selected", "true")

  await page.locator('[data-component="tile"]').first().click()
  await expect(page).toHaveURL(/\/catalog\/|\/item|\/marketplace\/./)
  await page.locator('[data-component="tab-strip"]').getByText("Marketplace", { exact: true }).click()

  await expect(row(page, "Shirts")).toHaveAttribute("aria-selected", "true")
  await expect(page.getByRole("textbox", { name: "Minimum price" })).toHaveValue("25")
})

test("price filters take digits only and write the URL when typing stops", async ({ page }) => {
  await page.goto("/marketplace?rows=8")
  const min = page.getByRole("textbox", { name: "Minimum price" })
  await min.pressSequentially("1a2", { delay: 10 })
  await expect(min).toHaveValue("12")
  expect(page.url()).not.toContain("min=")
  await expect(page).toHaveURL(/min=12/)
})

test("clear drops every filter, including text still being typed", async ({ page }) => {
  await page.goto("/marketplace?rows=8&sales=limited&max=9&offsale=1&cat=wishlist")
  await row(page, "All").click()
  const creator = page.getByRole("textbox", { name: "Creator" })
  await expect(page.getByRole("textbox", { name: "Maximum price" })).toHaveValue("9")
  await creator.pressSequentially("Tile")
  await page.getByRole("button", { name: "Clear" }).click()
  await expect(creator).toHaveValue("")
  await expect(page.getByRole("textbox", { name: "Maximum price" })).toHaveValue("")
  await page.waitForTimeout(500)
  await expect(creator).toHaveValue("")
  expect(new URL(page.url()).search).not.toMatch(/sales|max|offsale|creator/)
})

test("searching keeps the category, and leaves the wishlist", async ({ page }) => {
  await page.goto("/marketplace?rows=8&cat=wishlist")
  await expect(page.getByRole("textbox", { name: "Search the Marketplace" })).toHaveCount(0)
  await row(page, "Clothing").click()
  const search = page.getByRole("textbox", { name: "Search the Marketplace" })
  await search.fill("hat")
  await search.press("Enter")
  await expect(page).toHaveURL(/q=hat/)
  await expect(row(page, "Clothing")).toHaveAttribute("aria-selected", "true")
})

test("the tree moves by arrow keys and only Enter picks", async ({ page }) => {
  await page.goto("/marketplace?rows=8")
  const before = new URL(page.url()).search
  await row(page, "All").focus()
  await page.keyboard.press("ArrowDown")
  await expect(row(page, "Body")).toBeFocused()
  await page.keyboard.press("ArrowRight")
  await expect(row(page, "Hair")).toBeVisible()
  await page.keyboard.press("ArrowRight")
  await expect(row(page, "Full Bodies")).toBeFocused()
  await page.keyboard.press("ArrowDown")
  await expect(row(page, "Hair")).toBeFocused()
  expect(new URL(page.url()).search).toBe(before)

  await page.keyboard.press("Enter")
  await expect(row(page, "Hair")).toHaveAttribute("aria-selected", "true")
  await page.keyboard.press("ArrowLeft")
  await expect(row(page, "Body")).toBeFocused()
  await page.keyboard.press("ArrowLeft")
  await expect(row(page, "Hair")).toHaveCount(0)
  await expect(row(page, "Body")).toHaveAttribute("aria-expanded", "false")
})

test("the chevron opens a group without picking it", async ({ page }) => {
  await page.goto("/marketplace?rows=8")
  await row(page, "Makeup").locator('[data-slot="chevron"]').click()
  await expect(row(page, "Lips")).toBeVisible()
  await expect(row(page, "Makeup")).toHaveAttribute("aria-selected", "false")
  await expect(page).not.toHaveURL(/cat=/)
})

test("inventory groups pick their first slot, and a slot from the URL shows open", async ({ page }) => {
  await page.goto("/inventory?rows=8")
  await row(page, "Accessories").click()
  await expect(page).toHaveURL(/slot=a8/)
  await expect(row(page, "Head")).toHaveAttribute("aria-selected", "true")

  await page.goto("/inventory?rows=8&slot=a76")
  const eyebrows = page.getByRole("tree").getByRole("treeitem", { selected: true })
  await expect(eyebrows).toBeInViewport()
  await expect(row(page, "Makeup")).toHaveAttribute("aria-expanded", "true")
})

test("inventory keeps its slot and sort while another tab shows", async ({ page }) => {
  await page.goto("/inventory?rows=8")
  await page.getByRole("button", { name: /Recently added/ }).click()
  await page.getByRole("option", { name: "Oldest first" }).click()
  await expect(page).toHaveURL(/sort=oldest/)
  await page.locator('[data-component="tile"]').first().click()
  await page.locator('[data-component="tab-strip"]').getByText("Inventory", { exact: true }).click()
  await expect(page).toHaveURL(/sort=oldest/)
  await expect(page.getByRole("button", { name: /Oldest first/ })).toBeVisible()
})

test("panels stay in view and scroll apart from the page", async ({ page }) => {
  await page.setViewportSize({ width: 1100, height: 640 })
  await page.goto("/inventory?rows=60")
  await settle(page)
  const main = page.locator("main")
  const panel = page.locator('[data-component="side-panel"]')
  const body = panel.locator('> [data-slot="body"]')

  const sizes = await main.evaluate((m) => ({ top: m.getBoundingClientRect().top, height: m.clientHeight }))
  await expect(panel).toHaveCSS("height", `${sizes.height}px`)

  await main.evaluate((m) => {
    m.scrollTop = 900
  })
  await expect.poll(() => main.evaluate((m) => m.scrollTop)).toBe(900)
  expect((await panel.boundingBox())?.y).toBe(sizes.top)

  await body.evaluate((b) => {
    b.scrollTop = 200
  })
  expect(await main.evaluate((m) => m.scrollTop)).toBe(900)

  // Keyboard moves inside the panel leave the page where it is.
  await row(page, "Collectibles").focus()
  await page.keyboard.press("End")
  await expect(row(page, "Video")).toBeFocused()
  await expect(row(page, "Video")).toBeInViewport()
  expect(await main.evaluate((m) => m.scrollTop)).toBe(900)
  expect(await page.evaluate(() => document.documentElement.scrollWidth <= window.innerWidth)).toBe(true)
})
