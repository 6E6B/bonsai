# Known gaps

What the website does that Bonsai doesn't yet.
Remove a line when it ships.

## Not built

- Avatar: reset to default appearance. No reset endpoint in the tracker; capture what the site sends.
- Home: event cards and banners. `/v1/events/single` and `virtual-events` exist but their response shapes aren't recorded.
- Settings > Security: change password; 2-Step Verification setup (authenticator, email, passkeys).
- Settings > Language: app display language. Relative times ("5m ago") are English only.
- Themes: theme hub. See [theming.md](theming.md).
- Item page: audio player for music and sound effects.
- Profile > Creations: a user's models, decals, audio and other assets. No endpoint lists them.

