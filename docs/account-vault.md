# Account vault

PIN protection is optional and covers every saved Roblox login in a Bonsai profile.
Settings > Accounts and the command palette provide enable, change, disable and lock.
The lock screen accepts the same PIN or passphrase for all accounts. Numeric PINs
require at least eight digits; passphrases require at least twelve characters.
Repeating digits and straight numeric sequences are rejected. These checks do not
make a predictable secret strong.

## Encryption and threat model

A random 256-bit vault key encrypts the account-cookie map and private SQLite
payloads using RustCrypto XChaCha20-Poly1305. Every encryption uses a fresh random
192-bit nonce from the OS. Account IDs are inside the authenticated cookie map.
Cache AAD includes the table and complete row identity, including account ID.

Argon2id v19 derives a 256-bit wrapping key from the exact unlock secret, a random
128-bit salt and a separate random 256-bit secret in the OS keychain, using
Argon2's secret input. The cost is 64 MiB, three passes and four lanes. File-supplied
costs are checked before allocation: memory 64–256 MiB, passes 3–6, lanes exactly
four. The wrapping AAD authenticates the version, vault ID, salt and cost. Changing
the PIN refreshes the salt and wrapping ciphertext without rewriting cache data.
There is no separate PIN verifier. Unlock authenticates both envelopes.

Copying the vault without the keychain secret does not permit testing PIN guesses.
An attacker who extracts both can guess offline. The application cooldown is not
an offline defense. Use a strong passphrase for this threat. OS keychains are not
hardware-enforced PIN retry counters. Malware in an unlocked session and an attacker
who can modify the running application are outside this boundary.

`Secret` and vault plaintext/key buffers zeroize their owned storage when dropped.
This cannot guarantee erasure of copies inside JavaScript, HTTP/TLS libraries,
swap, crash dumps or operating-system buffers.

## Enforcement and storage

Rust rejects account work and private IPC while locked. Custom image/asset protocols
also reject requests. Authenticated clients are not loaded on locked startup.
Client revocation cancels pending sends and invalidates every clone, including late
cookie rotation. Manual lock, system lock/sleep and inactivity restart the process into
a locked session, destroying the prior webview, tasks and plugin VMs. Inactivity defaults
to five minutes and is configurable to 1, 5, 15, 30 or 60 minutes. Invalid file-supplied
timeouts are bounded to 1–60 minutes. Ordinary window focus loss does not lock.

Windows uses a hidden top-level window for WTS session and power notifications.
macOS observes distributed screen-lock notifications and NSWorkspace session/sleep
notifications on the main run loop. Linux uses GIO D-Bus subscriptions to desktop
ScreenSaver services and the current login1 session, plus PrepareForSleep. Resume
also locks and never unlocks. Linux coverage depends on the desktop publishing these
notifications. Missing providers or disconnected notification buses produce a warning;
disconnection also locks an active vault. Notification callbacks close backend admission
before scheduling UI cleanup. An ending-session flag stays closed even if a concurrent
setup or unlock finishes. No new Linux monitoring dependency is needed.

The vault holds an exclusive process lock before it may unlock or write. Files are
size-bounded, written to a temporary file, synced and atomically replaced. Unix
also syncs the parent directory. The keychain's enabled marker means deleting or
corrupting the file does not switch protection off. Missing device secrets do not
fall back to ordinary keychain credentials. An unavailable keychain prevents setup.

Enabling reads current, unprotected account credentials, durably writes and verifies
the encrypted vault, encrypts private cache payloads, then removes source credentials
before admitting work. An encrypted transition journal resumes after interruption.
Disabling requires the current PIN and journals cleanup before decrypting the cache
or restoring credentials. If interrupted before the keychain disable marker commits,
the next unlock re-encrypts cache data and removes restored source credentials before
admitting work. These are current on/off transitions, not old-format import paths.
Baseplate keychain and directory migration are intentionally unsupported.

Public shared caches, account names/order/labels, settings, downloaded assets and
plugin-owned files remain outside the encrypted payload boundary. Cache migration
uses SQLite secure deletion, compaction and WAL truncation, but cannot erase backups,
filesystem snapshots or SSD remnants. There is no recovery backdoor: loss of either
unlock secret or device secret requires forgetting the saved accounts and signing in
again. The lock screen's Forgot PIN? action requires typing RESET. It deletes source
credentials, private cache/ledger rows and the saved account list before rotating the
device secret to a disabled marker. It cannot decrypt or recover the old vault. Failed
cleanup stays locked; late cache and sign-in results cannot repopulate the reset session.
The app restarts before unprotected access is admitted.

## Validation and release work

Unit tests cover wrong PINs, tampered headers/ciphertext, bounded KDF costs and file
sizes, missing files/secrets, cross-account cache ciphertext, fresh nonces, writer
exclusion, failed writes, interrupted enable/disable, keychain errors, PIN changes,
cooldowns, locked cache access, destructive recovery, reset failures, late sign-in/cache
writes, OS-lock/setup races, native signal parsing and revocation of cloned clients.
Windows and macOS adapter sources were cross-compiled from Linux; that does not test
notification delivery on those desktops. A real-keychain smoke test uses a unique test
entry and synthetic credentials only. Run it on an unlocked native desktop with:

```sh
cargo test -p bonsai-core --locked real_os_keychain_round_trip -- --ignored
```

The development-host benchmark measured a median unlock of about 142 ms over five
attempts with the 64 MiB / three-pass profile. This is not a low-end hardware guarantee.

The isolated Linux smoke-test attempt in this environment could not start D-Bus because
Unix socket creation is restricted. No user keychain or Roblox cookie was used. Unlock
cost can be measured without a real keychain using:

```sh
cargo test -p bonsai-core --locked measure_unlock_cost -- --ignored --nocapture
```

This is a new implementation, not independently audited or battle tested. Before
release, exercise real keychain persistence and restart behavior on Windows, macOS
and Linux, benchmark unlock cost on supported low-end hardware, exercise real desktop
lock/sleep notification delivery, and obtain an independent security review.
