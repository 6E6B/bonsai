/**
 * Registers the Google fonts a theme uses with the webview. The backend downloads them and serves
 * each weight over `font://` (crates/bonsai-tauri/src/font.rs); `FontFace` loads it from there, so
 * no stylesheet is injected and nothing comes from the network. The theme's font stack names the
 * family first, so text switches to it as soon as it loads.
 */
import { convertFileSrc, isTauri } from "@tauri-apps/api/core"

export type FontFiles = { family: string; files: { weight: number; path: string }[] }

const added = new Set<string>()

/** Adds each file not added yet. Safe to call on every change. */
export function registerFonts(fonts: FontFiles[]) {
  if (!isTauri() || typeof FontFace === "undefined") return
  for (const f of fonts) {
    for (const file of f.files) {
      if (added.has(file.path)) continue
      added.add(file.path)
      const face = new FontFace(f.family, `url("${convertFileSrc(file.path, "font")}")`, {
        weight: String(file.weight),
        style: "normal",
        display: "swap",
      })
      document.fonts.add(face)
      face.load().catch(() => {
        // Missing or broken file: the stack falls back to the bundled font. Try again next time.
        document.fonts.delete(face)
        added.delete(file.path)
      })
    }
  }
}

export type FontChoice = { value: string; label: string; detail: string }

/** What the font picker lists for `query`: matching bundled fonts first (no download), then the
 * Google Fonts matches the backend found, skipping Google's copies of the bundled ones. */
export function fontChoices(
  query: string,
  mono: boolean,
  google: { family: string; category: string }[],
): FontChoice[] {
  const bundled = mono ? ["JetBrains Mono", "System"] : ["Inter", "System"]
  const q = query.trim().toLowerCase()
  const out: FontChoice[] = bundled
    .filter((b) => b.toLowerCase().includes(q))
    .map((b) => ({ value: b, label: b, detail: "Built in" }))
  for (const f of google) {
    if (bundled.some((b) => b.toLowerCase() === f.family.toLowerCase())) continue
    out.push({ value: f.family, label: f.family, detail: f.category })
  }
  return out
}
