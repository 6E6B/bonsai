//! Settings > Security: the password, two-step verification methods, recovery codes and passkeys.
//!
//! Quick changes (password, email codes, turning the authenticator off, removing a passkey) run
//! like any other write and report with a toast. Turning the authenticator on and making new
//! recovery codes take more than one step, so they show in a dialog driven by
//! [`StoreData::security`](crate::StoreData). Passwords and the setup token stay in core and are
//! dropped when the dialog closes; only the key to type into an authenticator app and the
//! recovery codes go to the UI, because the user has to see them.

use std::sync::atomic::{AtomicU64, Ordering};

use bonsai_api::generated::{auth, twostepverification};
use bonsai_api::{AccountId, Error, Secret};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::core::Inner;
use crate::res::{Key, NOT_SIGNED_IN, Res};

const INFO_AGE: i64 = 60;

/// Which two-step methods are on, and what else guards the account.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct SecurityInfo {
    pub email: bool,
    pub authenticator: bool,
    pub sms: bool,
    pub security_key: bool,
    /// Unused recovery codes; None when Roblox didn't say.
    pub recovery_codes: Option<u32>,
    /// When the recovery codes were made (ISO date as Roblox sends it).
    pub recovery_created: Option<String>,
    /// Passkey names, as Roblox lists them.
    pub passkeys: Vec<String>,
}

/// What a security dialog is doing.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SecurityTask {
    /// Turning on codes from an authenticator app.
    Authenticator,
    /// Replacing the recovery codes.
    RecoveryCodes,
}

/// Where a security dialog is.
#[derive(Clone, Debug, PartialEq)]
pub enum SecurityStep {
    /// Asking for the password (or checking it while `busy`).
    Password,
    /// Add this key to an authenticator app, then enter the code it shows.
    Key { manual_key: String },
    /// Finished. Recovery codes to write down, when Roblox made new ones.
    Done { codes: Vec<String> },
}

/// The dialog behind `security.authenticator_on` and `security.recovery_codes`.
pub struct SecurityFlow {
    /// Identifies this attempt in replies from the dialog.
    pub seq: u64,
    pub account: AccountId,
    pub task: SecurityTask,
    pub step: SecurityStep,
    pub busy: bool,
    pub error: Option<String>,
    password: Option<Secret>,
    setup_token: Option<Secret>,
}

static SEQ: AtomicU64 = AtomicU64::new(0);

/// A code from an authenticator app: six digits, spaces dropped.
pub fn normalize_totp(code: &str) -> Option<String> {
    let c: String = code.chars().filter(|c| !c.is_whitespace()).collect();
    (c.len() == 6 && c.chars().all(|c| c.is_ascii_digit())).then_some(c)
}

fn need_password(password: &str) -> Result<Secret, String> {
    if password.is_empty() {
        return Err("Enter your password.".into());
    }
    Ok(Secret::new(password))
}

fn info_from(
    config: Option<twostepverification::v1::UserConfiguration>,
    codes: Option<twostepverification::v1::RecoveryCodesStatusResponse>,
    passkeys: Option<auth::v1::ListPasskeyCredentialResponse>,
) -> SecurityInfo {
    use twostepverification::v1::UserConfigurationMethodMediaType as M;
    let mut info = SecurityInfo::default();
    for m in config.into_iter().flat_map(|c| c.methods.unwrap_or_default()) {
        let on = m.enabled == Some(true);
        match m.media_type {
            Some(M::Email) => info.email |= on,
            Some(M::Authenticator) => info.authenticator |= on,
            Some(M::Sms) => info.sms |= on,
            Some(M::SecurityKey) => info.security_key |= on,
            _ => {}
        }
    }
    if let Some(c) = codes {
        info.recovery_codes = c.active_count.map(|n| n.max(0) as u32);
        info.recovery_created = c.created.filter(|s| !s.is_empty());
    }
    info.passkeys = passkeys
        .and_then(|p| p.credentials)
        .unwrap_or_default()
        .into_iter()
        .filter_map(|c| c.nickname.filter(|n| !n.is_empty()))
        .collect();
    info
}

