import { createEffect, createMemo, createSignal, For, Show } from "solid-js"
import { AvatarViewer } from "~/lib/avatar3d/viewer"
import { useSubTabs } from "~/lib/commands/hooks"
import { menuItem, run } from "~/lib/commands/registry"
import {
  type AvatarView,
  commands,
  type ItemCard,
  type OutfitView,
  type Paged,
  type RecentView,
  type ScaleView,
  type SwatchView,
  type WardrobeSection,
} from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { itemMenu } from "~/lib/menus"
import { shell, useLive } from "~/lib/store"
import { thumbUrl } from "~/lib/thumb"
import { ItemTile, TILE_WIDTH } from "~/lib/tiles"
import {
  Button,
  ColorPicker,
  ContextMenu,
  EmptyState,
  IconButton,
  ListItem,
  Menu,
  type MenuEntry,
  Page,
  SectionHeader,
  SegmentedControl,
  Select,
  Slider,
  Swatch,
  TabPanel,
  Tabs,
  TextField,
  Thumbnail,
  Tile,
  usePrompt,
  VirtualGrid,
} from "~/lib/ui"

import { matchesWear, named, sortOptions } from "./wardrobe"

const WARDROBE = "wardrobe"
const OUTFITS = "outfits"
const RECENT = "recent"
const BODY = "body"

const sep: MenuEntry = { kind: "separator" }

/** The image of an outfit, reloaded when the outfit list is read again. */
const outfitImage = (o: OutfitView) => {
  const url = thumbUrl("outfit", o.id)
  return url && `${url}?r=${o.rev}`
}

/** Asset type id of an inventory slot key (`a<type>`). */
const slotType = (slot: string) => (slot.startsWith("a") ? slot.slice(1) : "")

/** Avatar editor: preview on the left, wardrobe on the right. Clicking an item wears or
 * removes it; a full slot swaps its oldest item out. */
