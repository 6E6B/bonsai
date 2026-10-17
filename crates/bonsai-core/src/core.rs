//! The async bridge. A tokio runtime runs on background threads; the UI calls the non-async
//! methods here (which only take short locks or send on a channel) and renders from the
//! [`StoreData`]. Background work writes the store and calls `repaint`.

use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::Duration;

use bonsai_api::generated::{
    apis, economy, friends, games, notifications, presence, privatemessages, trades, users,
};
use bonsai_api::{AccountId, Base, Client, Error, RateLimiter, Secret};
use parking_lot::{Mutex, RwLock, RwLockReadGuard};
use tokio::sync::mpsc;

use crate::account::{AccountMeta, ColorTag, CookieStore, Paths, parse_cookie_input};
use crate::cache::{Cache, Kind};
use crate::launcher::{self, JoinTarget, Opener};
use crate::settings::Settings;
use crate::store::*;
use crate::thumbs::{self, DiskCache, ThumbKey, ThumbState};

pub type Repaint = Arc<dyn Fn() + Send + Sync>;

/// Everything Core needs from the outside world. Tests swap in fakes.
pub struct CoreConfig {
    pub paths: Paths,
    pub base: Base,
    pub cookies: Arc<dyn CookieStore>,
    pub opener: Arc<dyn Opener>,
    pub limiter: Arc<RateLimiter>,
    /// Start background loops (session health, unread polling). Off in tests.
    pub background: bool,
    /// How often quick sign-in asks whether its code was approved.
    pub quick_poll: Duration,
}

impl CoreConfig {
    pub fn standard() -> CoreConfig {
        let paths = Paths::standard();
        let settings = Settings::load(&paths.settings_file());
        CoreConfig {
            paths,
            base: Base::Production,
            cookies: Arc::new(crate::account::KeychainWithFallback::default()),
            opener: Arc::new(launcher::SystemOpener {
                command: settings.player_command,
            }),
            limiter: Arc::new(
                RateLimiter::default()
                    .with_host("presence.roblox.com", 1.0, 3.0)
                    .with_host("friends.roblox.com", 2.0, 6.0)
                    // Transactions answer 429 after about 8 quick calls; 1.5 s apart still
                    // tripped it. The sustained limit isn't known.
                    .with_route("apis.roblox.com", "/transaction-records/", 0.4, 4.0),
            ),
            background: true,
            // What the site uses.
            quick_poll: Duration::from_secs(5),
        }
    }
}

/// How long a universe that failed to load waits before it is asked for again.
const UNIVERSE_RETRY_SECS: i64 = 60;

/// Requests from the UI. Each runs as its own task.
#[derive(Debug, Clone)]
pub enum Cmd {
    ValidateCookie(Secret),
    ConfirmLogin,
    CancelLogin,
    RemoveAccount(AccountId),
    /// Ends the session on Roblox, then removes the account like `RemoveAccount`.
    SignOut(AccountId),
    Refresh(AccountId),
    LoadPlace {
        account: AccountId,
        place_id: u64,
    },
    LoadUniverses(Vec<u64>),
    LoadUsers(Vec<u64>),
    LoadServers {
        account: AccountId,
        place_id: u64,
        more: bool,
    },
    Launch {
        account: AccountId,
        target: JoinTarget,
    },
    Search {
        account: AccountId,
        query: String,
    },
    SetSettings(Box<Settings>),
    /// Writes a user theme to the themes folder.
    SaveTheme(Box<crate::themes::Theme>),
    DeleteTheme(String),
    /// Copies or downloads a background picture, then shows it.
    LoadBackground(crate::backgrounds::Source),
    /// Deletes background pictures that settings no longer name.
    PruneBackgrounds,
    /// Home game sorts as this account (anonymous works too).
    LoadHome(AccountId),
}

pub(crate) struct Inner {
    pub(crate) store: Arc<RwLock<StoreData>>,
    pub(crate) cache: Cache,
    pub(crate) disk: DiskCache,
    clients: RwLock<HashMap<AccountId, Client>>,
    pub(crate) anon: Client,
    cookies: Arc<dyn CookieStore>,
    opener: RwLock<Arc<dyn Opener>>,
    pub(crate) limiter: Arc<RateLimiter>,
    pub(crate) base: Base,
    repaint: Repaint,
    pub(crate) paths: Paths,
    pending_login: Mutex<Option<(Secret, UserInfo)>>,
    /// Bumped whenever an add-account flow starts or is cancelled; older flows stop.
    pub(crate) login_seq: Mutex<u64>,
    /// The quick sign-in code that is waiting, so cancelling can tell Roblox.
    pub(crate) quick: Mutex<Option<crate::signin::QuickSession>>,
    pub(crate) quick_poll: Duration,
    thumb_tx: mpsc::UnboundedSender<ThumbKey>,
    /// Which loaded thumbnails stay in memory.
    pub(crate) thumb_budget: Mutex<thumbs::Budget>,
    pub(crate) browser_tracker_id: u64,
    pub(crate) settings: RwLock<Settings>,
    pub(crate) themes: RwLock<crate::themes::Themes>,
    pub(crate) plugins: Mutex<crate::plugins::Plugins>,
    /// Whether the OS prefers dark, as the UI last reported. Picks the theme in System mode.
    pub(crate) system_dark: std::sync::atomic::AtomicBool,
    toast_seq: Mutex<u64>,
    /// The challenge waiting for the user, with what answering it needs.
    pub(crate) challenge: Mutex<crate::challenge::ChallengeState>,
    /// Desktop alerts waiting for the shell (see [`crate::alerts`]).
    pub(crate) alerts: Mutex<crate::alerts::AlertState>,
    search_seq: Mutex<u64>,
    /// The last query sent to remote search, so asking again every frame sends it once.
    search_last: Mutex<String>,
    /// Random per run; explore and search APIs want one.
    pub(crate) session_id: String,
    pub(crate) res: crate::res::Resources,
    /// Transaction ledgers loaded into memory, per account and scope.
    pub(crate) ledgers: crate::transactions::Ledgers,
    users_pending: Mutex<std::collections::HashSet<u64>>,
    /// Universes asked for and not yet cached, with when they were last asked for. Failed loads
    /// stay here so they are retried after a pause instead of on every frame.
    universes_pending: Mutex<HashMap<u64, i64>>,
    /// Last time a rate-limited fire-and-forget call ran, by key (typing status).
    pub(crate) throttle: Mutex<HashMap<String, i64>>,
    /// Theme files queued to write; the themes watcher waits while any are.
    pub(crate) theme_writes: std::sync::atomic::AtomicUsize,
    pub(crate) rt: tokio::runtime::Handle,
}

pub struct Core {
    pub(crate) inner: Arc<Inner>,
    tx: mpsc::UnboundedSender<Cmd>,
    rt: Option<tokio::runtime::Runtime>,
}

impl Drop for Core {
    fn drop(&mut self) {
        if let Some(rt) = self.rt.take() {
            rt.shutdown_background();
        }
    }
}