impl Core {
    /// Two-step methods, recovery codes and passkeys of one of your accounts.
    pub fn security(&self, account: AccountId) -> Res<SecurityInfo> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "security", ""),
            INFO_AGE,
            move |f| async move {
                let id = account.0 as i64;
                let params = twostepverification::v1::GetUsersByUserIdConfigurationParams::default();
                let config =
                    twostepverification::v1::get_users_by_user_id_configuration(&f.client, id, &params)
                        .await?;
                // The rest is extra; the section still shows the methods without it.
                let codes = twostepverification::v1::get_users_by_user_id_recovery_codes(&f.client, id)
                    .await
                    .ok();
                let body = auth::v1::ListPasskeysRequest { all: Some(true) };
                let passkeys = auth::v1::post_passkey_list_credentials(&f.client, &body)
                    .await
                    .ok();
                Ok(info_from(Some(config), codes, passkeys))
            },
        )
    }

    /// Change the password. Roblox signs out other sessions and hands this one a new cookie,
    /// which the client saves.
    pub fn change_password(&self, account: AccountId, current: &str, new: &str) -> Result<(), String> {
        let current = need_password(current)?;
        if new.chars().count() < 8 {
            return Err("Use at least 8 characters.".into());
        }
        if new == current.expose() {
            return Err("Pick a password you aren't using now.".into());
        }
        let new = Secret::new(new);
        self.inner.mutate(
            account,
            "change the password",
            "Password changed. Other sessions were signed out.",
            &["security", "sessions"],
            move |f| async move {
                let body = auth::v2::PasswordChangeModel {
                    current_password: Some(current.expose().to_owned()),
                    new_password: Some(new.expose().to_owned()),
                    ..Default::default()
                };
                auth::v2::post_user_passwords_change(&f.client, &body).await?;
                Ok(())
            },
        );
        Ok(())
    }

    /// Turn email codes on or off.
    pub fn set_email_codes(&self, account: AccountId, on: bool, password: &str) -> Result<(), String> {
        let password = need_password(password)?;
        let (what, done) = if on {
            ("turn on email codes", "Email codes are on.")
        } else {
            ("turn off email codes", "Email codes are off.")
        };
        self.inner
            .mutate(account, what, done, &["security"], move |f| async move {
                let id = account.0 as i64;
                let pw = Some(password.expose().to_owned());
                if on {
                    let body = twostepverification::v1::EnableTwoStepVerificationRequest {
                        password: pw,
                        ..Default::default()
                    };
                    twostepverification::v1::post_users_by_user_id_configuration_email_enable(
                        &f.client, id, &body,
                    )
                    .await?;
                } else {
                    let body = twostepverification::v1::DisableTwoStepVerificationRequest {
                        password: pw,
                        ..Default::default()
                    };
                    twostepverification::v1::post_users_by_user_id_configuration_email_disable(
                        &f.client, id, &body,
                    )
                    .await?;
                }
                Ok(())
            });
        Ok(())
    }

    /// Stop asking for authenticator app codes.
    pub fn authenticator_off(&self, account: AccountId, password: &str) -> Result<(), String> {
        let password = need_password(password)?;
        self.inner.mutate(
            account,
            "turn off the authenticator app",
            "Authenticator app codes are off.",
            &["security"],
            move |f| async move {
                let body = twostepverification::v1::DisableTwoStepVerificationRequest {
                    password: Some(password.expose().to_owned()),
                    ..Default::default()
                };
                twostepverification::v1::post_users_by_user_id_configuration_authenticator_disable(
                    &f.client,
                    account.0 as i64,
                    &body,
                )
                .await?;
                Ok(())
            },
        );
        Ok(())
    }

    /// Remove a passkey by the name Roblox lists it under.
    pub fn remove_passkey(&self, account: AccountId, name: &str) -> Result<(), String> {
        let name = name.trim().to_owned();
        if name.is_empty() {
            return Err("Name the passkey to remove.".into());
        }
        if let Some(info) = self.security(account).data
            && !info.passkeys.contains(&name)
        {
            return Err("No passkey has that name.".into());
        }
        self.inner.mutate(
            account,
            "remove the passkey",
            "Passkey removed.",
            &["security"],
            move |f| async move {
                let body = auth::v1::DeletePasskeysRequest {
                    credential_nicknames: Some(vec![name]),
                };
                auth::v1::post_passkey_delete_credential_batch(&f.client, &body).await?;
                Ok(())
            },
        );
        Ok(())
    }

    /// Open the dialog for `task` at the password step.
    pub fn security_open(&self, account: AccountId, task: SecurityTask) -> Result<u64, String> {
        if self.inner.client(account).filter(|c| c.signed_in()).is_none() {
            return Err(NOT_SIGNED_IN.into());
        }
        let seq = SEQ.fetch_add(1, Ordering::Relaxed) + 1;
        self.inner.store.write().security = Some(SecurityFlow {
            seq,
            account,
            task,
            step: SecurityStep::Password,
            busy: false,
            error: None,
            password: None,
            setup_token: None,
        });
        self.inner.repaint();
        Ok(seq)
    }

    /// The password step: starts the authenticator setup, or makes new recovery codes.
    pub fn security_password(&self, seq: u64, password: &str) -> Result<(), String> {
        let password = need_password(password)?;
        let (account, task) = self.flow_at(seq, |s| matches!(s, SecurityStep::Password))?;
        let client = self.signed_in(account)?;
        self.inner.update_security(seq, |p| {
            p.busy = true;
            p.error = None;
            p.password = Some(password.clone());
        });
        let inner = self.inner.clone();
        self.inner.rt.spawn(async move {
            let id = account.0 as i64;
            let pw = Some(password.expose().to_owned());
            match task {
                SecurityTask::Authenticator => {
                    let body = twostepverification::v1::EnableTwoStepVerificationRequest {
                        password: pw,
                        ..Default::default()
                    };
                    let r =
                        twostepverification::v1::post_users_by_user_id_configuration_authenticator_enable(
                            &client, id, &body,
                        )
                        .await;
                    let r = r.and_then(|r| match (r.manual_entry_key, r.setup_token) {
                        (Some(k), Some(t)) if !k.is_empty() && !t.is_empty() => Ok((k, t)),
                        _ => Err(Error::Message("Roblox didn't send a key. Try again.".into())),
                    });
                    inner.finish_security(seq, account, r, |p, (key, token)| {
                        p.setup_token = Some(Secret::new(token));
                        p.step = SecurityStep::Key { manual_key: key };
                    });
                }
                SecurityTask::RecoveryCodes => {
                    let body = twostepverification::v1::RegenerateRecoveryCodesRequest { password: pw };
                    let r = twostepverification::v1::post_users_by_user_id_recovery_codes_regenerate(
                        &client, id, &body,
                    )
                    .await;
                    inner.finish_security(seq, account, r, |p, r| {
                        p.password = None;
                        p.step = SecurityStep::Done {
                            codes: r.recovery_codes.unwrap_or_default(),
                        };
                    });
                }
            }
        });
        Ok(())
    }

    /// The code step of the authenticator setup. Turns it on when the code matches.
    pub fn security_code(&self, seq: u64, code: &str) -> Result<(), String> {
        let code = normalize_totp(code).ok_or_else(|| "Enter the 6-digit code from the app.".to_owned())?;
        let (account, _) = self.flow_at(seq, |s| matches!(s, SecurityStep::Key { .. }))?;
        let client = self.signed_in(account)?;
        let (password, token) = {
            let s = self.inner.store.read();
            let p = s.security.as_ref().filter(|p| p.seq == seq);
            (
                p.and_then(|p| p.password.clone()),
                p.and_then(|p| p.setup_token.clone()),
            )
        };
        let (Some(password), Some(token)) = (password, token) else {
            return Err("Start again: the setup was lost.".into());
        };
        self.inner.update_security(seq, |p| {
            p.busy = true;
            p.error = None;
        });
        let inner = self.inner.clone();
        self.inner.rt.spawn(async move {
            let body = twostepverification::v1::EnableVerifyAuthenticatorRequest {
                code: Some(code),
                password: Some(password.expose().to_owned()),
                setup_token: Some(token.expose().to_owned()),
                ..Default::default()
            };
            let r = twostepverification::v1::post_users_by_user_id_configuration_authenticator_enable_verify(
                &client,
                account.0 as i64,
                &body,
            )
            .await;
            inner.finish_security(seq, account, r, |p, r| {
                p.password = None;
                p.setup_token = None;
                p.step = SecurityStep::Done {
                    codes: r.recovery_codes.unwrap_or_default(),
                };
            });
        });
        Ok(())
    }

    /// Close the dialog and forget what it held.
    pub fn security_close(&self, seq: u64) {
        let gone = {
            let mut s = self.inner.store.write();
            match &s.security {
                Some(p) if p.seq == seq => s.security.take(),
                _ => None,
            }
        };
        if let Some(p) = gone {
            self.inner.res.invalidate(p.account, "security");
            self.inner.repaint();
        }
    }

    fn flow_at(
        &self,
        seq: u64,
        at: impl Fn(&SecurityStep) -> bool,
    ) -> Result<(AccountId, SecurityTask), String> {
        match &self.inner.store.read().security {
            Some(p) if p.seq == seq && !p.busy && at(&p.step) => Ok((p.account, p.task)),
            _ => Err("This dialog has closed.".into()),
        }
    }

    fn signed_in(&self, account: AccountId) -> Result<bonsai_api::Client, String> {
        self.inner
            .client(account)
            .filter(|c| c.signed_in())
            .ok_or_else(|| NOT_SIGNED_IN.to_owned())
    }
}

