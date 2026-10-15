import { For, Match, Switch } from "solid-js"
import { run } from "~/lib/commands/registry"
import { commands, type MentionView } from "~/lib/ipc/bindings"
import { mentions, type RichPart, splitRich } from "~/lib/links"
import { useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { Mention, TextLink, type TextLinkProps } from "~/lib/ui"

type Users = { loading: boolean; byName: Record<string, MentionView> }

/** The users `text` @mentions, looked up in one request and kept current. */
function useMentions(text: () => string) {
  return useLive(
    () => mentions(text()).join(","),
    async (key): Promise<Users> => {
      if (!key) return { loading: false, byName: {} }
      const r = await commands.mentionedUsers(key.split(","))
      const byName: Record<string, MentionView> = {}
      for (const u of r.items) byName[u.requested] = u
      return { loading: r.loading, byName }
    },
    () => ({ loading: true, byName: {} }),
  )
}

type PartsProps = { text: string; tags: boolean; tone?: TextLinkProps["tone"]; users: Users }

/** One run of text with its links, hashtags and mentions live. Mentions show the user's
 * headshot once the name is found; a name nobody has stays plain text. */
function Parts(props: PartsProps) {
  const openLink = (p: RichPart, url: string) =>
    p.open ? run(p.open.command, p.open.arg) : run("entity.open_website", url)
  return (
    <For each={splitRich(props.text, { tags: props.tags })}>
      {(p) => (
        <Switch fallback={p.text}>
          <Match when={p.mention ? props.users.byName[p.mention] : undefined}>
            {(u) => (
              <Mention
                text={p.text}
                name={u().name}
                title={u().displayName && u().displayName !== u().name ? u().displayName : undefined}
                src={thumbUrl("headshot", u().id)}
                verified={u().verified}
                onClick={() => run("user.open", String(u().id))}
              />
            )}
          </Match>
          <Match when={p.mention && props.users.loading}>
            <Mention text={p.text} name={p.mention ?? ""} />
          </Match>
          <Match when={p.tag}>
            {(tag) => (
              <TextLink tone={props.tone} onClick={() => run("search.open", tag())}>
                {p.text}
              </TextLink>
            )}
          </Match>
          <Match when={p.url}>
            {(url) => (
              <TextLink tone={props.tone} onClick={() => openLink(p, url())}>
                {p.text}
              </TextLink>
            )}
          </Match>
        </Switch>
      )}
    </For>
  )
}

/** Text with its Roblox links and @mentions live: experiences, profiles, communities and items
 * open here, other Roblox pages on the website. Links to other sites stay plain text, since the
 * app only opens Roblox pages. `tags` also links hashtags, to a search for the tag. */
export function LinkedText(props: { text: string; tone?: TextLinkProps["tone"]; tags?: boolean }) {
  const users = useMentions(() => props.text)
  return <Parts text={props.text} tags={props.tags ?? false} tone={props.tone} users={users} />
}

/** A description, bio or post: paragraphs split on blank lines, with links, @mentions and
 * (with `tags`) hashtags live. All mentions are looked up together. */
export function Description(props: { text: string; tags?: boolean; class?: string }) {
  const paragraphs = () => props.text.split(/\n\s*\n/).filter((p) => p.trim())
  const users = useMentions(() => props.text)
  return (
    <div class={`flex flex-col gap-3 break-words ${props.class ?? ""}`}>
      <For each={paragraphs()}>
        {(para) => (
          <p class="whitespace-pre-line">
            <Parts text={para} tags={props.tags ?? false} users={users} />
          </p>
        )}
      </For>
    </div>
  )
}
