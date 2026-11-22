import { readdirSync } from "node:fs"
import { join } from "node:path"
import { describe, expect, it } from "vitest"

const src = join(__dirname, "..")

/** Every source module under `src`, without its extension: `features/avatar/Adjust`. */
function modules(dir: string, out: string[] = []) {
  for (const e of readdirSync(dir, { withFileTypes: true })) {
    const p = join(dir, e.name)
    if (e.isDirectory()) modules(p, out)
    else if (/\.(ts|tsx)$/.test(e.name)) out.push(p.slice(src.length + 1).replace(/\.(ts|tsx)$/, ""))
  }
  return out
}

describe("module names", () => {
  // macOS and Windows build on case-insensitive disks, where `./Adjust` can resolve to
  // `adjust.ts`. Linux CI never sees the clash, so check for it here.
  it("never differ only by case", () => {
    const seen = new Map<string, string>()
    const clashes: string[] = []
    for (const m of modules(src)) {
      const key = m.toLowerCase()
      const other = seen.get(key)
      if (other !== undefined && other !== m) clashes.push(`${other} / ${m}`)
      seen.set(key, m)
    }
    expect(clashes).toEqual([])
  })
})
