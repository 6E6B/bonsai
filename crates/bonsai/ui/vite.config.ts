import tailwindcss from "@tailwindcss/vite"
import solid from "vite-plugin-solid"
import { defineConfig } from "vitest/config"

const host = process.env.TAURI_DEV_HOST

export default defineConfig({
  plugins: [solid(), tailwindcss()],
  resolve: {
    alias: { "~": new URL("./src", import.meta.url).pathname },
  },
  // Tauri expects a fixed port and prints its own errors.
  clearScreen: false,
  server: {
    port: 1420,
    strictPort: true,
    host: host || false,
    hmr: host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch: { ignored: ["**/target/**"] },
  },
  envPrefix: ["VITE_", "TAURI_ENV_*"],
  build: {
    // WebKitGTK and WKWebView track Safari; WebView2 tracks Chrome.
    target: process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome120" : "safari16",
    minify: !process.env.TAURI_ENV_DEBUG,
    sourcemap: !!process.env.TAURI_ENV_DEBUG,
  },
  test: {
    environment: "jsdom",
    include: ["src/**/*.test.{ts,tsx}"],
  },
})
