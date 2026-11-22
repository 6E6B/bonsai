import { convertFileSrc } from "@tauri-apps/api/core"
import * as THREE from "three"
import { TransformControls } from "three/examples/jsm/controls/TransformControls.js"
import dracoUrl from "three/examples/jsm/libs/draco/draco_decoder.js?url"
import { shell } from "~/lib/store"
import {
  API,
  Authentication,
  CFrame,
  getOriginalSize,
  HOST,
  type Instance,
  Outfit,
  OutfitRenderer,
  RBXRenderer,
  ScaleAccessoryForRig,
  Vector3,
} from "./engine"
import { assetPath } from "./host"
import { type OutfitJson, type Subject, subjectKey, type TryOnItem } from "./outfit"

/**
 * The 3D engine keeps one WebGL canvas for the whole app. A viewer leases it while it is on
 * screen; when another viewer takes it, the first one is told and falls back to its image.
 * This module is loaded with `import()` so three.js and the engine stay out of the main bundle.
 */

export type StageState = "loading" | "ready" | "error"

export interface Xyz {
  x: number
  y: number
  z: number
}

/** Where a worn accessory sits relative to where it attaches: studs, degrees and a factor. */
export interface Placement {
  position: Xyz
  rotation: Xyz
  scale: Xyz
}

export type PlacementGroup = keyof Placement

/** Which handle the gizmo shows: arrows, rings or boxes. */
export type GizmoTool = "move" | "turn" | "size"

export interface Gizmo {
  /** The worn accessory's asset id. */
  id: number
  tool: GizmoTool
  min: Placement
  max: Placement
  /** Values snap to these steps, the ones Roblox stores. */
  steps: Record<PlacementGroup, number>
  /** The placement drawn now. Read when a drag starts. */
  placement: () => Placement
  /** While dragging, with the placement already clamped and drawn. */
  onChange: (p: Placement) => void
  /** When a drag that changed something ends. */
  onCommit: (p: Placement) => void
}

export interface Lease {
  /** Draws an outfit or an accessory. False when it is already showing. */
  show(subject: Subject): boolean
  /** Plays a set animation (`idle`, `walk`) or `emote.<id>`. False when it couldn't. */
  play(name: string): Promise<boolean>
  orbit(how: string): void
  /** Draws a worn accessory at a placement without saving it. False when it isn't drawn. */
  preview(id: number, placement: Placement): boolean
  /** Puts a move, turn or size handle on a worn accessory; null takes it away. */
  gizmo(g: Gizmo | null): void
  resize(width: number, height: number): void
  release(): void
}

interface Holder {
  lost: () => void
  state: (s: StageState) => void
}

/** Debug logging for the 3D viewer. Filter the console by `[3d]`. */
function log(...args: unknown[]) {
  console.info("[3d]", ...args)
}

/** What the scene holds and where the camera is, for "loads but shows nothing". */
function snapshot(when: string) {
  const scene = RBXRenderer.firstScene
  const cam = scene.camera.position
  const target = scene.controls?.target
  const canvas = RBXRenderer.renderer?.domElement
  log(
    when,
    JSON.stringify({
      instances: scene.addedInstances.length,
      descs: scene.renderDescs.size,
      sceneObjects: scene.scene.children.length,
      camera: [cam.x, cam.y, cam.z].map((n) => +n.toFixed(2)),
      target: target ? [target.x, target.y, target.z].map((n) => +n.toFixed(2)) : null,
      canvas: canvas ? [canvas.width, canvas.height, canvas.isConnected] : null,
      resolution: RBXRenderer.resolution,
      lastFrame: RBXRenderer.renderer
        ? { calls: RBXRenderer.renderer.info.render.calls, triangles: RBXRenderer.renderer.info.render.triangles }
        : null,
      lost: RBXRenderer.renderer?.getContext().isContextLost() ?? null,
      animating: scene.shouldAnimate,
      rig: renderer?.currentRig ? renderer.currentRig.GetDescendants().length : null,
      updating: renderer
        ? { updating: renderer.currentlyUpdating, rig: renderer.currentlyChangingRig, queued: renderer.hasNewUpdate }
        : null,
    }),
  )
}

