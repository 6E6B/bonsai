import { createSignal, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { compact } from "~/lib/format"
import type { AffiliatesView, CommunityHit } from "~/lib/ipc/bindings"
import { ListState } from "~/lib/list-state"
import { thumbUrl } from "~/lib/thumb"
import {
  Button,
  IconButton,
  ListItem,
  SegmentedControl,
  Thumbnail,
  Tooltip,
  VerifiedBadge,
  VirtualList,
} from "~/lib/ui"

const text = {
  allies: { add: "Send ally request", addCommand: "group.ally", remove: "group.remove_ally", empty: "No allies" },
  enemies: { add: "Declare enemy", addCommand: "group.enemy", remove: "group.remove_enemy", empty: "No enemies" },
}

type Kind = keyof typeof text

/** Allied and enemy communities, one list at a time. With the permission to manage
 * relationships, add and remove. */
export function Affiliates(props: { view: AffiliatesView; canManage: boolean }) {
  const [kind, setKind] = createSignal<Kind>("allies")
  const t = () => text[kind()]
  const label = (k: Kind, name: string) => (props.view.loading ? name : `${name} ${props.view[k].length}`)
  const list = () => ({
    loading: props.view.loading,
    error: props.view.error,
    items: props.view[kind()],
    more: false,
  })
  return (
    <div class="flex flex-col gap-2">
      <div class="flex items-center justify-between gap-2">
        <SegmentedControl
          aria-label="Affiliates"
          options={[
            { value: "allies", label: label("allies", "Allies") },
            { value: "enemies", label: label("enemies", "Enemies") },
          ]}
          value={kind()}
          onChange={(v) => setKind(v as Kind)}
        />
        <Show when={props.canManage}>
          <Button icon="plus" onClick={() => run(t().addCommand)}>
            {t().add}
          </Button>
        </Show>
      </div>
      <ListState list={list()} icon="communities" empty={t().empty}>
        <div class="flex flex-col">
          <VirtualList items={list().items} estimateRowHeight={56}>
            {(g: CommunityHit) => (
              <ListItem
                leading={<Thumbnail class="w-10" alt="" src={thumbUrl("group_icon", g.id)} fallback="communities" />}
                title={
                  <Show when={g.verified} fallback={g.name}>
                    {g.name} <VerifiedBadge />
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
          </VirtualList>
        </div>
      </ListState>
    </div>
  )
}
