# Bonsai

Portable Rust backend for the Bonsai desktop client (built with Tauri).

## Structure

```
bonsai/
├── crates/
│   ├── bonsai-api/        # Roblox API client (REST & SignalR WebSocket, rate limiting, auth)
│   ├── bonsai-core/       # Domain engine (StoreData, multi-account sessions, cache, launcher, commands)
│   ├── bonsai/            # Tauri shell + SolidJS UI (ui/)
│   └── bonsai-codegen/    # Generator xtask converting OpenAPI/Swagger specs to Rust API wrappers
├── specs/                 # Custom Swagger 2.0 specs for undocumented Roblox endpoints
├── vendor/                # roblox-api-tracker submodule/specs
├── .cargo/                # Cargo aliases (e.g. `cargo xtask codegen`)
└── Cargo.toml             # Clean workspace manifest containing the backend crates
```

## Useful Commands

- **Check build**: `cargo check`
- **Run tests**: `cargo nextest run --workspace --locked`
- **Run doctests**: `cargo test --workspace --locked --doc`
- **Regenerate API wrappers**: `cargo xtask codegen`

Install [cargo-nextest](https://nexte.st/docs/installation/) 0.9.148 to match CI. The source-install option is `cargo install cargo-nextest --version 0.9.148 --locked`. Nextest runs the existing Rust tests; doctests still use Cargo. `cargo test --workspace --locked` remains available without nextest.

## Desktop app

The UI is a Tauri 2 shell (`crates/bonsai`) around a SolidJS app (`crates/bonsai/ui`). Install [bun](https://bun.sh), then:

```sh
cd crates/bonsai/ui
bun install
bun run app      # desktop app with hot reload
bun run check    # typecheck, lint, tests
```

See [`AGENTS.md`](AGENTS.md) for the architecture and rules.

## CI and releases

`.github/workflows/ui.yml` runs `bun run check` and `bun run build` on PRs and pushes to `main` that touch `crates/bonsai/ui/`.

`.github/workflows/rust.yml` runs on PRs and pushes to `main` that touch anything else: `cargo nextest run --workspace --locked --profile ci` and `cargo test --workspace --locked --doc` on Linux, Windows and macOS, and on Linux `cargo fmt`, `cargo clippy` and checks that `bindings.ts` and generated API code are up to date.

`.github/workflows/release.yml` builds installers: `.deb`, `.AppImage` and `.rpm` on Linux, NSIS `.exe` and `.msi` on Windows, and a universal `.dmg` on macOS. Run it by hand from the Actions tab to get them as artifacts, or push a tag matching the workspace version (`git tag v0.1.0 && git push origin v0.1.0`) to also create a draft GitHub release. Builds are not code-signed yet.

## License

GPL-3.0-only. See [`LICENSE`](LICENSE).
