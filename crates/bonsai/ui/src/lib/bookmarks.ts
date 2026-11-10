import type { BookmarkKindView, BookmarkView } from "~/lib/ipc/bindings"
import type { IconName } from "~/lib/ui/icon"

export const bookmarkKinds: BookmarkKindView[] = ["experience", "person", "item", "community", "view", "unknown"]
export const bookmarkLabels: Record<BookmarkKindView, string> = {
  experience: "Experiences",
  person: "People",
  item: "Items",
  community: "Communities",
  view: "Views",
  unknown: "Other",
}
export const bookmarkIcons: Record<BookmarkKindView, IconName> = {
  experience: "game",
  person: "user",
  item: "marketplace",
  community: "communities",
  view: "bookmark",
  unknown: "bookmark",
}

export function bookmarkIdentity(path: string): string {
  const p = path.split(/[?#]/, 1)[0]
  return /^\/(games|users|catalog|bundles|passes|communities)\/[1-9]\d*$/.test(p) ? p : path
}
export function findBookmark(rows: readonly BookmarkView[], path: string) {
  return rows.find((b) => bookmarkIdentity(b.path) === bookmarkIdentity(path))
}
export function bookmarkPath(target: string): string | undefined {
  if (target.startsWith("/") && !target.startsWith("//")) return target
  const match = /^(user|game|item|bundle|pass|group):(\d+)$/.exec(target)
  const routes: Record<string, string> = {
    user: "users",
    game: "games",
    item: "catalog",
    bundle: "bundles",
    pass: "passes",
    group: "communities",
  }
  return match ? `/${routes[match[1]]}/${match[2]}` : undefined
}
export type BookmarkRow =
  | { type: "header"; kind: BookmarkKindView; count: number }
  | { type: "bookmark"; bookmark: BookmarkView }
export function bookmarkRows(
  items: readonly BookmarkView[],
  query: string,
  collapsed: Partial<Record<BookmarkKindView, boolean>> = {},
): BookmarkRow[] {
  const words = query.toLocaleLowerCase().trim().split(/\s+/).filter(Boolean)
  return bookmarkKinds.flatMap((kind): BookmarkRow[] => {
    const group = items.filter(
      (b) => b.kind === kind && words.every((w) => `${b.title} ${b.path}`.toLocaleLowerCase().includes(w)),
    )
    if (!group.length) return []
    return [
      { type: "header", kind, count: group.length },
      ...(collapsed[kind] && !words.length
        ? []
        : group.map((bookmark): BookmarkRow => ({ type: "bookmark", bookmark }))),
    ]
  })
}
