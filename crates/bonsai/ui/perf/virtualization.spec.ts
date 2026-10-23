import { expect, type Locator, type Page, test } from "@playwright/test"

async function settle(page: Page) {
  await page.waitForFunction(() => window.__bonsaiPerf?.pending === 0)
  await page.evaluate(
    () => new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))),
  )
}

async function expectTilesFillCells(collection: Locator) {
  await expect(async () => {
    const sizes = await collection.locator('[data-component="tile"]').evaluateAll((tiles) =>
      tiles.map((tile) => {
        const cell = tile.closest("[data-virtual-item]")?.getBoundingClientRect()
        const card = tile.getBoundingClientRect()
        const image = tile.querySelector('[data-component="thumbnail"]')?.getBoundingClientRect()
        return { cell: cell?.width ?? 0, card: card.width, image: image?.width ?? 0, height: image?.height ?? 0 }
      }),
    )
    expect(sizes.length).toBeGreaterThan(1)
    for (const size of sizes) {
      expect(size.cell).toBeGreaterThan(80)
      expect(Math.abs(size.card - size.cell)).toBeLessThan(1)
      expect(Math.abs(size.image - size.cell)).toBeLessThan(1)
      expect(Math.abs(size.height - size.image)).toBeLessThan(1)
    }
  }).toPass({ timeout: 5000 })
}

test("virtual grid tiles fill equal columns through resize and the last partial row", async ({ page }) => {
  await page.goto("/components")
  await settle(page)
  const grid = page.locator('[data-component="virtual-grid"]').last()
  await grid.scrollIntoViewIfNeeded()
  await expectTilesFillCells(grid)
  for (const width of [980, 1440]) {
    await page.setViewportSize({ width, height: 900 })
    await grid.scrollIntoViewIfNeeded()
    await expectTilesFillCells(grid)
  }
  const main = page.getByRole("main")
  await expect(async () => {
    await main.evaluate((el) => {
      el.scrollTop = el.scrollHeight
    })
    await expect(grid.getByRole("button", { name: "Item 500", exact: true })).toBeInViewport()
  }).toPass({ timeout: 5000 })
  await expectTilesFillCells(grid)
  expect(await grid.locator("[data-virtual-item]").count()).toBeLessThan(150)
})

test("profile tiles fill their grid and shelf cells with context menus", async ({ page }) => {
  await page.goto("/users/100?rows=503")
  await settle(page)
  const shelf = page.getByRole("region", { name: "Collectibles", exact: true })
  await shelf.scrollIntoViewIfNeeded()
  await expectTilesFillCells(shelf)
  await shelf.focus()
  await page.keyboard.press("End")
  await expect(shelf.getByText("Collectible 129", { exact: true })).toBeVisible()
  await expectTilesFillCells(shelf)
  await page
    .getByRole("tablist", { name: "Profile sections" })
    .getByRole("tab", { name: "Collectibles", exact: true })
    .click()
  const grid = page.locator('[data-component="virtual-grid"]').last()
  await expect(grid.getByText("Collectible 100", { exact: true })).toBeVisible()
  await expectTilesFillCells(grid)
  await page.setViewportSize({ width: 980, height: 900 })
  await expectTilesFillCells(grid)
})

test("profile shelves render a window and reach their last preview", async ({ page }) => {
  await page.goto("/users/100?rows=2000")
  await settle(page)
  const shelf = page.getByRole("region", { name: "Collectibles", exact: true })
  await shelf.scrollIntoViewIfNeeded()
  await expect(shelf.getByText("Collectible 100", { exact: true })).toBeVisible()
  expect(await shelf.locator("[data-virtual-item]").count()).toBeLessThan(20)
  await shelf.focus()
  await expect(shelf).toBeFocused()
  await page.keyboard.press("End")
  await expect(shelf.getByText("Collectible 129", { exact: true })).toBeVisible()
  await page.keyboard.press("Home")
  await expect(shelf.getByText("Collectible 100", { exact: true })).toBeVisible()
})

test("Tab reaches shelf items outside the initial window", async ({ page }) => {
  await page.goto("/components")
  await settle(page)
  const shelf = page.getByRole("region", { name: "Virtual shelf demo", exact: true })
  await shelf.scrollIntoViewIfNeeded()
  await shelf.getByRole("button").first().focus()
  for (let i = 0; i < 40; i++) await page.keyboard.press("Tab")
  const index = await page.evaluate(() =>
    Number(document.activeElement?.closest<HTMLElement>("[data-virtual-item]")?.dataset.virtualItem),
  )
  expect(index).toBeGreaterThan(30)
  expect(await shelf.locator("[data-virtual-item]").count()).toBeLessThan(20)
  await page.keyboard.press("Shift+Tab")
  expect(
    await page.evaluate(() =>
      Number(document.activeElement?.closest<HTMLElement>("[data-virtual-item]")?.dataset.virtualItem),
    ),
  ).toBe(index - 1)
})

test("cursor moves beyond unmounted rows and scrolling retains focused controls", async ({ page }) => {
  await page.goto("/friends?rows=2000")
  await settle(page)
  await page.evaluate(async () => {
    for (let i = 0; i < 150; i++) await window.__bonsaiPerf.moveCursor(1)
  })
  const selected = page.locator('[data-component="list-item"][data-selected]')
  await expect(selected).toHaveAttribute("data-index", "150")
  await expect(selected).toBeInViewport()
  const button = selected.getByRole("button").first()
  await button.focus()
  await page.getByRole("main").evaluate((el) => {
    el.scrollTop = el.scrollHeight
  })
  await settle(page)
  await expect(button).toBeFocused()
  expect(await page.locator('[data-component="virtual-grid"] [data-slot="row"]').count()).toBeLessThan(60)
  await page.getByRole("textbox", { name: "Filter", exact: true }).fill("Player 1999")
  await expect(page.getByText("Player 1999", { exact: true })).toBeVisible()
})

