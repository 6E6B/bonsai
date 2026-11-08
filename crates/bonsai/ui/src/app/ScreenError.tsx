import { useLocation } from "@solidjs/router"
import { createEffect, on } from "solid-js"
import { Button, Collapsible, EmptyState, showToast } from "~/lib/ui"

/** What a screen shows when it throws while rendering: a plain message, a way to retry, and
 * the details folded away for a bug report. Navigating to another screen clears it. */
export function ScreenError(props: { error: unknown; reset: () => void }) {
  const location = useLocation()
  createEffect(on(() => location.pathname, props.reset, { defer: true }))
  const message = () => (props.error instanceof Error ? props.error.message : String(props.error))
  const details = () =>
    props.error instanceof Error ? (props.error.stack ?? props.error.message) : String(props.error)
  const copy = () =>
    navigator.clipboard.writeText(details()).then(
      () => showToast({ title: "Copied error details", variant: "success" }),
      () => showToast({ title: "Couldn't copy", variant: "error" }),
    )
  return (
    <div role="alert" class="flex flex-col items-center gap-4 py-16">
      <EmptyState icon="alert" title="This screen hit an error" description={message()}>
        <Button variant="primary" icon="refresh" onClick={props.reset}>
          Try again
        </Button>
        <Button icon="copy" onClick={copy}>
          Copy details
        </Button>
      </EmptyState>
      <Collapsible title="Details" class="w-full max-w-2xl">
        <pre class="max-h-80 overflow-auto whitespace-pre-wrap p-3 font-mono text-xs text-fg-muted">{details()}</pre>
      </Collapsible>
    </div>
  )
}
