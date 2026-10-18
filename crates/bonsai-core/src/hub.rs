//! Hub orchestration. Public catalogs are independent of the selected Roblox account.
//! Only the hub transport performs HTTP; secrets stay in Rust and the OS credential manager.
use crate::{Core, themes::Theme};
use base64::{Engine, engine::general_purpose::URL_SAFE_NO_PAD};
use bonsai_api::Secret;
use bonsai_hub_api::{Client, Kind, Metadata, Page, Query, Submission, User};
use parking_lot::{Mutex, RwLock};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use std::sync::{
    Arc,
    atomic::{AtomicU64, Ordering},
};

#[derive(Clone, Default)]
pub struct Catalog {
    pub page: Page,
    pub loading: bool,
    pub loaded: bool,
    pub error: Option<String>,
}
#[derive(Default)]
pub struct State {
    pub histories: HashMap<String, History>,
    pub catalogs: HashMap<String, Catalog>,
    pub user: Option<User>,
    pub signing_in: bool,
    pub busy: bool,
    pub error: Option<String>,
    pub submissions: Vec<Submission>,
    pub revoked: Vec<String>,
    pub session_loaded: bool,
}
pub(crate) struct Hub {
    pub client: Option<Client>,
    pub state: RwLock<State>,
    session: Mutex<Option<Secret>>,
    auth_seq: AtomicU64,
    keychain_io: Mutex<()>,
}
impl Hub {
    pub fn new(limiter: Arc<bonsai_api::RateLimiter>) -> Self {
        let origin = std::env::var("BONSAI_HUB_URL").ok();
        let client = configured_client(origin.as_deref(), limiter);
        Self {
            client,
            state: Default::default(),
            session: Default::default(),
            auth_seq: AtomicU64::new(0),
            keychain_io: Default::default(),
        }
    }
    fn keychain(&self) -> Result<keyring::Entry, String> {
        let client = self
            .client
            .as_ref()
            .ok_or("Invalid hub configuration. Check BONSAI_HUB_URL and restart Bonsai.")?;
        let account: String = Sha256::digest(client.origin().as_bytes())
            .iter()
            .map(|b| format!("{b:02x}"))
            .collect();
        keyring::Entry::new("bonsai-hub-session", &account)
            .map_err(|_| "OS credential manager is unavailable.".into())
    }
    fn credentials(&self) -> Result<(Client, Secret), String> {
        Ok((
            self.client.clone().ok_or("Hub is not configured.")?,
            self.session
                .lock()
                .clone()
                .ok_or("Sign into Bonsai to publish.")?,
        ))
    }
}
pub fn query_key(q: &Query) -> String {
    serde_json::to_string(q).unwrap_or_default()
}
#[derive(Deserialize)]
pub struct Publish {
    pub local_id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub version: String,
    pub hub_id: String,
}
#[derive(Deserialize, Serialize)]
pub struct Target {
    pub kind: Kind,
    pub id: String,
    pub version: String,
}

