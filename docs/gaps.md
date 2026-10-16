# Known gaps

What the website does that Bonsai doesn't yet, and features never run against the live API.
Remove a line when it ships.

## Not built

- Avatar: reset to default appearance. No reset endpoint in the tracker; capture what the site sends.
- Home: event cards and banners. `/v1/events/single` and `virtual-events` exist but their response shapes aren't recorded.
- Settings > Security: change password; 2-Step Verification setup (authenticator, email, passkeys).
- Settings > Language: app display language. Relative times ("5m ago") are English only.
- Themes: import and export (`.bonsaitheme` files and share codes); theme hub. See [theming.md](theming.md).
- Item page: audio player for music and sound effects.
- Profile > Creations: a user's models, decals, audio and other assets. No endpoint lists them.

## Not possible

- Delete a private message: no endpoint.
- Mark a single notification read: the endpoint 404s.
- Send a private message: `/v1/messages/send` answers 404.

## Built but not run live

- Purchases (items, game passes): responses are synthetic, since a real run spends Robux.
- Transactions: sales, Premium stipends, group payouts and community ledgers. The fee split depends on whether a real sale's amount is net.
- Inventory: the game passes fixture is empty; place tabs other than "made by user" need a capture.
- Community forums: spec and fixture are synthetic until a test account with forums is available.
- Premium details: shape taken from the site bundle; fixture is synthetic.
