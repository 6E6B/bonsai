import { Dialog as KDialog } from "@kobalte/core/dialog"
import { createContext, createSignal, type JSX, type ParentProps, Show, splitProps, useContext } from "solid-js"
import { Icon } from "./icon"

export interface DialogOptions {
  /** Shown in the header with a close button. Without it there is no header. */
  title?: string
  description?: string
  /** Accessible name when there is no title. */
  label?: string
  size?: "sm" | "md" | "lg"
  /** `top` sits near the top of the window, for command bars and pickers. */
  position?: "center" | "top"
  class?: string
  /** Called when the dialog closes for any reason: Escape, the close button, `close()`, or
   * another dialog replacing it. */
  onClose?: () => void
}

export interface DialogProps extends DialogOptions, ParentProps {
  open: boolean
  onOpenChange: (open: boolean) => void
}

export function Dialog(props: DialogProps) {
  const [local, opts] = splitProps(props, ["open", "onOpenChange", "children"])
  return (
    <KDialog open={local.open} onOpenChange={local.onOpenChange}>
      <KDialog.Portal>
        <KDialog.Overlay data-component="dialog-overlay" />
        <div data-component="dialog-positioner" data-position={opts.position ?? "center"}>
          <KDialog.Content
            data-component="dialog"
            data-size={opts.size ?? "md"}
            class={opts.class}
            aria-label={opts.title ? undefined : opts.label}
          >
            <Show when={opts.title}>
              <header data-slot="header">
                <KDialog.Title data-slot="title">{opts.title}</KDialog.Title>
                <KDialog.CloseButton data-slot="close" aria-label="Close">
                  <Icon name="x" size={14} />
                </KDialog.CloseButton>
              </header>
            </Show>
            <Show when={opts.description}>
              <KDialog.Description data-slot="description">{opts.description}</KDialog.Description>
            </Show>
            {local.children}
          </KDialog.Content>
        </div>
      </KDialog.Portal>
    </KDialog>
  )
}

interface DialogApi {
  /** Opens a dialog, replacing any open one. */
  show: (render: () => JSX.Element, options?: DialogOptions) => void
  close: () => void
}

const DialogContext = createContext<DialogApi>()

/** Hosts the one app-wide imperative dialog. `useDialog()` opens and closes it. */
export function DialogProvider(props: ParentProps) {
  const [open, setOpen] = createSignal(false)
  const [entry, setEntry] = createSignal<{ render: () => JSX.Element; options: DialogOptions }>()
  const closed = () => entry()?.options.onClose?.()
  const api: DialogApi = {
    show: (render, options = {}) => {
      if (open()) closed()
      setEntry({ render, options })
      setOpen(true)
    },
    close: () => {
      if (!open()) return
      setOpen(false)
      closed()
    },
  }
  return (
    <DialogContext.Provider value={api}>
      {props.children}
      <Dialog
        open={open()}
        onOpenChange={(v) => (v ? setOpen(true) : api.close())}
        title={entry()?.options.title}
        description={entry()?.options.description}
        label={entry()?.options.label}
        size={entry()?.options.size}
        position={entry()?.options.position}
        class={entry()?.options.class}
      >
        <Show when={entry()} keyed>
          {(e) => e.render()}
        </Show>
      </Dialog>
    </DialogContext.Provider>
  )
}

export function useDialog() {
  const ctx = useContext(DialogContext)
  if (!ctx) throw new Error("useDialog needs a DialogProvider")
  return ctx
}
