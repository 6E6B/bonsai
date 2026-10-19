import { useLocation, useParams } from "@solidjs/router"
import { createEffect, on } from "solid-js"
import { useCommandContext } from "~/lib/commands/hooks"
import { setTabTitle } from "~/lib/tabs"
import { Page } from "~/lib/ui"
import { titleOf } from "~/lib/wiki"
import { useWikiPage, WikiArticleView, WikiState } from "~/lib/wiki-panels"

/** One wiki article: contents, text, infobox and a link to the page it is about in Bonsai.
 * `#section` in the path scrolls to that section. */
export default function Article() {
  const params = useParams()
  const location = useLocation()
  const title = () => titleOf(params.slug ?? "")
  const view = useWikiPage(title)
  useCommandContext(() => `wiki:${view.article?.title ?? title()}`)
  createEffect(() => {
    const a = view.article
    if (a) setTabTitle(location.pathname, a.name)
  })
  createEffect(
    on(
      () => [Boolean(view.article), location.hash] as const,
      ([ready, hash]) => {
        if (!ready || !hash) return
        const id = hash.slice(1)
        queueMicrotask(() => document.getElementById(`wiki-${id}`)?.scrollIntoView({ block: "start" }))
      },
    ),
  )
  return (
    <Page>
      <WikiState view={view} missing="It may have been renamed or deleted on the wiki.">
        {(a) => <WikiArticleView view={view} article={a} />}
      </WikiState>
    </Page>
  )
}
