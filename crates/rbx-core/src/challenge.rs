//! Challenges Roblox puts in front of a write: two-step verification and captcha.
//!
//! Roblox answers a write it wants confirmed with `rblx-challenge-*` headers. The transport
//! hands the challenge to [`Inner::ask_challenge`], which shows it to the user
//! (`StoreData::challenge`) and waits. When the user enters their code or solves the puzzle,
//! the answer goes to `/challenge/v1/continue` and the original request is sent again, so the
//! action finishes without the user starting it over. Cancelling fails the action.
//!
//! Nothing here solves anything by itself: codes come from the user, and the captcha is the
//! Arkose puzzle shown in a window for the user to solve.
//!
//! One challenge shows at a time. A second one that arrives meanwhile (another account, or a
//! request made while the dialog is open) fails at once instead of queueing behind it.
//!
//! The flow follows the site's Challenge.js (captured 2026-10-15). Not tried against a live
//! challenge yet: it needs an account that Roblox challenges.

use std::sync::Arc;
use std::time::Duration;

use rbx_api::generated::{apis, twostepverification};
use rbx_api::rt::ChallengeAnswer;
use rbx_api::{AccountId, Challenge, Error};
use serde_json::{Value, json};
use tokio::sync::oneshot;

use crate::Core;
use crate::core::Inner;

/// How long a challenge waits for the user before the action gives up.
const WAIT: Duration = Duration::from_secs(10 * 60);

/// Two-step verification error code for a wrong code.
const INVALID_CODE: i64 = 10;

const TWO_STEP: &str = "twostepverification";
const CAPTCHA: &str = "captcha";

/// Where a two-step code comes from. Other methods (security key, passkey, SMS) aren't offered;
/// Roblox's own page is the way for those.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CodeMethod {
    Authenticator,
    Email,
}

impl CodeMethod {
    pub fn key(self) -> &'static str {
        match self {
            CodeMethod::Authenticator => "authenticator",
            CodeMethod::Email => "email",
        }
    }
    pub fn from_key(s: &str) -> Option<CodeMethod> {
        match s {
            "authenticator" => Some(CodeMethod::Authenticator),
            "email" => Some(CodeMethod::Email),
            _ => None,
        }
    }
}

/// What the challenge dialog shows.
#[derive(Clone, Debug, PartialEq)]
pub enum PromptKind {
    TwoStep {
        method: CodeMethod,
        /// Methods the account has turned on that the dialog can use.
        methods: Vec<CodeMethod>,
        /// Roblox lets this device skip two-step for a while.
        remember_option: bool,
    },
    /// The puzzle is solved in its own window.
    Captcha,
}

/// The open challenge dialog.
#[derive(Clone, Debug, PartialEq)]
pub struct ChallengePrompt {
    /// Identifies this challenge in replies from the dialog.
    pub seq: u64,
    pub account: AccountId,
    pub kind: PromptKind,
    /// A verify or continue call is running.
    pub busy: bool,
    pub error: Option<String>,
    /// A neutral line, like "Code sent to your email."
    pub notice: Option<String>,
}

/// Two-step details from the challenge metadata.
#[derive(Clone, Debug, Default, PartialEq)]
struct TwoStepCtx {
    user_id: i64,
    /// The two-step challenge id (not the generic one).
    challenge_id: String,
    action_type: String,
}

/// Captcha details from the challenge metadata.
#[derive(Clone, Debug, Default, PartialEq)]
struct CaptchaCtx {
    unified_id: String,
    blob: String,
    action_type: String,
    public_key: String,
}

#[derive(Clone, Debug, PartialEq)]
enum Ctx {
    TwoStep(TwoStepCtx),
    Captcha(CaptchaCtx),
}

/// The challenge waiting for the user, with what answering it needs.
pub(crate) struct Pending {
    seq: u64,
    account: AccountId,
    /// The generic challenge id and type from the response headers.
    id: String,
    kind: String,
    ctx: Ctx,
    tx: Option<oneshot::Sender<Option<ChallengeAnswer>>>,
}