export default function AvatarEditor() {
  const view = useLive(
    () => shell.active,
    commands.avatar,
    (): AvatarView => ({
      loading: true,
      error: null,
      rig: "",
      wearing: [],
      emotes: [],
      colors: [],
      scales: [],
      rev: 0,
    }),
  )
  const outfits = useLive(
    () => shell.active,
    commands.outfits,
    (): Paged<OutfitView> => ({ list: emptyList<OutfitView>(), moreKey: "" }),
  )
  const sections = useLive(() => shell.active, commands.wardrobe, emptyList<WardrobeSection>)
  const [section, setSection] = createSignal(WARDROBE)
  const tabs = () => [
    { value: WARDROBE, label: "Wardrobe" },
    { value: OUTFITS, label: "Outfits" },
    { value: BODY, label: "Body" },
  ]
  const [category, setCategory] = createSignal(RECENT)
  const categories = () => [
    { value: RECENT, label: "Recent" },
    ...sections.items.map((s) => ({ value: s.name, label: s.name })),
  ]
  createEffect(() => {
    shell.active
    setSection(WARDROBE)
    setCategory(RECENT)
    setOutfitQuery("")
    setOutfitSort("default")
    setOutfitFilter("all")
  })
  useSubTabs(tabs, section, setSection)
  const prompt = usePrompt()
  const saveOutfit = async () => {
    const name = await prompt({ title: "Save outfit", label: "Name", submit: "Save" })
    if (name?.trim()) void run("avatar.save_outfit", name)
  }
  const renameOutfit = async (o: OutfitView) => {
    const name = await prompt({ title: "Rename outfit", label: "Name", initial: o.name, submit: "Rename" })
    if (name?.trim() && name.trim() !== o.name) void run("avatar.rename_outfit", `${o.id} ${name.trim()}`)
  }
  const outfitMenu = (o: OutfitView): MenuEntry[] => [
    menuItem("avatar.outfit", { arg: o.id, label: "Wear", icon: "avatar" }),
    menuItem("avatar.update_outfit", {
      arg: o.id,
      label: "Replace with current look",
      icon: "refresh",
      disabled: !o.editable,
    }),
    { label: "Rename…", icon: "edit", disabled: !o.editable, onSelect: () => void renameOutfit(o) },
    sep,
    menuItem("avatar.delete_outfit", {
      arg: o.id,
      label: "Delete",
      icon: "trash",
      danger: true,
      disabled: !o.editable,
    }),
  ]
  const [outfitQuery, setOutfitQuery] = createSignal("")
  const [outfitSort, setOutfitSort] = createSignal("default")
  const [outfitFilter, setOutfitFilter] = createSignal("all")
  const shownOutfits = createMemo(() =>
    named(
      outfits.list.items.filter((o) => outfitFilter() === "all" || (outfitFilter() === "editable") === o.editable),
      outfitQuery(),
      outfitSort(),
    ),
  )
  const previewMenu = (): MenuEntry[] => [
    { label: "Save as outfit…", icon: "plus", onSelect: () => void saveOutfit() },
    menuItem("avatar.rig", { label: view.rig === "R6" ? "Switch to R15" : "Switch to R6", icon: "avatar" }),
    menuItem("avatar.redraw", { icon: "refresh" }),
  ]
  const layered = () => view.wearing.filter((w) => w.layer !== null).length
  // Layered clothing outermost first, as it stacks; everything else after.
  const wearing = () =>
    [...view.wearing].sort((a, b) => (b.layer ?? Number.NEGATIVE_INFINITY) - (a.layer ?? Number.NEGATIVE_INFINITY))
  const preview = () => {
    const url = shell.active !== null ? thumbUrl("avatar", shell.active, true) : undefined
    return url && view.rev ? `${url}?r=${view.rev}` : url
  }

  return (
    <Page
      title="Avatar"
      actions={
        <>
          <IconButton icon="refresh" label="Redraw avatar image" onClick={() => run("avatar.redraw")} />
          <Button onClick={saveOutfit}>Save as outfit</Button>
        </>
      }
    >
      <div class="flex flex-col items-start gap-6 lg:flex-row">
        <aside class="flex w-64 max-w-full shrink-0 flex-col gap-3 lg:sticky lg:top-4">
          <AvatarViewer
            colorHex={view.colors.map((c) => c.hex)}
            source={{ kind: "me" }}
            image={preview()}
            alt="Your avatar"
            menu={previewMenu()}
            controls
          />
          <div class="flex items-center justify-between">
            <SegmentedControl
              aria-label="Body type"
              size="sm"
              options={["R6", "R15"]}
              value={view.rig}
              onChange={(v) => void run("avatar.rig", v)}
            />
            <Menu
              size="sm"
              label="Outfits"
              items={[
                { kind: "heading", label: "Wear outfit" },
                ...outfits.list.items.map((o) => menuItem("avatar.outfit", { arg: o.id, label: o.name })),
              ]}
            >
              Outfits
            </Menu>
          </div>
          <Show when={view.error}>{(e) => <span class="text-sm text-fg-muted">{e()}</span>}</Show>
          <SectionHeader title="Wearing" meta={view.wearing.length} size="group" />
          <div class="-mx-2 -mt-2 flex flex-col">
            <For each={wearing()}>
              {(w) => (
                <ContextMenu
                  items={itemMenu(w.id, false, w.typeId, {
                    actions: [
                      menuItem("avatar.wear", { arg: `${w.id}:${w.typeId}`, label: "Take off", icon: "x" }),
                      ...(w.layer !== null && layered() > 1
                        ? [
                            {
                              kind: "submenu" as const,
                              label: "Layer",
                              icon: "list" as const,
                              items: [
                                menuItem("avatar.layer", { arg: `${w.id} up`, label: "Move up", icon: "chevron-up" }),
                                menuItem("avatar.layer", {
                                  arg: `${w.id} down`,
                                  label: "Move down",
                                  icon: "chevron-down",
                                }),
                              ],
                            },
                          ]
                        : []),
                    ],
                  })}
                >
                  <ListItem
                    leading={<Thumbnail class="w-8" alt="" fallback="avatar" src={thumbUrl("asset", w.id)} />}
                    title={w.name}
                    subtitle={view.rig === "R6" && w.r15Only ? `${w.typeName} · R15 only` : w.typeName}
                    dim={view.rig === "R6" && w.r15Only}
                    onClick={() => run("market.open", String(w.id))}
                    actionsOnHover
                    actions={
                      <>
                        <Show when={w.layer !== null && layered() > 1}>
                          <IconButton
                            size="sm"
                            icon="chevron-up"
                            label="Move layer up"
                            onClick={() => run("avatar.layer", `${w.id} up`)}
                          />
                          <IconButton
                            size="sm"
                            icon="chevron-down"
                            label="Move layer down"
                            onClick={() => run("avatar.layer", `${w.id} down`)}
                          />
                        </Show>
                        <IconButton
                          size="sm"
                          icon="x"
                          label="Take off"
                          onClick={() => run("avatar.wear", `${w.id}:${w.typeId}`)}
                        />
                      </>
                    }
                  />
                </ContextMenu>
              )}
            </For>
          </div>
          <Show when={view.emotes.length > 0}>
            <SectionHeader title="Emotes" meta={view.emotes.length} size="group" />
            <div class="-mx-2 -mt-2 flex flex-col">
              <For each={view.emotes}>
                {(e) => (
                  <ContextMenu
                    items={itemMenu(e.id, false, undefined, {
                      actions: [
                        menuItem("avatar.play", { arg: `emote.${e.id}`, label: "Play in 3D", icon: "play" }),
                        menuItem("avatar.emote", { arg: String(e.typeId), label: "Clear slot", icon: "x" }),
                      ],
                    })}
                  >
                    <ListItem
                      leading={<Thumbnail class="w-8" alt="" fallback="avatar" src={thumbUrl("asset", e.id)} />}
                      title={e.name}
                      subtitle={e.typeName}
                      onClick={() => run("market.open", String(e.id))}
                      actionsOnHover
                      actions={
                        <IconButton
                          size="sm"
                          icon="play"
                          label="Play in 3D"
                          onClick={() => run("avatar.play", `emote.${e.id}`)}
                        />
                      }
                    />
                  </ContextMenu>
                )}
              </For>
            </div>
          </Show>
        </aside>

        <div class="flex w-full min-w-0 flex-1 flex-col gap-3">
          <Tabs aria-label="Wardrobe" items={tabs()} value={section()} onChange={setSection}>
            <TabPanel value={OUTFITS}>
              <div class="flex flex-col gap-3">
                <div class="flex flex-wrap items-center gap-2">
                  <TextField
                    class="min-w-40 flex-1"
                    icon="search"
                    aria-label="Search outfits"
                    placeholder="Search outfits"
                    value={outfitQuery()}
                    onChange={setOutfitQuery}
                  />
                  <Select
                    aria-label="Outfit source"
                    value={outfitFilter()}
                    onChange={setOutfitFilter}
                    options={[
                      { value: "all", label: "All outfits" },
                      { value: "editable", label: "My outfits" },
                      { value: "bundle", label: "From bundles" },
                    ]}
                  />
                  <Select
                    aria-label="Sort outfits"
                    value={outfitSort()}
                    onChange={setOutfitSort}
                    options={[
                      { value: "default", label: "Default order" },
                      ...sortOptions.filter((s) => s.value.startsWith("name")),
                    ]}
                  />
                </div>
                <ResultStatus
                  shown={shownOutfits().length}
                  loaded={outfits.list.items.length}
                  more={outfits.list.more}
                />
                <ListState list={outfits.list} icon="avatar" empty="No outfits yet" more={outfits.moreKey}>
                  <Show
                    when={shownOutfits().length}
                    fallback={
                      <NoMatches
                        reset={() => {
                          setOutfitQuery("")
                          setOutfitFilter("all")
                        }}
                      />
                    }
                  >
                    <VirtualGrid items={shownOutfits()} minItemWidth={TILE_WIDTH} estimateRowHeight={170} gap={12}>
                      {(o) => (
                        <ContextMenu items={outfitMenu(o)}>
                          <Tile
                            title={o.name}
                            subtitle={o.editable ? undefined : "From a bundle"}
                            src={outfitImage(o)}
                            fallback="avatar"
                            onClick={() => run("avatar.outfit", o.id)}
                          />
                        </ContextMenu>
                      )}
                    </VirtualGrid>
                  </Show>
                </ListState>
              </div>
            </TabPanel>
            <TabPanel value={WARDROBE}>
              <Tabs aria-label="Wardrobe category" items={categories()} value={category()} onChange={setCategory}>
                <TabPanel value={RECENT}>
                  <Recent />
                </TabPanel>
                <For each={sections.items}>
                  {(s) => (
                    <TabPanel value={s.name}>
                      <Section section={s} worn={view.wearing.map((w) => w.id)} />
                    </TabPanel>
                  )}
                </For>
              </Tabs>
            </TabPanel>
            <TabPanel value={BODY}>
              <Body view={view} />
            </TabPanel>
          </Tabs>
          <Show when={sections.error}>{(e) => <span class="text-sm text-fg-muted">{e()}</span>}</Show>
        </div>
      </div>
    </Page>
  )
}

