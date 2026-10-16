/**
 * Inspect: which theme tokens paint an element. It reads the app's own stylesheets (component
 * CSS and Tailwind utilities both use `var(--token)`), keeps the rules that match the element,
 * and adds the text color and font it inherits. Nothing here is guessed from class names.
 */
import { TOKEN } from "~/lib/theme"

const VAR = /var\(\s*--([\w-]+)/g

/** Custom property names a CSS value reads, fallbacks included, in order. */
export function varsIn(value: string): string[] {
  return [...value.matchAll(VAR)].map((m) => m[1])
}

/** Every style rule in `sheets`, flattened out of `@layer`, `@media`, `@supports` and nesting. */
export function styleRules(sheets: Iterable<CSSStyleSheet>): CSSStyleRule[] {
  const out: CSSStyleRule[] = []
  const walk = (rules: CSSRuleList) => {
    for (const r of Array.from(rules)) {
      if (r instanceof CSSStyleRule) out.push(r)
      if ("cssRules" in r && (r as CSSGroupingRule).cssRules) walk((r as CSSGroupingRule).cssRules)
    }
  }
  for (const s of sheets) {
    try {
      walk(s.cssRules)
    } catch {
      // A sheet from another origin can't be read. The app has none, but don't fail on one.
    }
  }
  return out
}

/** Properties that pass down to children, so a token on an ancestor still paints the element. */
const INHERITED = new Set(["color", "font-family", "font-size", "line-height", "text-shadow"])

function matches(el: Element, rule: CSSStyleRule): boolean {
  try {
    return el.matches(rule.selectorText)
  } catch {
    return false
  }
}

function tokensOf(style: CSSStyleDeclaration, only?: Set<string>): string[] {
  const out: string[] = []
  for (let i = 0; i < style.length; i++) {
    const prop = style[i]
    if (only && !only.has(prop)) continue
    for (const v of varsIn(style.getPropertyValue(prop))) out.push(v)
  }
  return out
}

/**
 * Theme tokens that style `el`, its own first, then those it inherits from ancestors up to
 * `root`. Each appears once.
 */
export function tokensAt(el: Element, root: Element, rules: CSSStyleRule[]): string[] {
  const seen = new Set<string>()
  const add = (names: string[]) => {
    for (const n of names) if (TOKEN.has(n)) seen.add(n)
  }
  for (let at: Element | null = el, own = true; at; at = at.parentElement, own = false) {
    const only = own ? undefined : INHERITED
    for (const r of rules) if (matches(at, r)) add(tokensOf(r.style, only))
    if (at instanceof HTMLElement) add(tokensOf(at.style, only))
    if (at === root) break
  }
  return [...seen]
}