#[derive(Default)]
pub(crate) struct ChallengeState {
    seq: u64,
    pending: Option<Pending>,
}

/// The Arkose key name for a captcha action type, from the site's Captcha.js.
fn captcha_key_name(action_type: &str) -> Option<&'static str> {
    Some(match action_type {
        "Login" | "AppLogin" => "ACTION_TYPE_WEB_LOGIN",
        "Signup" => "ACTION_TYPE_WEB_SIGNUP",
        "JoinGroup" => "ACTION_TYPE_GROUP_JOIN",
        "GroupWallPost" => "ACTION_TYPE_GROUP_WALL_POST",
        "ResetPassword" => "ACTION_TYPE_WEB_RESET_PASSWORD",
        "FollowUser" => "ACTION_TYPE_FOLLOW_USER",
        "Generic" => "ACTION_TYPE_GENERIC_CHALLENGE",
        "GameCardRedeem" | "ToyCodeRedeem" => "ACTION_TYPE_WEB_GAMECARD_REDEMPTION",
        "AssetComment" => "ACTION_TYPE_ASSET_COMMENT",
        "Marketplace" => "ACTION_TYPE_MARKETPLACE",
        "Passive" => "ACTION_TYPE_PASSIVE",
        "SupportRequest" => "ACTION_TYPE_SUPPORT_REQUEST",
        _ => return None,
    })
}

fn text(v: &Value, key: &str) -> String {
    match &v[key] {
        Value::String(s) => s.clone(),
        Value::Number(n) => n.to_string(),
        _ => String::new(),
    }
}

fn two_step_ctx(meta: &Value, account: AccountId) -> TwoStepCtx {
    TwoStepCtx {
        user_id: text(meta, "userId").parse().unwrap_or(account.0 as i64),
        challenge_id: text(meta, "challengeId"),
        action_type: Some(text(meta, "actionType"))
            .filter(|a| !a.is_empty())
            .unwrap_or_else(|| "Generic".into()),
    }
}

/// The methods the dialog can offer, from the account's configuration, and the one to start
/// with. Authenticator comes first when Roblox doesn't say which is primary.
fn pick_methods(
    config: Option<&twostepverification::v1::UserConfiguration>,
) -> (CodeMethod, Vec<CodeMethod>) {
    use twostepverification::v1::UserConfigurationMethodMediaType as M;
    let Some(c) = config else {
        return (
            CodeMethod::Authenticator,
            vec![CodeMethod::Authenticator, CodeMethod::Email],
        );
    };
    let mut methods = Vec::new();
    for m in c.methods.iter().flatten().filter(|m| m.enabled == Some(true)) {
        let method = match m.media_type {
            Some(M::Authenticator) => CodeMethod::Authenticator,
            Some(M::Email) => CodeMethod::Email,
            _ => continue,
        };
        if !methods.contains(&method) {
            methods.push(method);
        }
    }
    methods.sort_by_key(|m| *m != CodeMethod::Authenticator);
    let primary = match c.primary_media_type {
        Some(0) => Some(CodeMethod::Email),
        Some(2) => Some(CodeMethod::Authenticator),
        _ => None,
    }
    .filter(|p| methods.contains(p));
    let first = primary
        .or(methods.first().copied())
        .unwrap_or(CodeMethod::Authenticator);
    (first, methods)
}

/// What the continue call and the retried request carry for a passed two-step challenge.
fn two_step_answer(ctx: &TwoStepCtx, token: &str, remember: bool) -> Value {
    json!({
        "verificationToken": token,
        "rememberDevice": remember,
        "challengeId": ctx.challenge_id,
        "actionType": ctx.action_type,
    })
}

/// What the continue call and the retried request carry for a solved captcha.
fn captcha_answer(ctx: &CaptchaCtx, token: &str) -> Value {
    json!({
        "unifiedCaptchaId": ctx.unified_id,
        "captchaToken": token,
        "actionType": ctx.action_type,
    })
}