impl Core {
    pub fn hub_catalog(self: &Arc<Self>, query: Query) -> Catalog {
        let key = query_key(&query);
        let should_load = {
            let mut state = self.inner.hub.state.write();
            if state.catalogs.len() > 32 && !state.catalogs.contains_key(&key) {
                state.catalogs.retain(|_, c| c.loading);
            }
            let c = state.catalogs.entry(key.clone()).or_default();
            let load = !c.loading && !c.loaded;
            if load {
                c.loading = true;
            }
            load
        };
        if should_load {
            let core = self.clone();
            let task_key = key.clone();
            self.inner.rt.spawn(async move {
                let result = match &core.inner.hub.client {
                    Some(client) => client.browse(&query).await,
                    None => Err("Invalid hub configuration. Check BONSAI_HUB_URL and restart Bonsai.".into()),
                };
                let mut state = core.inner.hub.state.write();
                let catalog = state.catalogs.entry(task_key).or_default();
                catalog.loading = false;
                catalog.loaded = true;
                match result {
                    Ok(page) => {
                        catalog.page = page;
                        catalog.error = None;
                    }
                    Err(e) => catalog.error = Some(e),
                };
                drop(state);
                core.inner.repaint();
            });
            self.load_hub_session();
        }
        self.inner
            .hub
            .state
            .read()
            .catalogs
            .get(&key)
            .cloned()
            .unwrap_or_default()
    }
    pub fn refresh_hub(self: &Arc<Self>) {
        self.inner.hub.state.write().histories.retain(|_, h| h.loading);
        for catalog in self.inner.hub.state.write().catalogs.values_mut() {
            if !catalog.loading {
                catalog.loaded = false;
            }
        }
        self.inner.repaint();
    }
    fn load_hub_session(self: &Arc<Self>) {
        {
            let mut state = self.inner.hub.state.write();
            if state.session_loaded {
                return;
            }
            state.session_loaded = true;
        }
        let Some(client) = self.inner.hub.client.clone() else {
            return;
        };
        let seq = self.inner.hub.auth_seq.load(Ordering::SeqCst);
        let core = self.clone();
        self.inner.rt.spawn(async move {
            let reader = core.clone();
            let stored = core
                .inner
                .rt
                .spawn_blocking(move || {
                    let _io = reader.inner.hub.keychain_io.lock();
                    // An explicit local-only development session. Never read this for remote hubs.
                    let local = reader.inner.hub.client.as_ref().is_some_and(|c| {
                        c.origin().starts_with("http://127.0.0.1:")
                            || c.origin().starts_with("http://localhost:")
                    });
                    if local && let Ok(value) = std::env::var("BONSAI_HUB_SESSION") {
                        return Some(value);
                    }
                    reader.inner.hub.keychain().ok()?.get_password().ok()
                })
                .await
                .ok()
                .flatten();
            if let Some(stored) = stored {
                let secret = Secret::new(stored);
                if let Ok(user) = client.me(&secret).await {
                    let mut state = core.inner.hub.state.write();
                    if core.inner.hub.auth_seq.load(Ordering::SeqCst) != seq {
                        return;
                    }
                    *core.inner.hub.session.lock() = Some(secret);
                    state.user = Some(user);
                    drop(state);
                    core.inner.repaint();
                }
            }
        });
    }
    pub fn hub_sign_in(self: &Arc<Self>) -> Result<(), String> {
        let client = self.inner.hub.client.clone().ok_or("Hub is not configured.")?;
        let seq = self.inner.hub.auth_seq.fetch_add(1, Ordering::SeqCst) + 1;
        {
            let mut state = self.inner.hub.state.write();
            if state.busy {
                return Err("A hub operation is still running.".into());
            }
            state.signing_in = true;
            state.error = None;
        }
        let core = self.clone();
        self.inner.rt.spawn(async move {
            let result: Result<(), String> = async {
                // Generate the transient PKCE verifier with the platform CSPRNG.
                let verifier = core
                    .inner
                    .rt
                    .spawn_blocking(|| {
                        let mut bytes = [0u8; 32];
                        // Use the platform CSPRNG through rustls' installed ring provider.
                        rustls_random(&mut bytes)?;
                        Ok::<_, String>(URL_SAFE_NO_PAD.encode(bytes))
                    })
                    .await
                    .map_err(|_| "Could not start secure sign-in.".to_owned())??;
                let challenge = URL_SAFE_NO_PAD.encode(Sha256::digest(verifier.as_bytes()));
                let flow = client.start_auth(&challenge).await?;
                let authorize = github_authorization_url(&flow.authorization_url)?;
                let url = authorize.to_string();
                core.inner
                    .rt
                    .spawn_blocking(move || {
                        open::that_detached(url).map_err(|_| "Could not open the system browser.".to_owned())
                    })
                    .await
                    .map_err(|_| "Could not open browser.".to_owned())??;
                for _ in 0..flow.expires_in.min(300) / 2 {
                    tokio::time::sleep(std::time::Duration::from_secs(2)).await;
                    if core.inner.hub.auth_seq.load(Ordering::SeqCst) != seq {
                        return Ok(());
                    }
                    if let Some(session) = client.exchange(&flow.poll_token, &verifier).await? {
                        if session.expires_at <= crate::cache::now() as u64 {
                            return Err("Hub returned an expired session.".into());
                        }
                        let secret = Secret::new(&session.token);
                        let save = core.clone();
                        let saved_secret = secret.clone();
                        core.inner
                            .rt
                            .spawn_blocking(move || {
                                let _io = save.inner.hub.keychain_io.lock();
                                if save.inner.hub.auth_seq.load(Ordering::SeqCst) != seq {
                                    return Ok(());
                                }
                                save.inner
                                    .hub
                                    .keychain()?
                                    .set_password(saved_secret.expose())
                                    .map_err(|_| {
                                        "Could not save session in the OS credential manager.".to_owned()
                                    })
                            })
                            .await
                            .map_err(|_| "Could not save session.".to_owned())??;
                        if core.inner.hub.auth_seq.load(Ordering::SeqCst) != seq {
                            let _ = client.logout(&secret).await;
                            return Ok(());
                        }
                        *core.inner.hub.session.lock() = Some(secret);
                        core.inner.hub.state.write().user = Some(session.user);
                        return Ok(());
                    }
                }
                Err("Sign-in timed out. Try again.".into())
            }
            .await;
            if core.inner.hub.auth_seq.load(Ordering::SeqCst) == seq {
                let mut state = core.inner.hub.state.write();
                state.signing_in = false;
                state.error = result.err();
                if let Some(error) = state.error.as_ref() {
                    core.notify(error, true);
                }
            }
            core.inner.repaint();
        });
        Ok(())
    }
    pub fn hub_sign_out(self: &Arc<Self>) -> Result<(), String> {
        if self.inner.hub.state.read().busy {
            return Err("Wait for the current hub operation to finish.".into());
        }
        self.inner.hub.auth_seq.fetch_add(1, Ordering::SeqCst);
        let session = self.inner.hub.session.lock().take();
        {
            let mut state = self.inner.hub.state.write();
            state.user = None;
            state.signing_in = false;
            state.submissions.clear();
            state.busy = true;
        }
        let core = self.clone();
        self.inner.rt.spawn(async move {
            let clear = core.clone();
            let erased = core
                .inner
                .rt
                .spawn_blocking(move || {
                    let _io = clear.inner.hub.keychain_io.lock();
                    clear
                        .inner
                        .hub
                        .keychain()
                        .and_then(|entry| match entry.delete_credential() {
                            Ok(()) | Err(keyring::Error::NoEntry) => Ok(()),
                            Err(_) => {
                                Err("Could not remove saved session from the OS credential manager."
                                    .to_owned())
                            }
                        })
                })
                .await;
            if session.is_some()
                && let Ok(Err(error)) = erased
            {
                core.notify(&error, true);
            }
            if let (Some(client), Some(secret)) = (&core.inner.hub.client, session)
                && let Err(error) = client.logout(&secret).await
            {
                core.notify(&error, true);
            }
            core.inner.hub.state.write().busy = false;
            core.inner.repaint();
        });
        self.inner.repaint();
        Ok(())
    }
    pub fn hub_install(self: &Arc<Self>, target: Target) -> Result<(), String> {
        let client = self.inner.hub.client.clone().ok_or("Hub is not configured.")?;
        {
            let mut state = self.inner.hub.state.write();
            if state.busy {
                return Err("A hub operation is still running.".into());
            }
            state.busy = true;
            state.error = None;
        }
        let core = self.clone();
        self.inner.rt.spawn(async move {
            let result: Result<(), String> = async {
                let entry = client
                    .entry(target.kind, &target.id, Some(&target.version))
                    .await?;
                match entry.kind {
                    Kind::Themes => {
                        let document = entry.document.ok_or("Hub theme has no document.")?;
                        bonsai_hub_api::verify(document.as_bytes(), &entry.sha256)?;
                        let theme = Theme::parse(&document)?;
                        if theme.id != entry.id || theme.version != entry.version {
                            return Err("Theme identity differs from its listing.".into());
                        }
                        let installer = core.clone();
                        core.inner
                            .rt
                            .spawn_blocking(move || installer.stage_theme_import(&document))
                            .await
                            .map_err(|_| "Could not stage theme.".to_owned())??;
                    }
                    Kind::Plugins => {
                        if entry.api_version != Some(1) {
                            return Err("Plugin requires an unsupported API version.".into());
                        }
                        let revoked = client.revocations().await?;
                        if revoked
                            .iter()
                            .any(|r| r.id == entry.id && r.version == entry.version)
                        {
                            return Err("This plugin release was revoked.".into());
                        }
                        let bytes = client.download(&entry).await?;
                        let installer = core.clone();
                        let expected_id = entry.id;
                        let expected_version = entry.version;
                        core.inner
                            .rt
                            .spawn_blocking(move || {
                                let path = installer
                                    .inner
                                    .paths
                                    .plugins_dir()
                                    .join(format!("hub-{}.bonsaiplugin", fastrand::u64(..)));
                                std::fs::create_dir_all(installer.inner.paths.plugins_dir())
                                    .map_err(|_| "Could not create package staging folder.".to_owned())?;
                                std::fs::write(&path, bytes)
                                    .map_err(|_| "Could not stage package.".to_owned())?;
                                let result = installer.stage_plugin_install(&path.to_string_lossy(), false);
                                let _ = std::fs::remove_file(path);
                                result?;
                                let mismatch = installer.store().plugin_install.as_ref().is_none_or(|p| {
                                    p.manifest.id != expected_id || p.manifest.version != expected_version
                                });
                                if mismatch {
                                    installer.cancel_plugin_install();
                                    return Err("Plugin identity differs from its listing.".to_owned());
                                }
                                Ok(())
                            })
                            .await
                            .map_err(|_| "Could not stage plugin.".to_owned())??;
                    }
                    Kind::Unknown => return Err("Unknown hub content type.".into()),
                }
                Ok(())
            }
            .await;
            let mut state = core.inner.hub.state.write();
            state.busy = false;
            state.error = result.err();
            if let Some(error) = state.error.as_ref() {
                core.notify(error, true);
            }
            drop(state);
            core.inner.repaint();
        });
        self.inner.repaint();
        Ok(())
    }
    pub fn hub_publish_theme(self: &Arc<Self>, publish: Publish) -> Result<(), String> {
        let (client, session) = self.inner.hub.credentials()?;
        let mut theme = self.theme_or_shown(&publish.local_id)?;
        theme.id = publish.hub_id;
        theme.name = publish.name.clone();
        theme.version = publish.version;
        theme.check()?;
        let meta = Metadata {
            name: publish.name,
            description: publish.description,
            tags: publish.tags,
        };
        let document = serde_json::to_value(theme).map_err(|_| "Invalid theme.".to_owned())?;
        self.hub_job(async move {
            client
                .publish_theme(&meta, document, &session)
                .await
                .map(|_| "Theme published.".to_owned())
        })
    }
    pub fn hub_publish_plugin(self: &Arc<Self>, publish: Publish) -> Result<(), String> {
        let (client, session) = self.inner.hub.credentials()?;
        let meta = Metadata {
            name: publish.name,
            description: publish.description,
            tags: publish.tags,
        };
        let core = self.clone();
        self.hub_job(async move {
            let packer = core.clone();
            let bytes = core
                .inner
                .rt
                .spawn_blocking(move || {
                    let path = packer
                        .inner
                        .paths
                        .plugins_dir()
                        .join(format!("upload-{}.bonsaiplugin", fastrand::u64(..)));
                    packer.pack_plugin(&publish.local_id, &path)?;
                    let result =
                        std::fs::read(&path).map_err(|_| "Could not read packaged plugin.".to_owned());
                    let _ = std::fs::remove_file(path);
                    result
                })
                .await
                .map_err(|_| "Could not package plugin.".to_owned())??;
            client.publish_plugin(&meta, bytes, &session).await?;
            Ok("Plugin submitted for review. It is not public until approved.".into())
        })
    }
    pub fn hub_submissions(self: &Arc<Self>) -> Result<(), String> {
        let (client, session) = self.inner.hub.credentials()?;
        let core = self.clone();
        self.inner.rt.spawn(async move {
            match client.submissions(&session).await {
                Ok(rows) => core.inner.hub.state.write().submissions = rows,
                Err(e) => core.inner.hub.state.write().error = Some(e),
            }
            core.inner.repaint();
        });
        Ok(())
    }
    pub fn hub_unlist(self: &Arc<Self>, target: Target) -> Result<(), String> {
        let (client, session) = self.inner.hub.credentials()?;
        self.hub_job(async move {
            client
                .unlist(target.kind, &target.id, &session)
                .await
                .map(|_| "Unlisted.".into())
        })
    }
    pub fn hub_delete(self: &Arc<Self>, target: Target) -> Result<(), String> {
        let (client, session) = self.inner.hub.credentials()?;
        self.hub_job(async move {
            client
                .delete(target.kind, &target.id, &session)
                .await
                .map(|_| "Deleted hub content. The ID remains reserved.".into())
        })
    }
    pub fn hub_report(self: &Arc<Self>, target: Target, reason: String) -> Result<(), String> {
        let (client, session) = self.inner.hub.credentials()?;
        self.hub_job(async move {
            client
                .report(target.kind, &target.id, &reason, &session)
                .await
                .map(|_| "Report submitted.".into())
        })
    }
    fn hub_job(
        self: &Arc<Self>,
        job: impl std::future::Future<Output = Result<String, String>> + Send + 'static,
    ) -> Result<(), String> {
        {
            let mut s = self.inner.hub.state.write();
            if s.busy {
                return Err("A hub operation is still running.".into());
            }
            s.busy = true;
            s.error = None;
        }
        let core = self.clone();
        self.inner.rt.spawn(async move {
            let result = job.await;
            let mut s = core.inner.hub.state.write();
            s.busy = false;
            match result {
                Ok(message) => {
                    drop(s);
                    core.notify(&message, false);
                    core.refresh_hub();
                }
                Err(e) => {
                    core.notify(&e, true);
                    s.error = Some(e);
                    drop(s);
                }
            }
            core.inner.repaint();
        });
        self.inner.repaint();
        Ok(())
    }
    pub fn hub_check_revocations(self: &Arc<Self>) -> Result<(), String> {
        let client = self.inner.hub.client.clone().ok_or("Hub is not configured.")?;
        let inner = self.inner.clone();
        self.inner.rt.spawn(async move {
            check_revocations(inner, client).await;
        });
        Ok(())
    }
}
async fn check_revocations(inner: Arc<crate::core::Inner>, client: Client) {
    match client.revocations().await {
        Ok(releases) => {
            let result = crate::plugins::enforce_revocations(inner.clone(), releases).await;
            let warnings = inner.revoked_plugin_warnings();
            let mut state = inner.hub.state.write();
            state.revoked = warnings;
            if let Err(e) = result {
                state.error = Some(e);
            }
        }
        Err(e) => inner.hub.state.write().error = Some(e),
    }
    inner.repaint();
}