function ResultStatus(props: { shown: number; loaded: number; more?: boolean }) {
  return (
    <p class="text-xs text-fg-muted" role="status">
      {props.shown} of {props.loaded} loaded
      <Show when={props.more}> · More items available. Search and name sorting cover loaded items.</Show>
    </p>
  )
}

function NoMatches(props: { reset: () => void }) {
  return (
    <div class="flex flex-col items-center gap-2 py-6">
      <EmptyState icon="search" title="No matches" description="Try another search or clear your filters." />
      <Button variant="ghost" size="sm" onClick={props.reset}>
        Clear filters
      </Button>
    </div>
  )
}

function Section(props: { section: WardrobeSection; worn: number[] }) {
  const [slot, setSlot] = createSignal(props.section.slots[0]?.value ?? "")
  createEffect(() => {
    if (!props.section.slots.some((s) => s.value === slot())) setSlot(props.section.slots[0]?.value ?? "")
  })
  const [query, setQuery] = createSignal("")
  const [sort, setSort] = createSignal("newest")
  const [wearFilter, setWearFilter] = createSignal("all")
  const [kind, setKind] = createSignal("all")
  const reset = () => {
    setQuery("")
    setWearFilter("all")
    setKind("all")
  }
  const items = useLive(
    () => [shell.active, slot(), sort() === "oldest"] as const,
    ([me, s, oldest]) =>
      me !== null && s
        ? commands.inventoryItems(me, s, oldest)
        : Promise.resolve({ list: emptyList<ItemCard>(), moreKey: "" }),
    (): Paged<ItemCard> => ({ list: emptyList<ItemCard>(), moreKey: "" }),
  )
  const shown = createMemo(() =>
    named(
      items.list.items.filter(
        (i) =>
          matchesWear(i.id, props.worn, wearFilter()) &&
          (kind() !== "limited" || i.limited) &&
          (kind() !== "copies" || i.copies > 1),
      ),
      query(),
      sort(),
    ),
  )
  return (
    <div class="flex flex-col gap-3">
      <Tabs aria-label={`${props.section.name} type`} items={props.section.slots} value={slot()} onChange={setSlot}>
        <TabPanel value={slot()} class="flex flex-col gap-3">
          <div class="flex flex-wrap items-center gap-2">
            <TextField
              class="min-w-40 flex-1"
              icon="search"
              aria-label="Search wardrobe"
              placeholder={`Search ${props.section.slots.find((s) => s.value === slot())?.label.toLowerCase() ?? "items"}`}
              value={query()}
              onChange={setQuery}
            />
            <Select aria-label="Sort wardrobe" options={sortOptions} value={sort()} onChange={setSort} />
            <Select
              aria-label="Equipped filter"
              value={wearFilter()}
              onChange={setWearFilter}
              options={[
                { value: "all", label: "All items" },
                { value: "worn", label: "Wearing" },
                { value: "unworn", label: "Not wearing" },
              ]}
            />
            <Select
              aria-label="Item filter"
              value={kind()}
              onChange={setKind}
              options={[
                { value: "all", label: "Any item" },
                { value: "limited", label: "Limited" },
                { value: "copies", label: "Multiple copies" },
              ]}
            />
          </div>
          <ResultStatus shown={shown().length} loaded={items.list.items.length} more={items.list.more} />
          <ListState list={items.list} icon="avatar" empty="No items in this category" more={items.moreKey}>
            <Show when={shown().length} fallback={<NoMatches reset={reset} />}>
              <VirtualGrid items={shown()} minItemWidth={TILE_WIDTH} estimateRowHeight={170} gap={12}>
                {(i) => (
                  <ItemTile
                    item={i}
                    selected={props.worn.includes(i.id)}
                    onClick={() => run("avatar.wear", `${i.id}:${slotType(slot())}`)}
                    menu={{
                      actions: [
                        menuItem("avatar.wear", {
                          arg: `${i.id}:${slotType(slot())}`,
                          icon: "avatar",
                          label: props.worn.includes(i.id) ? "Take off" : "Wear",
                        }),
                      ],
                    }}
                  />
                )}
              </VirtualGrid>
            </Show>
          </ListState>
        </TabPanel>
      </Tabs>
    </div>
  )
}