let booted: Promise<void> | null = null
let renderer: OutfitRenderer | null = null
/** Set when the renderer failed an outfit, so the next one starts from a fresh rig. */
let broken = false
let holder: Holder | null = null
/** The accessory drawn by itself, and the tree it came from. */
let alone: { root: Instance; accessory: Instance } | null = null
/** Bumped on every `show`, so a slow accessory load can tell it was replaced. */
let generation = 0
const auth = new Authentication()

/** Until when a still scene keeps drawing, so late texture and mesh updates make it to screen. */
let drawUntil = 0
/** Whether the canvas is on screen. Scrolled away or in a hidden tab, nothing is drawn. */
let onScreen = true

/** Draws the next frames. A still scene (an accessory, a paused rig) is drawn only after
 * something changes, instead of every frame: redrawing an unchanged WebGL canvas makes the
 * webview recomposite the page each frame, which is what made the item page lag. */
function invalidate(ms = 500) {
  drawUntil = Math.max(drawUntil, performance.now() + ms)
}

function tick(now: number) {
  requestAnimationFrame(tick)
  if (!holder || !onScreen) return
  if (renderer?.animationInterval !== undefined || now < drawUntil) RBXRenderer.animateAll(false)
}

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
  RBXRenderer.firstScene.controls?.addEventListener("change", () => invalidate())
  RBXRenderer.firstScene.compiledRenderDesc.Connect(() => invalidate())
  RBXRenderer.canvasContainer.style.position = "absolute"
  RBXRenderer.canvasContainer.style.inset = "0"
  RBXRenderer.firstScene.shouldAnimate = false
  new IntersectionObserver(([e]) => {
    onScreen = e.isIntersecting
    if (onScreen) invalidate()
  }).observe(RBXRenderer.canvasContainer)
  requestAnimationFrame(tick)
  log("engine started")
}

function boot() {
  booted ??= setup().catch((e) => {
    booted = null
    throw e
  })
  return booted
}

function outfitFrom(json: OutfitJson, wear: TryOnItem[]) {
  const outfit = new Outfit()
  outfit.fromJson(json)
  for (const w of wear) outfit.addAsset(w.id, w.typeId, w.name)
  return outfit
}

function ensureRenderer(outfit: Outfit) {
  if (broken) dropRenderer()
  if (renderer) {
    renderer.setOutfit(outfit)
    return renderer
  }
  log("new outfit renderer")
  const r = new OutfitRenderer(auth, outfit)
  r.onSuccess.Connect(() => log("outfit applied"))
  r.onRenderSuccess.Connect(() => {
    invalidate(1000)
    snapshot("outfit drawn")
    holder?.state("ready")
  })
  r.onError.Connect((why) => {
    // An outfit set while this one loaded is applied next; its own result decides.
    if (r.hasNewUpdate) return
    console.warn("[3d] outfit failed:", why)
    snapshot("after failure")
    broken = true
    holder?.state("error")
  })
  r.onRenderError.Connect((...a: unknown[]) => {
    console.warn("[3d] a part failed to draw", ...a)
    holder?.state("error")
  })
  renderer = r
  return r
}

