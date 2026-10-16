import { isTauri } from "@tauri-apps/api/core"
import { createEffect, createMemo, createSignal, on, onCleanup, onMount, Show } from "solid-js"
import { createStore } from "solid-js/store"
import { handle, menuItem, run } from "~/lib/commands/registry"
import { type Avatar3dView, type AvatarSource, commands } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { IconButton, Menu, type MenuEntry, Select, Spinner, showToast, Thumbnail } from "~/lib/ui"
import { animationName, BASIC_ANIMATIONS, emoteIds, type TryOnItem, toOutfitJson } from "./outfit"
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
  source: AvatarSource
  /** The 2D image, shown in 2D mode, while the engine starts and when it can't. */
  image?: string
  alt: string
  /** An item worn on top of the avatar. */
  tryOn?: TryOnItem | null
  /** Played once the model has drawn, like an emote being previewed. */
  animation?: string
  /** Shows the animation and emote pickers. */
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
  const key = createMemo(() => JSON.stringify(props.source), undefined, { equals: (a, b) => a === b })
  const view = useLive(key, () => commands.avatar3d(props.source), blank)
  const [state, setState] = createSignal<StageState | "off" | "failed">("off")
  const [anim, setAnim] = createSignal("idle")
  const [lease, setLease] = createSignal<Lease | null>(null)
  // The image stays up until the first model has drawn; later changes redraw in place.
  const [drawn, setDrawn] = createSignal(false)
  let box!: HTMLDivElement
  let disposed = false

  const want3d = () => mode.on && isTauri()
  const ready = () => view.rig !== "" && view.error === null

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
    if (!l || !ready()) return
    if (l.show(toOutfitJson(view), props.tryOn ?? null)) setAnim("idle")
  })
  createEffect(
    on([lease, drawn, () => props.animation], ([l, d, a]) => {
      if (!l || !d || !a) return
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
  const emotes = () => emoteIds(view)
  const animOptions = () => [
    ...BASIC_ANIMATIONS.map((a) => ({ value: a, label: a[0].toUpperCase() + a.slice(1) })),
    ...emotes().map((e) => ({ value: `emote.${e.id}`, label: e.name })),
    // An emote being previewed that isn't equipped.
    ...(props.animation?.startsWith("emote.") && !emotes().some((e) => `emote.${e.id}` === props.animation)
      ? [{ value: props.animation, label: "This emote" }]
      : []),
  ]
  const toolbarMenu = (): MenuEntry[] => [
    menuItem("avatar.view_3d", { label: mode.on ? "Show image" : "Show 3D", icon: mode.on ? "image" : "model" }),
    menuItem("avatar.orbit", { arg: "reset", label: "Reset camera", icon: "refresh", disabled: !lease() }),
    ...(props.menu ?? []),
  ]

  return (
    <div class={`flex flex-col gap-2 ${props.class ?? ""}`}>
      <div
        ref={box}
        tabindex={lease() ? 0 : -1}
        role="img"
        aria-label={props.alt}
        onKeyDown={onKey}
        class="relative aspect-square w-full overflow-hidden rounded-md outline-none focus-visible:ring-2 focus-visible:ring-accent"
      >
        <Show when={showImage()}>
          <Thumbnail class="absolute inset-0 w-full" alt={props.alt} fallback="avatar" src={props.image} />
        </Show>
        <Show when={state() === "loading" && want3d()}>
          <span class="absolute right-2 bottom-2">
            <Spinner size="sm" />
          </span>
        </Show>
      </div>
      <div class="flex items-center gap-1">
        <IconButton
          size="sm"
          icon={mode.on ? "image" : "model"}
          label={mode.on ? "Show image" : "Show 3D"}
          onClick={() => run("avatar.view_3d")}
        />
        <Show when={props.controls && lease()}>
          <Select
            size="sm"
            aria-label="Animation"
            class="min-w-0 flex-1"
            options={animOptions()}
            value={anim()}
            onChange={(v) => void run("avatar.play", v)}
          />
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
