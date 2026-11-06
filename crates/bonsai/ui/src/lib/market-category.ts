import type { CategoryView } from "~/lib/ipc/bindings"

/** The `cat` param of the wishlist, which isn't a catalog category. */
export const WISHLIST = "wishlist"

/** The category a `cat` param means: a taxonomy, or a name typed in the command bar. Null when
 * nothing matches. Until the list loads, a param is taken as it is. */
export function findCategory(categories: readonly CategoryView[], param: string): string | null {
  const name = param.toLowerCase()
  if (name === WISHLIST) return WISHLIST
  if (categories.length === 0) return param
  const all = categories.flatMap((c) => [{ value: c.taxonomy, label: c.name }, ...c.subcategories])
  return (all.find((c) => c.value === param) ?? all.find((c) => c.label.toLowerCase() === name))?.value ?? null
}

/** A price filter as typed. Null when unset. */
export const priceOf = (s: string) => {
  const n = Number.parseInt(s, 10)
  return Number.isNaN(n) || n < 0 ? null : n
}
