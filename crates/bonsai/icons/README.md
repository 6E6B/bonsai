# App icons

`app-icon.svg` is the original engraved artwork. Keep its transparent padding and
rounded plate. `tray-template.svg` is the same tree silhouette without the plate,
for the macOS menu bar's light/dark template rendering.

From `crates/bonsai/ui`, run:

```sh
bun install --frozen-lockfile
bun run icons
```

This uses the locked Tauri CLI to regenerate the desktop PNGs, multi-resolution
Windows ICO, macOS ICNS, Store logos, macOS tray template and browser favicons.
Commit the generated files with any source changes. It adds no dependencies and
does not need to run during normal builds. Android and iOS are not Bonsai targets.

`tauri.conf.json` lists the desktop assets. Keep `icon.png` first: Tauri uses the
first PNG for the Unix window icon and the Linux tray. Windows embeds `icon.ico`
in the executable and uses it for installer shortcuts; macOS bundles `icon.icns`
for Finder and the Dock. Tauri includes the configured PNG sizes in Linux
packages. The runtime tray uses the app icon on Windows and Linux, and the
44px template with `icon_as_template(true)` on macOS. The local browser favicon
copies are for Vite development and previews.

The `Square*Logo.png` and `StoreLogo.png` assets are Tauri's generated Store
variants, retained for future packaging; the current NSIS/MSI installers use
`icon.ico`.