test("chat follows new lines at the end and preserves the reading position on prepend", async ({ page }) => {
  const errors: string[] = []
  page.on("pageerror", (error) => errors.push(error.message))
  await page.goto("/chat?rows=500")
  await expect(page.getByText("Chat line 599", { exact: true })).toBeInViewport()
  await page.evaluate(async () => {
    window.__bonsaiPerf.appendChat()
    await window.__bonsaiPerf.refresh()
  })
  await expect(page.getByText("Chat line 9000", { exact: true })).toBeInViewport()
  const list = page.locator('[data-component="virtual-grid"]').last()
  await list.evaluate((el) => {
    const parent = el.closest('[data-component="pane"]') ?? el.parentElement?.parentElement
    if (parent) parent.scrollTop = 5000
  })
  await settle(page)
  const anchor = await list.locator("[data-virtual-item]").evaluateAll((items) => {
    const scroller = items[0]?.closest('[data-component="virtual-grid"]')?.parentElement?.parentElement
    const top = scroller?.getBoundingClientRect().top ?? 0
    const item = items.find((item) => item.getBoundingClientRect().top >= top)
    return { text: item?.textContent ?? "", top: item?.getBoundingClientRect().top ?? 0 }
  })
  expect(anchor.text).toContain("Chat line")
  await page.evaluate(async () => {
    window.__bonsaiPerf.appendChat()
    await window.__bonsaiPerf.refresh()
  })
  await settle(page)
  await expect(page.getByText("Chat line 9001", { exact: true })).toHaveCount(0)
  await page.evaluate(async () => {
    window.__bonsaiPerf.prependChat()
    await window.__bonsaiPerf.refresh()
  })
  await settle(page)
  const after = await page
    .getByText(/Chat line/)
    .filter({ hasText: anchor.text.match(/Chat line \d+/)?.[0] ?? "missing" })
    .first()
    .boundingBox()
  expect(after).not.toBeNull()
  expect(Math.abs((after?.y ?? 0) - anchor.top)).toBeLessThan(10)
  expect(errors).toEqual([])
})

test("short chat histories stay above the composer", async ({ page }) => {
  await page.goto("/chat?rows=3")
  const line = page.getByText("Chat line 102", { exact: true })
  await expect(line).toBeInViewport()
  await settle(page)
  const message = await line.boundingBox()
  const composer = await page.getByRole("textbox", { name: "Message", exact: true }).boundingBox()
  expect(message).not.toBeNull()
  expect(composer).not.toBeNull()
  expect((composer?.y ?? 0) - ((message?.y ?? 0) + (message?.height ?? 0))).toBeLessThan(80)
})

test("Tab crosses the virtual table window", async ({ page }) => {
  await page.goto("/components")
  await settle(page)
  const table = page.getByRole("table", { name: "Servers", exact: true })
  await table.locator("thead").scrollIntoViewIfNeeded()
  await table.locator("tbody tr").first().focus()
  for (let i = 0; i < 60; i++) await page.keyboard.press("Tab")
  const index = await page.evaluate(() =>
    Number(document.activeElement?.closest<HTMLElement>("tr[data-index]")?.dataset.index),
  )
  expect(index).toBeGreaterThan(20)
  expect(await table.locator("tbody tr").count()).toBeLessThan(70)
})

test("Home windows many game groups and their nested friend rows", async ({ page }) => {
  await page.goto("/?rows=500&playing=groups")
  await expect(page.getByText("Fixture Game 1000", { exact: true })).toBeVisible()
  const main = page.getByRole("main")
  await settle(page)
  expect(await main.locator('[data-component="list-item"]').count()).toBeLessThan(150)
  // Friends playing is followed by capped home sections, so stop at the end of its own grid.
  const grid = main.locator('[data-component="virtual-grid"]').first()
  await expect(async () => {
    await grid.evaluate((el) => {
      const scroller = el.closest("[data-scroller]")
      if (scroller)
        scroller.scrollTop =
          el.getBoundingClientRect().bottom -
          scroller.getBoundingClientRect().top +
          scroller.scrollTop -
          scroller.clientHeight
    })
    await settle(page)
    await expect(page.getByText("Fixture Game 1099", { exact: true })).toBeInViewport()
  }).toPass({ timeout: 10_000 })
})

test("Home windows a large group of friends playing the same game", async ({ page }) => {
  await page.goto("/?rows=500&playing=single")
  await expect(page.getByText("Fixture Game 1000", { exact: true })).toBeVisible()
  const main = page.getByRole("main")
  const grid = main.locator('[data-component="virtual-grid"]').first()
  await expect(async () => {
    await grid.evaluate((el) => {
      const scroller = el.closest("[data-scroller]")
      if (scroller)
        scroller.scrollTop =
          el.getBoundingClientRect().bottom -
          scroller.getBoundingClientRect().top +
          scroller.scrollTop -
          scroller.clientHeight
    })
    await settle(page)
    await expect(page.getByText("Player 599", { exact: true })).toBeInViewport()
  }).toPass({ timeout: 10_000 })
  expect(await main.locator('[data-component="list-item"]').count()).toBeLessThan(100)
})