/// Refresh on startup and periodically. Cached revocations apply even without a connection.
pub(crate) async fn revocation_loop(inner: Arc<crate::core::Inner>) {
    let Some(client) = inner.hub.client.clone() else {
        return;
    };
    let mut tick = tokio::time::interval(std::time::Duration::from_secs(15 * 60));
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    loop {
        tick.tick().await;
        check_revocations(inner.clone(), client.clone()).await;
    }
}

fn rustls_random(bytes: &mut [u8]) -> Result<(), String> {
    rustls::crypto::ring::default_provider()
        .secure_random
        .fill(bytes)
        .map_err(|_| "Secure randomness is unavailable.".into())
}

#[derive(Clone)]
pub struct Snapshot {
    pub configured: bool,
    pub catalog: Catalog,
    pub user: Option<User>,
    pub busy: bool,
    pub signing_in: bool,
    pub error: Option<String>,
    pub submissions: Vec<Submission>,
    pub revoked: Vec<String>,
}
impl Core {
    pub fn hub_snapshot(self: &Arc<Self>, query: Query) -> Snapshot {
        let catalog = self.hub_catalog(query);
        let state = self.inner.hub.state.read();
        Snapshot {
            configured: self.inner.hub.client.is_some(),
            catalog,
            user: state.user.clone(),
            busy: state.busy,
            signing_in: state.signing_in,
            error: state.error.clone(),
            submissions: state.submissions.clone(),
            revoked: state.revoked.clone(),
        }
    }
}

