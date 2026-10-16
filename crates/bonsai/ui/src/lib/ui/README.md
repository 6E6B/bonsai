# UI components

Our own components, built on [Kobalte](https://kobalte.dev) and styled with plain CSS.
Feature code imports from `~/lib/ui` only (`index.ts`).

## What each component is for

Pick by purpose. If two seem to fit, the first one listed in its group is usually right.

### Actions

| Component | Use it for | Not for |
|---|---|---|
| `Button` | Any action. `primary` once per view (Join, Accept), `secondary` by default, `ghost` in dense rows and toolbars, `danger` for destructive actions. | Navigation inside text (use `TextLink`). |
| `IconButton` | Square icon-only actions in toolbars and rows. Always pass `label`; wrap in `Tooltip`. | Actions people need to discover; give those text. |
| `ToggleButton` | On/off state the server owns: favorite, like, notify, pin. `pressed` comes from the store. | Local view options (use `Switch` or `SegmentedControl`). |
| `Menu` | A button that opens a list of commands: "More", "Join as…". Build rows with `menuItem()` from the registry. | Picking a value (use `Select`). |
| `ContextMenu` | Right-click shortcuts on rows and tiles. Everything in it must also be reachable another way. | The only path to an action. |
| `TextLink` | Inline names that open something: "by Tilework", a seller name. A button, never an anchor. User and community names use `tone="name"`, or `tone="verified"` plus a `VerifiedBadge` when verified; no other name is blue. | Standalone actions. |
| `Mention` | An `@name` inside text: headshot plus name in a tinted pill, opens the profile. Rendered for you by `Description` / `LinkedText` in `~/lib/linked-text`. | Names in bylines and lists (use `TextLink`). |
| `SearchButton` | The command bar trigger in the top bar. Looks like a field, opens the palette. | Filtering a list (use `TextField`). |

### Navigation

| Component | Use it for |
|---|---|
| `RailItem` | One account slot in the far-left rail: active marker, color tag, tooltip with its `Ctrl+N`. |
| `NavItem` | Sidebar rows and in-screen category lists (Marketplace, Inventory, Settings). `compact` for the collapsed sidebar. |
| `TabStrip` | The open tabs above the page. One per window, driven by `~/lib/tabs`. |
| `Tabs` + `TabPanel` | Sub-sections of one screen (Servers / Store / Badges). Wire `[` and `]` with `useSubTabs`. |
| `SegmentedControl` | One of 2–5 options shown in place: server sort, R6/R15, chart. |

### Inputs

| Component | Use it for |
|---|---|
| `TextField` | Text input, search-as-you-type filters, the chat composer. |
| `Select` | One of many options, or options that need a label (privacy, genre, sort with long names). |
| `Combobox` | One of a long list the backend searches as you type: the theme font picker. A button that opens a search field over the list; the search never replaces the value. `onSearch` gets the text ("" on open). If `onChange` resolves to false, the button goes back. |
| `Checkbox` | Independent filters ("Hide full") and bulk selection in lists. |
| `Switch` | A setting that takes effect at once. `layout="row"` puts the label first and the switch at the far end, for settings lists. |
| `RadioGroup` | One of a few options in a settings form, where all should be visible. |
| `EmojiPicker` | A smiley button that opens a searchable emoji grid: the chat composer. Controlled `open` so a command can open it. The set is bundled. | Picking from a short list of options (use `Select`). |
| `Slider` | A number in a range: avatar scales. Commit in `onChangeEnd`, not `onChange`. |
| `ColorPicker` | Edits one color: area, hue and opacity sliders, hex field. The theme editor. Group undo on `onChangeEnd`. |
| `Swatch` | A pickable color chip whose color comes from data: body colors. |

### Showing data

| Component | Use it for |
|---|---|
| `ListItem` | A row: friend, conversation, message, trade, notification, request. Leading avatar, title, subtitle, meta, actions. `selected` for the keyboard cursor, `unread`, `dim`. |
| `DataTable` | Rows with real columns: servers, members, resellers, transactions, shortcuts. Virtual, scrolls with the page, sticky header. Column widths are fixed tracks, never `auto`. |
| `VirtualGrid` | Anything that can pass ~100 entries: tile grids (inventory, catalog) or one-column lists (leave out `minItemWidth`). Scrolls with the page. |
| `PersonCard` | A person in a grid of compact cards: friends playing. A `Card` with avatar, name, one muted line, an optional trailing action. |
| `PersonTile` | A person as a tile: big round headshot, display name, @username. Friends rows on a profile. |
| `Tile` | A picture with a title: games, items, outfits, communities. `selected` for worn or picked items. |
| `Shelf` | One sideways-scrolling row of tiles or people under a `SectionHeader`: profile overview rows. Arrows on hover. |
| `MediaFrame` | An image with previous and next arrows and a counter laid over it. |
| `Thumbnail` | A `thumb://` image with a fixed aspect ratio and a fallback icon. |
| `Avatar` | A user or account picture. `presence` dot, `count` badge, sizes `xs`–`xl`. |
| `Stat` | Label over value inside a `Card` grid (Playing / Visits / Rating). |
| `Facts` | An entity's details beside its description: `FactGroup`s of `Fact` label/value rows in one panel. |
| `Meter` | Amount of a known maximum: server fill (`tone="capacity"` warns near full), progress. |
| `Robux` | Any Robux amount; `null` shows a dash. |
| `Badge` | A short status word: Limited, Primary, Owned, Offline. |
| `VerifiedBadge` | Roblox's checkmark after a verified user or community name. Color the name with `tone="verified"` or `text-verified`. |
| `Kbd` | Shortcut caps in tooltips, menus and the shortcuts list. |
| `Collapsible` | Long text that can fold away (About). |
| `Clamp` | Long text cut to a few lines with Show more (descriptions, bios). |

### Structure

| Component | Use it for |
|---|---|
| `Page` | The frame of every non-split screen: padding, max width, heading with actions. |
| `SectionHeader` | Headings with optional meta and actions: `page`, `section`, or `group` (the small "In game · 2" label). |
| `Card` | A raised panel grouping related content: stats, trade sides. |
| `SettingsGroup` | A titled panel of settings, a line between each row. Rows are `SettingRow` (label and description left, control right; `layout="stacked"` for full-width controls) or `SettingSwitch`. `list` for a panel of `ListItem`s. |
| `Divider` | A line between groups in a row or column. |

### Feedback and state

| Component | Use it for |
|---|---|
| `Alert` | A problem that persists until fixed: offline, expired cookie, captcha needed. |
| `showToast` | Short-lived feedback after a command ("Copied link", errors from `run`). |
| `EmptyState` | A list or screen that loaded and has nothing. Give it a next step when there is one. |
| `Skeleton` | First load with no cached data. Prefer cached data. |
| `Spinner` | Inside buttons (`loading`) and the status bar job count. |
| `Tooltip` | Names for icon buttons, with their shortcut. |
| `Dialog` / `useDialog` | The one app-wide dialog: command bar, shortcuts, forms. |
| `usePrompt` / `useConfirm` | One-line text input (rename, new shout) and confirmation for destructive actions. Both return promises. |

## Icons

`Icon` draws [Phosphor](https://phosphoricons.com) icons (MIT) from `@phosphor-icons/core`, bundled at
build time. Names describe the use (`marketplace`, `trades`), not the drawing. To add one, import
its SVG in `icon.tsx` (`@phosphor-icons/core/assets/<weight>/<name>.svg?raw`) and add it to `svgs`.
Use the `regular` weight; add a `-fill` variant only for pressed states.

## Changing how things look

| To change | Edit |
|---|---|
| Any color, font, text size, radius or shadow, app-wide | `styles/theme.css` |
| Light theme colors | the `[data-theme="light"]` block in `styles/theme.css` |
| Control heights (density) | `--control-sm/md/lg` in `styles/theme.css` |
| One component | `<name>.css` next to its `.tsx` |
| A button variant | its `--button-*` lines in `button.css` |
| Shine and depth | `--sheen-*` gradients and `--shadow-*` elevations in `styles/theme.css` |

Every token in `theme.css` is also a Tailwind utility: `--color-fg-muted` → `text-fg-muted`,
`--color-surface` → `bg-surface`, `--radius-md` → `rounded-md`. Tailwind's default palette is
removed, so only our colors exist.

Colors live only in `theme.css`. Component CSS uses `var(--color-*)`, never a literal color.

## How a component is styled

- The `.tsx` renders Kobalte parts with `data-component="<name>"` on the root and
  `data-slot="<part>"` on the parts, plus `data-variant` / `data-size` where it has them.
- The `.css` targets those attributes inside `@layer components`, so a Tailwind class passed
  through `class` always wins.
- Kobalte adds state attributes (`data-checked`, `data-disabled`, `data-invalid`,
  `data-highlighted`, `data-expanded`, `data-closed`). Style states with those.
- Each component sets a few local variables at the top of its rule (`--button-bg`,
  `--avatar-size`, `--dialog-width`), and variants only change those.

## Adding a component

1. `foo.tsx` built on the Kobalte primitive, with `data-component="foo"`.
2. `foo.css` in `@layer components`, using only theme tokens.
3. Import the CSS in `styles/index.css` and export the component from `index.ts`.
4. Add it to the gallery in `src/features/components/Components.tsx`.

## Theme

Themes live in `~/lib/theme` (see `docs/theming.md`). `applyAppearance` sets `data-theme` on
`<html>` and every resolved token as an inline custom property, which wins over `theme.css`.
`index.tsx` calls it once at startup so the first paint follows the OS; `app/appearance.ts` then
keeps the window on the saved mode and themes.

`styles/theme.css` stays the first-paint fallback. Every property in it is listed in
`~/lib/theme/tokens.ts`, and the built-in Dark and Light themes (`crates/bonsai-core/themes/`) must
resolve to exactly its values; `tokens.test.ts` checks both. Adding a token means adding it to
the registry and to `derive()`.

`ThemeCard` draws a small window in any theme: it takes resolved tokens and sets them on its
preview, so the preview uses the same token names as the app.

## Fonts

`fonts/` holds Inter and JetBrains Mono Nerd Font, both under the SIL Open Font License 1.1.
Icons come from `@phosphor-icons/core` (see Icons above).
Loaded by `styles/base.css`.
