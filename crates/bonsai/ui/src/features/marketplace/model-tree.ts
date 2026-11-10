import type { ModelNodeView } from "~/lib/ipc/bindings"

/** Nodes are in depth-first order, so each node's descendants follow it directly. */
type Node = Pick<ModelNodeView, "parent" | "depth" | "class" | "name" | "children">

/** Indices of the rows to show. Without a query, every node whose ancestors are all expanded.
 * With one, the nodes whose name or class matches, plus their ancestors, ignoring collapse. */
export function visibleRows(nodes: readonly Node[], collapsed: Record<number, boolean>, query: string): number[] {
  const q = query.trim().toLowerCase()
  const rows: number[] = []
  if (q) {
    const keep = new Set<number>()
    nodes.forEach((n, i) => {
      if (!n.name.toLowerCase().includes(q) && !n.class.toLowerCase().includes(q)) return
      for (let at: number | null = i; at !== null && !keep.has(at); at = nodes[at]?.parent ?? null) keep.add(at)
    })
    nodes.forEach((_, i) => {
      if (keep.has(i)) rows.push(i)
    })
    return rows
  }
  let skipBelow = -1
  nodes.forEach((n, i) => {
    if (skipBelow >= 0 && n.depth > skipBelow) return
    skipBelow = -1
    rows.push(i)
    if (collapsed[i] && n.children > 0) skipBelow = n.depth
  })
  return rows
}

/** `Hat/Handle/Attachment`. */
export function nodePath(nodes: readonly Node[], index: number): string {
  const parts: string[] = []
  for (let at: number | null = index; at !== null && nodes[at]; at = nodes[at].parent) parts.push(nodes[at].name)
  return parts.reverse().join("/")
}

/** Every node with children, for Collapse all. */
export function allParents(nodes: readonly Node[]): Record<number, boolean> {
  const out: Record<number, boolean> = {}
  nodes.forEach((n, i) => {
    if (n.children > 0) out[i] = true
  })
  return out
}
