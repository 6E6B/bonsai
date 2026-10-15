import { For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { compact } from "~/lib/format"
import type { AffiliatesView, CommunityHit } from "~/lib/ipc/bindings"
import { ListState } from "~/lib/list-state"
import { thumbUrl } from "~/lib/thumb"
import { Button, IconButton, ListItem, Thumbnail, Tooltip, VerifiedBadge } from "~/lib/ui"

const text = {
  allies: { add: "Send ally request", addCommand: "group.ally", remove: "group.remove_ally", empty: "No allies" },
  enemies: { add: "Declare enemy", addCommand: "group.enemy", remove: "group.remove_enemy", empty: "No enemies" },
}

/** Allied or enemy communities. With the permission to manage relationships, add and remove. */
export function Affiliates(props: { view: AffiliatesView; kind: "allies" | "enemies"; canManage: boolean }) {
  const t = () => text[props.kind]
  const list = () => ({
    loading: props.view.loading,
    error: props.view.error,
    items: props.view[props.kind],
    more: false,
  })
  return (
    <div class="flex flex-col gap-2">
      <Show when={props.canManage}>
        <div class="flex justify-end">
          <Button icon="plus" onClick={() => run(t().addCommand)}>
            {t().add}
          </Button>
        </div>
      </Show>
      <ListState list={list()} icon="communities" empty={t().empty}>
        <div class="flex flex-col">
          <For each={list().items}>
            {(g: CommunityHit) => (
              <ListItem
                leading={<Thumbnail class="w-10" alt="" src={thumbUrl("group_icon", g.id)} fallback="communities" />}
                title={
                  <Show when={g.verified} fallback={g.name}>
                    <span class="text-verified">{g.name}</span> <VerifiedBadge />
                  </Show>
                }
                subtitle={`${compact(g.members)} members`}
                actions={
                  <Show when={props.canManage}>
                    <Tooltip content="Remove">
                      <IconButton
                        size="sm"
                        icon="x"
                        label={`Remove ${g.name}`}
                        onClick={(e: MouseEvent) => {
                          e.stopPropagation()
                          void run(t().remove, String(g.id))
                        }}
                      />
                    </Tooltip>
                  </Show>
                }
                onClick={() => run("group.open", String(g.id))}
              />
            )}
          </For>
        </div>
      </ListState>
    </div>
  )
}
