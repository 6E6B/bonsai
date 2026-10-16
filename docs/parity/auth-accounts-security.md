# Authentication, Multi-Account & Security

## Authentication & Sign In
- [x] **Cookie Login Flow**:
  - [x] Cookie paste input field (validates `.ROBLOSECURITY` format)
  - [x] Account confirmation preview (fetches avatar and usernames before saving)
  - [x] Secure storage (saves cookie to OS Keychain via Secret Service / Credential Manager / Keychain, with in-memory fallback)
  - [x] Redaction: Cookies, CSRF tokens, and auth tickets are never printed to logs or committed
- [x] **Quick Login (Cross-Device)**:
  - [x] "Enter Code" screen (enter 6-character code shown on another device to sign it in): `account.approve_code` (also the account rail menu and Settings → Account → Security). Enter the code, see the device and location, then Confirm (`auth-token-service/v1/login/enterCode`, `validateCode`); closing early sends `login/cancel`
  - [x] "Generate Code" screen (display 6-character code to sign in on another device)
  - [x] Quick sign-in adds an account (show code, poll status, finish login, save cookie). See [Sign-in plan](#sign-in-plan)
- [x] **Browser Sign-In Window**:
  - [x] Separate window that loads the Roblox login page and saves the cookie after login. See [Sign-in plan](#sign-in-plan)
- [x] **Sign Out**:
  - [x] Sign out current account (removes credentials from keyring)
  - [x] Sign out of all other sessions action (`account.sign_out_others`, auth v2 `logoutfromallsessionsandreauthenticate`; the new cookie Roblox sends back replaces the saved one)

## Multi-Account Management
- [x] **Account Switcher Rail**:
  - [x] Vertical rail displaying avatar headshots for all saved accounts
  - [x] Active account highlighted with color indicator
  - [x] 1-click instant account switching (renders from local cache in 1 frame)
  - [x] Keyboard shortcuts for switching (e.g. `Ctrl+1` through `Ctrl+9`, `Ctrl+Tab` to cycle last-used)
  - [x] Unread badges per account on rail (messages, trades, friend requests, chat)
  - [x] Add Account button (`+`)
- [x] **Account Customisation & Organisation**:
  - [x] Custom account label (e.g. "Main", "Trader", "Alt")
  - [x] Custom color tag selector
  - [x] Reorder accounts list
  - [x] Remove account from client (with confirmation)
- [x] **Act-as-Alt Actions**:
  - [x] Account picker on interactive actions (Join game as Alt, Buy item as Alt, Send friend request as Alt without switching active view): `game.join_as`, `market.buy_as` ("Buy as…" on the item page; the confirm step names the paying account) and `user.add_friend_as` ("Add friend as" in user menus)

## Security Challenges & Verification Modals
- [x] **Two-Step Verification (2FA)**:
  - [x] Authenticator App (TOTP) 6-digit code modal
  - [x] Email verification code modal (sends the code, Resend)
  - [x] Remember device checkbox
- [x] **Account PIN**:
  - [x] 4-digit Account PIN prompt to unlock sensitive settings or trading (`account.unlock_pin`, `account.lock_pin`; status in Settings → Account → Security)
- [x] **Captcha & Challenge Handling**:
  - [x] Arkose Labs Captcha challenge modal (surfaces web challenge when Roblox requires it). The puzzle opens in its own incognito window, which can't call IPC; the user solves it
  - [x] Resume action automatically upon successful challenge resolution (the transport hands `rblx-challenge-*` answers to bonsai-core, which calls `/challenge/v1/continue` and resends the write)

Challenge notes:
- One check shows at a time. A second one that arrives while the dialog is open fails straight away.
- Only writes go to the dialog. Reads that get a challenge fail as before.
- Security keys, passkeys and SMS aren't offered. Roblox's site handles those.
- A purchase whose body says `TwoStepVerificationRequired` (not a header challenge) still sends the user to the website.
- Not yet tried against a live challenge: the `/challenge/v1/continue` call, the captcha window, and whether PIN `unlockedUntil` is seconds left or a time. The flow follows the site's Challenge.js, captured 2026-10-15.
- [x] **Session & Error Feedback**:
  - [x] Session expired notice (prompts user to re-paste cookie without clearing cached data)
  - [x] Rate-limit toast notices (shows countdown when a host is rate-limited)
  - [x] Offline status banner (indicates cached mode when network is unreachable)

## Sign-in plan

There are three ways to add an account: paste a cookie (done), quick sign-in, and a browser sign-in window. All three produce a `.ROBLOSECURITY` cookie that goes to the keychain through the same code `account.add` uses today.

### Session binding (HBA / `x-bound-auth-token`)

Roblox can bind a session to an ECDSA P-256 key pair. The key is registered when the login request includes `secureAuthenticationIntent`, and later requests carry an `x-bound-auth-token` signed with that key. Its purpose is to stop stolen cookies from being used. It does not change rate limits. Roblox does not enforce it right now, and it is unknown when enforcement will start again ([RoSeal Roblox-BAT](https://github.com/RoSeal-Extension/Roblox-BAT), [DevForum](https://devforum.roblox.com/t/introducing-account-session-protection/2668306)).

Rules:
- Only sign with the key the session was created with. Never generate a new key for a session that already exists, such as a pasted cookie or a cookie from the browser window. A mismatched key looks worse than no token.
- Quick sign-in sends the login request itself, so we decide. At first, don't send `secureAuthenticationIntent`; the session is then unbound and needs no tokens.
- Leave room in the account record for an optional key (stored in the keychain, redacted like the cookie) so quick sign-in can register one later if enforcement returns. Signing would happen in `bonsai-api`, in Rust (`p256`), using the token format `v1|sha256(body)|timestamp|sig1|sig2`.
- In the browser window, roblox.com's own script makes the key and keeps it in that webview's IndexedDB, so the saved session is bound to a key we can't use. Accept this. Don't seed or tamper with that IndexedDB. If enforcement returns, mark these accounts "sign in again" and point users to quick sign-in.

### Quick sign-in

- Command `account.quickSignIn` (palette title "Add account with quick sign-in"). It opens a dialog in the main window.
- Flow: request a code, show the code and a countdown, poll its status, show who approved it (name and avatar) when it is validated, finish the login, and save the account. Steps inside the dialog call IPC directly, like `account.add`.
- Cancel, expiry and error states. If the login response comes back with an unexpected challenge, show it as an error that tells the user to use the browser window. Never try to solve it.
- Polling runs in `bonsai-core`, starts only when the dialog opens and stops when it closes. It shares the global rate limiter.
- Endpoints: none of them are in `vendor/roblox-api-tracker/auth` or `specs/undocumented/`. Capture the code create, status and login requests from roblox.com in devtools, add them to `specs/undocumented/`, and record replay fixtures before writing the wrapper.

### Browser sign-in window

- Command `account.signInWithBrowser` (palette title "Add account with Roblox sign-in page"). It opens a separate Tauri window, not a view in the main window.
- The window loads only the Roblox login page. That URL is defined in `bonsai-api` and listed in `specs/undocumented/www.roblox.com.yaml`, not hardcoded in `bonsai`. The window gets no IPC capabilities in `capabilities/` and no bindings. Navigation is limited to roblox.com hosts; any other link opens in the system browser.
- It uses its own non-persistent data store (Tauri `data_directory` / incognito per platform), wiped when the window closes, so cookies and IndexedDB are not shared with the main webview or with previous sign-ins.
- The main window's CSP stays as it is. The remote-content exception applies only to this window.
- Rust watches the window's navigation. Once it leaves the login page, read the `.ROBLOSECURITY` cookie from the webview's cookie store (Rust side only), close the window, and run the existing confirm-and-save step. The cookie never goes to the UI.
- The page shows its own captcha and two-step prompts to the user. That satisfies the challenge rule.
- Linux: check that WebKitGTK cookie access and the ephemeral data store behave correctly. Check `__NV_DISABLE_EXPLICIT_SYNC` covers the second window too.

### Order

1. Browser sign-in window (no undocumented endpoints needed).
2. Quick sign-in (needs captured specs and fixtures first).
3. Optional HBA signing for quick sign-in sessions, only if Roblox starts enforcing again.

### Status (2026-10-14)

Steps 1 and 2 are built. Both flows end in the same confirm step as a pasted cookie.

- Quick sign-in: `account.quick_sign_in` ("Add account with quick sign-in"). `bonsai-core/src/signin.rs` polls every 5 s like the site and sends the login without `secureAuthenticationIntent`. The endpoints are in `specs/undocumented/apis.roblox.com.yaml` (`/auth-token-service/v1/login/{create,status,cancel}`). Create, status and cancel were captured live (fixture `quick_sign_in.json`). `UserLinked`, `Validated` and the login body (`ctype` AuthToken, `cvalue` code, `password` privateKey, to the documented `auth /v2/login`) come from the site's `CrossDeviceLoginDisplayCode.js` and `ReactLogin.js`. The approved path is a synthetic fixture (`quick_sign_in_approved.json`) because recording it needs a second signed-in device.
- Browser window: `account.sign_in_browser` ("Add account with Roblox sign-in page"). `bonsai/src/signin.rs`, window label `signin`, page from `links::sign_in_page()` (`/login` in `specs/undocumented/www.roblox.com.yaml`). It allows pages and frames on `roblox.com` and `arkoselabs.com`, because the page embeds the captcha from there. Links that open a new window go to the system browser only if they are Roblox links; everything else is blocked. Besides the ACL, the invoke handler in `lib.rs` rejects every webview except `main`.
- The paste dialog links to both flows, so the `+` button reaches them too.

Not tried by hand yet:
- Quick sign-in approved from a real second device.
- The sign-in window on Linux (WebKitGTK cookie read, incognito store, NVIDIA/Wayland), Windows and macOS. Also whether the captcha frame loads with the host allowlist.
