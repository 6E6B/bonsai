# Application updates

Installed releases check after startup and every six hours, including in tray mode.
Turn this off in Settings > Desktop > Updates. Manual checks remain available there
and in the command bar. Optionally enable automatic downloads to prepare updates
in the background after checks. This is off by default. A notice appears above the current page when an update is
available. Downloading keeps the app usable; installation is a separate **Install
and restart** action. Close any unfinished forms before installing.

Downloads run in the Rust host and are verified by Tauri's updater before becoming
installable. No Roblox credentials are involved, and the webview CSP is unchanged.
Failed checks, downloads, signature verification and installs show retryable errors.
Pending packages are held in memory for this session, so quitting requires another
download. Nothing installs automatically on exit.

Supported packages: Windows NSIS/MSI, macOS universal app, Linux AppImage. Linux
deb/rpm and other unpackaged builds use their package manager. Debug builds and
release builds without a verification key clearly report that updates are unavailable.
Old versions without an updater require a one-time manual installation.

## Configure release signing once

1. Run `bun run tauri signer generate -w /secure/location/bonsai-updater.key`
   from `crates/bonsai/ui`. Use a password and keep an offline backup of the key.
2. Set repository Actions variable `BONSAI_UPDATER_PUBLIC_KEY` to the contents of
   the generated `.pub` file. This public key is compiled into the app.
3. Set Actions secret `TAURI_SIGNING_PRIVATE_KEY` to the private key contents and
   secret `TAURI_SIGNING_PRIVATE_KEY_PASSWORD` to its password. Never commit either.
4. Bump the workspace version and push its matching `v<version>` tag. All three
   build jobs sign their update packages. The release job validates the complete
   platform set, creates `latest.json` with release notes and signatures, and uploads
   it alongside the packages to the draft release. Review its assets and publish it.

Tagged builds fail if signing configuration is missing. Manual workflow runs may
build unsigned installers for testing, but these cannot receive in-app updates.
Keep the same key for subsequent releases. Changing the compiled verification key
without a migration breaks trust for existing installations. Updater signatures
are separate from Windows Authenticode and macOS signing/notarization.

The fixed endpoint is the public repository's `/releases/latest/download/latest.json`.
Draft and prerelease versions do not reach the stable channel. Publish only after
every package, signature and manifest is present. Never publish a stable release
without its manifest: GitHub's latest-release URL would then return a missing file.

## Release smoke test

Install an older signed build on each supported OS, publish a higher signed version,
and check from Settings. Verify notes, progress, continuing to use the app while it
downloads, installation/relaunch, and the new version after restart. Repeat with a
tampered package/signature, interrupted download, offline checks, and unwritable
install location. Confirm no unverified package installs, retries work, settings and
accounts persist, and a deb/rpm build directs users to their package manager.

`python3 -m unittest discover -s scripts -p 'test_update_manifest.py'` checks release
manifest generation, target mappings, URL escaping and missing signature failures.