#[derive(Clone, Default)]
pub struct History {
    pub loading: bool,
    pub loaded: bool,
    pub error: Option<String>,
    pub versions: Vec<bonsai_hub_api::Version>,
}
impl Core {
    pub fn hub_history(self: &Arc<Self>, target: Target) -> History {
        let key = format!("{}:{}", target.kind.path().unwrap_or("unknown"), target.id);
        let load = {
            let mut state = self.inner.hub.state.write();
            if state.histories.len() > 32 {
                state.histories.retain(|_, h| h.loading);
            }
            let history = state.histories.entry(key.clone()).or_default();
            let load = !history.loading && !history.loaded;
            if load {
                history.loading = true;
            }
            load
        };
        if load {
            let core = self.clone();
            let task_key = key.clone();
            self.inner.rt.spawn(async move {
                let result = match &core.inner.hub.client {
                    Some(client) => client.versions(target.kind, &target.id).await,
                    None => Err("Hub is not configured.".into()),
                };
                let mut state = core.inner.hub.state.write();
                let history = state.histories.entry(task_key).or_default();
                history.loading = false;
                history.loaded = true;
                match result {
                    Ok(versions) => history.versions = versions,
                    Err(e) => history.error = Some(e),
                }
                drop(state);
                core.inner.repaint();
            });
        }
        self.inner
            .hub
            .state
            .read()
            .histories
            .get(&key)
            .cloned()
            .unwrap_or_default()
    }
}