/** Items and outfits worn lately. Clicking wears them again. */
function Recent() {
  const recent = useLive(() => shell.active, commands.recentItems, emptyList<RecentView>)
  const [query, setQuery] = createSignal("")
  const [kind, setKind] = createSignal("all")
  const [sort, setSort] = createSignal("recent")
  const shown = createMemo(() =>
    named(
      recent.items.filter((r) => kind() === "all" || (kind() === "outfits") === r.outfit),
      query(),
      sort(),
    ),
  )
  const wear = (r: RecentView) => (r.outfit ? run("avatar.outfit", r.id) : run("avatar.wear", `${r.id}:${r.typeId}`))
  return (
    <div class="flex flex-col gap-3">
      <div class="flex flex-wrap items-center gap-2">
        <TextField
          class="min-w-40 flex-1"
          icon="search"
          aria-label="Search recent items"
          placeholder="Search recent items"
          value={query()}
          onChange={setQuery}
        />
        <Select
          aria-label="Recent item type"
          value={kind()}
          onChange={setKind}
          options={[
            { value: "all", label: "Items and outfits" },
            { value: "items", label: "Items" },
            { value: "outfits", label: "Outfits" },
          ]}
        />
        <Select
          aria-label="Sort recent items"
          value={sort()}
          onChange={setSort}
          options={[
            { value: "recent", label: "Recently worn" },
            ...sortOptions.filter((s) => s.value.startsWith("name")),
          ]}
        />
      </div>
      <ResultStatus shown={shown().length} loaded={recent.items.length} />
      <ListState list={recent} icon="avatar" empty="Nothing worn lately">
        <Show
          when={shown().length}
          fallback={
            <NoMatches
              reset={() => {
                setQuery("")
                setKind("all")
              }}
            />
          }
        >
          <VirtualGrid items={shown()} minItemWidth={TILE_WIDTH} estimateRowHeight={170} gap={12}>
            {(r) => (
              <ContextMenu
                items={
                  r.outfit
                    ? [menuItem("avatar.outfit", { arg: r.id, label: "Wear", icon: "avatar" })]
                    : itemMenu(Number(r.id), false, r.typeId, {
                        actions: [
                          menuItem("avatar.wear", {
                            arg: `${r.id}:${r.typeId}`,
                            label: "Wear or take off",
                            icon: "avatar",
                          }),
                        ],
                      })
                }
              >
                <Tile
                  title={r.name}
                  subtitle={r.outfit ? "Outfit" : r.typeName}
                  src={thumbUrl(r.outfit ? "outfit" : "asset", r.id)}
                  fallback="avatar"
                  onClick={() => void wear(r)}
                />
              </ContextMenu>
            )}
          </VirtualGrid>
        </Show>
      </ListState>
    </div>
  )
}

