import { Popover as KPopover } from "@kobalte/core/popover"
import { createEffect, createMemo, createSignal, createUniqueId, For, on, Show } from "solid-js"
import { Icon } from "./icon"
import { Spinner } from "./spinner"
import { TextField } from "./text-field"

/** A choice in a Combobox. `detail` is muted text after the label. */
export interface ComboboxOption {
  value: string
  label: string
  detail?: string
}

export interface ComboboxProps {
  /** Matches for the search text. The caller searches: the list is shown as given. */
  options: ComboboxOption[]
  /** The chosen value. Shown on the button even when it is not among `options`. */
  value?: string
  /** May return a promise; if it resolves to false, the button goes back to `value`. */
  onChange?: (value: string) => unknown
  /** Called with the search text as the user types, and with "" each time the list opens. */
  onSearch?: (text: string) => void
  /** Called when the list opens, to start loading `options`. */
  onOpen?: () => void
  /** Shows a spinner while matches load. */
  loading?: boolean
  /** Shown on the button when there is no value. */
  placeholder?: string
  /** Placeholder of the search field. */
  searchPlaceholder?: string
  label?: string
  /** Accessible name when there is no visible label. */
  "aria-label"?: string
  size?: "sm" | "md" | "lg"
  disabled?: boolean
  class?: string
}

/**
 * A button showing the chosen value that opens a search field over a list, for picking from more
 * options than a Select can hold. The search text never replaces the value: what was picked
 * stays on the button while the caller saves it. Arrows move, Enter picks, Esc closes.
 */
export function Combobox(props: ComboboxProps) {
  const id = createUniqueId()
  const [open, setOpen] = createSignal(false)
  const [text, setText] = createSignal("")
  const [active, setActive] = createSignal(0)
  // The pick shows at once; it gives way when the caller's value changes.
  const [picked, setPicked] = createSignal<ComboboxOption>()
  createEffect(
    on(
      () => props.value,
      () => setPicked(undefined),
      { defer: true },
    ),
  )

  const shown = createMemo(() => {
    const p = picked()
    if (p) return p.label
    const v = props.value
    if (!v) return undefined
    return props.options.find((o) => o.value === v)?.label ?? v
  })
  const selected = () => picked()?.value ?? props.value

  // Highlight the chosen option when the list shows it, else the first.
  createEffect(
    on(
      () => props.options,
      (options) => {
        const i = options.findIndex((o) => o.value === selected())
        setActive(text() === "" && i >= 0 ? i : 0)
      },
    ),
  )
  let list: HTMLDivElement | undefined
  createEffect(() => {
    const i = active()
    if (open()) list?.querySelector(`[data-index="${i}"]`)?.scrollIntoView({ block: "nearest" })
  })

  const setOpenState = (o: boolean) => {
    setOpen(o)
    if (!o) return
    setText("")
    props.onSearch?.("")
    props.onOpen?.()
  }
  const pick = (o: ComboboxOption) => {
    setOpen(false)
    if (o.value === selected()) return
    setPicked(o)
    void Promise.resolve(props.onChange?.(o.value)).then((ok) => {
      if (ok === false && picked() === o) setPicked(undefined)
    })
  }
  const onKeyDown = (e: KeyboardEvent) => {
    const n = props.options.length
    if (e.key === "ArrowDown" || e.key === "ArrowUp") {
      e.preventDefault()
      if (n > 0) setActive((i) => (i + (e.key === "ArrowDown" ? 1 : n - 1)) % n)
    } else if (e.key === "Enter") {
      e.preventDefault()
      const o = props.options[active()]
      if (o) pick(o)
    }
  }

  return (
    <div data-component="combobox" data-size={props.size ?? "md"} class={props.class}>
      <Show when={props.label}>
        <span data-slot="field-label" id={`${id}-label`}>
          {props.label}
        </span>
      </Show>
      <KPopover open={open()} onOpenChange={setOpenState} placement="bottom-start" gutter={4} sameWidth>
        <KPopover.Trigger
          data-slot="trigger"
          aria-label={props["aria-label"]}
          aria-labelledby={props.label ? `${id}-label` : undefined}
          disabled={props.disabled}
          data-disabled={props.disabled ? "" : undefined}
        >
          <span data-slot="value" data-placeholder-shown={shown() ? undefined : ""}>
            {shown() ?? props.placeholder}
          </span>
          <span data-slot="chevron">
            <Icon name="chevron-down" size={14} />
          </span>
        </KPopover.Trigger>
        <KPopover.Portal>
          <KPopover.Content data-component="combobox-content">
            <div data-slot="search">
              <TextField
                icon="search"
                size="sm"
                class="w-full"
                aria-label={props.searchPlaceholder ?? "Search"}
                placeholder={props.searchPlaceholder ?? "Search"}
                role="combobox"
                aria-expanded="true"
                aria-controls={`${id}-list`}
                aria-activedescendant={props.options[active()] ? `${id}-${active()}` : undefined}
                autocomplete="off"
                spellcheck={false}
                value={text()}
                onChange={(t) => {
                  setText(t)
                  props.onSearch?.(t)
                }}
                onKeyDown={onKeyDown}
              />
              <Show when={props.loading}>
                <Spinner size="sm" />
              </Show>
            </div>
            <div data-slot="listbox" role="listbox" id={`${id}-list`} ref={list}>
              <For
                each={props.options}
                fallback={
                  <p data-slot="empty">
                    <Show when={!props.loading} fallback="Loading…">
                      No matches
                    </Show>
                  </p>
                }
              >
                {(o, i) => (
                  <div
                    data-slot="item"
                    role="option"
                    // Focus stays in the search field, which points here with aria-activedescendant.
                    tabIndex={-1}
                    onKeyDown={onKeyDown}
                    id={`${id}-${i()}`}
                    data-index={i()}
                    aria-selected={o.value === selected()}
                    data-highlighted={i() === active() ? "" : undefined}
                    data-selected={o.value === selected() ? "" : undefined}
                    // Keep focus in the search field.
                    onPointerDown={(e) => e.preventDefault()}
                    onPointerMove={() => setActive(i())}
                    onClick={() => pick(o)}
                  >
                    <span data-slot="item-label">{o.label}</span>
                    <Show when={o.detail}>
                      <span data-slot="item-detail">{o.detail}</span>
                    </Show>
                  </div>
                )}
              </For>
            </div>
          </KPopover.Content>
        </KPopover.Portal>
      </KPopover>
    </div>
  )
}
