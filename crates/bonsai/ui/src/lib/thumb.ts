import { convertFileSrc, isTauri } from "@tauri-apps/api/core"
import { shell } from "~/lib/store"
import type { WindowControlIcons } from "~/lib/ui"

/** Thumbnail kinds served by the `thumb://` protocol (crates/bonsai/src/thumb.rs). */
export type ThumbKind =
  | "headshot"
  | "avatar"
  | "game_icon"
  | "game_thumbnail"
  | "asset"
  | "bundle"
  | "group_icon"
  | "badge_icon"
  | "game_pass"
  | "outfit"
  | "player"

/** URL for a thumbnail. Use it as an `<img src>`; the webview caches it. When the image changes
 * during the run (`shell.thumbRevs`), the URL changes too, so the new one shows. Undefined outside
 * the app (`bun run dev` in a browser), so components show their fallback. */
export function thumbUrl(kind: ThumbKind, id: number | string, large = false) {
  if (!isTauri()) return undefined
  const path = `${kind}.${id}.${large ? "l" : "s"}`
  return withRev(convertFileSrc(path, "thumb"), shell.thumbRevs[path])
}

/** `url` with a revision added, so a new revision loads afresh. Leaves it alone for none. */
export function withRev(url: string, rev: number | null | undefined) {
  if (!rev) return url
  return `${url}${url.includes("?") ? "&" : "?"}r=${rev}`
}

/** URL for the background picture, an external HTTPS link loaded locally through the
 * `bg://` protocol (crates/bonsai/src/bg.rs). Undefined outside the app. */
export function backgroundUrl(file: string) {
  if (!isTauri()) return undefined
  return `${convertFileSrc("image", "bg")}?url=${encodeURIComponent(file)}`
}

/** URL for an asset's file, served by the `rbxasset://` protocol (crates/bonsai/src/asset.rs)
 * and fetched as `account`. Use it as an `<audio src>`. Undefined outside the app. */
export function assetFileUrl(account: number, id: number) {
  if (!isTauri()) return undefined
  return convertFileSrc(`${account}.${id}`, "rbxasset")
}

/** URL for a wiki article picture (`WikiImage.src`), served by the `wikiimg://` protocol
 * (crates/bonsai/src/wikiimg.rs), which fetches it from Fandom. Undefined outside the app. */
export function wikiImageUrl(src: string) {
  if (!isTauri() || !src) return undefined
  return convertFileSrc(src, "wikiimg")
}

/** Title bar glyphs from the desktop's icon theme, served by the `wicon://` protocol
 * (crates/bonsai/src/titlebar.rs). Only exist on Linux when `windowButtons` says so; undefined
 * outside the app. */
export function windowIcons(): WindowControlIcons | undefined {
  if (!isTauri()) return undefined
  return {
    minimize: convertFileSrc("minimize", "wicon"),
    maximize: convertFileSrc("maximize", "wicon"),
    restore: convertFileSrc("restore", "wicon"),
    close: convertFileSrc("close", "wicon"),
  }
}
