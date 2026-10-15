import { Popover as KPopover } from "@kobalte/core/popover"
import { createEffect, createMemo, createSignal, For, on, Show } from "solid-js"
import { Button } from "./button"
import { searchEmoji } from "./emoji"
import { TextField } from "./text-field"

export interface EmojiPickerProps {
  /** Inserts the emoji the reader picked. The picker closes after it. */
  onPick: (emoji: string) => void
  /** Accessible name of the trigger. */
  label?: string
  /** Controlled: open from a command as well as from the button. */
  open?: boolean
  onOpenChange?: (open: boolean) => void
  placement?: "top-start" | "top-end" | "bottom-start" | "bottom-end"
  disabled?: boolean
}

/** A button that opens a searchable grid of emoji. Type to filter, Enter picks the first match,
 * Esc closes. The set is bundled; nothing is fetched. */
export function EmojiPicker(props: EmojiPickerProps) {
  const [query, setQuery] = createSignal("")
  const groups = createMemo(() => searchEmoji(query()))
  // After a pick the caller decides where focus goes (the composer); Esc returns it here.
  let picked = false
  const pick = (emoji: string) => {
    picked = true
    props.onOpenChange?.(false)
    props.onPick(emoji)
  }
  // A fresh search each time it opens.
  createEffect(
    on(
      () => props.open,
      (open) => {
        if (!open) setQuery("")
      },
    ),
  )
  return (
    <KPopover open={props.open} onOpenChange={props.onOpenChange} placement={props.placement ?? "top-end"} gutter={6}>
      <KPopover.Trigger
        as={Button}
        variant="ghost"
        icon="smiley"
        data-icon-only=""
        aria-label={props.label ?? "Emoji"}
        disabled={props.disabled}
      />
      <KPopover.Portal>
        <KPopover.Content
          data-component="emoji-picker"
          onCloseAutoFocus={(e) => {
            if (picked) e.preventDefault()
            picked = false
          }}
        >
          <TextField
            icon="search"
            size="sm"
            aria-label="Search emoji"
            placeholder="Search emoji"
            value={query()}
            onChange={setQuery}
            onKeyDown={(e) => {
              if (e.key !== "Enter") return
              e.preventDefault()
              const first = groups()[0]?.emoji[0]
              if (first) pick(first[0])
            }}
          />
          <div data-slot="scroll">
            <For each={groups()} fallback={<p data-slot="empty">No emoji match</p>}>
              {(g) => (
                <section data-slot="group">
                  <Show when={g.id !== "results" || groups().length > 1}>
                    <h3 data-slot="heading">{g.label}</h3>
                  </Show>
                  <div data-slot="grid">
                    <For each={g.emoji}>
                      {([emoji, words]) => (
                        <button type="button" data-slot="emoji" title={words.split(" ")[0]} onClick={() => pick(emoji)}>
                          {emoji}
                        </button>
                      )}
                    </For>
                  </div>
                </section>
              )}
            </For>
          </div>
        </KPopover.Content>
      </KPopover.Portal>
    </KPopover>
  )
}
