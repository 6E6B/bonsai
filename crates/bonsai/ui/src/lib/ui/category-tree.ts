import type { IconName } from "./icon"

export interface CategoryChild {
  value: string
  label: string
}

/** A top-level row. Without a `value` it only groups its `children`. */
export interface CategoryNode {
  value?: string
  label: string
  icon?: IconName
  children?: readonly CategoryChild[]
}

/** A run of rows. A `label` titles it; without one, sections after the first get a divider. */
export interface CategorySection {
  label?: string
  items: readonly CategoryNode[]
}

/** A row as the keyboard sees it: only rows that are showing, in display order. */
export interface TreeRow {
  key: string
  /** Key of the row this one is nested under. */
  parent?: string
  expandable: boolean
}

export const nodeKey = (n: CategoryNode) => n.value ?? `group:${n.label}`

export const expandable = (n: CategoryNode) => (n.children?.length ?? 0) > 0

export const containsSelected = (n: CategoryNode, selected: string) =>
  n.children?.some((c) => c.value === selected) ?? false

/** The group to open for `selected`: the one it is in, or itself if it has children. */
export function openFor(sections: readonly CategorySection[], selected: string): string | null {
  for (const s of sections) {
    for (const n of s.items) {
      if (expandable(n) && (n.value === selected || containsSelected(n, selected))) return nodeKey(n)
    }
  }
  return null
}

export function visibleRows(sections: readonly CategorySection[], open: string | null): TreeRow[] {
  const rows: TreeRow[] = []
  for (const s of sections) {
    for (const n of s.items) {
      const key = nodeKey(n)
      rows.push({ key, expandable: expandable(n) })
      if (key === open) for (const c of n.children ?? []) rows.push({ key: c.value, parent: key, expandable: false })
    }
  }
  return rows
}

export type TreeAction = { kind: "focus"; key: string } | { kind: "open"; key: string } | { kind: "close" }

/** What an arrow key does on the focused row. Arrows never select, so they never load anything;
 * Enter and Space reach the row as a click. Null when the key does nothing here. */
export function treeKey(rows: readonly TreeRow[], focused: string, key: string, open: string | null) {
  const i = rows.findIndex((r) => r.key === focused)
  const row = rows[i]
  if (!row) return null
  const focus = (r: TreeRow | undefined): TreeAction | null => (r ? { kind: "focus", key: r.key } : null)
  switch (key) {
    case "ArrowDown":
      return focus(rows[i + 1])
    case "ArrowUp":
      return focus(rows[i - 1])
    case "Home":
      return focus(rows[0])
    case "End":
      return focus(rows[rows.length - 1])
    case "ArrowRight":
      if (!row.expandable) return null
      return open === row.key ? focus(rows[i + 1]) : ({ kind: "open", key: row.key } as const)
    case "ArrowLeft":
      if (row.parent) return { kind: "focus", key: row.parent } as const
      return row.expandable && open === row.key ? ({ kind: "close" } as const) : null
    default:
      return null
  }
}

/** What clicking a top-level row does. Picks it, unless it is already showing what it stands
 * for: then it opens or closes. A group with no value of its own picks its first child. */
export function pressNode(n: CategoryNode, selected: string): { select: string } | "toggle" | null {
  const target = n.value ?? n.children?.[0]?.value
  if (target === undefined) return null
  const showing = n.value === undefined ? containsSelected(n, selected) : n.value === selected
  if (!showing) return { select: target }
  return expandable(n) ? "toggle" : null
}
