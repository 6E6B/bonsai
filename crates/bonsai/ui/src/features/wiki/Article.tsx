import { useLocation, useParams } from "@solidjs/router"
import { createEffect, on, Show } from "solid-js"
import { useCommandContext } from "~/lib/commands/hooks"
import { run } from "~/lib/commands/registry"
import { setTabTitle } from "~/lib/tabs"
import { Button, EmptyState, Page } from "~/lib/ui"
import { articleAt } from "~/lib/wiki"
import { WikiArticleView } from "~/lib/wiki-panels"

/** One wiki article: contents, text, infobox and a link to the page it is about in Bonsai.
 * `#section` in the path scrolls to that section. */
export default function Article() {
  const params = useParams()
  const location = useLocation()
  const article = () => articleAt(params.source ?? "", params.slug ?? "")
  useCommandContext(() => `wiki:${params.source}/${params.slug}`)
  createEffect(() => {
    const a = article()
    if (a) setTabTitle(location.pathname, a.title)
  })
  createEffect(
    on(
      () => [article(), location.hash] as const,
      ([a, hash]) => {
        if (!a || !hash) return
        const id = hash.slice(1)
        queueMicrotask(() => document.getElementById(`wiki-${id}`)?.scrollIntoView({ block: "start" }))
      },
    ),
  )
  return (
    <Page>
      <Show
        when={article()}
        fallback={
          <EmptyState icon="wiki" title="No such article" description="It may have been renamed on the wiki.">
            <Button onClick={() => run("nav.wiki")}>Search the wiki</Button>
          </EmptyState>
        }
      >
        {(a) => <WikiArticleView article={a()} />}
      </Show>
    </Page>
  )
}
