import { convertFileSrc, isTauri } from "@tauri-apps/api/core"

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

/** URL for a thumbnail. Use it as an `<img src>`; the webview caches it. Undefined outside the
 * app (`bun run dev` in a browser), so components show their fallback. */
export function thumbUrl(kind: ThumbKind, id: number | string, large = false) {
  if (!isTauri()) return undefined
  return convertFileSrc(`${kind}.${id}.${large ? "l" : "s"}`, "thumb")
}

/** URL for the background picture, a file name from `AppearanceView.background`, served by the
 * `bg://` protocol (crates/bonsai/src/bg.rs). Undefined outside the app. */
export function backgroundUrl(file: string) {
  if (!isTauri()) return undefined
  return convertFileSrc(file, "bg")
}

/** URL for an asset's file, served by the `rbxasset://` protocol (crates/bonsai/src/asset.rs)
 * and fetched as `account`. Use it as an `<audio src>`. Undefined outside the app. */
export function assetFileUrl(account: number, id: number) {
  if (!isTauri()) return undefined
  return convertFileSrc(`${account}.${id}`, "rbxasset")
}