impl Inner {
    /// Change the security dialog if it is still showing attempt `seq`.
    fn update_security(&self, seq: u64, f: impl FnOnce(&mut SecurityFlow)) {
        let mut s = self.store.write();
        match s.security.as_mut() {
            Some(p) if p.seq == seq => f(p),
            _ => return,
        }
        drop(s);
        self.repaint();
    }

    fn finish_security<T>(
        &self,
        seq: u64,
        account: AccountId,
        r: Result<T, Error>,
        ok: impl FnOnce(&mut SecurityFlow, T),
    ) {
        if let Err(e) = &r {
            self.note_error(account, e);
        }
        self.update_security(seq, |p| {
            p.busy = false;
            match r {
                Ok(v) => ok(p, v),
                Err(e) => {
                    p.error = Some(match &e {
                        Error::Challenge(c) => crate::challenge::failed_message(c),
                        e => e.user_message(),
                    })
                }
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use twostepverification::v1::{
        UserConfiguration, UserConfigurationMethod, UserConfigurationMethodMediaType as M,
    };

    #[test]
    fn totp_codes_are_six_digits() {
        assert_eq!(normalize_totp("123 456"), Some("123456".into()));
        assert_eq!(normalize_totp("12345"), None);
        assert_eq!(normalize_totp("12345a"), None);
    }

    #[test]
    fn info_reads_enabled_methods_and_skips_the_rest() {
        let method = |m, on| UserConfigurationMethod {
            enabled: Some(on),
            media_type: Some(m),
            ..Default::default()
        };
        let config = UserConfiguration {
            methods: Some(vec![
                method(M::Email, true),
                method(M::Authenticator, false),
                method(M::Passkey, true),
            ]),
            ..Default::default()
        };
        let codes = twostepverification::v1::RecoveryCodesStatusResponse {
            active_count: Some(-1),
            created: Some(String::new()),
        };
        let passkeys = auth::v1::ListPasskeyCredentialResponse {
            credentials: Some(vec![
                auth::v1::PasskeyCredential {
                    nickname: Some("Laptop".into()),
                },
                auth::v1::PasskeyCredential { nickname: None },
            ]),
        };
        let info = info_from(Some(config), Some(codes), Some(passkeys));
        assert!(info.email && !info.authenticator && !info.sms);
        assert_eq!(info.recovery_codes, Some(0));
        assert_eq!(info.recovery_created, None);
        assert_eq!(info.passkeys, vec!["Laptop".to_owned()]);
        assert_eq!(info_from(None, None, None), SecurityInfo::default());
    }
}