impl Core {
    pub fn start(cfg: CoreConfig, repaint: Repaint) -> Core {
        bonsai_api::transport::init_tls();
        let rt = tokio::runtime::Builder::new_multi_thread()
            .worker_threads(2)
            .thread_name("bonsai-core")
            .enable_all()
            .build()
            .unwrap_or_else(|e| panic!("cannot start the background runtime: {e}"));
        let cache = Cache::open(&cfg.paths.db_file())
            .or_else(|_| Cache::in_memory())
            .unwrap_or_else(|e| panic!("sqlite: {e}"));
        if let Some(o) = cache.local_offset() {
            crate::fmt::set_local_offset(o);
        }
        let settings = Settings::load(&cfg.paths.settings_file());
        let themes = crate::themes::Themes::load(&cfg.paths.themes_dir());
        let accounts = crate::account::AccountList::load(&cfg.paths.accounts_file());
        let anon = Client::builder(AccountId::ANONYMOUS)
            .base(cfg.base.clone())
            .limiter(cfg.limiter.clone())
            .build()
            .unwrap_or_else(|e| panic!("http client: {e}"));
        let (thumb_tx, thumb_rx) = mpsc::unbounded_channel();
        let mut store = StoreData {
            accounts,
            ..Default::default()
        };
        warm_store(&cache, &mut store);
        let inner = Arc::new(Inner {
            store: Arc::new(RwLock::new(store)),
            disk: DiskCache::new(&cfg.paths.thumbs_dir()),
            cache,
            clients: RwLock::new(HashMap::new()),
            anon,
            cookies: cfg.cookies,
            opener: RwLock::new(cfg.opener),
            limiter: cfg.limiter,
            base: cfg.base,
            repaint,
            paths: cfg.paths,
            pending_login: Mutex::new(None),
            login_seq: Mutex::new(0),
            quick: Mutex::new(None),
            quick_poll: cfg.quick_poll,
            thumb_tx,
            thumb_budget: Mutex::new(Default::default()),
            browser_tracker_id: 100_000_000_000 + fastrand::u64(0..899_999_999_999),
            settings: RwLock::new(settings),
            themes: RwLock::new(themes),
            plugins: Default::default(),
            system_dark: std::sync::atomic::AtomicBool::new(true),
            toast_seq: Mutex::new(0),
            challenge: Default::default(),
            alerts: Default::default(),
            search_seq: Mutex::new(0),
            search_last: Mutex::new(String::new()),
            session_id: random_uuid(),
            res: Default::default(),
            ledgers: Default::default(),
            users_pending: Mutex::new(Default::default()),
            universes_pending: Mutex::new(HashMap::new()),
            throttle: Mutex::new(HashMap::new()),
            theme_writes: Default::default(),
            rt: rt.handle().clone(),
        });
        inner.load_clients();
        crate::plugins::load(&inner);
        {
            let weak = Arc::downgrade(&inner);
            inner.limiter.set_on_block(move |host, wait| {
                if let Some(inner) = weak.upgrade() {
                    inner.note_rate_limit(host, wait);
                }
            });
        }
        let (tx, mut rx) = mpsc::unbounded_channel::<Cmd>();
        {
            let inner = inner.clone();
            rt.spawn(async move {
                while let Some(cmd) = rx.recv().await {
                    let inner = inner.clone();
                    tokio::spawn(async move { inner.handle(cmd).await });
                }
            });
        }
        rt.spawn(thumb_worker(inner.clone(), thumb_rx));
        if cfg.background {
            rt.spawn(health_loop(inner.clone()));
            rt.spawn(tidy_loop(inner.clone()));
            rt.spawn(crate::chat::realtime_loop(inner.clone()));
            rt.spawn(crate::themes::watch_loop(inner.clone()));
            rt.spawn(crate::plugins::watch_loop(inner.clone()));
        }
        let core = Core {
            inner,
            tx,
            rt: Some(rt),
        };
        // First refresh: active account first, then the others.
        let ids: Vec<AccountId> = {
            let s = core.inner.store.read();
            let mut v: Vec<AccountId> = s.accounts.active.into_iter().collect();
            v.extend(
                s.accounts
                    .accounts
                    .iter()
                    .map(|a| a.id)
                    .filter(|id| Some(*id) != s.accounts.active),
            );
            v
        };
        for id in ids {
            core.send(Cmd::Refresh(id));
        }
        core
    }

    /// Another handle to the same core, for work that finishes later on a task.
    pub(crate) fn handle(&self) -> Core {
        Core {
            inner: self.inner.clone(),
            tx: self.tx.clone(),
            rt: None,
        }
    }

    pub fn send(&self, cmd: Cmd) {
        let _ = self.tx.send(cmd);
    }

