import { Toast as KToast, toaster } from "@kobalte/core/toast"
import { createSignal, onCleanup, Show } from "solid-js"
import { Portal } from "solid-js/web"
import { Icon, type IconName } from "./icon"

export type ToastVariant = "neutral" | "success" | "warning" | "error"

export interface ToastOptions {
  title: string
  description?: string
  variant?: ToastVariant
  /** Live countdown in seconds; stays open until the timer reaches 0. */
  countdownSecs?: number
  /** Deduplication key (e.g. rate-limited host) to update an open countdown in place. */
  key?: string
}

const icons: Record<ToastVariant, IconName | undefined> = {
  neutral: undefined,
  success: "circle-check",
  warning: "alert",
  error: "circle-x",
}

const activeCountdowns = new Map<string, { id: number; bump: (secs: number) => void }>()

/** Shows a toast. Returns its id for `dismissToast`. */
export function showToast(toast: string | ToastOptions) {
  const t = typeof toast === "string" ? { title: toast } : toast
  const variant = t.variant ?? "neutral"
  const secs = t.countdownSecs && t.countdownSecs > 0 ? Math.ceil(t.countdownSecs) : 0
  if (secs > 0 && t.key) {
    const existing = activeCountdowns.get(t.key)
    if (existing) {
      existing.bump(secs)
      return existing.id
    }
  }
  return toaster.show((p) => {
    const [left, setLeft] = createSignal(secs)
    if (secs > 0) {
      let deadline = Date.now() + secs * 1000
      const bump = (nextSecs: number) => {
        deadline = Math.max(deadline, Date.now() + Math.ceil(nextSecs) * 1000)
        setLeft(Math.max(1, Math.ceil((deadline - Date.now()) / 1000)))
      }
      if (t.key) activeCountdowns.set(t.key, { id: p.toastId, bump })
      const timer = setInterval(() => {
        const rem = Math.ceil((deadline - Date.now()) / 1000)
        if (rem <= 0) {
          setLeft(0)
          clearInterval(timer)
          toaster.dismiss(p.toastId)
        } else {
          setLeft(rem)
        }
      }, 250)
      onCleanup(() => {
        clearInterval(timer)
        if (t.key && activeCountdowns.get(t.key)?.id === p.toastId) {
          activeCountdowns.delete(t.key)
        }
      })
    }
    const desc = () => {
      const rem = left()
      if (rem > 0) {
        return t.description ? `${t.description} · Retrying in ${rem}s…` : `Retrying in ${rem}s…`
      }
      return t.description
    }
    return (
      <KToast toastId={p.toastId} persistent={secs > 0} data-component="toast" data-variant={variant}>
        <Show when={icons[variant]}>
          {(name) => (
            <span data-slot="icon">
              <Icon name={name()} size={16} />
            </span>
          )}
        </Show>
        <div data-slot="body">
          <KToast.Title data-slot="title">{t.title}</KToast.Title>
          <Show when={desc()}>{(d) => <KToast.Description data-slot="description">{d()}</KToast.Description>}</Show>
        </div>
        <KToast.CloseButton data-slot="close" aria-label="Dismiss">
          <Icon name="x" size={12} />
        </KToast.CloseButton>
      </KToast>
    )
  })
}

export function dismissToast(id: number) {
  toaster.dismiss(id)
}

/** Where toasts appear. Render once at the app root. */
export function Toaster() {
  return (
    <Portal>
      <KToast.Region data-component="toaster" duration={4000} limit={5} swipeDirection="right">
        <KToast.List data-slot="list" />
      </KToast.Region>
    </Portal>
  )
}