/// The error text for an action that ended on a challenge the user didn't pass.
pub(crate) fn failed_message(c: &Challenge) -> String {
    if c.kind.eq_ignore_ascii_case(TWO_STEP) || c.kind.eq_ignore_ascii_case(CAPTCHA) {
        "Verification was cancelled.".into()
    } else {
        format!(
            "Roblox wants extra verification ({}). Do this once on the website, then try again.",
            c.kind
        )
    }
}

impl Inner {
    /// The transport's challenge hook. Shows the challenge and waits for the user's answer.
    pub(crate) async fn ask_challenge(
        self: Arc<Self>,
        account: AccountId,
        ch: Challenge,
    ) -> Option<ChallengeAnswer> {
        if self.challenge.lock().pending.is_some() {
            return None;
        }
        let client = self.client(account)?;
        let (ctx, kind) = match ch.kind.to_ascii_lowercase().as_str() {
            TWO_STEP => {
                let t = two_step_ctx(&ch.metadata, account);
                let action =
                    twostepverification::v1::RetractDialogRequestActionType::from_name(&t.action_type)
                        .value()
                        .map(|v| v as i32);
                let params = twostepverification::v1::GetUsersByUserIdConfigurationParams {
                    challenge_id: Some(t.challenge_id.clone()),
                    action_type: action,
                };
                let config =
                    twostepverification::v1::get_users_by_user_id_configuration(&client, t.user_id, &params)
                        .await
                        .ok();
                let (method, methods) = pick_methods(config.as_ref());
                if methods.is_empty() {
                    self.toast(
                        Some(account),
                        "Roblox wants two-step verification this app can't do (security key or SMS). Do it on the website.",
                        true,
                    );
                    return None;
                }
                let remember_option = ch.metadata["shouldShowRememberDeviceCheckbox"].as_bool() == Some(true);
                (
                    Ctx::TwoStep(t),
                    PromptKind::TwoStep {
                        method,
                        methods,
                        remember_option,
                    },
                )
            }
            CAPTCHA => {
                let action_type = text(&ch.metadata, "actionType");
                let keys = apis::undocumented::get_captcha_metadata(&self.anon).await.ok()?;
                let public_key = captcha_key_name(&action_type)
                    .and_then(|k| keys.fun_captcha_public_keys.as_ref()?.get(k).cloned())
                    .filter(|k| !k.is_empty())?;
                (
                    Ctx::Captcha(CaptchaCtx {
                        unified_id: text(&ch.metadata, "unifiedCaptchaId"),
                        blob: text(&ch.metadata, "dataExchangeBlob"),
                        action_type,
                        public_key,
                    }),
                    PromptKind::Captcha,
                )
            }
            _ => return None,
        };
        let (tx, rx) = oneshot::channel();
        let seq = {
            let mut st = self.challenge.lock();
            if st.pending.is_some() {
                return None;
            }
            st.seq += 1;
            st.pending = Some(Pending {
                seq: st.seq,
                account,
                id: ch.id.clone(),
                kind: ch.kind.clone(),
                ctx,
                tx: Some(tx),
            });
            st.seq
        };
        // Roblox emails the code itself when email is the account's main method.
        let notice = matches!(
            kind,
            PromptKind::TwoStep {
                method: CodeMethod::Email,
                ..
            }
        )
        .then(|| "Roblox sent a code to your email.".to_owned());
        self.store.write().challenge = Some(ChallengePrompt {
            seq,
            account,
            kind,
            busy: false,
            error: None,
            notice,
        });
        self.repaint();
        let answer = tokio::time::timeout(WAIT, rx)
            .await
            .ok()
            .and_then(Result::ok)
            .flatten();
        self.close_challenge(seq);
        answer
    }

