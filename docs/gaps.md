# Known gaps

What the website does that Bonsai doesn't yet.
Remove a line when it ships.

## Remaining limits

- Application updates need the repository signing variable/secrets configured and a published signed release. Cross-platform installation and restart still need the release smoke test in `docs/updates.md`; builds without a key show updates as unavailable.

- Avatar category tabs search only their loaded inventory type. There is no cross-type wardrobe search.
- Avatar **Recent** lists only what was put on through Bonsai on this computer. Roblox has no recent-items endpoint.

- Dependency browsing inspects references in downloadable source and processed accessory/head models. It does not traverse dependency files recursively, list embedded/built-in resources, or bypass Roblox asset permissions. Non-model assets show an explicit unavailable-model state.