    pub fn store(&self) -> RwLockReadGuard<'_, StoreData> {
        self.inner.store.read()
    }

    pub fn settings(&self) -> Settings {
        self.inner.settings.read().clone()
    }

    pub fn paths(&self) -> &Paths {
        &self.inner.paths
    }

    /// Paste flow step 1. Returns false if the text does not look like a cookie at all.
    pub fn submit_cookie(&self, text: &str) -> bool {
        match parse_cookie_input(text) {
            Some(c) => {
                self.inner.store.write().login = Some(LoginState::Validating);
                self.send(Cmd::ValidateCookie(c));
                true
            }
            None => {
                self.inner.store.write().login = Some(LoginState::Failed(
                    "That doesn't look like a .ROBLOSECURITY cookie. Copy the whole value.".into(),
                ));
                self.inner.repaint();
                false
            }
        }
    }

    /// Switch accounts: an in-memory change, rendered from cache on the next frame.
    pub fn switch(&self, id: AccountId) -> bool {
        let ok = {
            let mut s = self.inner.store.write();
            s.accounts.switch(id)
        };
        if ok {
            self.inner.save_accounts();
            self.inner.repaint();
            let stale = self
                .inner
                .store
                .read()
                .data
                .get(&id)
                .is_none_or(|d| d.friends.age_secs() > 60);
            if stale {
                self.send(Cmd::Refresh(id));
            }
        }
        ok
    }

    pub fn switch_last(&self) -> bool {
        let prev = self.inner.store.read().accounts.previous;
        prev.is_some_and(|p| self.switch(p))
    }

    pub fn rename(&self, id: AccountId, label: Option<String>) {
        if let Some(a) = self.inner.store.write().accounts.get_mut(id) {
            a.label = label.map(|l| l.trim().to_string()).filter(|l| !l.is_empty());
        }
        self.inner.save_accounts();
        self.inner.repaint();
    }

    pub fn set_color(&self, id: AccountId, c: ColorTag) {
        if let Some(a) = self.inner.store.write().accounts.get_mut(id) {
            a.color = c;
        }
        self.inner.save_accounts();
        self.inner.repaint();
    }

    pub fn reorder(&self, id: AccountId, to: usize) {
        self.inner.store.write().accounts.reorder(id, to);
        self.inner.save_accounts();
        self.inner.repaint();
    }

    /// Load Home sorts for `account` if missing or older than ten minutes. Cheap every frame.
    pub fn ensure_home(&self, account: AccountId) {
        {
            let mut s = self.inner.store.write();
            let h = &mut s.data.entry(account).or_default().home;
            let fresh = h.data.is_some() && h.age_secs() < 600;
            if h.loading || fresh || (h.error.is_some() && h.age_secs() < 60) {
                return;
            }
            h.start();
        }
        self.send(Cmd::LoadHome(account));
    }

    /// Load an experience's details by place id if unknown, or refresh them every ten minutes.
    /// Cheap every frame.
    pub fn ensure_place(&self, account: AccountId, place_id: u64) {
        let known = {
            let s = self.inner.store.read();
            s.place_universe
                .get(&place_id)
                .is_some_and(|u| s.games.contains_key(u))
        };
        if self
            .inner
            .due(&format!("place:{place_id}"), if known { 600 } else { 30 })
        {
            self.send(Cmd::LoadPlace { account, place_id });
        }
    }

    /// Search Roblox for `query` (with its scope prefix) unless that query was the last one
    /// asked. Debounced; results land in `store.remote`. Cheap every frame.
    pub fn ensure_remote_search(&self, account: AccountId, query: &str) {
        let q = query.trim();
        {
            let mut last = self.inner.search_last.lock();
            if *last == q {
                return;
            }
            *last = q.to_owned();
        }
        self.send(Cmd::Search {
            account,
            query: q.to_owned(),
        });
    }

    /// Load a place's public servers if missing or older than two minutes. Cheap every frame.
    pub fn ensure_servers(&self, account: AccountId, place_id: u64) {
        let fresh = self
            .inner
            .store
            .read()
            .servers
            .get(&place_id)
            .is_some_and(|l| l.loading || (l.data.is_some() && l.age_secs() < 120));
        if !fresh && self.inner.due(&format!("servers:{place_id}"), 30) {
            self.send(Cmd::LoadServers {
                account,
                place_id,
                more: false,
            });
        }
    }

    /// Ask for user names; cheap to call every frame for visible rows.
    pub fn want_users(&self, ids: &[u64]) {
        let missing: Vec<u64> = {
            let s = self.inner.store.read();
            let pending = self.inner.users_pending.lock();
            ids.iter()
                .filter(|i| **i > 0 && !s.users.contains_key(i) && !pending.contains(i))
                .copied()
                .collect()
        };
        if missing.is_empty() {
            return;
        }
        self.inner.users_pending.lock().extend(missing.iter().copied());
        self.send(Cmd::LoadUsers(missing));
    }

    /// Ask for game details by universe id; cheap to call every frame for visible rows.
    pub fn want_universes(&self, ids: &[u64]) {
        let t = crate::cache::now();
        let missing: Vec<u64> = {
            let s = self.inner.store.read();
            let mut pending = self.inner.universes_pending.lock();
            ids.iter()
                .filter(|u| **u > 0 && !s.games.contains_key(u))
                .filter(|u| {
                    if pending
                        .get(u)
                        .is_some_and(|asked| t - asked < UNIVERSE_RETRY_SECS)
                    {
                        return false;
                    }
                    pending.insert(**u, t);
                    true
                })
                .copied()
                .collect()
        };
        if !missing.is_empty() {
            self.send(Cmd::LoadUniverses(missing));
        }
    }

    /// Ask for thumbnails; cheap to call every frame for visible rows.
    pub fn want_thumbs(&self, keys: &[ThumbKey]) {
        let missing: Vec<ThumbKey> = {
            let s = self.inner.store.read();
            keys.iter()
                .filter(|k| !s.thumbs.contains_key(k))
                .copied()
                .collect()
        };
        if missing.is_empty() {
            return;
        }
        let mut s = self.inner.store.write();
        for k in missing {
            if let std::collections::hash_map::Entry::Vacant(e) = s.thumbs.entry(k) {
                e.insert(ThumbState::Loading);
                let _ = self.inner.thumb_tx.send(k);
            }
        }
    }

    /// Show a short message in the corner.
    pub fn notify(&self, text: &str, error: bool) {
        self.inner.toast(None, text, error);
    }

    /// Opens a Roblox page in the default browser. Anything else is refused.
    pub fn open_web(&self, url: &str) -> Result<(), String> {
        if !crate::links::is_roblox_url(url) {
            return Err("Not a Roblox link.".into());
        }
        let opener = self.inner.opener.read().clone();
        opener.open_web(url)
    }

    pub fn dismiss_toast(&self, id: u64) {
        self.inner.store.write().toasts.retain(|t| t.id != id);
        self.inner.repaint();
    }

    pub fn update_settings(&self, f: impl FnOnce(&mut Settings)) {
        let s = {
            let mut s = self.inner.settings.write();
            f(&mut s);
            s.clone()
        };
        self.send(Cmd::SetSettings(Box::new(s)));
    }

    /// Deletes cached data past the limits in Settings > Data now, in the background, instead
    /// of at the next tidy.
    pub fn tidy_soon(&self) {
        let inner = self.inner.clone();
        self.inner.rt.spawn_blocking(move || inner.tidy());
    }

    /// Writes the settings to `path` for another computer (see [`crate::prefs::export`]).
    pub fn export_settings(&self, path: &std::path::Path) -> Result<(), String> {
        std::fs::write(path, crate::prefs::export(&self.settings()))
            .map_err(|e| format!("Can't save {}: {e}", path.display()))
    }

    /// Takes the settings from a file `export_settings` wrote (see [`crate::prefs::import`]). Returns
    /// how many changed.
    pub fn import_settings(&self, path: &std::path::Path) -> Result<usize, String> {
        let text =
            std::fs::read_to_string(path).map_err(|e| format!("Can't read {}: {e}", path.display()))?;
        let mut s = self.settings();
        let n = crate::prefs::import(&mut s, &text)?;
        if n > 0 {
            self.update_settings(|cur| *cur = s);
            self.tidy_soon();
        }
        Ok(n)
    }

    /// The active account is in a game, as of its last presence refresh.
    pub fn playing(&self) -> bool {
        let s = self.inner.store.read();
        s.accounts
            .active
            .and_then(|a| s.data.get(&a)?.presence.get(&a.0))
            .is_some_and(|p| p.kind == PresenceKind::InGame)
    }

    /// The account that does something: the picked one, else the active one.
    pub fn acting(&self, picked: Option<AccountId>) -> Option<AccountId> {
        picked.or(self.inner.store.read().accounts.active)
    }
}

/// Fill the store from SQLite so the first frame has data.
fn warm_store(cache: &Cache, s: &mut StoreData) {
    for (id, u) in cache.all::<UserInfo>(Kind::User, AccountId::ANONYMOUS) {
        if let Ok(id) = id.parse() {
            s.users.insert(id, u.value);
        }
    }
    for (id, g) in cache.all::<GameInfo>(Kind::Universe, AccountId::ANONYMOUS) {
        if let Ok(id) = id.parse::<u64>() {
            s.place_universe.insert(g.value.root_place_id, id);
            s.games.insert(id, g.value);
        }
    }
    for (id, u) in cache.all::<u64>(Kind::Place, AccountId::ANONYMOUS) {
        if let Ok(p) = id.parse() {
            s.place_universe.insert(p, u.value);
        }
    }
    let ids: Vec<AccountId> = s.accounts.accounts.iter().map(|a| a.id).collect();
    for id in ids {
        let d = s.data.entry(id).or_default();
        if let Some(r) = cache.get::<i64>(Kind::Robux, id, "robux") {
            d.robux = Some(r.value);
        }
        if let Some(u) = cache.get::<Unread>(Kind::Unread, id, "unread") {
            d.unread = u.value;
        }
        if let Some(f) = cache.get::<Vec<u64>>(Kind::Friends, id, "friends") {
            d.friends = Loadable::cached(f.value, f.fetched_at);
        }
        if let Some(p) = cache.get::<HashMap<u64, Presence>>(Kind::Friends, id, "presence") {
            d.presence = p.value;
        }
        if let Some(l) = cache.get::<HashMap<u64, i64>>(Kind::Local, id, crate::seen::LAST_SEEN) {
            d.last_seen = l.value;
        }
        if let Some(u) = cache.get::<HashMap<u64, i64>>(Kind::Local, id, crate::seen::UNFRIENDED) {
            d.unfriended = u.value;
        }
        if let Some(h) = cache.get::<Vec<HomeSort>>(Kind::Home, id, "sorts") {
            d.home = Loadable::cached(h.value, h.fetched_at);
        }
        if let Some(r) = cache.get::<Vec<u64>>(Kind::Recent, id, "recent") {
            d.recent = r.value;
        }
        if let Some(w) = cache.get::<Vec<crate::market::WishItem>>(Kind::Wishlist, id, "wishlist") {
            d.wishlist = w.value;
        }
    }
}

