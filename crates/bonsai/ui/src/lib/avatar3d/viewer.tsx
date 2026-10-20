import { isTauri } from "@tauri-apps/api/core"
import { createEffect, createMemo, createSignal, on, onCleanup, onMount, Show } from "solid-js"
import { handle, menuItem, run } from "~/lib/commands/registry"
import { type Avatar3dView, type AvatarSource, commands } from "~/lib/ipc/bindings"
import { shell, useLive } from "~/lib/store"
import { IconButton, Menu, type MenuEntry, Spinner, showToast, Thumbnail, Viewport } from "~/lib/ui"
import {
  animationName,
  BASIC_ANIMATIONS,
  emoteIds,
  MANNEQUIN,
  type Subject,
  type TryOnItem,
  toOutfitJson,
} from "./outfit"
import type { Lease, StageState } from "./stage"

// Showing 3D or the image, once toggled. Each new viewer starts from the setting again.
const [picked, setPicked] = createSignal<boolean | null>(null)
const is3d = () => picked() ?? shell.avatar3d

/** Switches viewers to 3D, as Try On does. */
export const show3d = () => setPicked(true)

const blank = (): Avatar3dView => ({
  loading: true,
  error: null,
  rig: "",
  assets: [],
  emotes: [],
  colors: [],
  scales: { height: null, width: null, head: null, depth: null, proportion: null, bodyType: null },
  rev: null,
})

const ORBIT_KEYS: Record<string, string> = {
  ArrowLeft: "left",
  ArrowRight: "right",
  ArrowUp: "up",
  ArrowDown: "down",
  "+": "in",
  "=": "in",
  "-": "out",
  "0": "reset",
}

export interface AvatarViewerProps {
  /** Whose avatar to draw. Without one, a plain grey mannequin. */
  source?: AvatarSource
  /** The 2D image, shown in 2D mode and when the model can't load. */
  image?: string
  alt: string
  /** Exact body colors from the editor, in head/torso/arms/legs order. */
  colorHex?: string[]
  /** Items worn on top of the avatar, like an item or bundle being tried on. */
  wear?: TryOnItem[]
  /** An accessory drawn by itself, instead of any avatar. */
  accessory?: number
  /** Played once the model has drawn, like an emote being previewed. */
  animation?: string
  /** Adds the Animation submenu. */
  controls?: boolean
  class?: string
  /** Menu entries added to the toolbar's menu. */
  menu?: MenuEntry[]
}

/**
 * An avatar in 3D with orbit, zoom, pan and animations, falling back to its image. Drag to
 * orbit, right-drag to pan, scroll to zoom; with focus, arrows orbit, +/- zoom and 0 resets.
 * Only one viewer shows the model at a time; the engine loads on first use.
 */
