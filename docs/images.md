# Rendered image actions

`Avatar`, `Thumbnail`, and the shared `ContextMenu` provide Save Image As…, Copy Image,
and Copy Image URL for Roblox thumbnails across the app. This includes catalog/dependency
images, outfits, game icons and thumbnails, badges, groups, and user avatars.

Images inside an existing context menu append their actions there. Standalone images get a
menu automatically, with a tab stop and Menu/Shift+F10 support. Right-clicking a particular
image in a row selects that image. Keyboard row menus use the row's first image. The command
palette uses the selected image, falling back to an open user/item/bundle/group/game page.

The menu labels these as rendered thumbnails. Original textures, models, and classic clothing
templates use separate actions and asset delivery. Saving a thumbnail never downloads a model.

## Resolution and cache

Exports request supported sizes in descending resolution using the existing documented
thumbnail types and `POST /v1/batch` wrapper. The size lists come from
`vendor/roblox-api-tracker/thumbnails/v1.json`.

| Image | Largest requested render |
| --- | --- |
| Headshot / avatar | 720×720 |
| Catalog / dependency asset | 700×700 |
| Game icon | 512×512 |
| Game thumbnail | 768×432 |
| Outfit / bundle / group | 420×420 |
| Badge / game pass | 150×150 |

Pending renders get two short retries. Errors, missing responses and corrupt images fall back
to smaller supported sizes. Export lookup has a twelve-second budget; failure still uses the
best cached render or displayed bytes. URL and byte caches are shared with `thumb://` and obey
the same freshness rules. Avatar/outfit changes invalidate export sizes too. Export images do
not replace the small display image or increase the UI's image memory budget.

Rust handles every request through `bonsai-api` and its existing machine limiter and credential
rules. The webview passes only a validated thumbnail identity and suggested name, never image
bytes, a remote fetch URL, or an arbitrary file path. There are no new network endpoints,
permissions, dependencies, or generated IPC types.

## Destination and clipboard

The native save dialog chooses the filename and destination, with a default extension/filter
matching the returned PNG, JPEG or WebP bytes. Cancellation does nothing. Saving writes those
validated image bytes unchanged. Copy Image decodes to bounded RGBA for the native image
clipboard; Copy Image URL writes the resolved public CDN URL, never a local protocol URL.
Both clipboard operations stay in Rust. Errors appear through the existing notice system.
The native dialog and clipboard run off the main thread.