impl Inner {
    pub(crate) fn repaint(&self) {
        (self.repaint)();
    }

    /// True at most once per `secs` for `key`, so a view can ask on every frame.
    pub(crate) fn due(&self, key: &str, secs: i64) -> bool {
        let t = crate::cache::now();
        let mut th = self.throttle.lock();
        if th.get(key).is_some_and(|last| t - last < secs) {
            return false;
        }
        th.insert(key.to_owned(), t);
        true
    }

    fn load_clients(self: &Arc<Self>) {
        let ids: Vec<AccountId> = self.store.read().accounts.accounts.iter().map(|a| a.id).collect();
        for id in ids {
            match self.cookies.get(id) {
                Ok(Some(c)) => {
                    if let Ok(cl) = self.build_client(id, c) {
                        self.clients.write().insert(id, cl);
                    }
                }
                _ => {
                    self.store.write().data.entry(id).or_default().session = SessionState::NoCookie;
                }
            }
        }
    }

    fn build_client(self: &Arc<Self>, id: AccountId, cookie: Secret) -> Result<Client, Error> {
        let me = Arc::downgrade(self);
        let asker = me.clone();
        Client::builder(id)
            .cookie(cookie)
            .browser_tracker(self.browser_tracker_id)
            .base(self.base.clone())
            .limiter(self.limiter.clone())
            .on_cookie_rotated(move |acc, c| {
                if let Some(me) = me.upgrade() {
                    let _ = me.cookies.set(acc, c);
                }
            })
            .on_challenge(move |acc, ch| {
                let inner = asker.upgrade();
                async move { inner?.ask_challenge(acc, ch).await }
            })
            .build()
    }

    pub(crate) fn client(&self, id: AccountId) -> Option<Client> {
        if id.is_anonymous() {
            return Some(self.anon.clone());
        }
        self.clients.read().get(&id).cloned()
    }

    pub(crate) fn save_accounts(&self) {
        let list = self.store.read().accounts.clone();
        if let Err(e) = list.save(&self.paths.accounts_file()) {
            tracing::warn!("saving account list: {e}");
        }
    }

    pub(crate) fn toast(&self, account: Option<AccountId>, text: impl Into<String>, error: bool) {
        let id = {
            let mut n = self.toast_seq.lock();
            *n += 1;
            *n
        };
        let mut s = self.store.write();
        s.toasts.push(Toast {
            id,
            account,
            text: text.into(),
            error,
            host: None,
            retry_after_secs: None,
        });
        if s.toasts.len() > 5 {
            s.toasts.remove(0);
        }
        drop(s);
        self.repaint();
    }

    pub(crate) fn note_rate_limit(&self, host: &str, wait: Duration) {
        let secs = wait.as_secs_f64().ceil().max(1.0) as u32;
        let id = {
            let mut n = self.toast_seq.lock();
            *n += 1;
            *n
        };
        let mut s = self.store.write();
        if let Some(existing) = s.toasts.iter_mut().find(|t| t.host.as_deref() == Some(host)) {
            existing.id = id;
            existing.retry_after_secs = Some(secs);
        } else {
            s.toasts.push(Toast {
                id,
                account: None,
                text: format!("Rate limited ({host})"),
                error: false,
                host: Some(host.to_string()),
                retry_after_secs: Some(secs),
            });
            if s.toasts.len() > 5 {
                s.toasts.remove(0);
            }
        }
        drop(s);
        self.repaint();
    }

    /// Note an error from an account's request: 401 re-checks the session.
    pub(crate) fn note_error(&self, account: AccountId, e: &Error) {
        match e {
            Error::Unauthorized if !account.is_anonymous() => self.check_session(account),
            Error::Network(_) => self.store.write().offline = true,
            _ => {}
        }
    }

    /// Some endpoints answer 401 for reasons other than a dead cookie (a wrong route, a
    /// resource the account may not see). Only `users/authenticated` failing means the
    /// session is gone, so ask it before marking the account expired.
    fn check_session(&self, account: AccountId) {
        let expired = self
            .store
            .read()
            .account_data(account)
            .is_some_and(|d| d.session == SessionState::Expired);
        let Some(c) = self.client(account).filter(|_| !expired) else {
            return;
        };
        let store = self.store.clone();
        let repaint = self.repaint.clone();
        self.rt.spawn(async move {
            if let Err(Error::Unauthorized) = users::v1::get_users_authenticated(&c).await {
                store.write().data.entry(account).or_default().session = SessionState::Expired;
                repaint();
            }
        });
    }

    fn note_ok(&self) {
        let offline = self.store.read().offline;
        if offline {
            self.store.write().offline = false;
        }
    }

    async fn handle(self: Arc<Self>, cmd: Cmd) {
        self.store.write().jobs += 1;
        self.repaint();
        match cmd {
            Cmd::ValidateCookie(c) => self.validate_cookie(c).await,
            Cmd::ConfirmLogin => self.confirm_login().await,
            Cmd::CancelLogin => self.cancel_login(),
            Cmd::RemoveAccount(id) => self.remove_account(id),
            Cmd::SignOut(id) => self.sign_out(id).await,
            Cmd::Refresh(id) => self.refresh_account(id).await,
            Cmd::LoadPlace { account, place_id } => self.load_place(account, place_id).await,
            Cmd::LoadUniverses(ids) => {
                self.load_universes(&ids).await;
                // Loaded ones are cached now; failed ones keep their time so they wait to retry.
                let s = self.store.read();
                self.universes_pending
                    .lock()
                    .retain(|u, _| !s.games.contains_key(u));
            }
            Cmd::LoadUsers(ids) => {
                self.load_users(&ids).await;
                let mut p = self.users_pending.lock();
                for i in &ids {
                    p.remove(i);
                }
            }
            Cmd::LoadServers {
                account,
                place_id,
                more,
            } => self.load_servers(account, place_id, more).await,
            Cmd::Launch { account, target } => self.launch(account, target).await,
            Cmd::Search { account, query } => self.remote_search(account, query).await,
            Cmd::LoadHome(account) => self.load_home(account).await,
            Cmd::SetSettings(s) => {
                if let Err(e) = s.save(&self.paths.settings_file()) {
                    tracing::warn!("saving settings: {e}");
                }
                *self.opener.write() = Arc::new(launcher::SystemOpener {
                    command: s.player_command.clone(),
                });
                *self.settings.write() = *s;
                self.repaint();
            }
            Cmd::SaveTheme(t) => crate::themes::write_current(&self, &t.id),
            Cmd::LoadBackground(src) => self.load_background(src).await,
            Cmd::PruneBackgrounds => {
                let keep = self.settings.read().appearance.background.clone().map(|b| b.file);
                crate::backgrounds::prune(&self.paths.backgrounds_dir(), keep.as_deref());
            }
            Cmd::DeleteTheme(id) => crate::themes::write_current(&self, &id),
        }
        {
            let mut s = self.store.write();
            s.jobs = s.jobs.saturating_sub(1);
        }
        self.repaint();
    }

