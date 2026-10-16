# Theming

Themes are a first-class feature: easy to make inside the app, easy to share, and safe to import
from strangers. This is the design and the roadmap. The checklist line lives in
[`parity/settings-servers-discover.md`](parity/settings-servers-discover.md#settings).

## Principles

- **A theme is data, never code.** Hex colors, px lengths, shadow layers and font names. No raw
  CSS, no URLs. Imported and hub themes can't run anything or reach the network on their own. A
  Google Fonts family is only a name; `rbx-core` downloads it (see [Fonts](#fonts)).
- **Three colors make a theme.** A dozen seeds derive every token; single tokens can be pinned
  after that.
- **One path for every theme.** Dark and Light are theme documents like any user theme.
- **Everything is a command.** Applying, editing, importing and exporting are registry commands
  and show up in the command bar.

## The document

```jsonc
{
  "schema": 1,
  "id": "nord",                 // built-ins use their name; copies `<name>-copy`
  "name": "Nord",
  "author": "nick",
  "version": "1.0.0",
  "base": "dark",               // dark | light: color-scheme and the derivation recipe
  "seeds": {
    "background": "#2e3440", "foreground": "#eceff4", "accent": "#88c0d0",
    "success": "#a3be8c", "warning": "#ebcb8b", "danger": "#bf616a",
    "contrast": 1,              // surface step size; 0 is flat, 1 the default look
    "radius": 1,                // multiplies every radius
    "shadow": { "depth": 1, "ring": 1, "highlight": 1 },   // alpha multipliers per layer kind
    "font": { "google": "Space Grotesk" }, "mono": "jetbrains"   // bundled id or a Google family
  },
  "tokens": { "color-verified": "#4d94ff" }                 // pinned values; win over seeds
}
```

Reading is tolerant: a missing id, a bad base or a newer `schema` refuse the theme; anything
else (unknown tokens from a newer app, bad values, out-of-range seeds) is dropped or clamped and
listed so the import screen can say what was skipped.

Token values follow one grammar per kind:

| Kind | Accepts |
|---|---|
| color | `#rgb`, `#rgba`, `#rrggbb`, `#rrggbbaa`, or `var(--color-…)` naming another color token. Cycles fall back to derived values. |
| length | `0` or a px value |
| shadow | Up to 6 comma-separated layers: optional `inset`, 2–4 px lengths, one hex color |
| font | A bundled font id (`inter`, `jetbrains`, `system`). Pinned font tokens take bundled ids only; Google families go in the `font` and `mono` seeds. |

## Code

`crates/bonsai-tauri/ui/src/lib/theme/`:

| File | What it does |
|---|---|
| `tokens.ts` | The registry: every themeable token with group, kind and label, in editor order, plus `UNTHEMED` (density, motion, sheens) with the reason. |
| `color.ts` | sRGB parse, format, mix, alpha, WCAG contrast. |
| `derive.ts` | Seeds → every token. Mix amounts are fitted so the default seeds reproduce `theme.css`. Shadows are layer templates whose alpha the shadow seeds scale. |
| `theme.ts` | The document type, `readTheme` (validation), `tokenValue` (per-kind grammar), `resolve` (derive + pins). |
| `apply.ts` | `applyAppearance(mode, {dark, light})`: follows the OS in System mode, sets `data-theme`, `color-scheme` and every resolved token on `<html>`. |
| `fonts.ts` | `registerFonts` adds downloaded Google fonts with the `FontFace` API from `font://`; `fontChoices` builds the picker list. |
| `builtins.ts` | Tests only: reads the built-ins from `crates/rbx-core/themes/`. |

Elsewhere:

| File | What it does |
|---|---|
| `crates/rbx-core/themes/*.json` | The built-ins: Dark, Light, Midnight, Fjord, Ember, Paper, Moss, Dusk, Abyss, Phosphor, Sakura, Glacier, Adwaita, Adwaita Dark (GNOME's libadwaita colors). Bundled with `include_str!`. |
| `crates/rbx-core/src/themes.rs` | Parses and checks documents (shape, id, size, a small value charset), loads `<config>/themes/*.json`, and `Core::apply_theme` / `cycle_theme` / `save_theme` / `delete_theme`. Files are written on the worker. |
| `rbx-core/src/settings.rs` | `Appearance { mode, dark, light }`: theme ids per base. |
| `bonsai-tauri/src/ipc/account.rs` | `appearance()` (mode and the two documents), `themes()` (every installed theme as `ThemeView`), and the `theme.*` commands. |
| `ui/src/app/appearance.ts` | Applies `appearance()` live, on every `storeChanged`. |
| `ui/src/lib/ui/theme-card.tsx` | `ThemeCard`: a small window drawn with a theme's tokens. |
| `crates/rbx-core/src/fonts.rs` | The Google Fonts catalog, the download of one family into `<cache>/fonts/<slug>/<weight>.ttf`, and the checks on names, paths and file headers. |
| `bonsai-tauri/src/font.rs` | `font://<slug>/<weight>.ttf`: serves those files to the webview. |
| `ui/src/lib/ui/combobox.tsx` | `Combobox`: the searchable font picker. |

Rust checks structure and keeps every value to `[A-Za-z0-9#.,()\- ]`, so a theme can't carry a
URL, a quote or a `;`. The UI checks each value against its token's kind in `tokenValue`.

`tokens.test.ts` keeps three things in step: every property in `theme.css` is in the registry or
in `UNTHEMED`, the registry names nothing `theme.css` lacks, and Dark and Light resolve to
exactly the values in `theme.css`. `theme.css` stays as the first-paint fallback. If you change a
value there, the test names the pin to add or update in `rbx-core/themes/`. `builtins.test.ts`
checks that every built-in keeps text readable (WCAG ratios for text on surfaces and on fills).

Derivation mixes in sRGB, not OKLCH. With fitted amounts it reproduces the hand-tuned built-ins
exactly; OKLCH can come in with the editor's contrast fixes if tinted themes need it.

## Fonts

A theme's `font` or `mono` seed may be `{ "google": "Family Name" }`. Settings → Appearance →
Fonts picks one from a searchable list; `theme.font` and `theme.mono_font` do the same from the
command bar. Changing the font of a built-in makes a copy first and says so.

- **Catalog.** `fonts.google.com/metadata/fonts`, fetched only when the picker opens or a theme
  names a Google font, cached for 7 days. Italic-only families are left out.
- **Files.** `fonts.googleapis.com/css2` for weights 400–700 (nearest available), then each
  `fonts.gstatic.com` TTF. The headers are checked, then the files are saved to
  `<cache>/fonts/<slug>/<weight>.ttf` and refreshed after 30 days. If they go missing, they are
  downloaded again.
- **Network.** All of it goes through `rbx-api` (`Client::fetch_font` only accepts
  `fonts.gstatic.com`), under the shared rate limiter, with no cookies. Google sees the family
  name and the machine's IP, as with the Rolimons values.
- **Webview.** The CSP gains only the local `font:` scheme in `font-src`; the webview still
  fetches nothing remote. `font://` refuses any path that isn't `<slug>/<digits>.ttf`.
- **Stack.** A Google family is put in front of the bundled stack for its slot, so text stays
  readable while it downloads or if it fails.

## Background picture

Settings > Appearance > Background puts a picture or GIF behind the page area. It belongs to the
appearance settings, not to a theme, so it stays when the theme changes. The sidebar, title bar
and menus stay opaque.

- **Source.** `view.background_file` opens the system file picker (`rfd` over the XDG desktop
  portal, from Rust; the webview gets no file access). `view.background` takes an https link or a
  full path. Links download through `rbx-api` (`Client::fetch_user_image`: https only, no
  cookies, shared rate limiter, at most 32 MB).
- **Storage.** PNG, JPEG, GIF and WebP only, checked by header. The file is saved as
  `<config>/backgrounds/<hash>.<ext>` and older ones are deleted (`rbx-core/src/backgrounds.rs`).
  `view.background_clear` removes it.
- **Webview.** Served over the local `bg:` scheme (added to `img-src`); `bg://` refuses any name
  that isn't `<16 hex>.<ext>`.
- **See-through.** `view.background_level` sets `image` (picture opacity) and `panels` (opacity
  of `color-canvas` and `color-surface`, 0–100). `lib/theme/backdrop.ts` rewrites those two
  tokens with alpha; the picture sits on `--backdrop-base`, the opaque canvas.
- **Effects.** The same command sets `blur` (0–40 px), `dim`, `saturation` (0–200, 100 leaves
  it), `tint` (wash in `--color-accent`), `vignette`, `scanlines` and `grain`; the rest are 0–100.
  Blur, dim and saturation are a CSS `filter` on the picture, which overhangs the edges by twice
  the blur so they don't fade. The others are overlays: vignette and scanlines fade toward
  `--backdrop-base`, so they suit light themes too; grain is an SVG `feTurbulence` blended with
  `overlay`. `view.background_effects_reset` turns them all off. Sliders preview while dragging
  (`previewBackdrop`) and save on release.

## Roadmap

One PR per phase.

### 1. Token registry and format (done)

- [x] Registry with a sync test against `theme.css`
- [x] `derive()` and the document format with tolerant validation
- [x] Dark and Light as documents that resolve to exactly the current values

### 2. Storage and selection (done)

- [x] `rbx-core/src/themes.rs`: built-ins bundled from `rbx-core/themes/`, user themes one
  `<id>.json` each in `Paths::themes_dir()`, strict on shape, size and characters.
- [x] `Appearance { mode: System | Dark | Light, dark, light }` replaces `settings::Theme`. In
  System mode the OS picks between the two, so custom pairs still follow it. Old settings are
  not migrated.
- [x] `appearance()` and `themes()` over IPC; `applyAppearance` sets the tokens on `<html>`, so
  the whole app changes in one frame without a reload.
- [x] Settings → Appearance: Mode, then a grid of `ThemeCard`s drawn from each theme's tokens.
  Right-click to use or delete.
- [x] Commands: `theme.apply` (one palette row per installed theme), `theme.cycle`,
  `theme.delete` (user themes only, asks first). `view.theme` is the system/dark/light switch.
  Using a theme switches to its base.

### 3. Editor (`features/themes/`)

- `ColorPicker` in `lib/ui` on Kobalte `ColorArea`, `ColorSlider` and `ColorField` (in
  `@kobalte/core` 0.13 already), added to the gallery.
- Left: seeds, then all tokens grouped by the registry, searchable, with a reset dot on pinned
  ones. Right: a live preview of real components, and an "Apply to app while editing" switch.
- When a seed changes, tokens pinned only because the theme was duplicated from a built-in are
  released, so changing the accent of a copy of Light changes its hover too.
- Contrast warnings for the pairs that matter (text on each surface, text on fills), each with a
  Fix that nudges lightness. Undo and redo. Drafts autosave; Save writes the theme.
- Commands: `theme.new`, `theme.edit`, `theme.rename`. `theme.duplicate` is done (the copy's
  id is `<name>-copy`, or `-copy-N`).

### 4. Import and export

- `.bonsaitheme` files (the JSON above) and share codes: `bonsai-theme:1:` + base64url of the
  deflated JSON, usually under 1 KB, pasteable into chat.
- Import from a file, a pasted code or drag-and-drop shows a preview card with what was skipped
  and a contrast summary before installing. Same id: replace or keep both.
- A watcher on the themes folder sends `storeChanged`, so editing a theme file in a text editor
  updates the app live. It only notifies.
- Commands: `theme.import`, `theme.export`, `theme.copy_code`.

### 5. Inspect and more knobs

- `theme.inspect`: click any element; its `data-component` / `data-slot` maps to the tokens it
  uses, which open in the editor.
- Type scale and density as seeds. Maybe a per-account theme override, to tell alts apart at a
  glance.

### 6. Hub

The document is hub-ready: `id` + `version` for updates, a content hash for integrity, `author`
becomes an account id, and previews draw from tokens so no images are uploaded.

**Decide first:** `AGENTS.md` makes `rbx-api` the only crate that touches the network, and it is
the Roblox client. The hub needs a sibling crate (e.g. `hub-api`) under the same rules (no
secrets in logs, shared rate limits) or an amended rule.

## Not doing

- **Custom CSS.** It breaks on every update and makes hub moderation a security problem.
- **Font files inside themes.** A theme names a Google family; each machine downloads it.
- **Theming the sheens, motion or density.** Sheens are tuned for every theme alike; density and
  motion are settings.
