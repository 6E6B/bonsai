import { isTauri } from "@tauri-apps/api/core"
import { createEffect, createMemo, createSignal, on, onCleanup, onMount, Show } from "solid-js"
import { createStore } from "solid-js/store"
import { handle, menuItem, run } from "~/lib/commands/registry"
import { type Avatar3dView, type AvatarSource, commands } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
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

const KEY = "avatar3d"

function stored() {
  try {
    return localStorage.getItem(KEY) !== "off"
  } catch {
    return true
  }
}

/** Whether viewers show the 3D model or the image. Remembered per machine. */
const [mode, setMode] = createStore({ on: stored() })

/** Switches viewers to 3D, as Try On does. */
export const show3d = () => setOn(true)

function setOn(on: boolean) {
  setMode("on", on)
  try {
    localStorage.setItem(KEY, on ? "on" : "off")
  } catch {}
}

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
  /** The 2D image, shown in 2D mode, while the engine starts and when it can't. */
  image?: string
  alt: string
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
  const key = createMemo(() => JSON.stringify(props.source ?? null), undefined, { equals: (a, b) => a === b })
  const view = useLive(key, () => (props.source ? commands.avatar3d(props.source) : Promise.resolve(blank())), blank)
  const [state, setState] = createSignal<StageState | "off" | "failed">("off")
  const [anim, setAnim] = createSignal("idle")
  const [lease, setLease] = createSignal<Lease | null>(null)
  // The image stays up until the first model has drawn; later changes redraw in place.
  const [drawn, setDrawn] = createSignal(false)
  // Bumped each time a new subject is shown, so its showcase animation plays again.
  const [shown, setShown] = createSignal(0)
  let box!: HTMLDivElement
  let disposed = false

  const want3d = () => mode.on && isTauri()
  const subject = (): Subject | null => {
    if (props.accessory) return { kind: "accessory", id: props.accessory }
    const wear = props.wear ?? []
    if (!props.source) return { kind: "outfit", outfit: MANNEQUIN, wear }
    if (view.rig === "" || view.error !== null) return null
    return { kind: "outfit", outfit: toOutfitJson(view), wear }
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

  onCleanup(handle("avatar.view_3d", () => setOn(!mode.on)))
  onCleanup(handle("avatar.play", (arg) => void play(arg)))
  onCleanup(handle("avatar.orbit", (arg) => lease()?.orbit(arg?.trim().toLowerCase() || "reset")))

  const onKey = (e: KeyboardEvent) => {
    const how = ORBIT_KEYS[e.key]
    if (!how || !lease() || e.ctrlKey || e.metaKey || e.altKey) return
    e.preventDefault()
    e.stopPropagation()
    lease()?.orbit(how)
  }

  const showImage = () => !lease() || !drawn()
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
    menuItem("avatar.view_3d", { label: mode.on ? "Show image" : "Show 3D", icon: mode.on ? "image" : "model" }),
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
      <Viewport
        ref={box}
        tabindex={lease() ? 0 : -1}
        role="img"
        aria-label={props.alt}
        onKeyDown={onKey}
        class="w-full"
      >
        <Show when={showImage()}>
          <Thumbnail class="absolute inset-0 w-full" alt={props.alt} fallback="avatar" src={props.image} />
        </Show>
        <Show when={state() === "loading" && want3d()}>
          <span class="absolute right-2 bottom-2">
            <Spinner size="sm" />
          </span>
        </Show>
      </Viewport>
      <div class="flex items-center gap-1">
        <IconButton
          size="sm"
          icon={mode.on ? "image" : "model"}
          label={mode.on ? "Show image" : "Show 3D"}
          onClick={() => run("avatar.view_3d")}
        />
        <Show when={lease()}>
          <IconButton size="sm" icon="refresh" label="Reset camera" onClick={() => run("avatar.orbit", "reset")} />
        </Show>
        <Show when={state() === "failed" || state() === "error"}>
          <span class="text-xs text-fg-muted">3D unavailable</span>
        </Show>
        <span class="flex-1" />
        <Menu size="sm" label="Viewer options" icon="more" items={toolbarMenu()} />
      </div>
    </div>
  )
}
