# UI performance runs

Run from `crates/bonsai/ui`:

```sh
bun install
bunx playwright install chromium webkit
bun run perf
```

This runs a minified performance build of the actual app screens with fixed, synthetic data. It needs no Roblox cookie, network access to Roblox, Rust build, or paid CI minutes. The only new development dependency is Playwright, which supplies browser automation, assertions, traces and HTML reports.

The fixture bindings and store are selected only by `vite build --mode performance`. That build writes to `perf-results/dist`, separate from the desktop app's `dist`. Normal browser and desktop builds use the original store and generated bindings. Never package the performance build as an application.

The UI GitHub Actions workflow also runs the scenarios on UI pull requests and main pushes, and uploads the report and failure traces as `ui-performance`. Local runs remain independent of CI availability. Timing measurements are informational unless you set a machine-specific budget; errors, missing fixtures, inaccessible list tails and DOM growth are automatic failures.

## Coverage

| Scenario | Actions |
| --- | --- |
| Player profile | Initial overview, all non-Wiki top-level tabs, scrolling, store refresh |
| Profile large lists | Verify the last experience, badge, community and collectible is reachable after scrolling |
| Friends | Friends, followers, following, blocked, scrolling, filtering, store refresh |
| Marketplace | Initial results, scrolling, store refresh |
| Home | Initial populated screen, scrolling, store refresh |
| Messages, conversations, notifications, communities, trades, themes | Populated collections, scrolling and store refresh |
| Shared collection behavior | Shelf endpoints and Tab traversal, table Tab traversal, cursor movement past unmounted rows, focus retention, filtering, chat append/prepend and short chat alignment |

Coverage is explicit. Game and community detail pages, other dialogs, profile experience subfilters, Wiki enrichment, 3D avatars, image decoding and theme backgrounds are not yet benchmarked. Add fixtures and scenarios before claiming coverage for them. Unknown fixture calls, application exceptions and console errors fail the run rather than producing a timing for a broken or empty screen.

## Measurements

Each scenario checks populated content, then measures repeated interactions. The runner waits for outstanding fixture reads and two animation frames after each action. It records:

- Action duration, median and nearest-rank p95 across repetitions.
- DOM element count, animation-frame gaps and browser long tasks when supported.
- JavaScript heap usage in Chromium; `null` in WebKit.
- Cumulative fixture read counts, so unnecessary refreshes are visible.

The HTML report is `perf-results/report/index.html`. Each benchmark has an attached `performance.json`; a readable copy also lives under `perf-results/artifacts/<test-name>/performance.json`. Failed tests retain a screenshot and Playwright trace. All generated outputs are ignored by git and Biome.

```sh
bunx playwright show-report perf-results/report
bunx playwright show-trace perf-results/artifacts/<test-name>/trace.zip
```

The suite uses one worker and no retries. Run it on an otherwise idle machine and compare the same browser, data size, viewport and CPU setting. Action duration includes Playwright input dispatch, click stability checks and the settling frames. Scroll measurements also include the scripted pauses. A first navigation is recorded once per scenario, not as a repeated cold-start distribution. Small sample counts make p95 approximate.

## Stress and regression checks

```sh
# Chromium only; useful if WebKit dependencies are unavailable.
bun run perf --project=chromium

# More loaded items and repetitions.
PERF_ROWS=2000 PERF_SAMPLES=10 bun run perf --project=chromium

# Approximate a slower CPU in Chromium. WebKit has no equivalent CDP setting.
PERF_CPU_RATE=4 bun run perf --project=chromium

# Attach a Chrome DevTools CPU profile to identify expensive functions.
PERF_CPU_PROFILE=1 bun run perf --project=chromium --grep profile

# Optional machine-specific upper bound, applied to each measured action.
PERF_MAX_ACTION_MS=1500 bun run perf --project=chromium
```

CPU profiling adds overhead; use a separate run for diagnosis and compare timings with profiling disabled. The CPU profile can be loaded in Chrome DevTools Performance.

The four profile grids must remain below 3,000 total page elements, regardless of loaded list size. This catches accidental removal of virtualization without relying on hardware speed. Set `PERF_ALLOW_UNBOUNDED=1` only to measure the original unvirtualized implementation as a baseline. Fixture sizes are capped at 5,000 items per list. Save reports elsewhere before another run, which replaces the previous output.

## First finding

The profile's experiences, badges, communities and collectibles mounted all loaded entries. They now use the same `VirtualGrid` already used for connections and inventory.

A sequential before/after run in Chromium with 500 entries per list, a 1440 × 900 viewport, no CPU throttling and three repetitions measured:

| Profile tab | Before median | After median | Before elements | After elements |
| --- | ---: | ---: | ---: | ---: |
| Experiences | 1,335 ms | 295 ms | 9,273 | 1,724 |
| Badges | 830 ms | 205 ms | 5,269 | 1,280 |
| Communities | 725 ms | 131 ms | 6,269 | 517 |
| Collectibles | 1,056 ms | 228 ms | 8,269 | 1,560 |

These are synthetic frontend results, not production latency guarantees. Initial profile navigation was essentially unchanged (829 ms before, 812 ms after); this fix targets the full lists. The overview still renders roughly 2,900 elements and subscribes to multiple data views. Investigating real profile-opening latency still requires native traces to separate frontend work, IPC serialization, cache reads, Roblox requests and image loading.

## Native verification

Playwright Chromium approximates WebView2; Playwright WebKit is useful for cross-engine comparisons but is not the exact WebKitGTK/WKWebView build shipped by Tauri. Native Linux scrolling, compositing and image decode behavior must also be checked in Bonsai. WebKit browser tests require the distribution's GTK/GStreamer and related browser libraries. Chromium scenarios were validated in the implementation environment; WebKit execution was blocked by missing system libraries.

The synthetic store uses Solid stores and `reconcile` with fresh payload objects and refreshes mounted subscriptions. It does not exercise the production store's event coalescing, Rust/backend performance or real network scheduling. Background store refreshes here carry unchanged fixture data, so they measure the cost of unchanged snapshots rather than every possible update pattern.

## Shared collection rollout

The UI now uses `VirtualList`, `VirtualGrid`, `VirtualShelf` and the existing virtual `DataTable` as its collection primitives. The migration covers profile shelves; inbox and conversation lists; chat history and member pickers; notifications; trade lists and tradable-item pickers; community lists, stores, requests, audit logs, forum posts and replies; game passes, badges, events, recommendations and private/friend servers; similar items; themes and hub releases; search and wiki results; blocked users and private-server settings; and friends playing on Home.

Existing inventory, marketplace, avatar/outfit and tabular views retain their shared virtualization. Bounded forms, palettes, tabs, capped small home previews and short metadata strips keep ordinary rendering. No Roblox endpoints, credentials, generated bindings or CSP settings changed. See the [component guide](../crates/bonsai/ui/src/lib/ui/README.md) for the contracts.

In a 500-item synthetic profile run, virtualizing the six preview shelves reduced the full overview DOM from about 2,900 elements to about 1,200. Exact timings vary by machine; the stronger regression checks verify bounded mounting, reachability, focus and scroll anchoring. Real initial profile-opening latency still includes backend and image work outside this harness.
