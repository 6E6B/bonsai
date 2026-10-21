import { writeFile } from "node:fs/promises"
import { expect, type Page, test } from "@playwright/test"

type Probe = {
  pending: number
  revision: number
  calls: Record<string, number>
  refresh: () => Promise<void>
}
declare global {
  interface Window {
    __bonsaiPerf: Probe
    __frameProbe: { gaps: number[]; tasks: number[]; reset: () => void }
  }
}

const samples = Math.max(3, Number(process.env.PERF_SAMPLES) || 5)
const rowCount = Number(process.env.PERF_ROWS) || 500
const routes = [
  {
    name: "profile",
    path: "/users/100",
    ready: "Performance Player",
    tabs: ["Outfits", "Connections", "Experiences", "Badges", "Communities", "Collectibles", "Overview"],
  },
  { name: "friends", path: "/friends", ready: "Player 100", tabs: ["Followers", "Following", "Blocked", "Friends"] },
  { name: "marketplace", path: "/marketplace", ready: "Collectible 100", tabs: [] },
  { name: "home", path: "/", ready: "Recommended", tabs: [] },
]

async function settle(page: Page) {
  await page.waitForFunction(() => window.__bonsaiPerf?.pending === 0)
  await page.evaluate(
    () => new Promise<void>((resolve) => requestAnimationFrame(() => requestAnimationFrame(() => resolve()))),
  )
}

