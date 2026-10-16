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
import { ItemTile, TILE_WIDTH, tileGrid } from "~/lib/tiles"
import {
  Button,
  ContextMenu,
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
  Switch,
  TabPanel,
  Tabs,
  TextField,
  Thumbnail,
  Tile,
  usePrompt,
  VirtualGrid,
} from "~/lib/ui"

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
  const [section, setSection] = createSignal(OUTFITS)
  const tabs = () => [
    { value: OUTFITS, label: "Outfits" },
    { value: RECENT, label: "Recent" },
    { value: BODY, label: "Body" },
    ...sections.items.map((s) => ({ value: s.name, label: s.name })),
  ]
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
  const shownOutfits = createMemo(() => {
    const q = outfitQuery().trim().toLowerCase()
    return outfits.list.items.filter((o) => !q || o.name.toLowerCase().includes(q))
  })
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
      <div class="flex items-start gap-6">
        <aside class="sticky top-4 flex w-64 shrink-0 flex-col gap-3">
          <AvatarViewer source={{ kind: "me" }} image={preview()} alt="Your avatar" menu={previewMenu()} controls />
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

        <div class="flex min-w-0 flex-1 flex-col gap-3">
          <Tabs aria-label="Wardrobe" items={tabs()} value={section()} onChange={setSection}>
            <TabPanel value={OUTFITS}>
              <div class="flex flex-col gap-3">
                <TextField
                  icon="search"
                  aria-label="Filter outfits"
                  placeholder="Filter"
                  value={outfitQuery()}
                  onChange={setOutfitQuery}
                />
                <ListState list={outfits.list} icon="avatar" empty="No outfits yet" more={outfits.moreKey}>
                  <div class={tileGrid}>
                    <For each={shownOutfits()}>
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
                    </For>
                  </div>
                </ListState>
              </div>
            </TabPanel>
            <TabPanel value={RECENT}>
              <Recent />
            </TabPanel>
            <TabPanel value={BODY}>
              <Body view={view} />
            </TabPanel>
            <For each={sections.items}>
              {(s) => (
                <TabPanel value={s.name}>
                  <Section section={s} worn={view.wearing.map((w) => w.id)} />
                </TabPanel>
              )}
            </For>
          </Tabs>
          <Show when={sections.error}>{(e) => <span class="text-sm text-fg-muted">{e()}</span>}</Show>
        </div>
      </div>
    </Page>
  )
}

function Section(props: { section: WardrobeSection; worn: number[] }) {
  const [slot, setSlot] = createSignal(props.section.slots[0]?.value ?? "")
  createEffect(() => {
    if (!props.section.slots.some((s) => s.value === slot())) setSlot(props.section.slots[0]?.value ?? "")
  })
  const [query, setQuery] = createSignal("")
  const [wornOnly, setWornOnly] = createSignal(false)
  const items = useLive(
    () => [shell.active, slot()] as const,
    ([me, s]) =>
      me !== null && s
        ? commands.inventoryItems(me, s, false)
        : Promise.resolve({ list: emptyList<ItemCard>(), moreKey: "" }),
    (): Paged<ItemCard> => ({ list: emptyList<ItemCard>(), moreKey: "" }),
  )
  const shown = createMemo(() => {
    const q = query().trim().toLowerCase()
    return items.list.items.filter(
      (i) => (!q || i.name.toLowerCase().includes(q)) && (!wornOnly() || props.worn.includes(i.id)),
    )
  })
  return (
    <div class="flex flex-col gap-3">
      <div class="flex items-center gap-3">
        <TextField
          class="flex-1"
          icon="search"
          aria-label="Filter wardrobe"
          placeholder="Filter"
          value={query()}
          onChange={setQuery}
        />
        <Show when={props.section.slots.length > 1}>
          <Select aria-label="Type" options={props.section.slots} value={slot()} onChange={setSlot} />
        </Show>
        <Switch checked={wornOnly()} onChange={setWornOnly}>
          Worn only
        </Switch>
      </div>
      <ListState list={items.list} icon="avatar" empty="Nothing here" more={items.moreKey}>
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
      </ListState>
    </div>
  )
}

/** Items and outfits worn lately. Clicking wears them again. */
function Recent() {
  const recent = useLive(() => shell.active, commands.recentItems, emptyList<RecentView>)
  const wear = (r: RecentView) => (r.outfit ? run("avatar.outfit", r.id) : run("avatar.wear", `${r.id}:${r.typeId}`))
  return (
    <ListState list={recent} icon="avatar" empty="Nothing worn lately">
      <div class={tileGrid}>
        <For each={recent.items}>
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
        </For>
      </div>
    </ListState>
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
  const current = (id: number) => target().every((p) => props.view.colors.find((c) => c.part === p)?.color === id)
  const paint = (id: number) => run("avatar.color", parts().length ? `${parts().join(",")}:${id}` : String(id))
  const basic = () => palette.filter((s) => s.basic)
  const rest = () => palette.filter((s) => !s.basic)
  return (
    <div class="flex flex-col gap-6">
      <section class="flex flex-col gap-3">
        <SectionHeader title="Body colors" meta={parts().length ? "Selected parts" : "All parts"} size="group" />
        <div class="flex flex-wrap gap-2">
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
          <Show when={parts().length}>
            <Button size="sm" variant="ghost" onClick={() => setParts([])}>
              All parts
            </Button>
          </Show>
        </div>
        <div class="flex flex-wrap gap-1.5">
          <For each={basic()}>
            {(s) => <Swatch color={s.hex} label={s.name} selected={current(s.id)} onClick={() => void paint(s.id)} />}
          </For>
        </div>
        <Show when={rest().length}>
          <div class="flex flex-wrap gap-1.5">
            <For each={rest()}>
              {(s) => <Swatch color={s.hex} label={s.name} selected={current(s.id)} onClick={() => void paint(s.id)} />}
            </For>
          </div>
        </Show>
      </section>
      <section class="flex flex-col gap-3">
        <SectionHeader title="Scales" meta={props.view.rig === "R6" ? "R15 only" : undefined} size="group" />
        <div class="grid grid-cols-2 gap-x-8 gap-y-4">
          <For each={props.view.scales}>{(s) => <ScaleSlider scale={s} />}</For>
        </div>
      </section>
    </div>
  )
}

function ScaleSlider(props: { scale: ScaleView }) {
  // Follows the slider while dragging; the avatar catches up after the write.
  const [value, setValue] = createSignal(props.scale.value ?? 0)
  createEffect(() => setValue(props.scale.value ?? 0))
  return (
    <Slider
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
