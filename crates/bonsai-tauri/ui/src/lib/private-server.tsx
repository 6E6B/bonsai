import { For, Show } from "solid-js"
import { run } from "~/lib/commands/registry"
import { date } from "~/lib/format"
import { commands, type ServerSettings } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Avatar, Button, IconButton, Switch, Tooltip, useConfirm, usePrompt } from "~/lib/ui"

/** Settings of one of your own private servers: on/off, friends, renewal, name, link and who
 * else may join. Shown under its row on the experience page and in Settings. */
export function ServerControls(props: { serverId: number; class?: string }) {
  const cfg = useLive(
    () => props.serverId,
    async (id) => ({ s: await commands.serverSettings(id) }),
    () => ({ s: null as ServerSettings | null }),
  )
  const prompt = usePrompt()
  const confirm = useConfirm()
  const id = () => String(props.serverId)
  const expires = () => {
    const t = Date.parse(cfg.s?.expires ?? "")
    return Number.isNaN(t) ? undefined : t
  }
  const rename = async () => {
    const name = await prompt({ title: "Rename private server", label: "Name", initial: cfg.s?.name })
    if (name) void run("server.rename", `${id()} ${name}`)
  }
  const newLink = async () => {
    const ok = await confirm({
      title: "Make a new link?",
      description: "The current link stops working. People who already joined keep access.",
      confirm: "Make new link",
    })
    if (ok) void run("server.new_link", id())
  }
  const addUser = async () => {
    const who = await prompt({ title: "Let someone in", label: "Username or user ID" })
    if (who) void run("server.add_user", `${id()} ${who}`)
  }
  return (
    <Show when={cfg.s}>
      {(c) => (
        <div class={`flex flex-col gap-2 ${props.class ?? ""}`}>
          <div class="flex flex-wrap items-center gap-x-4 gap-y-1">
            <Switch checked={c().active} onChange={() => run("server.toggle_active", id())}>
              On
            </Switch>
            <Switch checked={c().friendsAllowed} onChange={() => run("server.toggle_friends", id())}>
              Friends can join
            </Switch>
            <Switch checked={c().renews} onChange={() => run("server.toggle_renew", id())}>
              Renews
            </Switch>
            <Show when={expires()}>
              {(t) => (
                <span class="text-sm text-fg-muted">
                  {c().renews ? "Renews" : "Ends"} {date(t())}
                </span>
              )}
            </Show>
            <div class="ml-auto flex gap-1">
              <Button size="sm" variant="ghost" icon="edit" onClick={() => void rename()}>
                Rename
              </Button>
              <Button
                size="sm"
                variant="ghost"
                icon="link"
                disabled={!c().hasLink}
                onClick={() => run("entity.copy_link", `server:${id()}`)}
              >
                Copy link
              </Button>
              <Button size="sm" variant="ghost" icon="refresh" onClick={() => void newLink()}>
                New link
              </Button>
            </div>
          </div>
          <div class="flex flex-wrap items-center gap-1.5">
            <span class="text-sm text-fg-muted">
              <Show when={c().users.length > 0} fallback="Only you and allowed friends can join.">
                Also allowed:
              </Show>
            </span>
            <For each={c().users}>
              {(u) => (
                <span class="flex items-center gap-1 rounded-md bg-raised py-0.5 pr-0.5 pl-1">
                  <Avatar size="xs" name={u.displayName || "?"} src={thumbUrl("headshot", u.id)} />
                  <span class="text-sm">{u.displayName || u.name || u.id}</span>
                  <Tooltip content="Remove">
                    <IconButton
                      size="sm"
                      icon="x"
                      label={`Remove ${u.displayName || u.name}`}
                      onClick={() => run("server.remove_user", `${id()} ${u.id}`)}
                    />
                  </Tooltip>
                </span>
              )}
            </For>
            <Button size="sm" variant="ghost" icon="user-plus" onClick={() => void addUser()}>
              Add someone
            </Button>
          </div>
        </div>
      )}
    </Show>
  )
}
