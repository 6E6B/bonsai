/** Where the 3D engine loads Roblox asset files from: the `rbxasset://` protocol
 * (crates/bonsai/src/asset.rs), which fetches and caches them in rbx-core. */

/** The path and query for one asset file. `representations` is the base64 JSON the engine
 * builds; the protocol takes it as base64url with the padding left in. The protocol rejects
 * anything else, so values are not percent-encoded. */
export function assetPath(account: number, id: number, format?: string, representations?: string) {
  const query: string[] = []
  if (format) query.push(`f=${format}`)
  if (representations) query.push(`r=${representations.replace(/\+/g, "-").replace(/\//g, "_")}`)
  return { path: `${account}.${id}`, query: query.length ? `?${query.join("&")}` : "" }
}
