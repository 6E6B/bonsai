/** Public thumbnails have an opaque identity on every webview platform. Remote URLs are
 * deliberately excluded: exports go through Rust and the existing thumb:// cache. */
export interface ImageTarget {
  key: string
  name: string
}

const kinds = "headshot|avatar|game_icon|game_thumbnail|asset|bundle|group_icon|badge_icon|game_pass|outfit"
const keyPattern = new RegExp(`^(${kinds})\\.([1-9]\\d*)\\.[sl]$`)

export function imageKey(src: string | undefined): string | undefined {
  if (!src) return
  try {
    const url = new URL(src)
    if (url.username || url.password || url.port || url.search || url.hash) return
    const local = url.protocol === "thumb:" && url.hostname === "localhost"
    const mapped = ["http:", "https:"].includes(url.protocol) && url.hostname === "thumb.localhost"
    if (!local && !mapped) return
    const key = decodeURIComponent(url.pathname.slice(1))
    const match = keyPattern.exec(key)
    if (match && Number.isSafeInteger(Number(match[2]))) return key
  } catch {
    // Invalid URLs and non-image protocols simply have no image actions.
  }
}

let selected: ImageTarget | undefined

export function selectImage(target: ImageTarget | undefined) {
  selected = target
}

/** A virtualized row or page may disappear while another image is selected. */
export function forgetImage(target: ImageTarget | undefined) {
  if (selected === target) selected = undefined
}

export function selectedImageArgument() {
  return selected ? JSON.stringify(selected) : null
}

/** Prefer the exact image under the pointer. Row/tile keyboard menus use their first image. */
export function imageAt(target: EventTarget | null, root: HTMLElement): ImageTarget | undefined {
  if (!(target instanceof Element)) return
  const selector = "[data-image-src], img"
  const source =
    target.closest<HTMLElement>(selector) ??
    target.querySelector<HTMLElement>(selector) ??
    root.querySelector<HTMLElement>(selector)
  if (!source || !root.contains(source)) return
  const src = source.dataset.imageSrc ?? source.getAttribute("src") ?? undefined
  const key = imageKey(src)
  if (key) return { key, name: source.dataset.imageName ?? source.getAttribute("alt") ?? "" }
}
