import { For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, type PlaytimeView } from "~/lib/ipc/bindings"
import { duration, utcOffsetMinutes } from "~/lib/playtime"
import { useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Button, Card, SectionHeader, Thumbnail } from "~/lib/ui"

const TOP = 3

/** This week's playtime for one of your own accounts, from the local playtime log. */
export function PlaytimeCard(props: { account: number }) {
  const view = useLive(
    () => props.account,
    (account) => commands.playtime("week", account, utcOffsetMinutes()),
    (): PlaytimeView => ({
      enabled: true,
      background: true,
      range: "week",
      secs: 0,
      sessions: 0,
      experiences: [],
      accounts: [],
      weeks: [],
    }),
  )
  return (
    <Card class="flex flex-col gap-2 px-3.5 py-3">
      <SectionHeader
        title="Playtime this week"
        size="group"
        actions={
          <Button size="sm" variant="ghost" onClick={() => void run("playtime.range", "week")}>
            All
          </Button>
        }
      />
      <Show
        when={view.secs > 0}
        fallback={
          <span class="text-sm text-fg-muted">
            {view.enabled ? "Nothing recorded this week." : "The playtime log is off."}
          </span>
        }
      >
        <span class="text-lg tabular-nums">{duration(view.secs)}</span>
        <For each={view.experiences.slice(0, TOP)}>
          {(g) => (
            <button
              type="button"
              class="-mx-1 flex items-center gap-2 rounded-md px-1 py-1 text-start text-sm hover:bg-hover"
              disabled={!g.placeId}
              onClick={() => void run("game.open", String(g.placeId))}
            >
              <Thumbnail
                class="w-6 shrink-0 rounded-sm"
                alt=""
                src={thumbUrl("game_icon", g.universeId)}
                fallback="game"
              />
              <span class="min-w-0 flex-1 truncate">{g.name || `Experience ${g.universeId}`}</span>
              <span class="shrink-0 text-fg-muted tabular-nums">{duration(g.secs)}</span>
            </button>
          )}
        </For>
      </Show>
    </Card>
  )
}
