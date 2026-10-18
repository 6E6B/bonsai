import type { ViewCell, ViewNode } from "~/lib/ipc/bindings"

/** Each input in a view tree with the value the plugin gave it, by key. */
export function inputValues(node: ViewNode | null | undefined, out: Record<string, string> = {}) {
  if (!node) return out
  if (node.type === "input") out[node.key] = node.value
  if (node.type === "row" || node.type === "column") for (const c of node.children) inputValues(c, out)
  return out
}

/**
 * What each input shows after the plugin redraws: what the user typed, unless the plugin
 * changed that input's value since the last tree. Inputs no longer in the tree are dropped.
 */
export function mergeValues(
  prev: ViewNode | null | undefined,
  next: ViewNode,
  typed: Record<string, string>,
): Record<string, string> {
  const before = inputValues(prev)
  const after = inputValues(next)
  const out: Record<string, string> = {}
  for (const [key, value] of Object.entries(after)) {
    const kept = typed[key]
    out[key] = key in before && before[key] === value && kept !== undefined ? kept : value
  }
  return out
}

/** A cell as text; numbers in the user's number format. */
export function cellText(c: ViewCell, format: (n: number) => string): string {
  if (c === null) return ""
  return typeof c === "number" ? format(c) : c
}
