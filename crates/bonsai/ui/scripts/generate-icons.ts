import { copyFileSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from "node:fs"
import { tmpdir } from "node:os"
import { join, resolve } from "node:path"

const shell = resolve(import.meta.dir, "../..")
const icons = join(shell, "icons")
const output = mkdtempSync(join(tmpdir(), "bonsai-icons-"))

function generate(source: string, destination: string, extra: string[] = []) {
  const result = Bun.spawnSync(["bun", "run", "tauri", "icon", source, "--output", destination, ...extra], {
    cwd: join(shell, "ui"),
    stdout: "inherit",
    stderr: "inherit",
  })
  if (result.exitCode !== 0) throw new Error("Tauri icon generation failed")
}

// macOS draws icons on Apple's grid, where the plate fills about 80% of the
// canvas. Windows and Linux add no margin of their own, so their icons scale
// the plate out to fill nearly the whole canvas.
const padded = readFileSync(join(icons, "app-icon.svg"), "utf8")
const fullBleed = padded
  .replace(/(<svg\b[^>]*>)/, '$1<g transform="translate(256 256) scale(1.1923) translate(-256 -256)">')
  .replace(/<\/svg>\s*$/, "</g></svg>")

try {
  const fullSource = join(output, "app-icon-full.svg")
  writeFileSync(fullSource, fullBleed)
  const full = join(output, "full")
  generate(fullSource, full)
  // Bonsai ships on desktop. Keep Tauri's desktop and Store assets, without
  // adding unused Android and iOS projects or resources.
  for (const entry of readdirSync(full, { withFileTypes: true })) {
    if (entry.isFile() && /\.(png|ico)$/.test(entry.name)) {
      copyFileSync(join(full, entry.name), join(icons, entry.name))
    }
  }
  const mac = join(output, "mac")
  generate(join(icons, "app-icon.svg"), mac)
  copyFileSync(join(mac, "icon.icns"), join(icons, "icon.icns"))
  const tray = join(output, "tray")
  generate(join(icons, "tray-template.svg"), tray, ["--png", "44"])
  copyFileSync(join(tray, "44x44.png"), join(icons, "tray-template.png"))
  const favicon = fullBleed.replace(/(<svg\b[^>]*>)/, "$1<title>Bonsai</title>")
  writeFileSync(join(shell, "ui/public/icon.svg"), favicon)
  copyFileSync(join(icons, "icon.ico"), join(shell, "ui/public/favicon.ico"))
} finally {
  rmSync(output, { recursive: true, force: true })
}
