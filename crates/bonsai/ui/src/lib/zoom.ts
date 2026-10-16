/** The zoom levels bonsai-core's `ui_scale` setting takes. */
export const ZOOMS = [0.67, 0.75, 0.8, 0.9, 1, 1.1, 1.25, 1.5, 1.75, 2]

/** The level `by` steps from `z`, starting from the nearest level when `z` isn't one. */
export function stepZoom(z: number, by: 1 | -1): number {
  const near = ZOOMS.reduce((best, x, i) => (Math.abs(x - z) < Math.abs(ZOOMS[best] - z) ? i : best), 0)
  return ZOOMS[Math.min(ZOOMS.length - 1, Math.max(0, near + by))]
}
