import { createEffect, createMemo, createSignal, For, onCleanup, Show } from "solid-js"
import { AvatarViewer } from "~/lib/avatar3d/viewer"
import { useSubTabs } from "~/lib/commands/hooks"
import { isRunning, menuItem, run } from "~/lib/commands/registry"
import {
  type AvatarView,
  commands,
  type ItemCard,
  type OutfitView,
  type Paged,
  type RecentView,
  type ScaleView,
  type SwatchView,
  type WardrobeSearch,
  type WardrobeSection,
  type WornView,
} from "~/lib/ipc/bindings"
import { emptyList, ListState } from "~/lib/list-state"
import { itemMenu } from "~/lib/menus"
import { isSoundHat } from "~/lib/sound-hats"
import { shell, useLive } from "~/lib/store"
import { thumbUrl, withRev } from "~/lib/thumb"
import { ItemName, ItemTile, SoundHatBadge } from "~/lib/tiles"
import {
  Badge,
  Button,
  ColorPicker,
  ContextMenu,
  EmptyState,
  IconButton,
  ListItem,
  Menu,
  type MenuEntry,
  Pane,
  SectionHeader,
  SegmentedControl,
  Select,
  Slider,
  Split,
  Swatch,
  TabPanel,
  Tabs,
  TextField,
  Thumbnail,
  Tile,
  usePrompt,
  VirtualGrid,
} from "~/lib/ui"
import { useParamDraft } from "~/lib/url-params"

import { Adjust, createAdjust } from "./Adjust"
import { isAdjusted, resetArg, ruleFor } from "./adjust"
import { BodyFigure } from "./BodyFigure"
import {
  groupTypes,
  groupWorn,
  matchesWear,
  named,
  searchStatus,
  sectionFor,
  sortOptions,
  type WornGroup,
} from "./wardrobe"

const WARDROBE = "wardrobe"
const OUTFITS = "outfits"
const RECENT = "recent"
const BODY = "body"
const ADJUST = "adjust"

/** Narrow enough for three tile columns in the side pane. */
const GRID_WIDTH = 116

const sep: MenuEntry = { kind: "separator" }

/** The image of an outfit, reloaded when the outfit list is read again. */
const outfitImage = (o: OutfitView) => {
  const url = thumbUrl("outfit", o.id)
  return url && withRev(url, o.rev)
}

/** Asset type id of an inventory slot key (`a<type>`). */
const slotType = (slot: string) => (slot.startsWith("a") ? slot.slice(1) : "")

/** Avatar editor: what the look wears on the left, the avatar in the middle, the wardrobe on
 * the right. Clicking an item puts it on or
 * takes it off the edited look; a full slot swaps its oldest item out. Nothing changes on
 * Roblox until the look is worn. */