export function AvatarViewer(props: AvatarViewerProps) {
  const key = createMemo(() => JSON.stringify([shell.active, props.source ?? null]), undefined, {
    equals: (a, b) => a === b,
  })
  const view = useLive(key, () => (props.source ? commands.avatar3d(props.source) : Promise.resolve(blank())), blank)
  // Saved outfits use RGB colors; read the existing UI view alongside the legacy
  // 3D payload so generated bindings remain unchanged.
  const rgb = useLive(
    () => [key(), shell.active] as const,
    async () => ({
      colors:
        props.source?.kind === "outfit" ? (await commands.outfitLook(props.source.id)).colors.map((c) => c.hex) : [],
    }),
    () => ({ colors: [] as string[] }),
  )
  const [state, setState] = createSignal<StageState | "off" | "failed">("off")
  const [anim, setAnim] = createSignal("idle")
  const [lease, setLease] = createSignal<Lease | null>(null)
  // Animations wait for the first model to draw.
  const [drawn, setDrawn] = createSignal(false)
  // Bumped each time a new subject is shown, so its showcase animation plays again.
  const [shown, setShown] = createSignal(0)
  let box!: HTMLDivElement
  let disposed = false

  setPicked(null)
  const want3d = () => is3d() && isTauri()
  const subject = (): Subject | null => {
    if (props.accessory) return { kind: "accessory", id: props.accessory }
    const wear = props.wear ?? []
    if (!props.source) return { kind: "outfit", outfit: MANNEQUIN, wear }
    if (view.rig === "" || view.error !== null) return null
    return { kind: "outfit", outfit: toOutfitJson(view, props.colorHex ?? rgb.colors), wear }
  }
  const ready = () => subject() !== null

  createEffect(
    on([want3d, ready], ([want, ok]) => {
      if (!want || !ok || lease()) return
      setState("loading")
      import("./stage")
        .then((s) =>
          s.acquire(
            box,
            () => {
              setLease(null)
              setDrawn(false)
            },
            (st) => {
              setState(st)
              if (st === "ready") setDrawn(true)
            },
          ),
        )
        .then((l) => {
          // Turned off or unmounted while the engine started.
          if (disposed || !want3d()) return l.release()
          setLease(l)
          l.resize(box.clientWidth, box.clientHeight)
        })
        .catch((e) => {
          console.warn("3D viewer:", e)
          setState("failed")
        })
    }),
  )
  createEffect(() => {
    if (!want3d()) {
      lease()?.release()
      setLease(null)
      setDrawn(false)
    }
  })
  createEffect(() => {
    const l = lease()
    const s = subject()
    if (l && s && l.show(s)) {
      setAnim("idle")
      setShown((n) => n + 1)
    }
  })
  createEffect(
    on([lease, drawn, () => props.animation, shown], ([l, d, a]) => {
      if (!l || !d || !a || props.accessory) return
      void l.play(a).then((ok) => ok && setAnim(a))
    }),
  )
  onCleanup(() => {
    disposed = true
    lease()?.release()
  })

  onMount(() => {
    const ro = new ResizeObserver(([e]) => lease()?.resize(e.contentRect.width, e.contentRect.height))
    ro.observe(box)
    onCleanup(() => ro.disconnect())
  })

  async function play(arg: string | null) {
    const name = animationName(arg)
    const l = lease()
    if (!name || !l) return
    if (await l.play(name)) setAnim(name)
    else showToast({ title: "Couldn't play that animation", variant: "error" })
  }

  onCleanup(handle("avatar.view_3d", () => setPicked(!is3d())))
  onCleanup(handle("avatar.play", (arg) => void play(arg)))
  onCleanup(handle("avatar.orbit", (arg) => lease()?.orbit(arg?.trim().toLowerCase() || "reset")))

  const onKey = (e: KeyboardEvent) => {
    const how = ORBIT_KEYS[e.key]
    if (!how || !lease() || e.ctrlKey || e.metaKey || e.altKey) return
    e.preventDefault()
    e.stopPropagation()
    lease()?.orbit(how)
  }

  // In 3D the image never shows unless the model can't; a spinner holds the space instead.
  const failed = () => state() === "failed" || state() === "error" || (!!props.source && view.error !== null)
  const showImage = () => !want3d() || failed()
  const waiting = () => want3d() && !failed() && state() !== "ready"
  const animated = () => !!lease() && !props.accessory
  const animations = (): MenuEntry[] => {
    const item = (value: string, label: string) =>
      menuItem("avatar.play", { arg: value, label, icon: value === anim() ? "check" : undefined })
    const emotes = emoteIds(view).map((e) => item(`emote.${e.id}`, e.name))
    // An emote or animation pack being previewed that isn't equipped.
    const extra = props.animation
    const listed =
      extra && (BASIC_ANIMATIONS.some((a) => a === extra) || emoteIds(view).some((e) => `emote.${e.id}` === extra))
    return [
      ...BASIC_ANIMATIONS.map((a) => item(a, a[0].toUpperCase() + a.slice(1))),
      ...(extra && !listed ? [item(extra, "This item")] : []),
      ...(emotes.length ? [{ kind: "heading" as const, label: "Emotes" }, ...emotes] : []),
    ]
  }
  const toolbarMenu = (): MenuEntry[] => [
    menuItem("avatar.view_3d", { label: is3d() ? "Show image" : "Show 3D", icon: is3d() ? "image" : "model" }),
    menuItem("avatar.orbit", { arg: "reset", label: "Reset camera", icon: "refresh", disabled: !lease() }),
    ...(props.controls
      ? [
          {
            kind: "submenu" as const,
            label: "Animation",
            icon: "play" as const,
            items: animations(),
            disabled: !animated(),
          },
        ]
      : []),
    ...(props.menu?.length ? [{ kind: "separator" as const }, ...props.menu] : []),
  ]

  return (
    <div class={`flex flex-col gap-2 ${props.class ?? ""}`}>
      <Viewport tabindex={lease() ? 0 : -1} role="img" aria-label={props.alt} onKeyDown={onKey} class="w-full">
        {/* The engine's canvas lives here. Solid clears an element whose dynamic children all
            go away, so the canvas can't share a parent with the <Show>s below. */}
        <div ref={box} class="absolute inset-0" classList={{ invisible: waiting() || showImage() }} />
        <Show when={showImage()}>
          <Thumbnail class="absolute inset-0 w-full" alt={props.alt} fallback="avatar" src={props.image} />
        </Show>
        <Show when={waiting()}>
          <span class="absolute inset-0 flex items-center justify-center">
            <Spinner />
          </span>
        </Show>
      </Viewport>
      <div class="flex items-center gap-1">
        <IconButton
          size="sm"
          icon={is3d() ? "image" : "model"}
          label={is3d() ? "Show image" : "Show 3D"}
          onClick={() => run("avatar.view_3d")}
        />
        <Show when={lease()}>
          <IconButton size="sm" icon="refresh" label="Reset camera" onClick={() => run("avatar.orbit", "reset")} />
        </Show>
        <Show when={want3d() && failed()}>
          <span class="text-xs text-fg-muted">3D unavailable</span>
        </Show>
        <span class="flex-1" />
        <Menu size="sm" label="Viewer options" icon="more" items={toolbarMenu()} />
      </div>
    </div>
  )
}