function resetCamera() {
  if (alone) {
    frame([alone.accessory, ...alone.accessory.GetDescendants()])
    return
  }
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

function dropRenderer() {
  renderer?.destroy()
  renderer = null
  broken = false
}

function dropAlone() {
  if (!alone) return
  RBXRenderer.removeInstance(alone.accessory)
  alone.root.Destroy()
  alone = null
}

/** Shows or hides the ground shadow, which would cut off an accessory drawn at the origin. */
function setGround(on: boolean) {
  const plane = RBXRenderer.firstScene.shadowPlane
  if (plane) plane.visible = on
}

/** Points the camera at what is drawn, from the front, so it fills the view. */
function frame(parts: Instance[]) {
  const scene = RBXRenderer.firstScene
  const controls = scene.controls
  const box = new THREE.Box3()
  for (const p of parts) for (const o of scene.renderDescs.get(p)?.results ?? []) box.expandByObject(o)
  if (!controls || box.isEmpty()) return log("nothing to frame", { parts: parts.length, controls: !!controls })
  const center = box.getCenter(new THREE.Vector3())
  const radius = box.getBoundingSphere(new THREE.Sphere()).radius
  const fov = THREE.MathUtils.degToRad(scene.camera.fov)
  const distance = (radius * 1.3) / Math.sin(fov / 2)
  const [x, y, z] = scene.lookAwayVector
  controls.target.copy(center)
  scene.camera.position.copy(center).add(new THREE.Vector3(x, y, z).normalize().multiplyScalar(distance))
  controls.update()
}

/** Loads an accessory and draws it at the origin, the way Roblox's accessory thumbnails do. */
async function showAccessory(id: number, gen: number, state: (s: StageState) => void) {
  log("loading accessory", id)
  const rbx = await API.Asset.GetRBX(`rbxassetid://${id}`, { "Roblox-AssetFormat": "avatar_meshpart_accessory" })
  if (gen !== generation) return log("accessory replaced before it loaded", id)
  if (rbx instanceof Response) console.warn("[3d] accessory fetch failed", id, rbx.status)
  const root = rbx instanceof Response ? undefined : rbx.generateTree()
  const accessory = root?.FindFirstChildOfClass("Accessory")
  if (!root || !accessory) {
    console.warn(
      "[3d] no Accessory in asset",
      id,
      root?.GetChildren().map((c) => c.className),
    )
    root?.Destroy()
    return state("error")
  }
  accessory.FindFirstChild("Handle")?.setProperty("CFrame", new CFrame())
  alone = { root, accessory }
  RBXRenderer.addInstance(accessory, auth)
  const parts = [accessory, ...accessory.GetDescendants()]
  const ok = await RBXRenderer.firstScene.waitUntilFullyCompiled(parts)
  if (gen !== generation) return
  if (!ok) {
    console.warn("[3d] accessory didn't compile", id)
    return state("error")
  }
  frame(parts)
  invalidate(1000)
  snapshot("accessory drawn")
  state("ready")
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

// Adjusting worn accessories.

const DEG = Math.PI / 180
const GROUPS: readonly PlacementGroup[] = ["position", "rotation", "scale"]
const AXES = ["x", "y", "z"] as const

interface Worn {
  rig: Instance
  description: Instance
  accessoryDescription: Instance
  accessory: Instance
  handle: Instance
}

/** The drawn accessory with this asset id, and what Roblox's placement math needs for it. */
function worn(id: number): Worn | null {
  const rig = renderer?.currentRig
  const description = rig?.FindFirstChildOfClass("Humanoid")?.FindFirstChildOfClass("HumanoidDescription")
  if (!rig || !description) return null
  const want = BigInt(id)
  for (const d of description.GetChildren()) {
    if (d.className !== "AccessoryDescription" || d.Prop("AssetId") !== want) continue
    const accessory = d.Prop("Instance") as Instance | undefined
    const handle = accessory?.FindFirstChild("Handle")
    if (accessory && handle && !accessory.destroyed && accessory.parent === rig) {
      return { rig, description, accessoryDescription: d, accessory, handle }
    }
  }
  return null
}

const vector = (v: Xyz) => new Vector3(v.x, v.y, v.z)
const clone = (p: Placement): Placement => ({
  position: { ...p.position },
  rotation: { ...p.rotation },
  scale: { ...p.scale },
})
const same = (a: Placement, b: Placement) => GROUPS.every((g) => AXES.every((x) => Math.abs(a[g][x] - b[g][x]) < 1e-6))

/** Applies a placement the way the engine does for a new outfit, on just this accessory. */
function place(id: number, p: Placement): boolean {
  const w = worn(id)
  if (!w || !renderer) return false
  w.accessoryDescription.setProperty("Position", vector(p.position))
  w.accessoryDescription.setProperty("Rotation", vector(p.rotation))
  w.accessoryDescription.setProperty("Scale", vector(p.scale))
  ScaleAccessoryForRig(w.accessory, w.rig, renderer.outfit, w.description)
  return true
}

const quaternion = (cf: CFrame) =>
  new THREE.Quaternion().setFromEuler(
    new THREE.Euler(cf.Orientation[0] * DEG, cf.Orientation[1] * DEG, cf.Orientation[2] * DEG, "YXZ"),
  )
const turn = (r: Xyz) => new THREE.Quaternion().setFromEuler(new THREE.Euler(r.x * DEG, r.y * DEG, r.z * DEG, "YXZ"))

/**
 * The axes an adjustment moves along. Roblox offsets the accessory in the frame of the body
 * attachment it hangs from, scaled with the body, and turns it about the handle's center.
 */
interface Axes {
  center: THREE.Vector3
  basis: THREE.Quaternion
  bodyScale: THREE.Vector3
}

function axesOf(w: Worn, p: Placement): Axes | null {
  let body: Instance | undefined
  for (const a of w.handle.GetChildren()) {
    if (a.className !== "Attachment") continue
    const name = a.Prop("Name")
    for (const part of w.rig.GetChildren()) {
      if (part.className !== "Part" && part.className !== "MeshPart") continue
      body = part.GetChildren().find((c) => c.className === "Attachment" && c.Prop("Name") === name)
      if (body) break
    }
    if (body) break
  }
  const part = body?.parent
  if (!body || !part) return null
  const world = (part.Prop("CFrame") as CFrame).multiply(body.Prop("CFrame") as CFrame)
  const size = w.handle.Prop("Size") as Vector3
  const original = getOriginalSize(w.handle)
  const ratio = (s: number, o: number, a: number) => (o > 0 && a > 0 ? s / o / a : 1)
  return {
    center: new THREE.Vector3(...(w.handle.Prop("CFrame") as CFrame).Position),
    basis: quaternion(world),
    bodyScale: new THREE.Vector3(
      ratio(size.X, original.X, p.scale.x),
      ratio(size.Y, original.Y, p.scale.y),
      ratio(size.Z, original.Z, p.scale.z),
    ),
  }
}

/** Keeps a placement inside the bounds, on the steps Roblox stores. */
function limit(p: Placement, g: Gizmo): Placement {
  const out = clone(p)
  for (const group of GROUPS) {
    const step = g.steps[group]
    for (const x of AXES) {
      const lo = g.min[group][x]
      const hi = g.max[group][x]
      const v = step > 0 ? Math.round(out[group][x] / step) * step : out[group][x]
      out[group][x] = Math.min(Math.max(Number(v.toFixed(4)), lo), hi)
    }
  }
  return out
}

let gizmo: Gizmo | null = null
let transform: TransformControls | null = null
let handleObject: THREE.Object3D | null = null
let drag: { start: Placement; last: Placement; axes: Axes } | null = null
let following = 0

const MODES: Record<GizmoTool, "translate" | "rotate" | "scale"> = { move: "translate", turn: "rotate", size: "scale" }

/** Sets the handle where the placement puts it; `from` is the drag start, if dragging. */
function poseHandle(o: THREE.Object3D, axes: Axes, p: Placement, tool: GizmoTool, from?: Placement) {
  const offset = from
    ? new THREE.Vector3(p.position.x - from.position.x, p.position.y - from.position.y, p.position.z - from.position.z)
        .multiply(axes.bodyScale)
        .applyQuaternion(axes.basis)
    : new THREE.Vector3()
  o.position.copy(axes.center).add(offset)
  // Arrows follow the body's axes; rings and boxes the accessory's turned ones.
  o.quaternion.copy(axes.basis)
  if (tool !== "move") o.quaternion.multiply(turn(p.rotation))
  if (from && tool === "size") o.scale.set(p.scale.x / from.scale.x, p.scale.y / from.scale.y, p.scale.z / from.scale.z)
  else o.scale.set(1, 1, 1)
}

function startDrag() {
  const g = gizmo
  const w = g && worn(g.id)
  const start = g && clone(g.placement())
  const axes = w && start && axesOf(w, start)
  drag = g && start && axes ? { start, last: start, axes } : null
}

function dragged() {
  const g = gizmo
  const o = handleObject
  if (!g || !o || !drag) return
  const { start, axes } = drag
  const p = clone(start)
  const back = axes.basis.clone().invert()
  if (g.tool === "move") {
    const d = o.position.clone().sub(axes.center).applyQuaternion(back).divide(axes.bodyScale)
    p.position = { x: start.position.x + d.x, y: start.position.y + d.y, z: start.position.z + d.z }
  } else if (g.tool === "turn") {
    const e = new THREE.Euler().setFromQuaternion(back.multiply(o.quaternion), "YXZ")
    p.rotation = { x: e.x / DEG, y: e.y / DEG, z: e.z / DEG }
  } else {
    p.scale = { x: start.scale.x * o.scale.x, y: start.scale.y * o.scale.y, z: start.scale.z * o.scale.z }
  }
  const next = limit(p, g)
  // The handle stops at the bounds instead of running ahead of the accessory.
  poseHandle(o, axes, next, g.tool, start)
  if (same(next, drag.last)) return
  drag.last = next
  place(g.id, next)
  g.onChange(next)
}

function endDrag() {
  const d = drag
  drag = null
  if (gizmo && d && !same(d.last, d.start)) gizmo.onCommit(d.last)
}

function transformControls() {
  if (transform) return transform
  const scene = RBXRenderer.firstScene
  const canvas = RBXRenderer.renderer?.domElement
  if (!canvas) return null
  const o = new THREE.Object3D()
  const t = new TransformControls(scene.camera, canvas)
  t.setSpace("local")
  t.setSize(0.8)
  t.addEventListener("dragging-changed", (e) => {
    if (scene.controls) scene.controls.enabled = !e.value
  })
  t.addEventListener("mouseDown", startDrag)
  t.addEventListener("objectChange", dragged)
  t.addEventListener("mouseUp", endDrag)
  scene.scene.add(o, t.getHelper())
  handleObject = o
  transform = t
  return t
}

/** Keeps the handle on the accessory between drags, as the camera and the avatar move. */
function follow() {
  following = 0
  const g = gizmo
  const t = transform
  const o = handleObject
  if (!g || !t || !o) return
  following = requestAnimationFrame(follow)
  if (renderer) renderer.deltaTimeMultiplier = 0
  if (drag) return
  const w = worn(g.id)
  const axes = w && axesOf(w, g.placement())
  if (!axes) {
    if (t.object) t.detach()
    return
  }
  if (!t.object) t.attach(o)
  poseHandle(o, axes, g.placement(), g.tool)
}

function setGizmo(g: Gizmo | null) {
  const before = gizmo
  gizmo = g
  if (!g) {
    if (following) cancelAnimationFrame(following)
    following = 0
    drag = null
    transform?.detach()
    const controls = RBXRenderer.firstScene.controls
    if (controls) controls.enabled = true
    // The pose holds still while adjusting, so the handle stays put.
    if (before && renderer) renderer.deltaTimeMultiplier = 1
    return
  }
  const t = transformControls()
  if (!t) return
  if (before && (before.id !== g.id || before.tool !== g.tool)) drag = null
  t.setMode(MODES[g.tool])
  if (!following) follow()
}

/** Takes the canvas and mounts it in `el`. Rejects when WebGL or the engine won't start. */
export async function acquire(el: HTMLElement, lost: () => void, state: (s: StageState) => void): Promise<Lease> {
  await boot()
  holder?.lost()
  log("acquired", el.clientWidth, el.clientHeight)
  const me: Holder = { lost, state }
  holder = me
  el.prepend(RBXRenderer.canvasContainer)
  RBXRenderer.firstScene.shouldAnimate = true
  invalidate()
  let shown = ""

  const mine = () => holder === me
  return {
    show(subject) {
      const key = subjectKey(subject)
      if (!mine() || key === shown) return false
      log(
        "show",
        subject.kind === "accessory"
          ? subject
          : { rig: subject.outfit.playerAvatarType, assets: subject.outfit.assets.length, wear: subject.wear },
      )
      shown = key
      const gen = ++generation
      state("loading")
      invalidate()
      dropAlone()
      if (subject.kind === "accessory") {
        dropRenderer()
        setGround(false)
        void showAccessory(subject.id, gen, state).catch(() => gen === generation && state("error"))
        return true
      }
      setGround(true)
      const first = !renderer
      ensureRenderer(outfitFrom(subject.outfit, subject.wear)).startAnimating()
      if (first) resetCamera()
      return true
    },
    async play(name) {
      if (!mine() || !renderer) return false
      const ok = await renderer.setMainAnimation(name)
      invalidate()
      log("play", name, ok)
      return ok
    },
    orbit(how) {
      if (mine()) orbit(how)
    },
    preview(id, placement) {
      return mine() && place(id, placement)
    },
    gizmo(g) {
      if (mine()) setGizmo(g)
    },
    resize(width, height) {
      const [w, h] = [Math.round(width), Math.round(height)]
      const [cw, ch] = RBXRenderer.resolution
      // Resizing to the same size re-lays out the canvas and loops the ResizeObserver.
      if (mine() && w > 0 && h > 0 && (w !== cw || h !== ch)) {
        RBXRenderer.setRendererSize(w, h)
        invalidate()
      }
    },
    release() {
      if (!mine()) return
      holder = null
      generation++
      setGizmo(null)
      renderer?.stopAnimating()
      RBXRenderer.firstScene.shouldAnimate = false
      RBXRenderer.canvasContainer.remove()
    },
  }
}
