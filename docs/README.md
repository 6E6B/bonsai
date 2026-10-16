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
- **Run tests**: `cargo test`
- **Regenerate API wrappers**: `cargo xtask codegen`

## Desktop app

The UI is a Tauri 2 shell (`crates/bonsai`) around a SolidJS app (`crates/bonsai/ui`). Install [bun](https://bun.sh), then:

```sh
cd crates/bonsai/ui
bun install
bun run app      # desktop app with hot reload
bun run check    # typecheck, lint, tests
```

See [`AGENTS.md`](../AGENTS.md) for the architecture and rules.

## License

GPL-3.0-only. See [`LICENSE`](../LICENSE).