const percent = (v: number) => `${Math.round(v * 100)}%`

/** Body colors per part and body scales. */
function Body(props: { view: AvatarView }) {
  const palette = useLive(
    () => shell.active,
    commands.bodyPalette,
    (): SwatchView[] => [],
  )
  // Parts the next color goes on; none picked means all of them.
  const [parts, setParts] = createSignal<string[]>([])
  const togglePart = (p: string) => setParts((ps) => (ps.includes(p) ? ps.filter((x) => x !== p) : [...ps, p]))
  const target = () => (parts().length ? parts() : props.view.colors.map((c) => c.part))
  const current = (id: number) => {
    const swatch = palette.find((s) => s.id === id)
    return (
      !!swatch &&
      target().length > 0 &&
      target().every((p) => props.view.colors.find((c) => c.part === p)?.hex.toLowerCase() === swatch.hex.toLowerCase())
    )
  }
  const paint = (id: number) => run("avatar.color", parts().length ? `${parts().join(",")}:${id}` : String(id))
  const [tab, setTab] = createSignal("colors")
  const [mode, setMode] = createSignal("basic")
  const [query, setQuery] = createSignal("")
  const [custom, setCustom] = createSignal("")
  const selectedColor = () => props.view.colors.find((c) => target().includes(c.part))?.hex ?? ""
  const mixed = () =>
    new Set(props.view.colors.filter((c) => target().includes(c.part)).map((c) => c.hex.toLowerCase())).size > 1
  const shownPalette = () =>
    palette.filter(
      (s) =>
        (mode() !== "basic" || s.basic) &&
        (!query().trim() || `${s.name} ${s.hex} ${s.id}`.toLowerCase().includes(query().trim().toLowerCase())),
    )
  const paintCustom = () => {
    const hex = custom() || selectedColor()
    if (hex) void run("avatar.color", parts().length ? `${parts().join(",")}:${hex}` : hex)
  }
  createEffect(() => {
    shell.active
    setParts([])
    setCustom("")
    setQuery("")
  })
  createEffect(() => {
    parts()
    setCustom("")
  })
  return (
    <Tabs
      aria-label="Body settings"
      items={[
        { value: "colors", label: "Colors" },
        { value: "scales", label: "Scale" },
      ]}
      value={tab()}
      onChange={setTab}
    >
      <TabPanel value="colors">
        <section class="flex flex-col gap-3">
          <SectionHeader title="Body colors" meta={parts().length ? "Selected parts" : "All parts"} size="group" />
          <p class="text-sm text-fg-muted">Choose all parts or select individual parts to color together.</p>
          <div class="flex flex-wrap gap-2">
            <Button
              size="sm"
              variant={parts().length ? "secondary" : "primary"}
              aria-pressed={!parts().length}
              onClick={() => setParts([])}
            >
              All parts
            </Button>
            <For each={props.view.colors}>
              {(c) => (
                <Button
                  size="sm"
                  variant={parts().includes(c.part) ? "primary" : "secondary"}
                  aria-pressed={parts().includes(c.part)}
                  onClick={() => togglePart(c.part)}
                >
                  <Swatch display color={c.hex} label={c.label} />
                  {c.label}
                </Button>
              )}
            </For>
          </div>
          <div class="flex items-center gap-2 text-sm text-fg-muted">
            <Show
              when={mixed()}
              fallback={
                <>
                  <Swatch display color={selectedColor()} label="Current color" />
                  {selectedColor()}
                </>
              }
            >
              Mixed colors
            </Show>
          </div>
          <SegmentedControl
            aria-label="Color mode"
            options={[
              { value: "basic", label: "Basic" },
              { value: "all", label: "All colors" },
              { value: "custom", label: "Custom" },
            ]}
            value={mode()}
            onChange={setMode}
          />
          <Show
            when={mode() !== "custom"}
            fallback={
              <div class="flex max-w-80 flex-col gap-3">
                <ColorPicker label="Custom body color" value={custom() || selectedColor()} onChange={setCustom} />
                <Button disabled={!props.view.colors.length || !custom()} onClick={paintCustom}>
                  Apply to{" "}
                  {parts().length
                    ? `${parts().length} selected ${parts().length === 1 ? "part" : "parts"}`
                    : "all parts"}
                </Button>
              </div>
            }
          >
            <TextField
              icon="search"
              aria-label="Search colors"
              placeholder="Search by name, hex or color ID"
              value={query()}
              onChange={setQuery}
            />
            <div class="flex max-w-xl flex-wrap gap-1.5">
              <For each={shownPalette()}>
                {(s) => (
                  <Swatch
                    color={s.hex}
                    label={s.name}
                    selected={current(s.id) && !mixed()}
                    onClick={() => void paint(s.id)}
                  />
                )}
              </For>
            </div>
            <Show when={palette.length && !shownPalette().length}>
              <EmptyState icon="search" title="No colors match" />
            </Show>
          </Show>
        </section>
      </TabPanel>
      <TabPanel value="scales">
        <section class="flex flex-col gap-3">
          <SectionHeader title="Scales" meta={props.view.rig === "R6" ? "R15 only" : undefined} size="group" />
          <div class="grid grid-cols-1 gap-x-8 gap-y-4 sm:grid-cols-2">
            <For each={props.view.scales}>{(s) => <ScaleSlider scale={s} disabled={props.view.rig !== "R15"} />}</For>
          </div>
        </section>
      </TabPanel>
    </Tabs>
  )
}

function ScaleSlider(props: { scale: ScaleView; disabled: boolean }) {
  // Follows the slider while dragging; the avatar catches up after the write.
  const [value, setValue] = createSignal(props.scale.value ?? 0)
  createEffect(() => setValue(props.scale.value ?? 0))
  return (
    <Slider
      disabled={props.disabled}
      label={props.scale.label}
      value={value()}
      min={props.scale.min ?? 0}
      max={props.scale.max ?? 1}
      step={props.scale.step ?? 0.01}
      format={percent}
      onChange={setValue}
      onChangeEnd={(v) => void run("avatar.scale", `${props.scale.name} ${v}`)}
    />
  )
}
