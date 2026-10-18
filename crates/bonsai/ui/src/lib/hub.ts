import { createStore } from "solid-js/store"
import { handle } from "~/lib/commands/registry"
import { commands, type HubEntry, type HubView } from "~/lib/ipc/bindings"
import { showToast } from "~/lib/ui"

export const [hubUi, setHubUi] = createStore({
  userId: null as string | null,
  kind: "themes",
  query: "",
  base: "all",
  sort: "updated",
  offset: 0,
  selected: null as HubEntry | null,
  publishing: false,
  localId: "",
  name: "",
  hubId: "",
  version: "1.0.0",
  description: "",
  tags: "",
  choices: [] as { id: string; name: string; version: string }[],
})
export const hubQuery = () =>
  JSON.stringify({ kind: hubUi.kind, q: hubUi.query, base: hubUi.base, sort: hubUi.sort, offset: hubUi.offset })
export const emptyHub = (): HubView => ({
  configured: false,
  loading: false,
  busy: false,
  signingIn: false,
  error: null,
  userId: null,
  userName: null,
  items: [],
  nextOffset: null,
  submissions: [],
  revoked: [],
})
export function target(entry: HubEntry) {
  return JSON.stringify({ kind: entry.kind, id: entry.id, version: entry.version })
}
export function publishProblem(input: {
  hubId: string
  name: string
  version: string
  description: string
  tags: string
}): string | undefined {
  if (!/^[a-z0-9][a-z0-9-]{0,63}$/.test(input.hubId)) return "Use a lowercase ID with letters, digits and dashes."
  if (!input.name.trim() || input.name.trim().length > 64) return "Name must contain 1 to 64 characters."
  if (
    !/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-[0-9A-Za-z.-]+)?$/.test(input.version) ||
    input.version.length > 64
  )
    return "Use a release version such as 1.0.0."
  if (input.description.length > 2000) return "Description must be at most 2,000 characters."
  const tags = input.tags
    .split(",")
    .map((t) => t.trim())
    .filter(Boolean)
  if (tags.length > 8 || tags.some((t) => !/^[a-z0-9-]{1,24}$/.test(t)))
    return "Use up to 8 lowercase tags, separated by commas."
}
export function registerHubCommands() {
  return [
    handle("hub.details", (arg) => {
      if (!arg) return showToast("Choose a release in the hub.")
      try {
        setHubUi("selected", JSON.parse(arg) as HubEntry)
      } catch {
        showToast("Invalid hub release.")
      }
    }),
    handle("hub.publish", async (arg) => {
      let kind = hubUi.kind,
        id = ""
      if (arg) {
        try {
          const parsed = JSON.parse(arg)
          kind = parsed.kind
          id = parsed.id ?? ""
        } catch {
          return showToast("Choose a theme or plugin to publish.")
        }
      }
      if (kind !== "themes" && kind !== "plugins") return
      const choices =
        kind === "themes"
          ? (await commands.themes())
              .filter((t) => !t.builtin)
              .map((t) => ({ id: t.id, name: t.name, version: JSON.parse(t.doc).version as string }))
          : (await commands.plugins()).map((p) => ({ id: p.id, name: p.name, version: p.version }))
      const picked = choices.find((c) => c.id === id) ?? choices[0]
      setHubUi({
        kind,
        choices,
        localId: picked?.id ?? "",
        hubId: picked?.id ?? "",
        name: picked?.name ?? "",
        version: picked?.version ?? "1.0.0",
        description: "",
        tags: "",
        publishing: true,
      })
    }),
  ]
}
