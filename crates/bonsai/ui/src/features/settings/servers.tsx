import { createSignal, For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { date } from "~/lib/format"
import { commands, type MyServerRow } from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { ServerControls } from "~/lib/private-server"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Button, ListItem, Robux, SegmentedControl, SettingsGroup } from "~/lib/ui"

const serverTabs = [
  { value: "owned", label: "Yours" },
  { value: "invited", label: "Invited to" },
]

/** Private servers across every experience: yours (with their settings) and ones you were let
 * into. */
export function ServersSection() {
  const [tab, setTab] = createSignal("owned")
  const list = useLive(
    () => `${shell.active}:${tab()}`,
    () => commands.myPrivateServers(tab()),
    emptyList<MyServerRow>,
  )
  const [open, setOpen] = createSignal<number>()
  const when = (iso: string) => {
    const t = Date.parse(iso)
    return Number.isNaN(t) ? null : date(t)
  }
  const subtitle = (s: MyServerRow) =>
    [
      s.name || "Private server",
      tab() === "invited" ? `by ${s.ownerName || "…"}` : null,
      s.active ? null : "Off",
      s.expires && when(s.expires) ? `${s.renews ? "Renews" : "Ends"} ${when(s.expires)}` : null,
    ]
      .filter(Boolean)
      .join(" · ")
  return (
    <SettingsGroup
      title={tab() === "owned" ? "Your servers" : "Servers you can join"}
      list
      actions={
        <SegmentedControl aria-label="Which servers" size="sm" value={tab()} onChange={setTab} options={serverTabs} />
      }
    >
      <ListState
        list={list}
        icon="server"
        empty={tab() === "owned" ? "You have no private servers" : "No one let you into theirs"}
        more={`my_servers:${tab()}`}
      >
        <For each={list.items}>
          {(s) => (
            <div class="flex flex-col">
              <ListItem
                leading={<Avatar name={s.game || "?"} src={thumbUrl("game_icon", s.universeId)} />}
                title={s.game || "Experience"}
                subtitle={subtitle(s)}
                onClick={() => run("game.open", String(s.placeId))}
                meta={
                  <Show when={s.price} fallback={<span class="text-sm text-fg-muted">Free</span>}>
                    {(p) => (
                      <span class="text-sm">
                        <Robux amount={p()} />
                        /mo
                      </span>
                    )}
                  </Show>
                }
                actions={
                  <>
                    <Show when={tab() === "owned"}>
                      <Button
                        size="sm"
                        variant="ghost"
                        icon="sliders"
                        aria-expanded={open() === s.id}
                        onClick={() => setOpen(open() === s.id ? undefined : s.id)}
                      >
                        Settings
                      </Button>
                    </Show>
                    <Button
                      size="sm"
                      disabled={!s.active}
                      onClick={() => run("game.join_private_server", `${s.id} ${s.placeId}`)}
                    >
                      Join
                    </Button>
                  </>
                }
              />
              <Show when={open() === s.id}>
                <ServerControls serverId={s.id} class="px-3 pb-3" />
              </Show>
            </div>
          )}
        </For>
      </ListState>
    </SettingsGroup>
  )
}
