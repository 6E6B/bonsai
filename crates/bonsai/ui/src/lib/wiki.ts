/**
 * Wiki articles about items, experiences and players, from the Roblox Wiki and each
 * experience's own fan wiki on Fandom.
 *
 * Prototype: the shapes here are what `ipc.rs` would send once a wiki client exists in a
 * network crate. Until then every article comes from `wiki-sample.ts`, in a browser and in
 * the app alike. See docs/wiki.md.
 */
import { articles as sampleArticles, sources as sampleSources } from "./wiki-sample"

export type WikiKind = "item" | "game" | "player" | "page"

export interface WikiSource {
  key: string
  name: string
  /** Fandom host, like `roblox.fandom.com`. Only the Rust side would ever talk to it. */
  host: string
  license: string
  /** Set for an experience's own wiki. */
  placeId?: number
}

/** The Bonsai page an article is about, so each can link to the other. */
export type WikiTarget = { kind: "item"; id: number } | { kind: "game"; placeId: number } | { kind: "user"; id: number }

export type WikiBlock =
  | { type: "p"; text: string }
  | { type: "list"; items: string[] }
  | { type: "table"; columns: string[]; rows: string[][] }
  | { type: "note"; text: string }

export interface WikiSection {
  id: string
  title: string
  blocks: WikiBlock[]
}

export interface WikiFact {
  label: string
  value: string
}

export interface WikiArticle {
  source: string
  slug: string
  title: string
  kind: WikiKind
  /** The wiki's own main category: "Hats", "Experiences", "Ingredients". */
  category: string
  /** The lead paragraph, plain text with `[[links]]`. */
  extract: string
  target: WikiTarget | null
  /** Last edit, ms. */
  revised: number
  editors: number
  infobox: { title?: string; facts: WikiFact[] }[]
  sections: WikiSection[]
  categories: string[]
}

export const kindLabel: Record<WikiKind, string> = {
  item: "Item",
  game: "Experience",
  player: "Player",
  page: "Page",
}

export const sources: WikiSource[] = sampleSources
export const articles: WikiArticle[] = sampleArticles

export const ROBLOX_WIKI = "roblox"

export const sourceOf = (key: string) => sources.find((s) => s.key === key)

export const articleAt = (source: string, slug: string) => articles.find((a) => a.source === source && a.slug === slug)

export const sameTarget = (a: WikiTarget, b: WikiTarget) =>
  a.kind === b.kind &&
  (a.kind === "game" ? a.placeId === (b as { placeId: number }).placeId : a.id === (b as { id: number }).id)

/** The Roblox Wiki article about a Bonsai page, if it has one. Experience wikis don't count:
 * their pages are about things inside the game. */
export function articleFor(target: WikiTarget, all: WikiArticle[] = articles) {
  return all.find((a) => a.source === ROBLOX_WIKI && a.target && sameTarget(a.target, target))
}

/** Whether a page has anything to show in a Wiki tab. */
export const hasWiki = (target: WikiTarget) =>
  Boolean(articleFor(target) || (target.kind === "game" && gameWiki(target.placeId)))

/** An experience's own wiki and its pages. */
export function gameWiki(placeId: number) {
  const source = sources.find((s) => s.placeId === placeId)
  if (!source) return undefined
  return { source, pages: articles.filter((a) => a.source === source.key) }
}

/** `game:9001`, `item:42`, `user:7` (the command context) → a target. */
export function parseTarget(context: string | null): WikiTarget | null {
  const m = context?.match(/^(game|item|user):(\d+)$/)
  if (!m) return null
  const id = Number(m[2])
  return m[1] === "game" ? { kind: "game", placeId: id } : { kind: m[1] as "item" | "user", id }
}

/** The path of an article in the Wiki screen. */
export const articlePath = (a: Pick<WikiArticle, "source" | "slug">) => `/wiki/${a.source}/${a.slug}`

/** The article on the wiki's own site. */
export function siteUrl(a: Pick<WikiArticle, "source" | "slug">) {
  const host = sourceOf(a.source)?.host ?? "roblox.fandom.com"
  return `https://${host}/wiki/${a.slug}`
}

/** Title → slug the way MediaWiki makes them: spaces to underscores. */
export const slugOf = (title: string) => title.trim().replace(/\s+/g, "_")

/**
 * Articles matching `query`, best first: the exact title, then the title starts with it, then a word in the title does,
 * then the title or category contains it, then the lead does. Empty query keeps the order.
 */
export function searchWiki(all: WikiArticle[], query: string, kind: WikiKind | "all" = "all") {
  const q = query.trim().toLowerCase()
  const pool = kind === "all" ? all : all.filter((a) => a.kind === kind)
  if (!q) return pool
  const score = (a: WikiArticle) => {
    const t = a.title.toLowerCase()
    if (t === q) return 0
    if (t.startsWith(q)) return 1
    if (t.split(/[\s_()-]+/).some((w) => w.startsWith(q))) return 2
    if (t.includes(q) || a.category.toLowerCase().includes(q)) return 3
    if (a.extract.toLowerCase().includes(q)) return 4
    return -1
  }
  return pool
    .map((a, i) => ({ a, i, s: score(a) }))
    .filter((x) => x.s >= 0)
    .sort((x, y) => x.s - y.s || x.i - y.i)
    .map((x) => x.a)
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

/** The article a `[[link]]` points to: in the same wiki first, then the Roblox Wiki, then any. */
export function resolveLink(from: string, link: string, all: WikiArticle[] = articles) {
  const slug = slugOf(link).toLowerCase()
  const find = (src?: string) =>
    all.find((a) => (src === undefined || a.source === src) && a.slug.toLowerCase() === slug)
  return find(from) ?? find(ROBLOX_WIKI) ?? find()
}

/** Articles that link to this one, for "Linked from". */
export function backlinks(to: WikiArticle, all: WikiArticle[] = articles) {
  const texts = (a: WikiArticle) => [
    a.extract,
    ...a.sections.flatMap((s) =>
      s.blocks.flatMap((b) => (b.type === "list" ? b.items : b.type === "table" ? b.rows.flat() : [b.text])),
    ),
  ]
  return all.filter(
    (a) =>
      a !== to &&
      texts(a).some((t) => parseLinks(t).some((s) => "link" in s && resolveLink(a.source, s.link, all) === to)),
  )
}
