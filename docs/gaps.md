# Known gaps

What the website does that Bonsai doesn't yet.
Remove a line when it ships.

## Not built

- Chat: group creation, rename and membership writes have replay coverage but still need live verification with consenting participants. The live chat smoke test only reads settings, conversations and group history.

- Avatar: reset to default appearance. No reset endpoint in the tracker; capture what the site sends.
- Home: event cards and banners. `/v1/events/single` and `virtual-events` exist but their response shapes aren't recorded.
- Settings > Security: adding passkeys and security keys, and SMS codes. WebAuthn only works on roblox.com, so these link to the site. Untested live: Roblox may want a `secureAuthenticationIntent` on these calls.
- Settings > Language: app display language. UI text is English only; numbers, dates and relative times follow Settings > General > Number and date format.
- Themes: theme hub. See [theming.md](theming.md).
- Plugins: tiers, the Luau sandbox and install/manage (Settings > Plugins), plugin commands in the command bar and context menus, confirmed actions, reads (users, games, items, presence, friends, friend requests, inventory, trades), events, timers, per-account storage, manifest settings, automation (`ctx.run`, the per-account toggle, rate caps, the activity log and the kill switch) and tier 3 budgets (Robux and trades per day, with purchases at the price looked up before they run) are built. See [plugins.md](plugins.md#roadmap).
- Profile > Creations: a user's models, decals, audio and other assets. No endpoint lists them.