    pub(crate) async fn validate_cookie(self: &Arc<Self>, cookie: Secret) {
        let client = match Client::builder(AccountId::ANONYMOUS)
            .cookie(cookie.clone())
            .base(self.base.clone())
            .limiter(self.limiter.clone())
            .build()
        {
            Ok(c) => c,
            Err(e) => {
                self.store.write().login = Some(LoginState::Failed(e.user_message()));
                return;
            }
        };
        match users::v1::get_users_authenticated(&client).await {
            Ok(me) => {
                let user = UserInfo {
                    id: me.id.unwrap_or_default() as u64,
                    name: me.name.unwrap_or_default(),
                    display_name: me.display_name.unwrap_or_default(),
                    verified: false,
                };
                let already = self.store.read().accounts.get(AccountId(user.id)).is_some();
                self.store.write().login = Some(LoginState::Confirm {
                    user: user.clone(),
                    already_added: already,
                });
                self.thumb_tx
                    .send(ThumbKey::large(thumbs::ThumbKind::Headshot, user.id))
                    .ok();
                self.store
                    .write()
                    .thumbs
                    .entry(ThumbKey::large(thumbs::ThumbKind::Headshot, user.id))
                    .or_insert(ThumbState::Loading);
                *self.pending_login.lock() = Some((cookie, user));
            }
            Err(Error::Unauthorized) => {
                self.store.write().login = Some(LoginState::Failed(
                    "Roblox rejected this cookie. It may be expired or signed out.".into(),
                ));
            }
            Err(e) => self.store.write().login = Some(LoginState::Failed(e.user_message())),
        }
    }

    pub(crate) fn forget_pending_login(&self) {
        *self.pending_login.lock() = None;
    }

    async fn confirm_login(self: &Arc<Self>) {
        let Some((cookie, user)) = self.pending_login.lock().take() else {
            return;
        };
        let id = AccountId(user.id);
        if let Err(e) = self.cookies.set(id, &cookie) {
            self.store.write().login =
                Some(LoginState::Failed(format!("Could not save to the keychain: {e}")));
            return;
        }
        match self.build_client(id, cookie) {
            Ok(c) => {
                self.clients.write().insert(id, c);
            }
            Err(e) => {
                self.store.write().login = Some(LoginState::Failed(e.user_message()));
                return;
            }
        }
        {
            let mut s = self.store.write();
            s.accounts.upsert(AccountMeta {
                id,
                name: user.name.clone(),
                display_name: user.display_name.clone(),
                added_at: crate::cache::now(),
                ..Default::default()
            });
            s.accounts.switch(id);
            s.data.entry(id).or_default().session = SessionState::Ok;
            s.users.insert(user.id, user.clone());
            s.login = None;
        }
        self.cache
            .put(Kind::User, AccountId::ANONYMOUS, &user.id.to_string(), &user);
        self.save_accounts();
        self.refresh_account(id).await;
    }

    async fn sign_out(&self, id: AccountId) {
        let ended = match self.client(id) {
            Some(c) => bonsai_api::generated::auth::v2::post_logout(&c).await.map(|_| ()),
            None => Ok(()),
        };
        self.remove_account(id);
        match ended {
            Ok(()) => self.toast(None, "Signed out.", false),
            Err(e) => self.toast(
                None,
                format!(
                    "Removed from this app, but Roblox didn't confirm the sign out: {}",
                    e.user_message()
                ),
                true,
            ),
        }
    }

    fn remove_account(&self, id: AccountId) {
        let _ = self.cookies.delete(id);
        self.clients.write().remove(&id);
        self.cache.forget_account(id);
        self.res.forget_account(id);
        self.ledgers.forget_account(id);
        {
            let mut s = self.store.write();
            s.accounts.remove(id);
            s.data.remove(&id);
        }
        self.save_accounts();
    }

    /// Me, Robux, unread counts, friends and their presence.
    pub(crate) async fn refresh_account(self: &Arc<Self>, id: AccountId) {
        let Some(c) = self.client(id) else { return };
        match users::v1::get_users_authenticated(&c).await {
            Ok(me) => {
                self.note_ok();
                let mut s = self.store.write();
                s.data.entry(id).or_default().session = SessionState::Ok;
                if let Some(a) = s.accounts.get_mut(id) {
                    a.name = me.name.clone().unwrap_or_default();
                    a.display_name = me.display_name.clone().unwrap_or_default();
                }
            }
            Err(Error::Unauthorized) => {
                self.store.write().data.entry(id).or_default().session = SessionState::Expired;
                return;
            }
            Err(e) => {
                self.note_error(id, &e);
                return;
            }
        }
        self.save_accounts();
        let (robux, unread) = tokio::join!(economy::v1::get_user_currency(&c), self.unread(&c));
        match robux {
            Ok(r) => {
                let r = r.robux.unwrap_or_default();
                self.store.write().data.entry(id).or_default().robux = Some(r);
                self.cache.put(Kind::Robux, id, "robux", &r);
            }
            Err(e) => self.note_error(id, &e),
        }
        self.store.write().data.entry(id).or_default().unread = unread;
        self.cache.put(Kind::Unread, id, "unread", &unread);
        self.note_unread(id, unread);
        self.repaint();
        self.refresh_friends(id).await;
    }

    async fn unread(&self, c: &Client) -> Unread {
        let (m, f, t, n, ch) = tokio::join!(
            privatemessages::v1::get_messages_unread_count(c),
            friends::v1::get_user_friend_requests_count(c),
            trades::v1::get_trades_by_trade_status_type_count(c, 1),
            notifications::v2::get_stream_notifications_unread_count(c),
            bonsai_api::generated::apis::undocumented::get_chat_unread(c),
        );
        Unread {
            messages: m.ok().and_then(|x| x.count).unwrap_or(0),
            friend_requests: f.ok().and_then(|x| x.count).unwrap_or(0) as i64,
            trades: t.ok().and_then(|x| x.count).unwrap_or(0),
            notifications: n.ok().and_then(|x| x.unread_notifications).unwrap_or(0),
            chat: ch.ok().and_then(|x| x.global_unread_count).unwrap_or(0),
        }
    }

    /// Re-read the unread badges for one account.
    pub(crate) async fn refresh_unread(self: &Arc<Self>, id: AccountId) {
        let Some(c) = self.client(id) else { return };
        let u = self.unread(&c).await;
        self.store.write().data.entry(id).or_default().unread = u;
        self.cache.put(Kind::Unread, id, "unread", &u);
        self.note_unread(id, u);
        self.repaint();
    }

    pub(crate) async fn refresh_friends(self: &Arc<Self>, id: AccountId) {
        let Some(c) = self.client(id) else { return };
        self.store.write().data.entry(id).or_default().friends.start();
        let r = friends::v1::get_users_by_user_id_friends(&c, id.0 as i64, &Default::default()).await;
        let list = match r {
            Ok(r) => r.data.unwrap_or_default(),
            Err(e) => {
                self.note_error(id, &e);
                self.store
                    .write()
                    .data
                    .entry(id)
                    .or_default()
                    .friends
                    .fail(e.user_message());
                return;
            }
        };
        let ids: Vec<u64> = list.iter().filter_map(|u| u.id.map(|i| i as u64)).collect();
        let before = self
            .store
            .read()
            .data
            .get(&id)
            .and_then(|d| d.friends.data.clone());
        {
            let mut s = self.store.write();
            for u in &list {
                let Some(uid) = u.id else { continue };
                let name = u.name.clone().unwrap_or_default();
                if !name.is_empty() {
                    s.users.insert(
                        uid as u64,
                        UserInfo {
                            id: uid as u64,
                            name,
                            display_name: u.display_name.clone().unwrap_or_default(),
                            verified: u.has_verified_badge.unwrap_or(false),
                        },
                    );
                }
            }
            s.data.entry(id).or_default().friends.done(ids.clone());
        }
        self.cache.put(Kind::Friends, id, "friends", &ids);
        self.note_friends(id, before.as_deref(), &ids);
        // Names are often blank in the friends list now; fill them from the users multi-get.
        let unknown: Vec<u64> = {
            let s = self.store.read();
            ids.iter().filter(|u| !s.users.contains_key(u)).copied().collect()
        };
        self.load_users(&unknown).await;
        self.refresh_presence(id, &ids).await;
        self.repaint();
    }

