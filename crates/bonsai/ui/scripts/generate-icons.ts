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

try {
  generate(join(icons, "app-icon.svg"), output)
  // Bonsai ships on desktop. Keep Tauri's desktop and Store assets, without
  // adding unused Android and iOS projects or resources.
  for (const entry of readdirSync(output, { withFileTypes: true })) {
    if (entry.isFile() && /\.(png|ico|icns)$/.test(entry.name)) {
      copyFileSync(join(output, entry.name), join(icons, entry.name))
    }
  }
  const tray = join(output, "tray")
  generate(join(icons, "tray-template.svg"), tray, ["--png", "44"])
  copyFileSync(join(tray, "44x44.png"), join(icons, "tray-template.png"))
  const favicon = readFileSync(join(icons, "app-icon.svg"), "utf8").replace(/(<svg\b[^>]*>)/, "$1<title>Bonsai</title>")
  writeFileSync(join(shell, "ui/public/icon.svg"), favicon)
  copyFileSync(join(icons, "icon.ico"), join(shell, "ui/public/favicon.ico"))
} finally {
  rmSync(output, { recursive: true, force: true })
}