for (const route of routes) {
  test(`${route.name}: mount, tabs, scroll, store refresh`, async ({ page, browserName }, info) => {
    const errors: string[] = []
    page.on("pageerror", (error) => errors.push(error.message))
    page.on("console", (message) => {
      if (message.type() === "error") errors.push(message.text())
    })
    // No remote resources, credentials or native writes in a synthetic performance run.
    await page.route("**/*", async (request) => {
      const url = new URL(request.request().url())
      if (url.hostname !== "127.0.0.1" && url.protocol !== "data:") {
        errors.push(`Unexpected external request: ${url.origin}`)
        await request.abort()
      } else await request.continue()
    })
    await page.addInitScript(() => {
      const gaps: number[] = []
      const tasks: number[] = []
      let previous = 0
      const tick = (now: number) => {
        if (previous) gaps.push(now - previous)
        previous = now
        requestAnimationFrame(tick)
      }
      requestAnimationFrame(tick)
      if (PerformanceObserver.supportedEntryTypes.includes("longtask")) {
        new PerformanceObserver((list) => {
          for (const entry of list.getEntries()) tasks.push(entry.duration)
        }).observe({ type: "longtask" })
      }
      window.__frameProbe = {
        gaps,
        tasks,
        reset() {
          gaps.length = 0
          tasks.length = 0
          previous = 0
        },
      }
    })
    const cdp = browserName === "chromium" ? await page.context().newCDPSession(page) : null
    const cpuRate = Number(process.env.PERF_CPU_RATE) || 1
    if (cdp) {
      await cdp.send("Emulation.setCPUThrottlingRate", { rate: cpuRate })
      await cdp.send("Performance.enable")
      if (process.env.PERF_CPU_PROFILE) {
        await cdp.send("Profiler.enable")
        await cdp.send("Profiler.start")
      }
    }
    const measurements: { name: string; durationMs: number; nodes?: number; heapBytes?: number | null }[] = []
    const measure = async (name: string, action: () => Promise<unknown>) => {
      await page.evaluate(() => window.__frameProbe.reset())
      const start = await page.evaluate(() => performance.now())
      await action()
      await settle(page)
      const value = await page.evaluate(
        (start) => ({
          durationMs: performance.now() - start,
          nodes: document.querySelectorAll("*").length,
          frameGapsMs: [...window.__frameProbe.gaps],
          longTasksMs: [...window.__frameProbe.tasks],
          calls: { ...window.__bonsaiPerf.calls },
        }),
        start,
      )
      const metrics = cdp ? (await cdp.send("Performance.getMetrics")).metrics : []
      measurements.push({ name, ...value, heapBytes: metrics.find((m) => m.name === "JSHeapUsedSize")?.value ?? null })
      expect(errors, `${name}: application errors`).toEqual([])
      // Structural regression guard for virtualized lists; timing budgets are opt-in per machine.
      if (
        !process.env.PERF_ALLOW_UNBOUNDED &&
        route.name === "profile" &&
        ["Badges", "Communities", "Collectibles", "Experiences"].includes(name)
      ) {
        expect(value.nodes, `${name} should render only visible rows`).toBeLessThan(3000)
      }
      const budget = Number(process.env.PERF_MAX_ACTION_MS)
      if (budget > 0) expect(value.durationMs, name).toBeLessThan(budget)
    }
    const start = Date.now()
    await page.goto(`${route.path}?rows=${rowCount}`)
    await expect(page.getByText(route.ready, { exact: true }).first()).toBeVisible()
    await settle(page)
    measurements.push({ name: "cold-navigation", durationMs: Date.now() - start })
    for (let sample = 0; sample < samples; sample++) {
      for (const tab of route.tabs) {
        await measure(tab, () =>
          page
            .getByRole("tablist", { name: route.name === "profile" ? "Profile sections" : "Friends sections" })
            .getByRole("tab", { name: tab, exact: true })
            .click(),
        )
        await expect(
          page
            .getByRole("tablist", { name: route.name === "profile" ? "Profile sections" : "Friends sections" })
            .getByRole("tab", { name: tab, exact: true }),
        ).toHaveAttribute("aria-selected", "true")
      }
      await measure("scroll", async () => {
        for (let i = 0; i < 8; i++) {
          await page.mouse.move(950, 650)
          await page.mouse.wheel(0, 400)
          await page.waitForTimeout(40)
        }
        await page.mouse.wheel(0, -10000)
      })
      if (route.name === "friends") {
        await measure("filter", async () => {
          await page.getByRole("textbox", { name: "Filter", exact: true }).fill("Player 100")
        })
        await expect(page.getByText("Player 100", { exact: true }).first()).toBeVisible()
        await page.getByRole("textbox", { name: "Filter", exact: true }).clear()
        await settle(page)
      }
      await measure("store-refresh", () => page.evaluate(() => window.__bonsaiPerf.refresh()))
    }
    if (cdp && process.env.PERF_CPU_PROFILE) {
      const { profile } = await cdp.send("Profiler.stop")
      await info.attach("cpu-profile.cpuprofile", { body: JSON.stringify(profile), contentType: "application/json" })
    }
    const summary = [...new Set(measurements.map((m) => m.name))].map((name) => {
      const rows = measurements.filter((m) => m.name === name)
      const times = rows.map((m) => m.durationMs).sort((a, b) => a - b)
      return {
        name,
        p50Ms: times[Math.floor(times.length * 0.5)],
        p95Ms: times[Math.ceil(times.length * 0.95) - 1],
        maxNodes: Math.max(...rows.map((m) => m.nodes ?? 0)),
      }
    })
    const report = {
      route: route.name,
      browserName,
      rowCount,
      samples,
      cpuRate: cdp ? cpuRate : null,
      summary,
      measurements,
    }
    console.table(summary)
    await writeFile(info.outputPath("performance.json"), JSON.stringify(report, null, 2))
    await info.attach("performance.json", { body: JSON.stringify(report, null, 2), contentType: "application/json" })
    expect(errors).toEqual([])
  })
}

// Verify virtualization preserves access to the full list, including rows never mounted initially.
test("profile virtual lists reach their last item", async ({ page }) => {
  await page.goto(`/users/100?rows=${rowCount}`)
  await expect(page.getByRole("heading", { name: "Performance Player", exact: true })).toBeVisible()
  const lastId = 99 + Math.min(5000, Math.max(1, rowCount))
  for (const [tab, last] of [
    ["Experiences", `Experience ${lastId}`],
    ["Badges", `Badge ${lastId}`],
    ["Communities", `Community ${lastId}`],
    ["Collectibles", `Collectible ${lastId}`],
  ]) {
    await page.getByRole("tablist", { name: "Profile sections" }).getByRole("tab", { name: tab, exact: true }).click()
    await settle(page)
    await expect(async () => {
      await page.getByRole("main").evaluate((el) => {
        el.scrollTop = el.scrollHeight
      })
      await settle(page)
      await expect(page.getByText(last, { exact: true })).toBeVisible()
    }).toPass({ timeout: 10_000 })
    await page.getByRole("main").evaluate((el) => {
      el.scrollTop = 0
    })
    await settle(page)
  }
})
