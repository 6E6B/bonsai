# Item dependencies

The item page's **Dependencies** tab inspects the model only when opened. It lists each referenced asset once, with its name, type, creator, thumbnail and usage labels. Click a usage label for the instance paths, classes and properties. Search matches names, IDs, creators, paths and properties. Existing item and creator navigation, Share menus and copy-ID commands work on each row. Bundles link to their constituent items instead of scanning the bundle itself.

## Data and permissions

- Resolve source models with the existing generated `assetdelivery::v2::get_asset_id_by_asset_id` wrapper (`GET /v2/assetId/{assetId}`). This documented route supports `Roblox-AssetFormat`, so a separate batch endpoint is unnecessary for one item.
- For classic accessories and heads, also request `avatar_meshpart_accessory` or `avatar_meshpart_head`. Merge references from both representations. Layered clothing keeps its source model. If either representation fails, show the valid one with a partial-scan notice.
- Download returned locations through the credential-free, allowlisted CDN transport. Parse XML and RBXM with rbx-dom, including LZ4 and Zstd compression. The new Rust dependencies avoid maintaining another binary model decoder and provide a shared, typed DOM for both formats.
- Walk descendant instances and inspect content properties, including mesh/texture, surface maps, clothing, animation, audio and cage references. Recognize exact `rbxassetid://` URIs and Roblox asset URLs, excluding zero, self-references and IDs outside JavaScript's exact integer range. Script text and ordinary names are not interpreted as asset references. Scripts are never executed, and referenced files are not scanned recursively.
- Resolve optional metadata with the existing economy `GET /v2/assets/{assetId}/details` wrapper, at most six requests at once. Missing metadata retains the reference and its actions. Thumbnails reuse the existing `thumb://` cache and API batching, with no remote webview requests.
- Cache scan results for 15 minutes under the requesting account. An account or item change remounts the UI scan state. Cached rows stay visible while revalidating; refresh failures are shown beside them.

Downloads stop at 16 MiB. Binary headers/chunks are checked before decoding, with a 64 MiB expanded chunk budget. Scans reject models over 20,000 instances, 128 levels, 2,048-byte paths, 256 distinct dependencies or 4,096 references. Parsing runs on a blocking worker. Invalid, restricted, non-model or oversized assets produce an explicit error rather than an empty successful result. Embedded and built-in resources are not listed.

## Validation

- Core tests cover exact asset URI recognition, binary/XML parity, three compression modes, duplicate uses, self-references, invalid/oversized data, processed format selection, partial model availability and unavailable metadata.
- `dependencies.json` replays source/processed delivery, CDN downloads and economy metadata using synthetic IDs and names. Transport tests verify download size limits, CDN host rejection and missing cookie headers.
- An ignored live test checks a public classic accessory without a cookie: `cargo test -p bonsai-core --lib live_accessory_dependencies -- --ignored`.
- UI unit tests cover usage labels and search. `bun x playwright test -c perf/playwright.config.ts dependencies.spec.ts` checks on-demand fetching, virtualization, keyboard reference inspection, unavailable metadata and empty search results with synthetic data.
