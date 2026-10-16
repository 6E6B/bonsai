import { convertFileSrc } from "@tauri-apps/api/core"
import * as THREE from "three"
import dracoUrl from "three/examples/jsm/libs/draco/draco_decoder.js?url"
import { shell } from "~/lib/store"
import { Authentication, HOST, Outfit, OutfitRenderer, RBXRenderer } from "./engine"
import { assetPath } from "./host"
import { type OutfitJson, outfitKey, type TryOnItem } from "./outfit"

/**
 * The 3D engine keeps one WebGL canvas for the whole app. A viewer leases it while it is on
 * screen; when another viewer takes it, the first one is told and falls back to its image.
 * This module is loaded with `import()` so three.js and the engine stay out of the main bundle.
 */

export type StageState = "loading" | "ready" | "error"

export interface Lease {
  /** Builds the outfit. False when it is already showing. */
  show(json: OutfitJson, tryOn: TryOnItem | null): boolean
  /** Plays a set animation (`idle`, `walk`) or `emote.<id>`. False when it couldn't. */
  play(name: string): Promise<boolean>
  orbit(how: string): void
  resize(width: number, height: number): void
  release(): void
}

interface Holder {
  lost: () => void
  state: (s: StageState) => void
}

let booted: Promise<void> | null = null
let renderer: OutfitRenderer | null = null
let holder: Holder | null = null

function loadScript(src: string) {
  return new Promise<void>((resolve, reject) => {
    const el = document.createElement("script")
    el.src = src
    el.onload = () => resolve()
    el.onerror = () => reject(new Error("Couldn't load the mesh decoder"))
    document.head.append(el)
  })
}

async function setup() {
  // FileMesh decodes Draco-compressed meshes with the global DracoDecoderModule.
  if (!("DracoDecoderModule" in globalThis)) await loadScript(dracoUrl)
  // Asset files are fetched as the active account; the protocol falls back to signed out.
  HOST.assetUrl = (id, format, representations) => {
    const { path, query } = assetPath(shell.active ?? 0, id, format, representations)
    return convertFileSrc(path, "rbxasset") + query
  }
  RBXRenderer.createContainer()
  await RBXRenderer.boilerplateSetup()
  RBXRenderer.create()
  RBXRenderer.setupScene("WellLit")
  RBXRenderer.setBackgroundTransparent(true)
  RBXRenderer.setupControls()
  RBXRenderer.canvasContainer.style.position = "absolute"
  RBXRenderer.canvasContainer.style.inset = "0"
  RBXRenderer.firstScene.shouldAnimate = false
  RBXRenderer.animateAll()
}

function boot() {
  booted ??= setup().catch((e) => {
    booted = null
    throw e
  })
  return booted
}

function outfitFrom(json: OutfitJson, tryOn: TryOnItem | null) {
  const outfit = new Outfit()
  outfit.fromJson(json)
  if (tryOn) outfit.addAsset(tryOn.id, tryOn.typeId, tryOn.name)
  return outfit
}

function ensureRenderer(outfit: Outfit) {
  if (renderer) {
    renderer.setOutfit(outfit)
    return renderer
  }
  const r = new OutfitRenderer(new Authentication(), outfit)
  r.onRenderSuccess.Connect(() => holder?.state("ready"))
  r.onError.Connect(() => holder?.state("error"))
  r.onRenderError.Connect(() => holder?.state("error"))
  renderer = r
  return r
}

function resetCamera() {
  const scene = RBXRenderer.firstScene
  const controls = scene.controls
  if (!controls) return
  const [x, y, z] = scene.lookAwayVector
  const d = scene.lookAwayDistance
  controls.target.set(...RBXRenderer.orbitControlsTarget)
  scene.camera.position.set(x * d, 3 + y * d, z * d)
  controls.update()
  renderer?.centerCamera()
}

const STEP = Math.PI / 12

function orbit(how: string) {
  const scene = RBXRenderer.firstScene
  const controls = scene.controls
  if (!controls) return
  if (how === "reset") return resetCamera()
  const offset = new THREE.Vector3().subVectors(scene.camera.position, controls.target)
  const s = new THREE.Spherical().setFromVector3(offset)
  if (how === "left") s.theta -= STEP
  else if (how === "right") s.theta += STEP
  else if (how === "up") s.phi -= STEP
  else if (how === "down") s.phi += STEP
  else if (how === "in") s.radius /= 1.2
  else if (how === "out") s.radius *= 1.2
  else return
  s.phi = Math.min(Math.max(s.phi, 0.05), Math.PI - 0.05)
  s.radius = Math.min(Math.max(s.radius, controls.minDistance || 0.5), controls.maxDistance)
  scene.camera.position.copy(controls.target).add(offset.setFromSpherical(s))
  controls.update()
}

/** Takes the canvas and mounts it in `el`. Rejects when WebGL or the engine won't start. */
export async function acquire(el: HTMLElement, lost: () => void, state: (s: StageState) => void): Promise<Lease> {
  await boot()
  holder?.lost()
  const me: Holder = { lost, state }
  holder = me
  el.prepend(RBXRenderer.canvasContainer)
  RBXRenderer.firstScene.shouldAnimate = true
  let shown = ""

  const mine = () => holder === me
  return {
    show(json, tryOn) {
      const key = outfitKey(json, tryOn)
      if (!mine() || key === shown) return false
      const first = !renderer
      shown = key
      state("loading")
      ensureRenderer(outfitFrom(json, tryOn)).startAnimating()
      if (first) resetCamera()
      return true
    },
    async play(name) {
      if (!mine() || !renderer) return false
      return renderer.setMainAnimation(name)
    },
    orbit(how) {
      if (mine()) orbit(how)
    },
    resize(width, height) {
      if (mine() && width > 0 && height > 0) RBXRenderer.setRendererSize(Math.round(width), Math.round(height))
    },
    release() {
      if (!mine()) return
      holder = null
      renderer?.stopAnimating()
      RBXRenderer.firstScene.shouldAnimate = false
      RBXRenderer.canvasContainer.remove()
    },
  }
}
