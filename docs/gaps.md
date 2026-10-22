# Known gaps

What the website does that Bonsai doesn't yet.
Remove a line when it ships.

## Performance coverage

- Automated frontend performance scenarios cover profiles, friends, marketplace, home, messages, chat, notifications, communities, trades and themes, including WebKit resize errors and keyboard navigation through virtual collections. Detail pages, image decoding, 3D, real IPC/API latency and native webview traces remain uncovered. See [ui-performance.md](ui-performance.md).

## Not built

- Account PIN vault: device-bound encryption, lock enforcement and interrupted on/off transitions are implemented. Native keychain/restart and lock/sleep delivery testing on all platforms, low-end unlock-cost benchmarking and independent security review remain release gates. Native notification adapters, configurable inactivity and confirmed destructive recovery are implemented. See [account-vault.md](account-vault.md).
- Chat: group creation, rename and membership writes have replay coverage but still need live verification with consenting participants. The live chat smoke test only reads settings, conversations and group history.
- Avatar editor search and name sorting cover the currently loaded inventory type. Global wardrobe search needs an index or an API that searches owned wearables across types.
- Avatar: reset to default appearance. No reset endpoint in the tracker; capture what the site sends.
- Home: event cards and banners. `/v1/events/single` and `virtual-events` exist but their response shapes aren't recorded.
- Settings > Language: app display language. UI text is English only; numbers, dates and relative times follow Settings > General > Number and date format.
- Plugins: outside network access (the `net` permission). Needs the network rule in `AGENTS.md` amended first. See [plugins.md](plugins.md#9-network).
