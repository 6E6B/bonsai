import { createMemo, createSignal } from "solid-js"
import { commands, type FontCatalogView } from "~/lib/ipc/bindings"
import { useLive } from "~/lib/store"
import { fontChoices } from "~/lib/theme"
import { Combobox } from "~/lib/ui"

/** Searches the bundled fonts and Google Fonts. Nothing is fetched until the list opens. */
export function FontPicker(props: {
  label: string
  mono: boolean
  value: string
  onChange: (v: string) => unknown
  class?: string
}) {
  const [query, setQuery] = createSignal<string | null>(null)
  const catalog = useLive(
    () => query(),
    (q): Promise<FontCatalogView> =>
      q === null ? Promise.resolve(emptyCatalog()) : commands.fontCatalog(q, props.mono),
    emptyCatalog,
    { keep: true },
  )
  const options = createMemo(() => fontChoices(query() ?? "", props.mono, catalog.fonts))
  return (
    <Combobox
      size="sm"
      aria-label={props.label}
      class={props.class ?? "w-56"}
      searchPlaceholder="Search Google Fonts"
      value={props.value}
      options={options()}
      loading={catalog.loading}
      onSearch={setQuery}
      onChange={props.onChange}
    />
  )
}

const emptyCatalog = (): FontCatalogView => ({ fonts: [], loading: false, error: null })
