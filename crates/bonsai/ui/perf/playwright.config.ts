import { fileURLToPath } from "node:url"
import { defineConfig } from "@playwright/test"

export default defineConfig({
  testDir: ".",
  testMatch: ["ui.perf.ts", "virtualization.spec.ts", "dependencies.spec.ts"],
  fullyParallel: false,
  workers: 1,
  retries: 0,
  timeout: 90_000,
  outputDir: "../perf-results/artifacts",
  reporter: [
    ["list"],
    ["html", { outputFolder: fileURLToPath(new URL("../perf-results/report", import.meta.url)), open: "never" }],
  ],
  use: {
    baseURL: "http://127.0.0.1:1420",
    viewport: { width: 1440, height: 900 },
    trace: "retain-on-failure",
    screenshot: "only-on-failure",
  },
  projects: [
    { name: "chromium", use: { browserName: "chromium" } },
    { name: "webkit", use: { browserName: "webkit" } },
  ],
  webServer: {
    command:
      "bun run perf:build && bun run preview --outDir perf-results/dist --host 127.0.0.1 --port 1420 --strictPort",
    url: "http://127.0.0.1:1420",
    reuseExistingServer: false,
    timeout: 120_000,
  },
})
