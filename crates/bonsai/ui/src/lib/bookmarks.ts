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
/** Folders in use, sorted ignoring case. */
export function bookmarkFolders(items: readonly BookmarkView[]): string[] {
  return [...new Set(items.map((b) => b.folder).filter(Boolean))].sort((a, b) =>
    a.localeCompare(b, undefined, { sensitivity: "base" }),
  )
}

/** A folder, or one kind outside folders. Rows only reorder inside their group. */
export type BookmarkGroup = { folder: string; kind: null } | { folder: null; kind: BookmarkKindView }
export function bookmarkGroup(b: BookmarkView): BookmarkGroup {
  return b.folder ? { folder: b.folder, kind: null } : { folder: null, kind: b.kind }
}
export function groupKey(g: BookmarkGroup): string {
  return g.folder === null ? g.kind : `folder:${g.folder}`
}
export function groupLabel(g: BookmarkGroup): string {
  return g.folder ?? bookmarkLabels[g.kind]
}

/** Every word matches the title, path or folder; `#word` matches the start of a tag. */
export function matchesBookmark(b: BookmarkView, words: readonly string[]): boolean {
  const text = `${b.title} ${b.path} ${b.folder}`.toLocaleLowerCase()
  const tags = b.tags.map((t) => t.toLocaleLowerCase())
  return words.every((w) =>
    w.startsWith("#") && w.length > 1
      ? tags.some((t) => t.startsWith(w.slice(1)))
      : text.includes(w) || tags.some((t) => t.includes(w.replace(/^#/, ""))),
  )
}

export type BookmarkRow =
  | { type: "header"; group: BookmarkGroup; count: number }
  | { type: "bookmark"; bookmark: BookmarkView }
/** Folders first, then the kinds outside folders. `collapsed` is keyed by `groupKey`. */
export function bookmarkRows(
  items: readonly BookmarkView[],
  query: string,
  collapsed: Partial<Record<string, boolean>> = {},
): BookmarkRow[] {
  const words = query.toLocaleLowerCase().trim().split(/\s+/).filter(Boolean)
  const shown = items.filter((b) => matchesBookmark(b, words))
  const groups: BookmarkGroup[] = [
    ...bookmarkFolders(shown).map((folder): BookmarkGroup => ({ folder, kind: null })),
    ...bookmarkKinds.map((kind): BookmarkGroup => ({ folder: null, kind })),
  ]
  return groups.flatMap((group): BookmarkRow[] => {
    const key = groupKey(group)
    const rows = shown.filter((b) => groupKey(bookmarkGroup(b)) === key)
    if (!rows.length) return []
    return [
      { type: "header", group, count: rows.length },
      ...(collapsed[key] && !words.length ? [] : rows.map((bookmark): BookmarkRow => ({ type: "bookmark", bookmark }))),
    ]
  })
}
