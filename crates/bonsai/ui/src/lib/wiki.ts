/**
 * Roblox Wiki articles about items, experiences and players. The Rust side fetches and parses
 * them (`bonsai-core/src/wiki.rs`); text arrives plain with `[[Title|label]]` links, and nothing
 * here talks to Fandom. See docs/wiki.md.
 */
import type { WikiTarget } from "./ipc/bindings"

export type { WikiArticle, WikiBlock, WikiFact, WikiPageView, WikiSearchView, WikiTarget } from "./ipc/bindings"

export type WikiKind = "item" | "game" | "player" | "page"

export const kindLabel: Record<string, string> = {
  item: "Item",
  game: "Experience",
  player: "Player",
  page: "Article",
}

/** What `wiki_about` takes for a target. */
export type AboutKind = "item" | "bundle" | "game" | "user"

/** `game:9001`, `item:42`, `user:7` (the command context) → a target. */
export function parseTarget(context: string | null): WikiTarget | null {
  const m = context?.match(/^(game|item|user):(\d+)$/)
  if (!m) return null
  const id = Number(m[2])
  return m[1] === "game" ? { kind: "game", placeId: id } : { kind: m[1] as "item" | "user", id }
}

export const targetKey = (t: WikiTarget) => `${t.kind}:${t.kind === "game" ? t.placeId : t.id}`

/** Title → slug the way MediaWiki makes them: spaces to underscores. */
export const slugOf = (title: string) => title.trim().replace(/\s+/g, "_")

/** Slug (from a path) → title. */
export const titleOf = (slug: string) => {
  let s = slug
  try {
    s = decodeURIComponent(slug)
  } catch {
    // A stray % stays as it is.
  }
  return s.replace(/_/g, " ").trim()
}

/** The path of an article in the Wiki screen, with an optional section. */
export const articlePath = (title: string, section?: string) =>
  `/wiki/${encodeURIComponent(slugOf(title)).replace(/%3A/gi, ":")}${section ? `#${section}` : ""}`

/** `Catalog:Dominus Infernus#Trivia` (a `wiki.open` argument) → title and section. */
export function splitArg(arg: string | null) {
  const [title, ...rest] = (arg ?? "").trim().split("#")
  return { title: titleOf(title ?? ""), section: rest.join("#") || undefined }
}

export type Span = { text: string } | { text: string; link: string }

/** Splits wikitext-style `[[Target]]` and `[[Target|label]]` links out of plain text. */
export function parseLinks(text: string): Span[] {
  const out: Span[] = []
  const re = /\[\[([^\]|]+)(?:\|([^\]]+))?\]\]/g
  let last = 0
  for (const m of text.matchAll(re)) {
    const at = m.index ?? 0
    if (at > last) out.push({ text: text.slice(last, at) })
    out.push({ text: m[2] ?? m[1], link: m[1].trim() })
    last = at + m[0].length
  }
  if (last < text.length) out.push({ text: text.slice(last) })
  return out
}

/** Text with `[[links]]` turned into their labels. */
export const plain = (text: string) =>
  parseLinks(text)
    .map((s) => s.text)
    .join("")

/** A nested list item: the Rust side prefixes each level with "– ". */
export function listDepth(item: string) {
  let depth = 0
  let rest = item
  while (rest.startsWith("– ")) {
    depth++
    rest = rest.slice(2)
  }
  return { depth, text: rest }
}

/** Which article was found for which page, so "Read the wiki article about this" can open it
 * without fetching again. Filled as wiki tabs and cards load. */
const known = new Map<string, string>()

export const rememberArticle = (t: WikiTarget, title: string) => known.set(targetKey(t), title)
export const knownArticle = (t: WikiTarget) => known.get(targetKey(t))