fn configured_client(origin: Option<&str>, limiter: Arc<bonsai_api::RateLimiter>) -> Option<Client> {
    Client::new(origin.unwrap_or(bonsai_hub_api::DEFAULT_ORIGIN), limiter).ok()
}

fn github_authorization_url(raw: &str) -> Result<url::Url, String> {
    let authorize = url::Url::parse(raw).map_err(|_| "Invalid authorization URL.".to_owned())?;
    if authorize.scheme() != "https"
        || authorize.host_str() != Some("github.com")
        || authorize.path() != "/login/oauth/authorize"
        || !authorize.username().is_empty()
        || authorize.password().is_some()
        || authorize.port().is_some()
        || authorize.fragment().is_some()
    {
        return Err("Hub returned an untrusted authorization URL.".into());
    }
    Ok(authorize)
}

#[cfg(test)]
mod auth_url_tests {
    use super::{configured_client, github_authorization_url};
    use std::sync::Arc;

    #[test]
    fn defaults_to_live_hub_and_preserves_explicit_overrides() {
        let client = |origin| configured_client(origin, Arc::new(bonsai_api::RateLimiter::default()));
        assert_eq!(
            client(None).map(|c| c.origin()).as_deref(),
            Some("https://hub.6e6b.dev")
        );
        assert_eq!(
            client(Some("http://127.0.0.1:8787"))
                .map(|c| c.origin())
                .as_deref(),
            Some("http://127.0.0.1:8787")
        );
        assert!(client(Some("http://untrusted.test")).is_none());
        assert!(client(Some("")).is_none());
    }

    #[test]
    fn only_opens_github_authorization() {
        assert!(
            github_authorization_url("https://github.com/login/oauth/authorize?client_id=test&state=abc")
                .is_ok()
        );
        for url in [
            "http://github.com/login/oauth/authorize",
            "https://github.com.evil.test/login/oauth/authorize",
            "https://apis.roblox.com/oauth/v1/authorize",
            "https://github.com/login/device",
            "https://github.com:444/login/oauth/authorize",
            "https://user:password@github.com/login/oauth/authorize",
            "https://github.com/login/oauth/authorize#fragment",
        ] {
            assert!(github_authorization_url(url).is_err(), "{url}");
        }
    }
}
