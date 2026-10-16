# Avatar engine

A trimmed copy of the renderer from [RoAvatar](https://github.com/steinann/RoAvatar), which
is GPL-3.0. It reads Roblox rig, mesh, animation and texture files and draws avatars with
three.js, including layered clothing, body scaling, facial animation and emotes.

- Upstream renderer commit: `dcc79481cf1ef204c7a0ab73499b16f5e3dd03da`
- Upstream repo commit: `8b72adbd316de0b86c76b7c319af6c65df3b5ce0`

Biome ignores this folder, so the code keeps upstream's style and diffs against upstream
stay readable. Only `../stage.ts` imports it, and the app loads that module with `import()`.

## Changes from upstream

- `api.ts` is a host shim, not the upstream web API client. Asset files load from the app's
  `rbxasset://` protocol through `HOST.assetUrl`, and rbx-core fetches them through rbx-api.
  Catalog, economy, outfit and bundle lookups are stubs, because the app passes the outfit in.
- Post-processing (`postprocessing`, `n8ao`), GLTF export, the HTML loading icon and error
  overlay, and double-size rendering are removed. `renderScene` draws straight to the canvas.
- HumanoidDescription import and export, accessory blob export, and `imageUrlToDataUrl` are
  removed from `avatar/outfit.ts` and `misc/misc.ts`.
- Deleted files:
  - `avatar/local-outfit.ts`, `avatar/sorts.ts`, `browser.ts`
  - `mesh/draco.ts`, `mesh/mesh-deform-weight-worker.ts`
  - `misc/exposer.ts`, `misc/extents.ts`, `misc/tests.ts`, `misc/thumbnail-generator.ts`, `misc/thumbnail-position.ts`
  - `rblx/roavatar-data-parser.ts`
  - `thumbnails/`
  - `render/subDescs/legacy-skeletonDesc.ts`, `render/subDescs/local-skeletonDesc.ts`
- `misc/worker-functions.ts` loads its worker with `?worker` instead of `?worker&inline`,
  because the CSP blocks blob workers.
- `misc/flags.ts` points `ASSETS_PATH` and `RIG_PATH` at the bundled files in
  `ui/public/avatar3d/`, so the webview never loads them from the network.
- `index.ts` exports only what the app uses.

The Draco mesh decoder is three.js's `draco_decoder.js` (asm.js, no `eval`), which
`../stage.ts` loads as a script before the engine starts.
