import { Show } from "solid-js"
import { run, shortcutOf } from "~/lib/commands/registry"
import { shell } from "~/lib/store"
import { Badge, Divider, Robux, Spinner, TextLink } from "~/lib/ui"

/** The bottom line: who you are acting as, balance, background work, connectivity. */
export function StatusBar(props: { zoom: number }) {
  const active = () => shell.accounts.find((a) => a.id === shell.active)
  return (
    <footer
      data-window-corner="bottom"
      class="flex h-7 shrink-0 items-center gap-3 border-t border-line bg-deep px-3 text-sm text-fg-muted"
    >
      <Show when={active()} fallback={<span>No account</span>}>
        {(a) => (
          <>
            <TextLink tone="muted" onClick={() => run("nav.profile")}>
              <span class="text-fg">{a().label ?? a().displayName}</span> @{a().name}
            </TextLink>
            <Show when={shell.robux !== null}>
              <Divider orientation="vertical" class="my-1.5" />
              <Robux amount={shell.robux ?? 0} />
            </Show>
          </>
        )}
      </Show>
      <Show when={shell.jobs > 0}>
        <span class="flex items-center gap-1.5">
          <Spinner size="sm" />
          {shell.jobs} running
        </span>
      </Show>
      <Show when={shell.offline}>
        <Badge variant="warning">Offline</Badge>
      </Show>
      <div class="ml-auto flex items-center gap-3">
        <Show when={props.zoom !== 1}>
          <TextLink tone="muted" onClick={() => run("view.zoom_reset")}>
            {Math.round(props.zoom * 100)}%
          </TextLink>
        </Show>
        <TextLink tone="muted" onClick={() => run("help.shortcuts")}>
          <span class="text-fg">{shortcutOf("help.shortcuts") ?? "?"}</span> shortcuts
        </TextLink>
      </div>
    </footer>
  )
}
