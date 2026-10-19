# Known gaps

What the website does that Bonsai doesn't yet.
Remove a line when it ships.

## Not built

- Chat: group creation, rename and membership writes have replay coverage but still need live verification with consenting participants. The live chat smoke test only reads settings, conversations and group history.

- Avatar: reset to default appearance. No reset endpoint in the tracker; capture what the site sends.
- Home: event cards and banners. `/v1/events/single` and `virtual-events` exist but their response shapes aren't recorded.
- Settings > Security: adding passkeys and security keys, and SMS codes. WebAuthn only works on roblox.com, so these link to the site. Untested live: Roblox may want a `secureAuthenticationIntent` on these calls.
- Settings > Language: app display language. UI text is English only; numbers, dates and relative times follow Settings > General > Number and date format.
- Hubs: deployed API, Rust transport, browsing, theme publishing, plugin submission/moderation, install review and explicit revocation checks are implemented. GitHub OAuth is implemented with PKCE and the Worker callback, replacing Roblox OAuth. Live GitHub OAuth verification, platform credential persistence, platform testing of installed-plugin revocation enforcement and recovery drills remain release gates. See [hub.md](hub.md).
- Plugins: tiers, the Luau sandbox and install/manage (Settings > Plugins), plugin commands in the command bar and context menus, confirmed actions, reads (users, games, items, presence, friends, friend requests, inventory, trades), events, timers, per-account storage, manifest settings, automation (`ctx.run`, the per-account toggle, rate caps, the activity log and the kill switch), tier 3 budgets (Robux and trades per day, with purchases at the price looked up before they run), views (component trees with events back) and `.bonsaiplugin` packages with a content hash are built. The hub connects to `https://hub.6e6b.dev` by default (see [hub.md](hub.md)); outside network access for plugins is not built. See [plugins.md](plugins.md#roadmap).
- Wiki: Roblox Wiki articles from Fandom on items, experiences and profiles, behind an opt-in setting, with pictures, working links, and experiences' own Fandom wikis when the Roblox Wiki links one. Checked against the live site 2026-10-19; experiences without a Roblox Wiki article don't get their own wiki. See [wiki.md](wiki.md).
- Profile > Creations: a user's models, decals, audio and other assets. No endpoint lists them.


Theme backgrounds now travel as external HTTPS links and effects with theme exports and hub releases. The editor previews drafts directly in the app.

## Discord community

- The top bar and command palette open the community invite dialog. The server invite code lives in `discord-invite.txt` at the repository root and enables its Discord protocol and browser actions. Change only the code (not a URL) and rebuild when replacing the invite. If the code is cleared, the dialog shows “Invite coming soon” and disables both actions.
