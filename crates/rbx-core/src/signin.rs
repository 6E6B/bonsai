//! Quick sign-in and the sign-in window. Both end with a fresh `.ROBLOSECURITY` that goes
//! through the same check-and-confirm step as a pasted cookie (`LoginState::Validating` →
//! `Confirm`), so nothing is saved until the user confirms.
//!
//! Quick sign-in never sends `secureAuthenticationIntent`, so its sessions are not bound to a
//! key and need no `x-bound-auth-token`. See the sign-in plan in
//! `docs/parity/auth-accounts-security.md`.

use std::sync::Arc;

use parking_lot::Mutex;
use rbx_api::generated::{apis, auth};
use rbx_api::{AccountId, Client, Error, Secret};

use crate::Core;
use crate::core::Inner;
use crate::store::LoginState;

/// Shown when Roblox wants more than the code: a captcha, two-step or identity check. The app
/// never answers those itself; the sign-in page shows them to the user.
const USE_WINDOW: &str =
    "Roblox wants an extra check for this sign-in. Use \"Add account with Roblox sign-in page\" instead.";

/// Codes last about five minutes; used when Roblox sends no expiry.
const DEFAULT_LIFETIME: i64 = 300;

/// A code that is waiting for approval.
pub(crate) struct QuickSession {
    client: Client,
    code: String,
}

impl Core {
    /// `account.quick_sign_in`: get a code, show it, and sign in once another device approves
    /// it. Progress shows in `login`.
    pub fn start_quick_sign_in(&self) {
        let seq = self.inner.next_login();
        self.inner.store.write().login = Some(LoginState::QuickStarting);
        self.inner.repaint();
        let inner = self.inner.clone();
        self.inner.rt.spawn(async move {
            inner.quick_sign_in(seq).await;
            inner.repaint();
        });
    }

    /// The sign-in window opened.
    pub fn browser_sign_in_started(&self) {
        self.inner.next_login();
        self.inner.store.write().login = Some(LoginState::Browser);
        self.inner.repaint();
    }

    /// The sign-in window closed. A cookie it already handed over keeps being checked.
    pub fn browser_sign_in_closed(&self) {
        let mut s = self.inner.store.write();
        if s.login == Some(LoginState::Browser) {
            s.login = None;
        }
        drop(s);
        self.inner.repaint();
    }

    /// The sign-in window has a session. Checked like a pasted cookie. Ignored unless the
    /// window is still what the user is waiting on. Returns whether it was taken.
    pub fn submit_session(&self, cookie: Secret) -> bool {
        {
            let mut s = self.inner.store.write();
            if s.login != Some(LoginState::Browser) {
                return false;
            }
            s.login = Some(LoginState::Validating);
        }
        self.inner.repaint();
        self.send(crate::Cmd::ValidateCookie(cookie));
        true
    }

    /// Stop whichever add-account flow is running and forget an unconfirmed account.
    pub fn cancel_login(&self) {
        self.inner.cancel_login();
        self.inner.repaint();
    }
}

impl Inner {
    /// Start a new flow: stops older ones and cancels a waiting code on Roblox.
    pub(crate) fn next_login(&self) -> u64 {
        let seq = {
            let mut n = self.login_seq.lock();
            *n += 1;
            *n
        };
        if let Some(q) = self.quick.lock().take() {
            self.cancel_code(q);
        }
        seq
    }

    /// Tell Roblox a code is no longer wanted, so the other device can't approve it.
    fn cancel_code(&self, q: QuickSession) {
        self.rt.spawn(async move {
            let body = apis::undocumented::QuickLoginCancel { code: Some(q.code) };
            if let Err(e) = apis::undocumented::post_quick_login_cancel(&q.client, &body).await {
                tracing::debug!("cancelling a quick sign-in code: {}", e.user_message());
            }
        });
    }

    pub(crate) fn cancel_login(&self) {
        self.next_login();
        self.forget_pending_login();
        self.store.write().login = None;
    }

    fn live(&self, seq: u64) -> bool {
        *self.login_seq.lock() == seq
    }

    /// Set `login` if this flow is still the current one.
    fn set_login(&self, seq: u64, state: LoginState) -> bool {
        let mut s = self.store.write();
        if !self.live(seq) {
            return false;
        }
        s.login = Some(state);
        drop(s);
        self.repaint();
        true
    }

    fn fail(&self, seq: u64, message: impl Into<String>) {
        self.set_login(seq, LoginState::Failed(message.into()));
    }

