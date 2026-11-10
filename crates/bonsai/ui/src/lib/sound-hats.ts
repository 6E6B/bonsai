import { createStore } from "solid-js/store"

/** What a sound hat's in-game script does: every `wait` seconds (a whole number picked between
 * the two, like `wait(math.random(a, b))`), it plays one of `sounds` at random. The sounds are
 * the hat's own files, bundled under `public/sound-hats/`. */
export interface SoundHatScript {
  sounds: readonly string[]
  wait: readonly [number, number]
}

/** Hats that play a sound when worn, by asset id. Bundles never match. */
const SOUND_HATS: ReadonlyMap<number, SoundHatScript> = new Map([
  // Transient Harmonica: "Harm" .. math.random(1, 4), every 5–20 s.
  [33070696, { sounds: ["Harm1", "Harm2", "Harm3", "Harm4"], wait: [5, 20] }],
  // Telamon's Chicken Suit: "Quack" .. math.random(1, 4), every 5–20 s.
  [24112667, { sounds: ["Quack1", "Quack2", "Quack3", "Quack4"], wait: [5, 20] }],
  // Brighteyes' Bloxy Cola Hat: "Quack" .. math.random(1, 3), every 5–20 s.
  [24114402, { sounds: ["Quack1_Bloxy", "Quack2_Bloxy", "Quack3_Bloxy"], wait: [5, 20] }],
  // Brighteyes' Witches Brew Hat: any Sound in the handle, every 5–15 s.
  [305888394, { sounds: ["Laugh_New", "Laugh2_New", "Laugh3_New"], wait: [5, 15] }],
])

export function soundHat(id: number, bundle?: boolean): SoundHatScript | undefined {
  return bundle ? undefined : SOUND_HATS.get(id)
}

export function isSoundHat(id: number, bundle?: boolean): boolean {
  return soundHat(id, bundle) !== undefined
}

/** Roblox's `math.random(a, b)`: a whole number from `a` to `b`, both included. */
const between = (a: number, b: number, random: () => number) => a + Math.floor(random() * (b - a + 1))

/** One turn of the script's loop: how long to wait, then which sound to play. */
export function nextSound(hat: SoundHatScript, random: () => number = Math.random) {
  const wait = between(hat.wait[0], hat.wait[1], random)
  const sound = hat.sounds[between(0, hat.sounds.length - 1, random)] as string
  return { wait, sound }
}

export const soundHatUrl = (sound: string) => `/sound-hats/${sound}.mp3`

const KEY = "sound-hats.on"
const stored = () => {
  try {
    return localStorage.getItem(KEY) !== "0"
  } catch {
    return true
  }
}
const [state, setState] = createStore({ on: stored() })

/** Whether item pages run a sound hat's script. On unless turned off; remembered. */
export const soundHatsOn = () => state.on

export function setSoundHatsOn(on: boolean) {
  setState("on", on)
  try {
    localStorage.setItem(KEY, on ? "1" : "0")
  } catch {}
}

/** Runs `hat`'s script until the returned function is called. Like `Sound:Play()`, playing a
 * sound that is still going restarts it. */
export function runSoundHat(hat: SoundHatScript): () => void {
  const audio = new Map(hat.sounds.map((s) => [s, new Audio(soundHatUrl(s))]))
  for (const a of audio.values()) a.preload = "auto"
  let timer: ReturnType<typeof setTimeout> | undefined
  const loop = () => {
    const step = nextSound(hat)
    timer = setTimeout(() => {
      play(step.sound)
      loop()
    }, step.wait * 1000)
  }
  const play = (sound: string) => {
    const a = audio.get(sound)
    if (!a) return
    a.currentTime = 0
    a.play().catch(() => {})
  }
  loop()
  return () => {
    clearTimeout(timer)
    for (const a of audio.values()) {
      a.pause()
      a.removeAttribute("src")
      a.load()
    }
  }
}