    /// Forget challenge `seq` and close its dialog. Its action fails unless it was answered.
    fn close_challenge(&self, seq: u64) {
        let mut st = self.challenge.lock();
        if st.pending.as_ref().is_some_and(|p| p.seq == seq) {
            st.pending = None;
        }
        drop(st);
        let mut s = self.store.write();
        if s.challenge.as_ref().is_some_and(|c| c.seq == seq) {
            s.challenge = None;
        }
        drop(s);
        self.repaint();
    }

    /// Change the open dialog, if it is still `seq`.
    fn update_prompt(&self, seq: u64, f: impl FnOnce(&mut ChallengePrompt)) {
        let mut s = self.store.write();
        if let Some(p) = s.challenge.as_mut().filter(|p| p.seq == seq) {
            f(p);
        }
        drop(s);
        self.repaint();
    }

    fn pending_parts(&self, seq: u64) -> Option<(AccountId, String, String, Ctx)> {
        let st = self.challenge.lock();
        let p = st.pending.as_ref().filter(|p| p.seq == seq)?;
        Some((p.account, p.id.clone(), p.kind.clone(), p.ctx.clone()))
    }

    /// Tell Roblox the challenge was passed, then let the waiting request go again.
    async fn finish_challenge(
        &self,
        seq: u64,
        account: AccountId,
        id: &str,
        kind: &str,
        meta: Value,
    ) -> Result<(), Error> {
        let client = self
            .client(account)
            .ok_or_else(|| Error::Message(crate::res::NOT_SIGNED_IN.into()))?;
        let body = apis::undocumented::ChallengeContinueRequest {
            challenge_id: Some(id.to_owned()),
            challenge_type: Some(kind.to_owned()),
            challenge_metadata: Some(meta.to_string()),
        };
        apis::undocumented::post_challenge_continue(&client, &body).await?;
        let tx = {
            let mut st = self.challenge.lock();
            st.pending
                .as_mut()
                .filter(|p| p.seq == seq)
                .and_then(|p| p.tx.take())
        };
        if let Some(tx) = tx {
            let _ = tx.send(Some(ChallengeAnswer::new(id, kind, &meta)));
        }
        Ok(())
    }
}

impl Core {
    /// Check a two-step code from the challenge dialog. A wrong code shows in the dialog.
    pub fn challenge_submit_code(&self, seq: u64, code: &str, remember: bool) -> Result<(), String> {
        let code: String = code.chars().filter(|c| !c.is_whitespace()).collect();
        if code.len() != 6 || !code.chars().all(|c| c.is_ascii_digit()) {
            return Err("Enter the 6-digit code.".into());
        }
        let inner = self.inner.clone();
        let (account, id, kind, Ctx::TwoStep(ctx)) = inner
            .pending_parts(seq)
            .ok_or_else(|| "This check has closed.".to_owned())?
        else {
            return Err("This check has no code.".into());
        };
        let method = match &inner.store.read().challenge {
            Some(ChallengePrompt {
                kind: PromptKind::TwoStep { method, .. },
                ..
            }) => *method,
            _ => return Err("This check has closed.".into()),
        };
        inner.update_prompt(seq, |p| {
            p.busy = true;
            p.error = None;
        });
        let task = inner.clone();
        inner.rt.spawn(async move {
            let r = async {
                let client = task
                    .client(account)
                    .ok_or_else(|| Error::Message(crate::res::NOT_SIGNED_IN.into()))?;
                let body = twostepverification::v1::VerifyCodeRequest {
                    action_type: Some(
                        twostepverification::v1::RetractDialogRequestActionType::from_name(&ctx.action_type),
                    ),
                    challenge_id: Some(ctx.challenge_id.clone()),
                    code: Some(code),
                };
                let verified = match method {
                    CodeMethod::Authenticator => {
                        twostepverification::v1::post_users_by_user_id_challenges_authenticator_verify(
                            &client,
                            ctx.user_id,
                            &body,
                        )
                        .await
                    }
                    CodeMethod::Email => {
                        twostepverification::v1::post_users_by_user_id_challenges_email_verify(
                            &client,
                            ctx.user_id,
                            &body,
                        )
                        .await
                    }
                }
                .map_err(|e| match e.code() {
                    Some(INVALID_CODE) => {
                        Error::Message("That code didn't work. Check it and try again.".into())
                    }
                    _ => e,
                })?;
                let token = verified
                    .verification_token
                    .filter(|t| !t.is_empty())
                    .ok_or_else(|| Error::Message("Roblox didn't accept the code.".into()))?;
                task.finish_challenge(seq, account, &id, &kind, two_step_answer(&ctx, &token, remember))
                    .await
            }
            .await;
            if let Err(e) = r {
                task.update_prompt(seq, |p| {
                    p.busy = false;
                    p.error = Some(e.user_message());
                });
            }
        });
        Ok(())
    }

