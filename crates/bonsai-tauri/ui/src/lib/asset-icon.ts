import type { IconName } from "~/lib/ui"

/** Fallback picture for an asset type (Roblox's AssetType ids), for assets without a thumbnail. */
export function assetIcon(assetType: number | null | undefined): IconName {
  switch (assetType) {
    case 1:
    case 13:
      return "image"
    case 3:
      return "audio"
    case 4:
    case 10:
    case 32:
    case 40:
      return "model"
    case 5:
      return "code"
    case 9:
      return "game"
    case 21:
      return "badge"
    case 34:
      return "pass"
    case 38:
      return "plugin"
    case 62:
      return "video"
    default:
      return "avatar"
  }
}