    /// Presence of `ids` and of the account itself, which says whether it is in a game for
    /// "Mute while playing".
    pub(crate) async fn refresh_presence(self: &Arc<Self>, id: AccountId, ids: &[u64]) {
        let Some(c) = self.client(id) else { return };
        let mut ids = ids.to_vec();
        if !ids.contains(&id.0) {
            ids.push(id.0);
        }
        let mut all = HashMap::new();
        for chunk in ids.chunks(50) {
            let body = presence::v1::UserPresenceRequest {
                user_ids: Some(chunk.iter().map(|i| *i as i64).collect()),
            };
            match presence::v1::post_presence_users(&c, &body).await {
                Ok(r) => {
                    for p in r.user_presences.unwrap_or_default() {
                        if let Some((uid, p)) = Presence::from_api(p) {
                            all.insert(uid, p);
                        }
                    }
                }
                Err(e) => {
                    self.note_error(id, &e);
                    return;
                }
            }
        }
        let universes: Vec<u64> = all.values().filter_map(|p| p.universe_id).collect();
        let seen: Vec<_> = all.iter().map(|(u, p)| (*u, p.kind)).collect();
        self.note_presence(id, &seen);
        let before = {
            let mut s = self.store.write();
            let d = s.data.entry(id).or_default();
            let before = d.presence.clone();
            d.presence.extend(all.clone());
            let snapshot = d.presence.clone();
            drop(s);
            self.cache.put(Kind::Friends, id, "presence", &snapshot);
            before
        };
        self.note_joins(id, &before, &all);
        self.note_presence_changes(id, &before, &all);
        self.load_universes(&universes).await;
    }

    pub(crate) async fn load_users(&self, ids: &[u64]) {
        for chunk in ids.chunks(100) {
            if chunk.is_empty() {
                continue;
            }
            let body = users::v1::MultiGetByUserIdRequest {
                user_ids: Some(chunk.iter().map(|i| *i as i64).collect()),
                exclude_banned_users: Some(false),
            };
            let Ok(r) = users::v1::post_users(&self.anon, &body).await else {
                continue;
            };
            let mut s = self.store.write();
            for u in r.data.unwrap_or_default() {
                let Some(id) = u.id else { continue };
                let info = UserInfo {
                    id: id as u64,
                    name: u.name.unwrap_or_default(),
                    display_name: u.display_name.unwrap_or_default(),
                    verified: u.has_verified_badge.unwrap_or(false),
                };
                self.cache
                    .put(Kind::User, AccountId::ANONYMOUS, &info.id.to_string(), &info);
                s.users.insert(info.id, info);
            }
        }
    }

    pub(crate) async fn load_universes(&self, ids: &[u64]) {
        let missing: Vec<u64> = {
            let s = self.store.read();
            let mut seen = HashSet::new();
            ids.iter()
                .filter(|u| seen.insert(**u) && !s.games.contains_key(u))
                .copied()
                .collect()
        };
        self.fetch_universes(&missing).await;
    }

    pub(crate) async fn fetch_universes(&self, ids: &[u64]) {
        use games::v1::GameDetailResponseUniverseAvatarType as AvatarType;
        for chunk in ids.chunks(50) {
            if chunk.is_empty() {
                continue;
            }
            let p = games::v1::GetGamesParams {
                universe_ids: chunk.iter().map(|i| *i as i64).collect(),
            };
            let vp = games::v1::GetGamesVotesParams {
                universe_ids: p.universe_ids.clone(),
            };
            let (details, votes) = tokio::join!(
                games::v1::get_games(&self.anon, &p),
                games::v1::get_games_votes(&self.anon, &vp)
            );
            let Ok(details) = details else { continue };
            let votes: HashMap<i64, (i64, i64)> = votes
                .ok()
                .and_then(|v| v.data)
                .unwrap_or_default()
                .into_iter()
                .filter_map(|v| Some((v.id?, (v.up_votes.unwrap_or(0), v.down_votes.unwrap_or(0)))))
                .collect();
            let mut s = self.store.write();
            for g in details.data.unwrap_or_default() {
                let Some(id) = g.id else { continue };
                let creator = g.creator.clone().unwrap_or_default();
                let v = votes.get(&id).copied();
                let info = GameInfo {
                    universe_id: id as u64,
                    root_place_id: g.root_place_id.unwrap_or(0) as u64,
                    name: g.name.unwrap_or_default(),
                    description: g.description.unwrap_or_default(),
                    creator_id: creator.id.unwrap_or(0) as u64,
                    creator_name: creator.name.unwrap_or_default(),
                    creator_is_group: creator.type_.as_deref() == Some("Group"),
                    creator_verified: creator.has_verified_badge.unwrap_or(false),
                    playing: g.playing.unwrap_or(0),
                    visits: g.visits.unwrap_or(0),
                    favorites: g.favorited_count.unwrap_or(0),
                    max_players: g.max_players.unwrap_or(0),
                    created: g.created.unwrap_or_default(),
                    updated: g.updated.unwrap_or_default(),
                    genre: g.genre.unwrap_or_default(),
                    price: g.price,
                    private_servers_allowed: g.create_vip_servers_allowed.unwrap_or(false),
                    up_votes: v.map(|v| v.0),
                    down_votes: v.map(|v| v.1),
                    genre_l1: g.genre_l1.unwrap_or_default(),
                    genre_l2: g.genre_l2.unwrap_or_default(),
                    copying_allowed: g.copying_allowed.unwrap_or(false),
                    avatar_type: match g.universe_avatar_type {
                        Some(AvatarType::MorphToR6) => "R6",
                        Some(AvatarType::MorphToR15) => "R15",
                        Some(AvatarType::PlayerChoice) => "Player choice",
                        _ => "",
                    }
                    .to_owned(),
                    gear: g.allowed_gear_categories.unwrap_or_default(),
                };
                self.cache
                    .put(Kind::Universe, AccountId::ANONYMOUS, &id.to_string(), &info);
                s.place_universe.insert(info.root_place_id, info.universe_id);
                s.games.insert(info.universe_id, info);
            }
        }
    }

    /// Place → universe → details. Always revalidates; cached data is already on screen.
    pub(crate) async fn load_place(&self, _account: AccountId, place_id: u64) {
        let known = self.store.read().place_universe.get(&place_id).copied();
        let universe = match known {
            Some(u) => Some(u),
            None => match apis::undocumented::get_place_universe(&self.anon, place_id as i64).await {
                Ok(r) => r.universe_id.map(|u| u as u64),
                Err(e) => {
                    self.note_error(AccountId::ANONYMOUS, &e);
                    None
                }
            },
        };
        if let Some(u) = universe {
            self.store.write().place_universe.insert(place_id, u);
            self.cache
                .put(Kind::Place, AccountId::ANONYMOUS, &place_id.to_string(), &u);
            self.fetch_universes(&[u]).await;
        }
    }

