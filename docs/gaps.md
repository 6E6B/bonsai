# Known gaps

What the website does that Bonsai doesn't yet.
Remove a line when it ships.

## Remaining limits

- Application updates need the repository signing variable/secrets configured and a published signed release. Cross-platform installation and restart still need the release smoke test in `docs/updates.md`; builds without a key show updates as unavailable.

- Avatar category tabs search only their loaded inventory type. Use **All items** for the full wardrobe index and cross-type name sorting. The first index fills in pages; partial results stay visible until it completes.

- Dependency browsing inspects references in downloadable source and processed accessory/head models. It does not traverse dependency files recursively, list embedded/built-in resources, or bypass Roblox asset permissions. Non-model assets show an explicit unavailable-model state.