export default function AvatarEditor() {
  const view = useLive(
    () => shell.active,
    () => commands.avatar(true),
    (): AvatarView => ({
      loading: true,
      error: null,
      rig: "",
      wearing: [],
      emotes: [],
      colors: [],
      scales: [],
      adjustRules: [],
      adjustSteps: { position: 0.01, rotation: 0.1, scale: 0.01 },
      rev: 0,
      edited: false,
      outfit: null,
      outfitEdited: false,
      loadingOutfit: false,
      canUndo: false,
      canRedo: false,
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
    { value: ADJUST, label: "Adjust" },
  ]
  const find = useParamDraft("find", 200)
  // The accessory the Adjust tab edits.
  const [adjusting, setAdjusting] = createSignal<number | null>(null)
  const adjust = (id: number) => {
    setAdjusting(id)
    setSection(ADJUST)
  }
  // The wardrobe slot a group's add button opened.
  const [pick, setPick] = createSignal<{ section: string; slot: string } | null>(null)
  const [category, setCategory] = createSignal(RECENT)
  const categories = () => [
    { value: RECENT, label: "Recent" },
    ...sections.items.map((s) => ({ value: s.name, label: s.name })),
  ]
  createEffect(() => {
    shell.active
    setSection(WARDROBE)
    setAdjusting(null)
    setPick(null)
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
  // Ctrl+S updates the outfit the look came from, or saves a new one.
  const save = () => {
    const o = view.outfit
    if (o && view.outfitEdited) void run("avatar.update_outfit", o.id)
    else if (!o) void saveOutfit()
  }
  const onKey = (e: KeyboardEvent) => {
    const t = e.target
    if (!(e.ctrlKey || e.metaKey) || e.altKey) return
    if (t instanceof HTMLElement && (t.isContentEditable || ["INPUT", "TEXTAREA"].includes(t.tagName))) return
    const key = e.key.toLowerCase()
    if (key === "enter") void run("avatar.apply")
    else if (key === "s") save()
    else if (key === "y" || (key === "z" && e.shiftKey)) void run("avatar.redo")
    else if (key === "z") void run("avatar.undo")
    else return
    e.preventDefault()
  }
  window.addEventListener("keydown", onKey)
  onCleanup(() => window.removeEventListener("keydown", onKey))
  const saveMenu = (): MenuEntry[] => [
    { label: "Save as new outfit…", icon: "plus", onSelect: () => void saveOutfit() },
  ]
  const outfitMenu = (o: OutfitView): MenuEntry[] => [
    menuItem("avatar.load_outfit", { arg: o.id, label: "Open in editor", icon: "edit" }),
    menuItem("avatar.outfit", { arg: o.id, label: "Wear now", icon: "avatar" }),
    // Outfits Roblox made can only be worn or opened, so the editing rows are left out.
    ...(o.editable
      ? [
          menuItem("avatar.update_outfit", { arg: o.id, label: "Replace with edited look", icon: "refresh" }),
          { label: "Rename…", icon: "edit", onSelect: () => void renameOutfit(o) } as const,
          sep,
          menuItem("avatar.delete_outfit", { arg: o.id, label: "Delete", icon: "trash", danger: true }),
        ]
      : []),
    menuItem("avatar.copy_to", { arg: `outfit:${o.id}`, label: "Copy to another account…", icon: "copy" }),
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
    menuItem("avatar.copy_to", { label: "Copy look to another account…", icon: "copy" }),
  ]
  const layered = () => view.wearing.filter((w) => w.layer !== null).length
  const groups = createMemo(() => groupWorn(view.wearing))
  // Opens the wardrobe on the first slot of these types that the account can wear.
  const browse = (types: readonly number[], otherwise = WARDROBE) => {
    const to = sectionFor(sections.items, types)
    if (!to) return setSection(otherwise)
    setPick({ ...to })
    setCategory(to.section)
    setSection(WARDROBE)
  }
  const add = (g: WornGroup) => browse(groupTypes(g), g === "body" ? BODY : WARDROBE)
  // An accessory that can be moved opens in Adjust; anything else opens its wardrobe slot.
  const select = (w: WornView) => (ruleFor(view, w.typeId) ? adjust(w.id) : browse([w.typeId]))
  const WornRow = (p: { item: WornView }) => {
    const w = () => p.item
    const layerMenu = () =>
      w().layer !== null && layered() > 1
        ? [
            {
              kind: "submenu" as const,
              label: "Layer",
              icon: "list" as const,
              items: [
                menuItem("avatar.layer", { arg: `${w().id} up`, label: "Move up", icon: "chevron-up" }),
                menuItem("avatar.layer", { arg: `${w().id} down`, label: "Move down", icon: "chevron-down" }),
              ],
            },
          ]
        : []
    const adjustMenu = () =>
      ruleFor(view, w().typeId)
        ? [
            { label: "Adjust…", icon: "settings" as const, onSelect: () => adjust(w().id) },
            ...(isAdjusted(w().placement)
              ? [menuItem("avatar.adjust", { arg: resetArg(w().id), label: "Reset adjustment", icon: "undo" })]
              : []),
          ]
        : []
    return (
      <ContextMenu
        items={itemMenu(w().id, false, w().typeId, {
          actions: [
            menuItem("avatar.try", { arg: `${w().id}:${w().typeId}`, label: "Take off", icon: "x" }),
            ...layerMenu(),
            ...adjustMenu(),
          ],
        })}
      >
        <ListItem
          leading={<Thumbnail class="w-8" alt="" fallback="avatar" src={thumbUrl("asset", w().id)} />}
          title={<ItemName name={w().name} id={w().id} />}
          subtitle={wornNote(w())}
          dim={view.rig === "R6" && w().r15Only}
          selected={section() === ADJUST && adjustState.current()?.id === w().id}
          onClick={() => select(w())}
          actionsOnHover
          actions={
            <>
              <Show when={w().layer !== null && layered() > 1}>
                <IconButton
                  size="sm"
                  icon="chevron-up"
                  label="Move layer up"
                  onClick={() => run("avatar.layer", `${w().id} up`)}
                />
                <IconButton
                  size="sm"
                  icon="chevron-down"
                  label="Move layer down"
                  onClick={() => run("avatar.layer", `${w().id} down`)}
                />
              </Show>
              <IconButton
                size="sm"
                icon="x"
                label="Take off"
                onClick={() => run("avatar.try", `${w().id}:${w().typeId}`)}
              />
            </>
          }
        />
      </ContextMenu>
    )
  }
  const adjustState = createAdjust(() => view, adjusting)
  const wornNote = (w: WornView) => {
    if (view.rig === "R6" && w.r15Only) return `${w.typeName} · R15 only`
    return isAdjusted(w.placement) ? `${w.typeName} · Adjusted` : w.typeName
  }
  const preview = () => {
    const url = shell.active !== null ? thumbUrl("avatar", shell.active, true) : undefined
    return url && withRev(url, view.rev)
  }

  return (
    <div class="flex min-h-0 flex-1 flex-col">
      <div class="flex flex-wrap items-center gap-2 border-b border-line px-5 py-2">
        <span class="truncate font-semibold">{view.outfit?.name ?? "Your avatar"}</span>
        <Show when={view.loadingOutfit}>
          <span class="text-sm text-fg-muted">Loading outfit…</span>
        </Show>
        <Show when={view.edited}>
          <Badge>Not worn</Badge>
        </Show>
        <Show when={view.outfit && view.outfitEdited}>
          <span class="text-sm text-fg-muted">Edited</span>
        </Show>
        <span class="flex-1" />
        <IconButton
          size="sm"
          icon="undo"
          label="Undo (Ctrl+Z)"
          disabled={!view.canUndo}
          onClick={() => void run("avatar.undo")}
        />
        <IconButton
          size="sm"
          icon="redo"
          label="Redo (Ctrl+Shift+Z)"
          disabled={!view.canRedo}
          onClick={() => void run("avatar.redo")}
        />
        <Button size="sm" disabled={!view.edited && !view.outfit} onClick={() => void run("avatar.revert")}>
          Revert
        </Button>
        <Show
          when={view.outfit}
          fallback={
            <Button size="sm" icon="plus" onClick={saveOutfit}>
              Save as outfit
            </Button>
          }
        >
          {(o) => (
            <>
              <Button
                size="sm"
                disabled={!view.outfitEdited}
                loading={isRunning("avatar.update_outfit", o().id)}
                onClick={() => void run("avatar.update_outfit", o().id)}
              >
                Update “{o().name}”
              </Button>
              <Menu size="sm" icon="chevron-down" label="More save options" items={saveMenu()} />
            </>
          )}
        </Show>
        <Button
          size="sm"
          variant="primary"
          disabled={!view.edited}
          loading={isRunning("avatar.apply")}
          onClick={() => void run("avatar.apply")}
        >
          Wear
        </Button>
      </div>
      <Split>
        <Pane side class="w-64 gap-1 px-3">
          <Show when={view.error}>{(e) => <span class="text-sm text-fg-muted">{e()}</span>}</Show>
          <For each={groups()}>
            {(g) => (
              <section class="flex flex-col" aria-label={g.label}>
                <SectionHeader
                  title={g.label}
                  meta={g.items.length || undefined}
                  size="group"
                  actions={
                    <IconButton
                      size="sm"
                      icon="plus"
                      label={`Add to ${g.label.toLowerCase()}`}
                      onClick={() => add(g.key)}
                    />
                  }
                />
                <Show when={g.key === "body"}>
                  <ListItem
                    leading={<Swatch display color={view.colors[0]?.hex ?? ""} label="Skin color" />}
                    title="Colors and scale"
                    subtitle={view.rig || undefined}
                    onClick={() => setSection(BODY)}
                  />
                </Show>
                <div class="-mx-2 flex flex-col">
                  <For each={g.items}>{(w) => <WornRow item={w} />}</For>
                </div>
              </section>
            )}
          </For>
          <Show when={view.emotes.length > 0}>
            <SectionHeader title="Emotes" meta={view.emotes.length} size="group" />
            <div class="-mx-2 flex flex-col">
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
                      onClick={() => run("avatar.play", `emote.${e.id}`)}
                    />
                  </ContextMenu>
                )}
              </For>
            </div>
          </Show>
        </Pane>

        <div class="flex min-w-0 flex-1 p-4">
          <AvatarViewer
            stage
            class="h-full min-h-80 w-full"
            colorHex={view.colors.map((c) => c.hex)}
            source={{ kind: "draft" }}
            image={view.edited ? undefined : preview()}
            alt="Your avatar"
            menu={previewMenu()}
            corner={
              <>
                <SegmentedControl
                  aria-label="Body type"
                  size="sm"
                  options={["R6", "R15"]}
                  value={view.rig}
                  onChange={(v) => void run("avatar.rig", v)}
                />
                <Show when={section() === ADJUST && adjustState.target()}>{adjustState.toolSwitch()}</Show>
              </>
            }
            adjust={section() === ADJUST ? adjustState.target() : undefined}
          />
        </div>

        <Pane class="w-112 flex-none border-l border-line">
          <Tabs
            aria-label="Wardrobe"
            items={tabs()}
            value={section()}
            onChange={setSection}
            actions={
              <IconButton
                size="sm"
                icon="refresh"
                label="Redraw avatar image"
                loading={isRunning("avatar.redraw")}
                onClick={() => run("avatar.redraw")}
              />
            }
          >
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
                    <VirtualGrid items={shownOutfits()} minItemWidth={GRID_WIDTH} estimateRowHeight={160} gap={10}>
                      {(o) => (
                        <ContextMenu items={outfitMenu(o)}>
                          <Tile
                            title={o.name}
                            subtitle={o.editable ? undefined : "From a bundle"}
                            src={outfitImage(o)}
                            fallback="avatar"
                            selected={view.outfit?.id === o.id}
                            onClick={() => run("avatar.load_outfit", o.id)}
                            onDblClick={() => run("avatar.outfit", o.id)}
                          />
                        </ContextMenu>
                      )}
                    </VirtualGrid>
                  </Show>
                </ListState>
              </div>
            </TabPanel>
            <TabPanel value={WARDROBE} class="flex flex-col gap-3">
              <TextField
                icon="search"
                aria-label="Search all items"
                placeholder="Search everything you can wear"
                value={find.value()}
                onChange={find.set}
              />
              <Show
                when={find.value().trim()}
                fallback={
                  <Tabs aria-label="Wardrobe category" items={categories()} value={category()} onChange={setCategory}>
                    <TabPanel value={RECENT}>
                      <Recent />
                    </TabPanel>
                    <For each={sections.items}>
                      {(s) => (
                        <TabPanel value={s.name}>
                          <Section
                            section={s}
                            worn={view.wearing.map((w) => w.id)}
                            pick={pick()?.section === s.name ? pick() : undefined}
                          />
                        </TabPanel>
                      )}
                    </For>
                  </Tabs>
                }
              >
                {(q) => <AllItems query={q()} worn={view.wearing.map((w) => w.id)} />}
              </Show>
            </TabPanel>
            <TabPanel value={BODY}>
              <Body view={view} />
            </TabPanel>
            <TabPanel value={ADJUST}>
              <Adjust view={view} state={adjustState} onSelect={setAdjusting} />
            </TabPanel>
          </Tabs>
          <Show when={sections.error}>{(e) => <span class="text-sm text-fg-muted">{e()}</span>}</Show>
        </Pane>
      </Split>
    </div>
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

/** Name matches across every wearable type. Types load in the background as the search needs them. */
function AllItems(props: { query: string; worn: number[] }) {
  const result = useLive(
    () => [shell.active, props.query] as const,
    ([, q]) => commands.wardrobeSearch(q),
    (): WardrobeSearch => ({
      loading: true,
      error: null,
      items: [],
      loaded: 0,
      total: 0,
      failed: 0,
      partial: false,
      truncated: false,
    }),
    { keep: true },
  )
  const list = () => ({ loading: result.loading, error: result.error, items: result.items, more: false })
  const wearArg = (i: ItemCard) => `${i.id}:${i.assetType}`
  return (
    <div class="flex flex-col gap-3">
      <p class="text-xs text-fg-muted" role="status">
        {searchStatus(result)}
      </p>
      <ListState list={list()} icon="search" empty="No matches" description="Nothing you own has that name.">
        <VirtualGrid items={result.items} minItemWidth={GRID_WIDTH} estimateRowHeight={160} gap={10}>
          {(h) => (
            <ItemTile
              item={h.item}
              selected={props.worn.includes(h.item.id)}
              onClick={() => run("avatar.wear", wearArg(h.item))}
              menu={{
                actions: [
                  menuItem("avatar.wear", {
                    arg: wearArg(h.item),
                    icon: "avatar",
                    label: props.worn.includes(h.item.id) ? "Take off" : "Wear",
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

function Section(props: { section: WardrobeSection; worn: number[]; pick?: { slot: string } | null }) {
  const [slot, setSlot] = createSignal(props.section.slots[0]?.value ?? "")
  // A slot asked for from the worn list.
  createEffect(() => {
    const want = props.pick?.slot
    if (want) setSlot(want)
  })
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
              <VirtualGrid items={shown()} minItemWidth={GRID_WIDTH} estimateRowHeight={160} gap={10}>
                {(i) => (
                  <ItemTile
                    item={i}
                    selected={props.worn.includes(i.id)}
                    onClick={() => run("avatar.try", `${i.id}:${i.assetType || slotType(slot())}`)}
                    menu={{
                      actions: [
                        menuItem("avatar.try", {
                          arg: `${i.id}:${i.assetType || slotType(slot())}`,
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

/** Items and outfits worn lately. Clicking puts them on the edited look again. */
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
  const wear = (r: RecentView) =>
    r.outfit ? run("avatar.load_outfit", r.id) : run("avatar.try", `${r.id}:${r.typeId}`)
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
          <VirtualGrid items={shown()} minItemWidth={GRID_WIDTH} estimateRowHeight={160} gap={10}>
            {(r) => (
              <ContextMenu
                items={
                  r.outfit
                    ? [
                        menuItem("avatar.load_outfit", { arg: r.id, label: "Open in editor", icon: "edit" }),
                        menuItem("avatar.outfit", { arg: r.id, label: "Wear now", icon: "avatar" }),
                      ]
                    : itemMenu(Number(r.id), false, r.typeId, {
                        actions: [
                          menuItem("avatar.try", {
                            arg: `${r.id}:${r.typeId}`,
                            label: "Put on or take off",
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
                  flair={!r.outfit && isSoundHat(Number(r.id)) ? <SoundHatBadge /> : undefined}
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
          <SectionHeader
            title="Body colors"
            meta={parts().length ? `${parts().length} selected` : "All parts"}
            size="group"
          />
          <div class="flex items-start gap-4">
            <BodyFigure
              class="h-44 w-36 flex-none"
              colors={props.view.colors}
              selected={parts()}
              onToggle={togglePart}
            />
            <div class="flex min-w-0 flex-1 flex-col gap-2">
              <div class="flex items-center gap-2 text-sm">
                <Show
                  when={mixed()}
                  fallback={
                    <>
                      <Swatch display color={selectedColor()} label="Current color" />
                      {selectedColor()}
                    </>
                  }
                >
                  <span class="text-fg-muted">Mixed colors</span>
                </Show>
              </div>
              <div class="flex flex-wrap gap-2">
                <Button size="sm" variant="secondary" disabled={!parts().length} onClick={() => setParts([])}>
                  All parts
                </Button>
                <Button
                  size="sm"
                  variant="secondary"
                  disabled={parts().length === props.view.colors.length}
                  onClick={() => setParts(props.view.colors.map((c) => c.part))}
                >
                  Select all
                </Button>
              </div>
            </div>
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
            <div class="flex flex-wrap gap-1.5">
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
  // Follows the slider while dragging; the avatar catches up when the edit lands.
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
