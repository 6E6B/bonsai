/** A command that opens a Roblox page in the app instead of on the website. */
export type OpenIn = { command: "game.open" | "user.open" | "group.open" | "market.open"; arg: string }

/** A run of description text, or a Roblox link in it. Links to other sites stay plain text:
 * the app only opens Roblox pages. `open` is set for pages the app has a screen for. */
export type TextPart = { text: string; url?: string; placeId?: number; open?: OpenIn }

// Roblox pages the app shows itself, by path. Locale prefixes (`/es/games/1`) are allowed.
const pages: [RegExp, (id: string) => OpenIn][] = [
  [/^(?:\/[a-z]{2}(?:-[a-z]{2})?)?\/games\/(\d+)/i, (id) => ({ command: "game.open", arg: id })],
  [/^(?:\/[a-z]{2}(?:-[a-z]{2})?)?\/users\/(\d+)/i, (id) => ({ command: "user.open", arg: id })],
  [/^(?:\/[a-z]{2}(?:-[a-z]{2})?)?\/(?:groups|communities)\/(\d+)/i, (id) => ({ command: "group.open", arg: id })],
  [/^(?:\/[a-z]{2}(?:-[a-z]{2})?)?\/catalog\/(\d+)/i, (id) => ({ command: "market.open", arg: `item:${id}` })],
  [/^(?:\/[a-z]{2}(?:-[a-z]{2})?)?\/bundles\/(\d+)/i, (id) => ({ command: "market.open", arg: `bundle:${id}` })],
]

/** Where a Roblox link opens in the app, if it has a screen for it. Only www.roblox.com and
 * roblox.com pages: other subdomains (create, devforum) are different sites. */
export function openIn(url: string): OpenIn | undefined {
  const m = /^https?:\/\/(?:www\.)?roblox\.com(\/[^?#]*)?/i.exec(url)
  if (!m) return undefined
  for (const [re, f] of pages) {
    const id = re.exec(m[1] ?? "")?.[1]
    if (id) return f(id)
  }
  return undefined
}

const link = /\bhttps?:\/\/(?:[a-z0-9-]+\.)*roblox\.com(?![\w.-]*\w)(?:\/[^\s<>"]*)?/gi

/** Split `text` into plain runs and Roblox links. Sentence punctuation after a link is dropped
 * from it; an experience link carries its place id so it can open in the app. */
export function splitLinks(text: string): TextPart[] {
  const out: TextPart[] = []
  let last = 0
  for (const m of text.matchAll(link)) {
    const url = m[0].replace(/[.,!?;:)\]}'"]+$/, "")
    const at = m.index ?? 0
    if (at > last) out.push({ text: text.slice(last, at) })
    const open = openIn(url)
    const part: TextPart = { text: url, url }
    if (open) part.open = open
    if (open?.command === "game.open") part.placeId = Number(open.arg)
    out.push(part)
    last = at + url.length
  }
  if (last < text.length) out.push({ text: text.slice(last) })
  return out
}

/** Place ids of the experience links in `text`, in order, each once. */
export function experienceLinks(text: string): number[] {
  const ids: number[] = []
  for (const p of splitLinks(text)) if (p.placeId && !ids.includes(p.placeId)) ids.push(p.placeId)
  return ids
}

/** The text is nothing but experience links, so a card can stand in for it. */
export function isOnlyExperienceLinks(text: string): boolean {
  const parts = splitLinks(text)
  return parts.some((p) => p.placeId) && parts.every((p) => p.placeId || !p.text.trim())
}

const anyLink = /\bhttps?:\/\/[^\s<>"]+/gi

const isRobloxHost = (host: string) => host === "roblox.com" || host.endsWith(".roblox.com")

/** Hosts of the links to other sites in `text`, lowercase, each once. These are only shown,
 * never opened, so the reader can see where a message wants to send them. */
export function externalHosts(text: string): string[] {
  const hosts: string[] = []
  for (const m of text.matchAll(anyLink)) {
    let host: string
    try {
      host = new URL(m[0].replace(/[.,!?;:)\]}'"]+$/, "")).hostname.toLowerCase()
    } catch {
      continue
    }
    if (host && !isRobloxHost(host) && !hosts.includes(host)) hosts.push(host)
  }
  return hosts
}

/** A run of rich text: plain, a Roblox link, a hashtag (`tag` without the `#`) or an @mention
 * (`mention`: the username, lowercased, without the `@`). */
export type RichPart = TextPart & { tag?: string; mention?: string }

// A # at the start or after a space or punctuation, then a word with at least one letter.
// Roblox's filter turns words into runs of #, and `&#39;` is an entity, so neither counts.
const hashtag = /(^|[^\p{L}\p{N}_#&])#([\p{L}\p{N}_]*\p{L}[\p{L}\p{N}_]*)/gu

// An @ at the start or after anything but a word character (so not an email), then a name.
const mention = /(^|[^\p{L}\p{N}_@])@([A-Za-z0-9_]{3,20})(?![A-Za-z0-9_])/gu

/** Roblox usernames: 3 to 20 letters, digits and at most one underscore, not at either end. */
export const isUsername = (s: string) =>
  /^[A-Za-z0-9_]{3,20}$/.test(s) && !s.startsWith("_") && !s.endsWith("_") && s.split("_").length <= 2

type Found = { at: number; end: number; part: RichPart }

/** `splitLinks`, with @mentions and (unless `tags` is false) hashtags in the plain runs picked
 * out too. */
export function splitRich(text: string, opts: { tags?: boolean } = {}): RichPart[] {
  const out: RichPart[] = []
  for (const p of splitLinks(text)) {
    if (p.url) {
      out.push(p)
      continue
    }
    const found: Found[] = []
    if (opts.tags !== false) {
      for (const m of p.text.matchAll(hashtag)) {
        const at = (m.index ?? 0) + m[1].length
        found.push({ at, end: at + 1 + m[2].length, part: { text: `#${m[2]}`, tag: m[2] } })
      }
    }
    for (const m of p.text.matchAll(mention)) {
      if (!isUsername(m[2])) continue
      const at = (m.index ?? 0) + m[1].length
      found.push({ at, end: at + 1 + m[2].length, part: { text: `@${m[2]}`, mention: m[2].toLowerCase() } })
    }
    found.sort((a, b) => a.at - b.at)
    let last = 0
    for (const f of found) {
      if (f.at < last) continue
      if (f.at > last) out.push({ text: p.text.slice(last, f.at) })
      out.push(f.part)
      last = f.end
    }
    if (last < p.text.length) out.push({ text: p.text.slice(last) })
  }
  return out
}

/** The usernames `text` @mentions, lowercased, each once, at most 100 (one lookup). */
export function mentions(text: string): string[] {
  const names: string[] = []
  for (const p of splitRich(text, { tags: false })) {
    if (p.mention && !names.includes(p.mention)) names.push(p.mention)
  }
  return names.slice(0, 100)
}