    pub(crate) async fn load_servers(&self, account: AccountId, place_id: u64, more: bool) {
        let cursor = if more {
            self.store.read().server_cursor.get(&place_id).cloned().flatten()
        } else {
            None
        };
        if more && cursor.is_none() {
            return;
        }
        self.store.write().servers.entry(place_id).or_default().start();
        let c = self.client(account).unwrap_or_else(|| self.anon.clone());
        let p = games::v1::GetGamesByPlaceIdServersByServerTypeParams {
            sort_order: Some(2),
            exclude_full_games: Some(false),
            limit: Some(100),
            cursor,
        };
        match games::v1::get_games_by_place_id_servers_by_server_type(&c, place_id as i64, 0, &p).await {
            Ok(page) => {
                self.note_ok();
                let mut list: Vec<Server> = page
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|s| {
                        Some(Server {
                            job_id: s.id?,
                            playing: s.playing.unwrap_or(0),
                            max_players: s.max_players.unwrap_or(0),
                            ping: s.ping,
                            fps: s.fps,
                            friends: Vec::new(),
                            player_tokens: s.player_tokens.unwrap_or_default(),
                        })
                    })
                    .collect();
                let mut s = self.store.write();
                // Mark friends of the viewing account by their presence job id.
                if let Some(d) = s.data.get(&account) {
                    for srv in &mut list {
                        srv.friends = d
                            .presence
                            .iter()
                            .filter(|(_, p)| p.job_id.as_deref() == Some(srv.job_id.as_str()))
                            .map(|(id, _)| *id)
                            .collect();
                    }
                }
                let entry = s.servers.entry(place_id).or_default();
                if more {
                    let mut all = entry.data.take().unwrap_or_default();
                    let seen: HashSet<String> = all.iter().map(|x| x.job_id.clone()).collect();
                    all.extend(list.into_iter().filter(|x| !seen.contains(&x.job_id)));
                    entry.done(all);
                } else {
                    entry.done(list);
                }
                s.server_cursor
                    .insert(place_id, page.next_page_cursor.filter(|c| !c.is_empty()));
            }
            Err(e) => {
                self.note_error(c.account(), &e);
                self.store
                    .write()
                    .servers
                    .entry(place_id)
                    .or_default()
                    .fail(e.user_message());
            }
        }
    }

    pub(crate) async fn launch(&self, account: AccountId, target: JoinTarget) {
        let name = self
            .store
            .read()
            .accounts
            .get(account)
            .map(|a| a.title().to_string())
            .unwrap_or_default();
        let Some(c) = self.client(account).filter(|c| c.signed_in()) else {
            self.toast(Some(account), "That account is not signed in.", true);
            return;
        };
        let ticket = match launcher::mint_ticket(&c).await {
            Ok(t) => t,
            Err(e) => {
                self.note_error(account, &e);
                self.toast(
                    Some(account),
                    format!("Could not start the game as {name}: {}", e.user_message()),
                    true,
                );
                return;
            }
        };
        let now_ms = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map(|d| d.as_millis() as u64)
            .unwrap_or(0);
        let locale = self.settings.read().locale.clone();
        let uri = launcher::launch_uri(&ticket, &target, self.browser_tracker_id, now_ms, &locale);
        let opener = self.opener.read().clone();
        match opener.open(&uri) {
            Ok(()) => {
                if let Some(p) = target.place_id() {
                    let mut s = self.store.write();
                    let d = s.data.entry(account).or_default();
                    d.last_launch = Some(format!("{target:?}"));
                    if let Some(u) = s.place_universe.get(&p).copied() {
                        let d = s.data.entry(account).or_default();
                        d.recent.retain(|x| *x != u);
                        d.recent.insert(0, u);
                        d.recent.truncate(30);
                        let r = d.recent.clone();
                        drop(s);
                        self.cache.put(Kind::Recent, account, "recent", &r);
                    }
                }
                self.toast(Some(account), format!("Starting Roblox as {name}"), false);
            }
            Err(e) => self.toast(Some(account), e, true),
        }
    }

    /// Remote search for the command bar: people and communities (experiences in a later step).
    /// Results for an outdated query are dropped.
    pub(crate) async fn load_home(&self, account: AccountId) {
        let c = self.client(account).unwrap_or_else(|| self.anon.clone());
        let p = apis::undocumented::GetExploreSortsParams {
            session_id: self.session_id.clone(),
            device: Some("computer".into()),
            country: Some("all".into()),
            ..Default::default()
        };
        match apis::undocumented::get_explore_sorts(&c, &p).await {
            Ok(r) => {
                let sorts: Vec<HomeSort> = r
                    .sorts
                    .unwrap_or_default()
                    .into_iter()
                    .filter(|s| s.content_type.as_deref() == Some("Games"))
                    .map(|s| HomeSort {
                        id: s.sort_id.unwrap_or_default(),
                        title: s.sort_display_name.unwrap_or_default(),
                        subtitle: s.subtitle.unwrap_or_default(),
                        games: s
                            .games
                            .unwrap_or_default()
                            .into_iter()
                            .filter_map(GameCard::from_api)
                            .collect(),
                        ..Default::default()
                    })
                    .filter(|s| !s.games.is_empty())
                    .collect();
                self.cache.put(Kind::Home, account, "sorts", &sorts);
                self.store
                    .write()
                    .data
                    .entry(account)
                    .or_default()
                    .home
                    .done(sorts);
            }
            Err(e) => {
                self.note_error(c.account(), &e);
                self.store
                    .write()
                    .data
                    .entry(account)
                    .or_default()
                    .home
                    .fail(e.user_message());
            }
        }
    }

    pub(crate) async fn remote_search(&self, account: AccountId, query: String) {
        let seq = {
            let mut n = self.search_seq.lock();
            *n += 1;
            *n
        };
        tokio::time::sleep(Duration::from_millis(250)).await;
        if *self.search_seq.lock() != seq {
            return;
        }
        let (scope, q) = crate::search::parse_scope(&query);
        let q = q.trim().to_string();
        if !crate::remote::searchable(&q) {
            return;
        }
        self.store.write().remote.start();
        let c = self.client(account).unwrap_or_else(|| self.anon.clone());
        let rows = crate::remote::search(self, &c, scope, &q, false).await;
        if *self.search_seq.lock() != seq {
            return;
        }
        let mut s = self.store.write();
        s.remote.done(RemoteResults { query, rows });
    }
}

/// How long a looked-up thumbnail URL is trusted. An older one still shows at once and is
/// checked again in the background.
const THUMB_URL_AGE: i64 = 24 * 3600;
/// Encoded thumbnails kept in memory. Older ones load again from disk when asked for.
const THUMB_MEMORY_BYTES: usize = 64 << 20;

/// A thumbnail URL looked up before whose image is still on disk, and whether it is fresh.
fn cached_thumb_url(inner: &Inner, k: &ThumbKey) -> Option<(String, bool)> {
    let c = inner
        .cache
        .get::<String>(Kind::ThumbUrl, AccountId::ANONYMOUS, &k.request_id())?;
    let fresh = c.is_fresh(THUMB_URL_AGE);
    inner.disk.has(&c.value).then_some((c.value, fresh))
}

