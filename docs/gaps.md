# Known gaps

What the website does that Bonsai doesn't yet.
Remove a line when it ships.

## Remaining limits

- Trusted friendships use Roblox's eligibility rules. Sending, accepting, declining, canceling and removing trust are available; Bonsai does not perform Roblox's age checks or verification flow.

- Application updates need the repository signing variable/secrets configured and a published signed release. Cross-platform installation and restart still need the release smoke test in `docs/updates.md`; builds without a key show updates as unavailable.

- Plugins on the hub are switched off (`PLUGINS_ENABLED` in `bonsai-core/src/hub.rs`, `hubPlugins` in `ui/src/lib/hub.ts`). The hub lists and publishes themes only; plugins install from a folder. Revocation checks for installed plugins still run.

- A moderated account's human check (Roblox's "locked" state) and moderation appeals happen on the website. "Open on website" opens the system browser, which may be signed in as a different account.

- Avatar category tabs search only their loaded inventory type. There is no cross-type wardrobe search.
- Avatar **Recent** lists only what was put on through Bonsai on this computer. Roblox has no recent-items endpoint.

- The Model tab shows one model at a time and is read only: no 3D preview of the selected instance, no export, and script sources are cut at 4 KiB.
- Dependency browsing inspects references in downloadable source and processed accessory/head models. It does not traverse dependency files recursively, list embedded/built-in resources, or bypass Roblox asset permissions. Non-model assets show an explicit unavailable-model state.

- Launch tracking reads the player's own log and the process list. Sober logs its lifecycle, so a crash there is certain; on Windows, macOS and Wine a crash is inferred from the player closing mid-game without leaving. The Windows path (`tasklist`, `taskkill`, `%LOCALAPPDATA%\Roblox\logs`) has not been run on a Windows machine yet. Custom launch commands for other players show as "Opened" when Bonsai finds neither a log nor a process.

- Bookmarks are local only. Folders, tags, import/export, alerts and individual server bookmarks are not included.
