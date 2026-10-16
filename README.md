# Bonsai

Portable Rust backend for the Bonsai desktop client (built with Tauri), cloned and decoupled from Baseplate.

## Structure

```
bonsai/
├── crates/
│   ├── rbx-api/       # Roblox API client (REST & SignalR WebSocket, rate limiting, auth)
│   ├── rbx-core/      # Domain engine (StoreData, multi-account sessions, cache, launcher, commands)
│   ├── bonsai-tauri/   # Tauri shell + SolidJS UI (ui/)
│   └── rbx-codegen/   # Generator xtask converting OpenAPI/Swagger specs to Rust API wrappers
├── specs/             # Custom Swagger 2.0 specs for undocumented Roblox endpoints
├── vendor/            # roblox-api-tracker submodule/specs
├── .cargo/            # Cargo aliases (e.g. `cargo xtask codegen`)
└── Cargo.toml         # Clean workspace manifest containing the backend crates
```

## Useful Commands

- **Check build**: `cargo check`
- **Run tests**: `cargo test`
- **Regenerate API wrappers**: `cargo xtask codegen`

## Desktop app

The UI is a Tauri 2 shell (`crates/bonsai-tauri`) around a SolidJS app (`crates/bonsai-tauri/ui`). Install [bun](https://bun.sh), then:

```sh
cd crates/bonsai-tauri/ui
bun install
bun run app      # desktop app with hot reload
bun run check    # typecheck, lint, tests
```

See `AGENTS.md` for the architecture and rules.

## License

GPL-3.0-only. See `LICENSE`.