    async fn quick_sign_in(self: &Arc<Self>, seq: u64) {
        let session: Arc<Mutex<Option<Secret>>> = Arc::default();
        let sink = session.clone();
        // Its own client: the login response sets a cookie, and the shared anonymous client
        // must stay signed out.
        let client = match Client::builder(AccountId::ANONYMOUS)
            .base(self.base.clone())
            .limiter(self.limiter.clone())
            .browser_tracker(self.browser_tracker_id)
            .on_cookie_rotated(move |_, c| *sink.lock() = Some(c.clone()))
            .build()
        {
            Ok(c) => c,
            Err(e) => return self.fail(seq, e.user_message()),
        };
        let created = match apis::undocumented::post_quick_login_create(&client).await {
            Ok(c) => c,
            Err(e) => return self.fail(seq, e.user_message()),
        };
        let (Some(code), Some(key)) = (created.code, created.private_key.map(Secret::new)) else {
            return self.fail(seq, "Roblox didn't send a code. Try again.");
        };
        let now = crate::cache::now;
        let mut expires_at = created
            .expiration_time
            .as_deref()
            .and_then(crate::fmt::parse_iso)
            .unwrap_or_else(|| now() + DEFAULT_LIFETIME);
        let mut linked = None;
        *self.quick.lock() = Some(QuickSession {
            client: client.clone(),
            code: code.clone(),
        });
        // Cancelled while the code was being created: cancel it on Roblox too.
        if !self.live(seq) {
            let mine = self.quick.lock().take_if(|q| q.code == code);
            if let Some(q) = mine {
                self.cancel_code(q);
            }
            return;
        }
        self.set_login(
            seq,
            LoginState::QuickCode {
                code: code.clone(),
                expires_at,
                linked: None,
            },
        );
        let key_body = || apis::undocumented::QuickLoginKey {
            code: Some(code.clone()),
            private_key: Some(key.expose().to_owned()),
        };
        loop {
            tokio::time::sleep(self.quick_poll).await;
            if !self.live(seq) {
                return;
            }
            if now() >= expires_at {
                self.quick.lock().take();
                return self.fail(seq, "The code expired. Get a new one.");
            }
            let status = match apis::undocumented::post_quick_login_status(&client, &key_body()).await {
                Ok(s) => s,
                // Try again on the next tick.
                Err(Error::Network(_) | Error::RateLimited { .. }) => continue,
                // A code Roblox no longer knows answers 403 "CodeNotFound".
                Err(Error::Status { status: 403, .. }) => {
                    self.quick.lock().take();
                    return self.fail(seq, "The code expired. Get a new one.");
                }
                Err(e) => return self.fail(seq, e.user_message()),
            };
            if let Some(t) = status.expiration_time.as_deref().and_then(crate::fmt::parse_iso) {
                expires_at = t;
            }
            match status.status.as_deref() {
                Some("Validated") => break,
                Some("Cancelled") => {
                    self.quick.lock().take();
                    return self.fail(seq, "The code was cancelled on the other device.");
                }
                Some("UserLinked") => linked = status.account_name.filter(|n| !n.is_empty()),
                _ => {}
            }
            self.set_login(
                seq,
                LoginState::QuickCode {
                    code: code.clone(),
                    expires_at,
                    linked: linked.clone(),
                },
            );
        }
        // Approved: the code is used up, so there is nothing left to cancel.
        self.quick.lock().take();
        if !self.set_login(seq, LoginState::Validating) {
            return;
        }
        let body = auth::v2::LoginRequest {
            ctype: Some(auth::v2::LoginRequestCtype::AuthToken),
            cvalue: Some(code.clone()),
            password: Some(key.expose().to_owned()),
            ..Default::default()
        };
        let r = match auth::v2::post_login(&client, &body).await {
            Ok(r) => r,
            Err(Error::Challenge(_)) => return self.fail(seq, USE_WINDOW),
            Err(e) => return self.fail(seq, e.user_message()),
        };
        if r.two_step_verification_data.is_some() || r.identity_verification_login_ticket.is_some() {
            return self.fail(seq, USE_WINDOW);
        }
        let cookie = session.lock().take();
        match cookie {
            Some(c) if self.live(seq) => self.validate_cookie(c).await,
            Some(_) => {}
            None => self.fail(seq, "Roblox didn't start a session. Try again."),
        }
    }
}