async fn thumb_worker(inner: Arc<Inner>, mut rx: mpsc::UnboundedReceiver<ThumbKey>) {
    let mut pending: Vec<ThumbKey> = Vec::new();
    // Keys already shown from a stale cached URL and waiting to be checked again. A failed
    // check leaves the shown image alone.
    let mut stale: HashSet<ThumbKey> = HashSet::new();
    loop {
        // Collect what arrives within one frame.
        let mut incoming = Vec::new();
        if pending.is_empty() {
            match rx.recv().await {
                Some(k) => incoming.push(k),
                None => return,
            }
        }
        let deadline = tokio::time::Instant::now() + Duration::from_millis(16);
        while let Ok(Some(k)) = tokio::time::timeout_at(deadline, rx.recv()).await {
            incoming.push(k);
        }
        incoming.sort();
        incoming.dedup();
        for k in incoming {
            if let Some((url, fresh)) = cached_thumb_url(&inner, &k) {
                let inner = inner.clone();
                tokio::spawn(async move { fetch_image(&inner, k, &url).await });
                if fresh {
                    continue;
                }
                stale.insert(k);
            }
            pending.push(k);
        }
        pending.sort();
        pending.dedup();
        if pending.is_empty() {
            continue;
        }
        let batch: Vec<ThumbKey> = pending.drain(..pending.len().min(100)).collect();
        let mut retry = Vec::new();
        match thumbs::lookup(&inner.anon, &batch).await {
            Ok(found) => {
                let mut done = HashSet::new();
                for (k, url, unavailable) in found {
                    done.insert(k);
                    let was_stale = stale.remove(&k);
                    let id = k.request_id();
                    match url {
                        Some(url) => {
                            let prev = inner
                                .cache
                                .get::<String>(Kind::ThumbUrl, AccountId::ANONYMOUS, &id);
                            inner.cache.put(Kind::ThumbUrl, AccountId::ANONYMOUS, &id, &url);
                            // Same image as the one on screen: nothing to load.
                            if was_stale && prev.is_some_and(|p| p.value == url) {
                                continue;
                            }
                            let inner = inner.clone();
                            tokio::spawn(async move { fetch_image(&inner, k, &url).await });
                        }
                        None if unavailable => {
                            inner.cache.delete(Kind::ThumbUrl, AccountId::ANONYMOUS, &id);
                            inner.store.write().thumbs.insert(k, ThumbState::Unavailable);
                        }
                        None if was_stale => {}
                        None => retry.push(k),
                    }
                }
                for k in batch.iter().filter(|k| !done.contains(k)) {
                    if !stale.remove(k) {
                        inner.store.write().thumbs.insert(*k, ThumbState::Unavailable);
                    }
                }
            }
            Err(_) => retry.extend(batch.into_iter().filter(|k| !stale.remove(k))),
        }
        if !retry.is_empty() {
            // `Pending`: Roblox is still rendering. Poll again shortly.
            let tx = inner.thumb_tx.clone();
            tokio::spawn(async move {
                tokio::time::sleep(Duration::from_millis(1500)).await;
                for k in retry {
                    let _ = tx.send(k);
                }
            });
        }
        inner.repaint();
    }
}

async fn fetch_image(inner: &Inner, k: ThumbKey, url: &str) {
    let bytes = match inner.disk.get(url) {
        Some(b) => b,
        None => match inner.anon.fetch_cdn(url).await {
            Ok(b) => {
                inner.disk.put(url, &b);
                b
            }
            Err(_) => {
                inner.store.write().thumbs.insert(k, ThumbState::Unavailable);
                return;
            }
        },
    };
    match thumbs::sniff(bytes) {
        Some(img) => {
            let evict = inner
                .thumb_budget
                .lock()
                .insert(k, img.bytes.len(), THUMB_MEMORY_BYTES);
            let mut s = inner.store.write();
            for old in evict {
                if matches!(s.thumbs.get(&old), Some(ThumbState::Ready(_))) {
                    s.thumbs.remove(&old);
                }
            }
            s.thumbs.insert(k, ThumbState::Ready(Arc::new(img)));
        }
        None => {
            inner.store.write().thumbs.insert(k, ThumbState::Unavailable);
        }
    }
    inner.repaint();
}

impl Inner {
    /// Drop the images of `id` in every size, cached URL included, so the next ask looks them
    /// up again (after an avatar change Roblox renders new ones).
    pub(crate) fn forget_thumbs(&self, kinds: &[thumbs::ThumbKind], id: u64) {
        let mut budget = self.thumb_budget.lock();
        let mut s = self.store.write();
        for &kind in kinds {
            for k in [ThumbKey::small(kind, id), ThumbKey::large(kind, id)] {
                self.cache
                    .delete(Kind::ThumbUrl, AccountId::ANONYMOUS, &k.request_id());
                s.thumbs.remove(&k);
                budget.remove(&k);
            }
        }
    }

    /// Drop cached data nobody has used in a while, on disk and in memory.
    fn tidy(&self) {
        const DAY: i64 = 24 * 3600;
        let (days, mb) = {
            let s = self.settings.read();
            (s.cache_days.clamp(1, 3650), s.image_cache_mb.max(64))
        };
        // Rows and images unused this long are deleted.
        let keep_secs = i64::from(days) * DAY;
        let rows = self.cache.prune(crate::cache::now() - keep_secs);
        let cutoff = std::time::SystemTime::now() - Duration::from_secs(keep_secs as u64);
        // Thumbnails and 3D asset files share this.
        let images = self.disk.prune(cutoff, u64::from(mb) << 20);
        tracing::debug!("cache tidy: {rows} rows, {images} images");
    }
}

/// Cache upkeep: idle in-memory resources every few minutes, old rows and images every few
/// hours. Waits a little after start so it never competes with the first screens.
async fn tidy_loop(inner: Arc<Inner>) {
    tokio::time::sleep(Duration::from_secs(30)).await;
    let mut tick = tokio::time::interval(Duration::from_secs(5 * 60));
    loop {
        tick.tick().await;
        inner.res.trim(crate::cache::now());
        if inner.due("cache_tidy", 6 * 3600) {
            let i = inner.clone();
            let _ = inner.rt.spawn_blocking(move || i.tidy()).await;
        }
    }
}

/// Session health and unread badges for every account, at a polite interval.
async fn health_loop(inner: Arc<Inner>) {
    let mut tick = tokio::time::interval(Duration::from_secs(120));
    tick.tick().await;
    loop {
        tick.tick().await;
        let (ids, active, background) = {
            let s = inner.store.read();
            (
                s.accounts.accounts.iter().map(|a| a.id).collect::<Vec<_>>(),
                s.accounts.active,
                inner.settings.read().background_accounts,
            )
        };
        for id in ids {
            if !background && Some(id) != active {
                continue;
            }
            let Some(c) = inner.client(id) else { continue };
            match users::v1::get_users_authenticated(&c).await {
                Ok(_) => {
                    let u = inner.unread(&c).await;
                    let mut s = inner.store.write();
                    let d = s.data.entry(id).or_default();
                    d.session = SessionState::Ok;
                    d.unread = u;
                    drop(s);
                    inner.note_unread(id, u);
                }
                Err(e) => inner.note_error(id, &e),
            }
            inner.watch_sales(&c, id).await;
            // The full list now and then, to notice friends who removed you.
            if inner.due(&format!("friends_list:{}", id.0), 600) {
                inner.refresh_friends(id).await;
            } else if Some(id) == active {
                let friends = inner
                    .store
                    .read()
                    .data
                    .get(&id)
                    .and_then(|d| d.friends.data.clone())
                    .unwrap_or_default();
                inner.refresh_presence(id, &friends).await;
            }
        }
        inner.repaint();
    }
}

/// A random version-4 UUID string.
pub(crate) fn random_uuid() -> String {
    let a = fastrand::u128(..);
    let b = (a & !(0xf000 << 64) & !(0xc000 << 48)) | (0x4000 << 64) | (0x8000 << 48);
    let h = format!("{b:032x}");
    format!(
        "{}-{}-{}-{}-{}",
        &h[..8],
        &h[8..12],
        &h[12..16],
        &h[16..20],
        &h[20..]
    )
}

impl Core {
    /// A short message for the user, shown as a toast.
    pub fn notice(&self, text: impl Into<String>, error: bool) {
        self.inner.toast(None, text, error);
        self.inner.repaint();
    }
}
