import { Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { compact } from "~/lib/format"
import { commands, type GameCard } from "~/lib/ipc/bindings"
import { gameMenu } from "~/lib/menus"
import { useLive } from "~/lib/store"
import { GameIcon } from "~/lib/tiles"
import { Button, Card, ContextMenu, TextLink } from "~/lib/ui"

/** An experience linked from a chat line or a message: icon, name, creator, how many are
 * playing, and Join. Shows the place's id until the experience is known; the name opens its
 * page, right-click for the usual experience menu. */
export function ExperienceCard(props: { placeId: number }) {
  const game = useLive(
    () => props.placeId,
    async (id) => ({ card: await commands.linkedGame(id) }),
    () => ({ card: null as GameCard | null }),
  )
  const open = () => run("game.open", String(props.placeId))
  return (
    <ContextMenu items={gameMenu(props.placeId)}>
      <Card class="flex w-72 max-w-full items-center gap-3 p-2">
        <GameIcon universeId={game.card?.universeId ?? null} class="w-12 shrink-0 rounded-md" />
        <div class="flex min-w-0 flex-1 flex-col">
          <span class="truncate font-medium">
            <TextLink tone="name" onClick={open}>
              {game.card?.name ?? `Experience ${props.placeId}`}
            </TextLink>
          </span>
          <Show when={game.card}>
            {(c) => (
              <span class="truncate text-xs text-fg-muted">
                <Show when={c().creator}>{(by) => <>by {by()} · </>}</Show>
                {compact(c().playing)} playing
              </span>
            )}
          </Show>
        </div>
        <Button size="sm" variant="primary" icon="play" onClick={() => run("game.join", String(props.placeId))}>
          Join
        </Button>
      </Card>
    </ContextMenu>
  )
}