    /// Switch the dialog between authenticator and email. Picking email (again, to resend)
    /// asks Roblox to send a code.
    pub fn challenge_use_method(&self, seq: u64, method: &str) -> Result<(), String> {
        let method = CodeMethod::from_key(method).ok_or_else(|| "Unknown method.".to_owned())?;
        let inner = self.inner.clone();
        let (account, _, _, Ctx::TwoStep(ctx)) = inner
            .pending_parts(seq)
            .ok_or_else(|| "This check has closed.".to_owned())?
        else {
            return Err("This check has no code.".into());
        };
        inner.update_prompt(seq, |p| {
            if let PromptKind::TwoStep { method: m, .. } = &mut p.kind {
                *m = method;
            }
            p.error = None;
            p.notice = None;
        });
        if method != CodeMethod::Email {
            return Ok(());
        }
        inner.update_prompt(seq, |p| p.busy = true);
        let task = inner.clone();
        inner.rt.spawn(async move {
            let r = match task.client(account) {
                Some(c) => {
                    let body = twostepverification::v1::SendCodeRequest {
                        action_type: Some(
                            twostepverification::v1::RetractDialogRequestActionType::from_name(
                                &ctx.action_type,
                            ),
                        ),
                        challenge_id: Some(ctx.challenge_id.clone()),
                    };
                    twostepverification::v1::post_users_by_user_id_challenges_email_send_code(
                        &c,
                        ctx.user_id,
                        &body,
                    )
                    .await
                    .map(|_| ())
                }
                None => Err(Error::Message(crate::res::NOT_SIGNED_IN.into())),
            };
            task.update_prompt(seq, |p| {
                p.busy = false;
                match r {
                    Ok(()) => p.notice = Some("Code sent to your email.".into()),
                    Err(e) => p.error = Some(e.user_message()),
                }
            });
        });
        Ok(())
    }

    /// Close the dialog without answering. The action it held fails.
    pub fn challenge_cancel(&self, seq: u64) {
        let tx = {
            let mut st = self.inner.challenge.lock();
            st.pending
                .as_mut()
                .filter(|p| p.seq == seq)
                .and_then(|p| p.tx.take())
        };
        match tx {
            Some(tx) => {
                let _ = tx.send(None);
            }
            None => self.inner.close_challenge(seq),
        }
    }

    /// The Arkose public key and data blob for the captcha window of challenge `seq`.
    pub fn captcha_params(&self, seq: u64) -> Option<(String, String)> {
        match self.inner.pending_parts(seq)?.3 {
            Ctx::Captcha(c) => Some((c.public_key, c.blob)),
            Ctx::TwoStep(_) => None,
        }
    }

    /// The captcha window reported a solved puzzle.
    pub fn captcha_solved(&self, seq: u64, token: String) {
        let inner = self.inner.clone();
        let Some((account, id, kind, Ctx::Captcha(ctx))) = inner.pending_parts(seq) else {
            return;
        };
        inner.update_prompt(seq, |p| {
            p.busy = true;
            p.error = None;
        });
        let task = inner.clone();
        inner.rt.spawn(async move {
            let meta = captcha_answer(&ctx, &token);
            if let Err(e) = task.finish_challenge(seq, account, &id, &kind, meta).await {
                task.update_prompt(seq, |p| {
                    p.busy = false;
                    p.error = Some(e.user_message());
                });
            }
        });
    }

