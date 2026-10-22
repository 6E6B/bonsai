# Theme and plugin hub

The desktop connects to `https://hub.6e6b.dev` by default. Backend code lives in the separate private `6E6B/bonsai-backend` repository. Contract: [hub-openapi.json](hub-openapi.json), v1.0.0.

## Desktop setup

The hub works without environment configuration. Browsing and installing are anonymous; publishing uses **Sign in with GitHub**. For local backend development, start the backend with its README instructions and override the origin:

```sh
export BONSAI_HUB_URL=http://127.0.0.1:8787
cd crates/bonsai/ui
bun run app
```

The Rust transport defines the production origin. `BONSAI_HUB_URL` overrides it; an empty or invalid override disables the hub instead of silently falling back to production. Remote origins require HTTPS, and only literal loopback permits HTTP. The hub client rejects redirects and embedded credentials and bounds response/package bytes. It shares the machine rate limiter with `bonsai-api`, but never uses Roblox cookies or its session store. The CSP is unchanged.

For local publication tests, the backend's `session:local` helper prints an explicit temporary `BONSAI_HUB_SESSION` environment assignment. The desktop accepts this only for loopback HTTP and verifies it with `/v1/auth/me`; it never sends it to the webview or stores it in a plaintext file. The normal sign-in command uses GitHub authorization code with PKCE in the system browser and the OS credential manager. The Worker receives the GitHub callback and exchanges the code using its server-only client secret. No repository permissions, device codes or local HTTP listener are needed. The desktop only opens GitHub’s exact authorization endpoint and receives a Bonsai session, never a GitHub token.

## Interface

- Hub is available in the sidebar and command palette. Themes and Settings > Plugins link to it.
- Search, base filters, sorting and bounded 24-item pages use cached core snapshots. Network errors retain the last snapshot for that query.
- Theme previews use the same declarative parser and ThemeCard as installed themes. No preview images or remote assets are fetched by the webview.
- Details show publisher ID, version, exact archive/document hash, requested plugin permissions and release history. A specific historic release can be reviewed for installation.
- Installing a theme opens the existing preview, contrast and collision dialog. Plugins pass archive SHA-256 verification and the existing safe extraction, manifest checks and permission review. Updates do not automatically grant new permissions.
- Publish opens a form using saved user themes or installed plugins. Themes support a hub ID, name, release version, description and tags. Plugin ID and version come from its actual manifest. Edit the plugin manifest before publishing a new version.
- Hub sessions are independent of the selected Roblox account. Anonymous users can browse and install. Publication, reports, unlisting and deletion require a verified hub session.
- Submission status shows pending/approved/rejected/revoked results. Plugin moderation is an administrator CLI/API operation in the backend, not a desktop approval shortcut.
- Owners can unlist or permanently soft-delete hub entries. IDs remain reserved. Local installed copies are unchanged.
- Revocations refresh on startup and every 15 minutes when background jobs are enabled, with a manual check in the hub. Known revoked plugin IDs/versions are cached in plugin settings and blocked before startup/reload, enabling, and installation. Refresh stops running revoked workers and removes their commands. The cache survives offline restarts, removal/reinstallation, and empty server responses. A newer version can run after normal permission review. No files are automatically deleted. New revocations cannot be learned while offline; existing plugins may run until the first successful refresh. Disk persistence failures are reported while runtime blocking still applies.

Every new action is a registry command. Hub publishing/authentication actions have a separate `Effect::Hub` classification and are unavailable to plugins. Read-only IPC commands only return core snapshots or schedule work. HTTP, packaging, keychain work and disk staging run on background workers.

## Local validation and limitations

The backend suite exercises real emulated D1/R2 bindings, themes, ownership, immutable versions, pagination, pending-metadata isolation, moderation, reports, archive limits, OAuth state/PKCE/one-time exchange, session expiry/revocation and cookie isolation. The Rust transport has replay-style wiremock checks for credentials, origin restrictions, encoding, identity and contract version. UI validation and the existing core/UI suites cover surrounding behavior.

The local Rust-to-Worker round trip passes browsing, theme staging/install, publication and search against emulated D1/R2. Browser layout review uses mocked IPC with the real hub components and local sample themes, including details/publishing at desktop size and the publishing form at 900×700; it does not replace native Tauri end-to-end testing. Revocation regression tests cover stopping a running worker, command removal, offline restart, enable/reload/reinstall refusal and a newer version becoming runnable.

The service is deployed and verified with the live desktop app: GitHub sign-in, OS credential-manager persistence, end-to-end installation/publishing, plugin revocation and backup/rollback/recovery. Public abuse monitoring remains. The main D1/R2 storage is provisioned with migrations applied, with `hub.6e6b.dev` configured as the Worker custom domain; the Worker is deployed with GitHub client configuration and its secret stored only in Cloudflare. The backend’s OPERATIONS.md records setup and recovery procedures. Ratings, reviews, favorites, download counters and ranking remain deferred as the plan specifies.

GitHub sign-in validation: backend typecheck and 27 tests, UI check (340 tests) and production build, and focused Rust tests (248 tests) plus strict Clippy pass. Workspace nextest, Clippy and doc tests cannot build native desktop dependencies here because `pkg-config` is unavailable (GLib/GIO). Live service health, empty catalogs, GitHub authorization initiation, pending handoff and verifier rejection have been checked. Complete GitHub browser consent and OS credential persistence have since been verified with the live app.
