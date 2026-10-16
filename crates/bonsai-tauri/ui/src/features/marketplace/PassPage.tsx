import { useLocation, useParams } from "@solidjs/router"
import { createEffect, Show } from "solid-js"
import { useCommandContext } from "~/lib/commands/hooks"
import { run } from "~/lib/commands/registry"
import { compact, date } from "~/lib/format"
import { commands, type PassView } from "~/lib/ipc/bindings"
import { Description } from "~/lib/linked-text"
import { entityMenu } from "~/lib/menus"
import { useLive } from "~/lib/store"
import { setTabTitle } from "~/lib/tabs"
import { thumbUrl } from "~/lib/thumb"
import { Price } from "~/lib/tiles"
import { Button, Card, Menu, Page, Skeleton, Stat, TextLink, Thumbnail } from "~/lib/ui"

const ms = (iso: string) => {
  const t = Date.parse(iso)
  return Number.isNaN(t) ? null : t
}

/** One game pass: picture, creator, price, description and dates. Roblox doesn't say which
 * experience sells a pass, so buying happens on the website or in the experience's Store tab. */
export default function PassPage() {
  const params = useParams()
  const location = useLocation()
  const id = () => Number(params.id)
  const target = () => `pass:${id()}`
  const view = useLive(id, commands.passPage, (): PassView => ({ loading: true, error: null, info: null }))
  useCommandContext(target)
  createEffect(() => {
    const name = view.info?.name
    if (name) setTabTitle(location.pathname, name)
  })

  return (
    <Page>
      <Show
        when={view.info}
        fallback={
          <Show when={view.error} fallback={<Skeleton class="h-72 w-full rounded-md" />}>
            {(e) => <span class="text-fg-muted">{e()}</span>}
          </Show>
        }
      >
        {(p) => (
          <div class="flex flex-wrap gap-6">
            <Thumbnail class="w-72 self-start" alt={p().name} src={thumbUrl("game_pass", id(), true)} fallback="pass" />
            <div class="flex min-w-64 flex-1 flex-col gap-3">
              <div class="flex flex-col gap-1">
                <h1 class="truncate text-2xl font-medium">{p().name}</h1>
                <span class="flex items-center gap-1 text-fg-muted">
                  Pass by{" "}
                  <TextLink
                    tone="name"
                    onClick={() => run(p().creatorIsGroup ? "group.open" : "user.open", String(p().creatorId))}
                  >
                    {p().creator}
                  </TextLink>
                </span>
              </div>
              <Price class="text-xl font-medium" amount={p().price} />
              <div class="flex flex-wrap gap-1.5">
                <Button icon="external" onClick={() => run("entity.open_website", target())}>
                  Open on website
                </Button>
                <Menu icon="more" label="More" items={entityMenu(target())} />
              </div>
              <Show when={p().description.trim()}>
                {(d) => <Description text={d()} class="max-w-prose text-fg-muted" />}
              </Show>
              <Card class="grid grid-cols-[repeat(auto-fill,minmax(8rem,1fr))] gap-4 p-3">
                <Stat label="Pass ID">{p().id}</Stat>
                <Stat label="Price">{p().priceLabel}</Stat>
                <Show when={p().sales > 0}>
                  <Stat label="Sales">{compact(p().sales)}</Stat>
                </Show>
                <Show when={ms(p().created)}>{(t) => <Stat label="Created">{date(t())}</Stat>}</Show>
                <Show when={ms(p().updated)}>{(t) => <Stat label="Updated">{date(t())}</Stat>}</Show>
              </Card>
            </div>
          </div>
        )}
      </Show>
    </Page>
  )
}