    /// The captcha window failed to load the puzzle.
    pub fn captcha_failed(&self, seq: u64, message: &str) {
        self.inner.update_prompt(seq, |p| {
            p.busy = false;
            p.error = Some(message.to_owned());
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use twostepverification::v1::{
        UserConfiguration, UserConfigurationMethod, UserConfigurationMethodMediaType as M,
    };

    fn method(m: M, enabled: bool) -> UserConfigurationMethod {
        UserConfigurationMethod {
            enabled: Some(enabled),
            media_type: Some(m),
            updated: None,
        }
    }

    #[test]
    fn methods_prefer_primary_then_authenticator() {
        let c = UserConfiguration {
            methods: Some(vec![
                method(M::Email, true),
                method(M::Authenticator, true),
                method(M::SecurityKey, true),
            ]),
            primary_media_type: None,
        };
        assert_eq!(
            pick_methods(Some(&c)),
            (
                CodeMethod::Authenticator,
                vec![CodeMethod::Authenticator, CodeMethod::Email]
            )
        );
        let c = UserConfiguration {
            primary_media_type: Some(0),
            ..c
        };
        assert_eq!(pick_methods(Some(&c)).0, CodeMethod::Email);
    }

    #[test]
    fn methods_skip_disabled_and_unsupported() {
        let c = UserConfiguration {
            methods: Some(vec![method(M::Authenticator, false), method(M::Email, true)]),
            primary_media_type: Some(2),
        };
        assert_eq!(
            pick_methods(Some(&c)),
            (CodeMethod::Email, vec![CodeMethod::Email])
        );
        let c = UserConfiguration {
            methods: Some(vec![method(M::SecurityKey, true)]),
            primary_media_type: None,
        };
        assert!(pick_methods(Some(&c)).1.is_empty());
        // No configuration: offer both and let Roblox say which works.
        assert_eq!(pick_methods(None).1.len(), 2);
    }

    #[test]
    fn two_step_metadata_round_trip() {
        let meta = json!({
            "userId": "123",
            "challengeId": "abc",
            "actionType": "Generic",
            "shouldShowRememberDeviceCheckbox": true,
        });
        let ctx = two_step_ctx(&meta, AccountId(9));
        assert_eq!(ctx.user_id, 123);
        assert_eq!(ctx.challenge_id, "abc");
        let a = two_step_answer(&ctx, "tok", true);
        assert_eq!(a["verificationToken"], "tok");
        assert_eq!(a["rememberDevice"], true);
        assert_eq!(a["challengeId"], "abc");
        assert_eq!(a["actionType"], "Generic");
        // Missing fields fall back to the account and the generic action.
        let ctx = two_step_ctx(&json!({}), AccountId(9));
        assert_eq!((ctx.user_id, ctx.action_type.as_str()), (9, "Generic"));
    }

    #[test]
    fn captcha_keys_follow_the_site() {
        assert_eq!(captcha_key_name("JoinGroup"), Some("ACTION_TYPE_GROUP_JOIN"));
        assert_eq!(captcha_key_name("Generic"), Some("ACTION_TYPE_GENERIC_CHALLENGE"));
        assert_eq!(captcha_key_name("Nope"), None);
        let ctx = CaptchaCtx {
            unified_id: "u".into(),
            blob: "b".into(),
            action_type: "JoinGroup".into(),
            public_key: "k".into(),
        };
        let a = captcha_answer(&ctx, "t");
        assert_eq!(
            a,
            json!({"unifiedCaptchaId": "u", "captchaToken": "t", "actionType": "JoinGroup"})
        );
    }
}
