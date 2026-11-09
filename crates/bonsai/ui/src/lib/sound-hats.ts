/** Hats that play a sound when worn. Asset ids; bundles never match. */
const SOUND_HATS: ReadonlySet<number> = new Set([
  33070696, // Transient Harmonica
  24112667, // Telamon's Chicken Suit
  24114402, // Brighteyes' Bloxy Cola Hat
  305888394, // Brighteyes' Witches Brew Hat
])

export function isSoundHat(id: number, bundle?: boolean): boolean {
  return !bundle && SOUND_HATS.has(id)
}
