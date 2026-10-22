//! One Luau VM per plugin. Luau's sandbox mode makes the built-in libraries read-only and gives
//! scripts their own globals; on top of that we load no `os`, `io` or `debug`, remove
//! `getfenv`, `setfenv` and `loadstring`, only load source text, and give `require` the plugin's
//! own folder and nothing else. An interrupt enforces the time limit and `mlua` the memory cap.

use std::cell::{Cell, RefCell};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::rc::Rc;
use std::time::{Duration, Instant};

use mlua::chunk::ChunkMode;
use mlua::{Function, Lua, LuaOptions, MultiValue, StdLib, Table, Value, VmState};

use crate::manifest::valid_module;
use crate::view::{self, Node, ViewEvent};
use crate::{Error, Manifest, Permission, is_plugin_id};

/// Most commands a plugin may register.
const MAX_COMMANDS: usize = 100;
/// Most reads one handler call may make.
const MAX_READS: usize = 200;
/// Most ids in one `ctx.presence` call.
const MAX_PRESENCE: usize = 100;
/// Most `ctx.store.set` calls in one handler call; each one writes to disk.
const MAX_SETS: usize = 100;
/// Most `ctx.run` calls one call may make.
const MAX_RUNS: usize = 50;
/// Most timers a plugin may start, and the shortest and longest gap between runs.
const MAX_TIMERS: usize = 10;
const MIN_EVERY: u64 = 60;
const MAX_EVERY: u64 = 86_400;
/// How deep a stored table may nest.
const MAX_DEPTH: usize = 16;

/// What `bonsai.on` can listen for. See `docs/plugins.md#events`.
pub const EVENTS: [&str; 6] = [
    "presence",
    "friend_request",
    "trade",
    "message",
    "chat",
    "game_update",
];

#[derive(Clone, Copy, Debug)]
pub struct Limits {
    /// For one handler call. Time spent waiting on the host doesn't count.
    pub call: Duration,
    /// For running the entry module when the plugin loads.
    pub load: Duration,
    /// Bytes for the whole VM.
    pub memory: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Limits {
            call: Duration::from_millis(50),
            load: Duration::from_millis(250),
            memory: 32 << 20,
        }
    }
}

/// Everything a plugin can reach outside its VM. `bonsai-core` implements it and does the
/// grant, tier, cap and automation checks; the sandbox only checks the manifest.
pub trait Host {
    /// Runs a built-in command now as `account`. `Err` is shown to the plugin as a Luau error.
    fn run(&self, plugin: &str, account: u64, action: &str, arg: Option<&str>) -> Result<(), String>;
    /// Reads data as `account` (0 for none). The sandbox has checked the manifest asks for
    /// [`Read::permission`]. `Ok(Null)` reads as `nil`.
    fn read(&self, plugin: &str, account: u64, read: &Read) -> Result<serde_json::Value, String>;
    /// A line from `print`, for the developer log.
    fn log(&self, plugin: &str, line: &str);
    /// The value stored under `key` for `account` (0 for the plugin's own storage), or `Null`.
    fn store_get(&self, plugin: &str, account: u64, key: &str) -> Result<serde_json::Value, String>;
    /// Stores `value` under `key`; `Null` removes it. The host caps the size.
    fn store_set(
        &self,
        plugin: &str,
        account: u64,
        key: &str,
        value: serde_json::Value,
    ) -> Result<(), String>;
    /// The manifest's settings with the user's values, by key.
    fn settings(&self, plugin: &str) -> serde_json::Map<String, serde_json::Value>;
}

/// A read a plugin asked for through `ctx`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Read {
    /// Credential-free HTTPS to a manifest host.
    Http(HttpRequest),
    User(u64),
    /// By place id, like game commands get.
    Game(u64),
    Item {
        id: u64,
        bundle: bool,
    },
    Presence(Vec<u64>),
    /// The account's friends.
    Friends,
    /// Collectibles, or one asset or bundle type. `user` defaults to the account.
    Inventory {
        user: Option<u64>,
        asset_type: Option<i32>,
        bundle_type: Option<i32>,
        limit: usize,
    },
    Trades {
        status: TradeStatus,
        limit: usize,
    },
    /// Friend requests to the account, newest first.
    FriendRequests {
        limit: usize,
    },
}

impl Read {
    /// What the manifest needs to ask for to make this read.
    pub fn permission(&self) -> Permission {
        match self {
            Read::Http(_) => Permission::Net,
            Read::User(_) | Read::Game(_) | Read::Item { .. } => Permission::ReadPublic,
            _ => Permission::ReadAccount,
        }
    }
}

/// Input to `ctx.http`. No headers, credentials, redirects or arbitrary methods.
#[derive(Clone, PartialEq, Eq)]
pub struct HttpRequest {
    pub url: String,
    pub method: String,
    /// UTF-8 JSON body for POST. At most 64 KiB.
    pub body: Option<String>,
}

impl std::fmt::Debug for HttpRequest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("HttpRequest")
            .field("method", &self.method)
            .finish_non_exhaustive()
    }
}

/// Which trades `ctx.trades` lists.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum TradeStatus {
    #[default]
    Inbound,
    Outbound,
    Completed,
    Inactive,
}

/// What a command takes, from `arg = "..."` in `bonsai.command`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Arg {
    None,
    Text,
    User,
    Game,
    Item,
    Account,
}

impl Arg {
    fn parse(s: &str) -> Option<Arg> {
        Some(match s {
            "none" => Arg::None,
            "text" => Arg::Text,
            "user" => Arg::User,
            "game" => Arg::Game,
            "item" => Arg::Item,
            "account" => Arg::Account,
            _ => return None,
        })
    }
}

/// A command the plugin registered with `bonsai.command`.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Command {
    pub id: String,
    pub title: String,
    pub keywords: Vec<String>,
    pub arg: Arg,
    /// `user`, `game`, `item` or `group`: shown in that kind's menus.
    pub context: Option<String>,
}

/// Who a handler runs for.
#[derive(Clone, Copy, Debug)]
pub struct Ctx {
    pub account: u64,
    /// Unix seconds.
    pub now: i64,
}

/// A built-in command a plugin proposes. Its key is one the manifest declares.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Action {
    pub id: String,
    pub arg: Option<String>,
}

/// What a handler returned.
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    Nothing,
    /// To run, after the user confirms the ones that need it.
    Actions(Vec<Action>),
    Toast(String),
    Open {
        kind: String,
        id: u64,
    },
    Alert {
        title: String,
        body: String,
        command: Option<String>,
    },
    /// A view to show, or to redraw the one this command shows.
    View(Node),
    /// Close the view this command shows.
    Close,
}

/// A deadline the interrupt checks. Host calls push it back by however long they took.
#[derive(Default)]
struct Clock {
    deadline: Cell<Option<Instant>>,
    tripped: Cell<bool>,
}

impl Clock {
    fn start(&self, limit: Duration) {
        self.deadline.set(Some(Instant::now() + limit));
        self.tripped.set(false);
    }
    fn stop(&self) {
        self.deadline.set(None);
    }
    /// Runs `f` off the clock.
    fn pause<T>(&self, f: impl FnOnce() -> T) -> T {
        let t = Instant::now();
        let out = f();
        if let Some(d) = self.deadline.get() {
            self.deadline.set(Some(d + t.elapsed()));
        }
        out
    }
}

/// What the entry module registered while it loaded.
#[derive(Default)]
struct Registered {
    /// Each command with its `run` and, when it has views, its `event` function.
    commands: Vec<(Command, Function, Option<Function>)>,
    events: Vec<(String, Function)>,
    timers: Vec<(Duration, Function)>,
}

pub struct Plugin {
    lua: Lua,
    manifest: Rc<Manifest>,
    commands: Vec<Command>,
    handlers: HashMap<String, Function>,
    /// Each command's `event(ctx, e)`, for its views.
    view_handlers: HashMap<String, Function>,
    events: Vec<(String, Function)>,
    timers: Vec<(Duration, Function)>,
    clock: Rc<Clock>,
    limits: Limits,
    host: Rc<dyn Host>,
    /// Reads made in the current call.
    reads: Rc<Cell<usize>>,
    network_calls: Rc<Cell<usize>>,
    /// `ctx.store.set` calls made in the current call.
    sets: Rc<Cell<usize>>,
    /// `ctx.run` calls made in the current call.
    runs: Rc<Cell<usize>>,
}

impl Plugin {
    /// Starts a VM for the plugin in `dir` and runs its entry module, which registers commands.
    /// `dir`'s name must be the manifest's id.
    pub fn load(dir: &Path, manifest: Manifest, host: Rc<dyn Host>, limits: Limits) -> Result<Plugin, Error> {
        if dir.file_name().and_then(|n| n.to_str()) != Some(manifest.id.as_str()) {
            return Err(Error::Manifest(format!(
                "The folder must be named \"{}\", like the plugin's id.",
                manifest.id
            )));
        }
        let root = dir
            .canonicalize()
            .map_err(|e| Error::Io(dir.display().to_string(), e))?;
        let manifest = Rc::new(manifest);
        let libs =
            StdLib::TABLE | StdLib::STRING | StdLib::MATH | StdLib::UTF8 | StdLib::BIT | StdLib::BUFFER;
        let lua = Lua::new_with(libs, LuaOptions::default()).map_err(script)?;
        lua.set_memory_limit(limits.memory).map_err(script)?;

        let clock = Rc::new(Clock::default());
        let c = clock.clone();
        lua.set_interrupt(move |_| match c.deadline.get() {
            Some(d) if c.tripped.get() || Instant::now() > d => {
                // Keeps failing once tripped, so a `pcall` can't swallow it and carry on.
                c.tripped.set(true);
                Err(mlua::Error::runtime("time limit reached"))
            }
            _ => Ok(VmState::Continue),
        });

        let registered = Rc::new(RefCell::new(Registered::default()));
        let loading = Rc::new(Cell::new(true));
        install_globals(
            &lua,
            &manifest,
            host.clone(),
            &root,
            &clock,
            &registered,
            &loading,
        )
        .map_err(script)?;
        lua.sandbox(true).map_err(script)?;

        let entry = manifest.entry.clone();
        let source = read_module(&root, &entry)?;
        clock.start(limits.load);
        let ran = lua
            .load(source)
            .set_name(format!("={entry}"))
            .set_mode(ChunkMode::Text)
            .exec();
        clock.stop();
        loading.set(false);
        check(&clock, &lua, limits, limits.load, ran)?;

        let reg = registered.take();
        let mut commands = Vec::new();
        let mut handlers = HashMap::new();
        let mut view_handlers = HashMap::new();
        for (cmd, f, event) in reg.commands {
            handlers.insert(cmd.id.clone(), f);
            if let Some(e) = event {
                view_handlers.insert(cmd.id.clone(), e);
            }
            commands.push(cmd);
        }
        Ok(Plugin {
            lua,
            manifest,
            commands,
            handlers,
            view_handlers,
            events: reg.events,
            timers: reg.timers,
            clock,
            limits,
            host,
            reads: Rc::default(),
            network_calls: Rc::default(),
            sets: Rc::default(),
            runs: Rc::default(),
        })
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
    }

    /// Events it listens for with `bonsai.on`, from [`EVENTS`].
    pub fn events(&self) -> Vec<String> {
        self.events.iter().map(|(e, _)| e.clone()).collect()
    }

    /// How often each `bonsai.every` timer runs, in the order they were started.
    pub fn timers(&self) -> Vec<Duration> {
        self.timers.iter().map(|(d, _)| *d).collect()
    }

    /// Calls the `bonsai.on(event, fn)` handler with `fn(ctx, payload)`. Nothing happens for an
    /// event it doesn't listen for.
    pub fn run_event(&self, event: &str, ctx: Ctx, payload: &serde_json::Value) -> Result<Outcome, Error> {
        let Some((_, f)) = self.events.iter().find(|(e, _)| e == event) else {
            return Ok(Outcome::Nothing);
        };
        let payload = to_lua(&self.lua, payload).map_err(script)?;
        self.call(f, ctx, payload)
    }

    /// Calls timer `index` (from [`Plugin::timers`]) with `fn(ctx)`.
    pub fn run_timer(&self, index: usize, ctx: Ctx) -> Result<Outcome, Error> {
        let Some((_, f)) = self.timers.get(index) else {
            return Ok(Outcome::Nothing);
        };
        self.call(f, ctx, Value::Nil)
    }

    fn call(&self, f: &Function, ctx: Ctx, arg: Value) -> Result<Outcome, Error> {
        let ctx = self.ctx_table(ctx).map_err(script)?;
        self.reads.set(0);
        self.network_calls.set(0);
        self.sets.set(0);
        self.runs.set(0);
        self.clock.start(self.limits.call);
        // Reading the result can run the plugin's metamethods, so it stays on the clock.
        let ran = f.call::<Value>((ctx, arg)).map(|v| outcome(&self.manifest, v));
        self.clock.stop();
        check(&self.clock, &self.lua, self.limits, self.limits.call, ran)?
    }

    /// Calls a command's `run(ctx, arg)`.
    pub fn run_command(&self, id: &str, ctx: Ctx, arg: Option<&str>) -> Result<Outcome, Error> {
        let (Some(cmd), Some(f)) = (self.commands.iter().find(|c| c.id == id), self.handlers.get(id)) else {
            return Err(Error::NoCommand(id.to_owned()));
        };
        let arg = match (cmd.arg, arg.map(str::trim).filter(|a| !a.is_empty())) {
            (Arg::None, _) | (_, None) => Value::Nil,
            (Arg::Text, Some(a)) => Value::String(self.lua.create_string(a).map_err(script)?),
            (_, Some(a)) => match a.parse::<u64>() {
                // Roblox ids fit in a Luau number exactly.
                Ok(n) if n < (1 << 53) => Value::Number(n as f64),
                _ => return Err(Error::Script(format!("\"{a}\" isn't an id."))),
            },
        };
        self.call(f, ctx, arg)
    }

    /// Calls a command's `event(ctx, e)` for something the user did in its view.
    pub fn run_view_event(&self, id: &str, ctx: Ctx, event: &ViewEvent) -> Result<Outcome, Error> {
        let Some(f) = self.view_handlers.get(id) else {
            return Err(Error::Script(format!("Command \"{id}\" has no event function.")));
        };
        let e = view::event_table(&self.lua, event).map_err(script)?;
        self.call(f, ctx, Value::Table(e))
    }

    fn ctx_table(&self, ctx: Ctx) -> mlua::Result<Table> {
        let t = self.lua.create_table()?;
        t.set("account", ctx.account as f64)?;
        t.set("now", ctx.now as f64)?;
        let run: Function = self.lua.named_registry_value("bonsai.run")?;
        // `ctx.run(id, arg)` acts as this context's account.
        let account = ctx.account;
        let runs = self.runs.clone();
        let bound = self
            .lua
            .create_function(move |_, (id, arg): (String, Option<Value>)| {
                if runs.get() >= MAX_RUNS {
                    return Err(mlua::Error::runtime(format!(
                        "A call can make at most {MAX_RUNS} ctx.run calls."
                    )));
                }
                runs.set(runs.get() + 1);
                run.call::<()>((account, id, arg))
            })?;
        t.set("run", bound)?;
        self.reads_into(&t, account)?;
        t.set("store", self.store_table(account)?)?;
        let settings = self.host.settings(&self.manifest.id);
        let settings = match to_lua(&self.lua, &serde_json::Value::Object(settings))? {
            Value::Table(s) => s,
            _ => self.lua.create_table()?,
        };
        settings.set_readonly(true);
        t.set("settings", settings)?;
        t.set_readonly(true);
        Ok(t)
    }

    /// `ctx.store.get(key)` and `ctx.store.set(key, value)`, for `account`'s storage.
    fn store_table(&self, account: u64) -> mlua::Result<Table> {
        let t = self.lua.create_table()?;
        let (host, id, clock) = (self.host.clone(), self.manifest.id.clone(), self.clock.clone());
        t.set(
            "get",
            self.lua.create_function(move |lua, key: Value| {
                let key = store_key(&key)?;
                let v = clock
                    .pause(|| host.store_get(&id, account, &key))
                    .map_err(mlua::Error::runtime)?;
                to_lua(lua, &v)
            })?,
        )?;
        let (host, id, clock, sets) = (
            self.host.clone(),
            self.manifest.id.clone(),
            self.clock.clone(),
            self.sets.clone(),
        );
        t.set(
            "set",
            self.lua.create_function(move |_, (key, value): (Value, Value)| {
                let key = store_key(&key)?;
                if sets.get() >= MAX_SETS {
                    return Err(mlua::Error::runtime(format!(
                        "A call can make at most {MAX_SETS} ctx.store.set calls."
                    )));
                }
                sets.set(sets.get() + 1);
                let value = from_lua(&value, 0)?;
                clock
                    .pause(|| host.store_set(&id, account, &key, value))
                    .map_err(mlua::Error::runtime)
            })?,
        )?;
        t.set_readonly(true);
        Ok(t)
    }

    /// `ctx.user`, `ctx.friends` and the other reads, as `account`.
    fn reads_into(&self, t: &Table, account: u64) -> mlua::Result<()> {
        let lua = &self.lua;
        let read = {
            let (host, manifest, clock, reads) = (
                self.host.clone(),
                self.manifest.clone(),
                self.clock.clone(),
                self.reads.clone(),
            );
            let network_calls = self.network_calls.clone();
            Rc::new(move |lua: &Lua, r: Read| -> mlua::Result<Value> {
                let p = r.permission();
                if !manifest.permissions.contains(&p) {
                    return Err(mlua::Error::runtime(format!(
                        "Add \"{}\" to permissions in plugin.toml.",
                        p.key()
                    )));
                }
                if reads.get() >= MAX_READS {
                    return Err(mlua::Error::runtime(format!(
                        "A call can make at most {MAX_READS} reads."
                    )));
                }
                if matches!(r, Read::Http(_)) {
                    if network_calls.get() >= 10 {
                        return Err(mlua::Error::runtime("At most 10 HTTP requests per call."));
                    }
                    network_calls.set(network_calls.get() + 1);
                }
                reads.set(reads.get() + 1);
                let v = clock
                    .pause(|| host.read(&manifest.id, account, &r))
                    .map_err(mlua::Error::runtime)?;
                to_lua(lua, &v)
            })
        };
        macro_rules! def {
            ($name:literal, $args:ty, $f:expr) => {{
                let read = read.clone();
                let parse = $f;
                t.set(
                    $name,
                    lua.create_function(move |lua, args: $args| read(lua, parse(args)?))?,
                )?;
            }};
        }
        def!("http", Table, |t: Table| -> mlua::Result<Read> {
            let url: String = t.get("url")?;
            let method: String = t.get::<Option<String>>("method")?.unwrap_or_else(|| "GET".into());
            let body: Option<String> = t.get("body")?;
            if url.len() > 8192
                || !matches!(method.as_str(), "GET" | "POST")
                || body.as_ref().is_some_and(|b| b.len() > 65536)
                || (method == "GET" && body.is_some())
            {
                return Err(mlua::Error::runtime(
                    "HTTP takes GET or POST, a URL up to 8192 bytes and a POST body up to 64 KiB.",
                ));
            }
            Ok(Read::Http(HttpRequest { url, method, body }))
        });
        def!("user", Value, |id: Value| -> mlua::Result<Read> {
            Ok(Read::User(id_arg("ctx.user", &id)?))
        });
        def!("game", Value, |id: Value| -> mlua::Result<Read> {
            Ok(Read::Game(id_arg("ctx.game", &id)?))
        });
        def!("item", (Value, Option<String>), |(id, kind): (
            Value,
            Option<String>
        )|
         -> mlua::Result<Read> {
            let bundle = match kind.as_deref() {
                None | Some("asset") => false,
                Some("bundle") => true,
                Some(k) => {
                    return Err(mlua::Error::runtime(format!(
                        "ctx.item: kind is asset or bundle, not \"{k}\"."
                    )));
                }
            };
            Ok(Read::Item {
                id: id_arg("ctx.item", &id)?,
                bundle,
            })
        });
        def!("presence", Table, |ids: Table| -> mlua::Result<Read> {
            let mut out = Vec::new();
            for v in ids.sequence_values::<Value>() {
                out.push(id_arg("ctx.presence", &v?)?);
            }
            if out.len() > MAX_PRESENCE {
                return Err(mlua::Error::runtime(format!(
                    "ctx.presence takes at most {MAX_PRESENCE} ids."
                )));
            }
            Ok(Read::Presence(out))
        });
        def!("friends", (), |()| -> mlua::Result<Read> { Ok(Read::Friends) });
        def!(
            "inventory",
            Option<Table>,
            |f: Option<Table>| -> mlua::Result<Read> {
                let Some(f) = f else {
                    return Ok(Read::Inventory {
                        user: None,
                        asset_type: None,
                        bundle_type: None,
                        limit: 100,
                    });
                };
                let user = match f.get::<Value>("user")? {
                    Value::Nil => None,
                    v => Some(id_arg("ctx.inventory: user", &v)?),
                };
                let asset_type = f.get::<Option<i32>>("asset_type")?;
                let bundle_type = f.get::<Option<i32>>("bundle_type")?;
                if asset_type.is_some() && bundle_type.is_some() {
                    return Err(mlua::Error::runtime(
                        "ctx.inventory: give asset_type or bundle_type, not both.",
                    ));
                }
                Ok(Read::Inventory {
                    user,
                    asset_type,
                    bundle_type,
                    limit: limit_arg("ctx.inventory", &f, 100, 1000)?,
                })
            }
        );
        def!(
            "trades",
            Option<Table>,
            |f: Option<Table>| -> mlua::Result<Read> {
                let Some(f) = f else {
                    return Ok(Read::Trades {
                        status: TradeStatus::Inbound,
                        limit: 25,
                    });
                };
                let status = match f.get::<Option<String>>("status")?.as_deref() {
                    None | Some("inbound") => TradeStatus::Inbound,
                    Some("outbound") => TradeStatus::Outbound,
                    Some("completed") => TradeStatus::Completed,
                    Some("inactive") => TradeStatus::Inactive,
                    Some(s) => {
                        return Err(mlua::Error::runtime(format!(
                            "ctx.trades: status is inbound, outbound, completed or inactive, not \"{s}\"."
                        )));
                    }
                };
                Ok(Read::Trades {
                    status,
                    limit: limit_arg("ctx.trades", &f, 25, 100)?,
                })
            }
        );
        def!(
            "friend_requests",
            Option<Table>,
            |f: Option<Table>| -> mlua::Result<Read> {
                let limit = match f {
                    Some(f) => limit_arg("ctx.friend_requests", &f, 50, 500)?,
                    None => 50,
                };
                Ok(Read::FriendRequests { limit })
            }
        );
        Ok(())
    }
}

/// A Roblox id from Luau: a whole number from 1 to 2^53, or a string of one.
fn id_arg(what: &str, v: &Value) -> mlua::Result<u64> {
    let n = match v {
        Value::Integer(n) => Some(*n as f64),
        Value::Number(n) => Some(*n),
        Value::String(s) => s.to_str()?.trim().parse::<u64>().ok().map(|n| n as f64),
        _ => None,
    };
    match n {
        Some(n) if n >= 1.0 && n.fract() == 0.0 && n < (1u64 << 53) as f64 => Ok(n as u64),
        _ => Err(mlua::Error::runtime(format!("{what} takes an id."))),
    }
}

fn limit_arg(what: &str, f: &Table, default: usize, max: usize) -> mlua::Result<usize> {
    match f.get::<Option<f64>>("limit")? {
        None => Ok(default),
        Some(n) if n >= 1.0 && n.fract() == 0.0 && n <= max as f64 => Ok(n as usize),
        Some(_) => Err(mlua::Error::runtime(format!(
            "{what}: limit is a whole number from 1 to {max}."
        ))),
    }
}

/// A `ctx.store` key: a string of 1 to 128 bytes.
fn store_key(v: &Value) -> mlua::Result<String> {
    match v {
        Value::String(s) => {
            let s = s.to_str()?.to_owned();
            if s.is_empty() || s.len() > 128 {
                return Err(mlua::Error::runtime("ctx.store keys are 1 to 128 bytes."));
            }
            Ok(s)
        }
        _ => Err(mlua::Error::runtime("ctx.store keys are strings.")),
    }
}

/// A Luau value to store: nil, booleans, numbers, strings, and tables of them. A table whose
/// keys are 1 to n is a list; any other table needs string keys.
fn from_lua(v: &Value, depth: usize) -> mlua::Result<serde_json::Value> {
    use serde_json::Value as J;
    let bad = |what: &str| mlua::Error::runtime(format!("ctx.store can't keep {what}."));
    Ok(match v {
        Value::Nil => J::Null,
        Value::Boolean(b) => J::Bool(*b),
        Value::Integer(n) => J::from(*n),
        Value::Number(n) if !n.is_finite() => return Err(bad("nan or infinity")),
        Value::Number(n) if n.fract() == 0.0 && n.abs() < (1u64 << 53) as f64 => J::from(*n as i64),
        Value::Number(n) => serde_json::Number::from_f64(*n).map_or(J::Null, J::Number),
        Value::String(s) => J::String(s.to_str().map_err(|_| bad("text that isn't UTF-8"))?.to_owned()),
        Value::Table(t) => {
            if depth >= MAX_DEPTH {
                return Err(bad(&format!("tables nested more than {MAX_DEPTH} deep")));
            }
            let len = t.raw_len();
            let mut pairs = 0;
            let mut list = len > 0;
            for pair in t.pairs::<Value, Value>() {
                let (k, _) = pair?;
                pairs += 1;
                list &= matches!(k, Value::Integer(i) if i >= 1 && i as usize <= len)
                    || matches!(k, Value::Number(n) if n >= 1.0 && n.fract() == 0.0 && n as usize <= len);
            }
            if list && pairs == len {
                let mut out = Vec::with_capacity(len);
                for i in 1..=len {
                    out.push(from_lua(&t.raw_get::<Value>(i)?, depth + 1)?);
                }
                J::Array(out)
            } else {
                let mut out = serde_json::Map::new();
                for pair in t.pairs::<Value, Value>() {
                    let (k, x) = pair?;
                    let Value::String(k) = k else {
                        return Err(bad("tables with keys that aren't strings, unless it is a list"));
                    };
                    let k = k.to_str().map_err(|_| bad("keys that aren't UTF-8"))?.to_owned();
                    out.insert(k, from_lua(&x, depth + 1)?);
                }
                J::Object(out)
            }
        }
        other => return Err(bad(&format!("a {}", other.type_name()))),
    })
}

/// Host data as Luau values: objects and arrays become tables, `null` becomes `nil`.
fn to_lua(lua: &Lua, v: &serde_json::Value) -> mlua::Result<Value> {
    use serde_json::Value as J;
    Ok(match v {
        J::Null => Value::Nil,
        J::Bool(b) => Value::Boolean(*b),
        J::Number(n) => Value::Number(n.as_f64().unwrap_or(0.0)),
        J::String(s) => Value::String(lua.create_string(s)?),
        J::Array(a) => {
            let t = lua.create_table_with_capacity(a.len(), 0)?;
            for (i, x) in a.iter().enumerate() {
                t.raw_set(i + 1, to_lua(lua, x)?)?;
            }
            Value::Table(t)
        }
        J::Object(o) => {
            let t = lua.create_table_with_capacity(0, o.len())?;
            for (k, x) in o {
                t.raw_set(k.as_str(), to_lua(lua, x)?)?;
            }
            Value::Table(t)
        }
    })
}

fn install_globals(
    lua: &Lua,
    manifest: &Rc<Manifest>,
    host: Rc<dyn Host>,
    root: &Path,
    clock: &Rc<Clock>,
    registered: &Rc<RefCell<Registered>>,
    loading: &Rc<Cell<bool>>,
) -> mlua::Result<()> {
    let g = lua.globals();
    for name in [
        "getfenv",
        "setfenv",
        "loadstring",
        "load",
        "dofile",
        "loadfile",
        "require",
    ] {
        g.raw_set(name, Value::Nil)?;
    }

    let (h, m) = (host.clone(), manifest.clone());
    g.raw_set(
        "print",
        lua.create_function(move |_, args: MultiValue| {
            let parts: Vec<String> = args
                .iter()
                .map(|v| v.to_string().unwrap_or_else(|_| v.type_name().to_owned()))
                .collect();
            h.log(&m.id, &parts.join("\t"));
            Ok(())
        })?,
    )?;

    // `require("lib/util")` loads `lib/util.luau` from the plugin's folder, once.
    let modules = Rc::new(RefCell::new(HashMap::<String, Option<Value>>::new()));
    let (root, c) = (root.to_path_buf(), clock.clone());
    g.raw_set(
        "require",
        lua.create_function(move |lua, name: String| {
            if !valid_module(&name) {
                return Err(mlua::Error::runtime(format!(
                    "require(\"{name}\"): use a module path inside the plugin, like \"lib/util\"."
                )));
            }
            match modules.borrow().get(&name) {
                Some(Some(v)) => return Ok(v.clone()),
                Some(None) => {
                    return Err(mlua::Error::runtime(format!(
                        "require(\"{name}\"): circular require."
                    )));
                }
                None => {}
            }
            let file = format!("{name}.luau");
            let source = c
                .pause(|| read_module(&root, &file))
                .map_err(|e| mlua::Error::runtime(e.to_string()))?;
            modules.borrow_mut().insert(name.clone(), None);
            let loaded = lua
                .load(source)
                .set_name(format!("={file}"))
                .set_mode(ChunkMode::Text)
                .call::<Value>(());
            let v = match loaded {
                Ok(v) => v,
                Err(e) => {
                    modules.borrow_mut().remove(&name);
                    return Err(e);
                }
            };
            let v = if v.is_nil() { Value::Boolean(true) } else { v };
            modules.borrow_mut().insert(name, Some(v.clone()));
            Ok(v)
        })?,
    )?;

    // The write path behind `ctx.run`. Kept in the registry, not in globals.
    let (h, m, c) = (host, manifest.clone(), clock.clone());
    let run = lua.create_function(move |_, (account, id, arg): (f64, String, Option<Value>)| {
        if !m.declares(&id) {
            return Err(mlua::Error::runtime(format!(
                "\"{id}\" isn't in this plugin's actions in plugin.toml."
            )));
        }
        let arg = arg.map(|a| arg_text(&a)).transpose()?;
        c.pause(|| h.run(&m.id, account as u64, &id, arg.as_deref()))
            .map_err(mlua::Error::runtime)
    })?;
    lua.set_named_registry_value("bonsai.run", run)?;

    let bonsai = lua.create_table()?;
    let (reg, l) = (registered.clone(), loading.clone());
    bonsai.set(
        "command",
        lua.create_function(move |_, spec: Table| {
            if !l.get() {
                return Err(mlua::Error::runtime(
                    "bonsai.command only works while the plugin loads.",
                ));
            }
            let (cmd, run, event) = command_spec(&spec)?;
            let reg = &mut reg.borrow_mut().commands;
            if reg.len() >= MAX_COMMANDS {
                return Err(mlua::Error::runtime(format!(
                    "A plugin can have at most {MAX_COMMANDS} commands."
                )));
            }
            if reg.iter().any(|(c, _, _)| c.id == cmd.id) {
                return Err(mlua::Error::runtime(format!(
                    "Command \"{}\" is registered twice.",
                    cmd.id
                )));
            }
            reg.push((cmd, run, event));
            Ok(())
        })?,
    )?;
    let (reg, l, m) = (registered.clone(), loading.clone(), manifest.clone());
    bonsai.set(
        "on",
        lua.create_function(move |_, (event, f): (String, Function)| {
            if !l.get() {
                return Err(mlua::Error::runtime(
                    "bonsai.on only works while the plugin loads.",
                ));
            }
            if !EVENTS.contains(&event.as_str()) {
                return Err(mlua::Error::runtime(format!(
                    "bonsai.on: no event \"{event}\". Events are {}.",
                    EVENTS.join(", ")
                )));
            }
            // Events tell what happens on an account, which is private.
            if !m.has(Permission::ReadAccount) {
                return Err(mlua::Error::runtime(
                    "bonsai.on needs \"read.account\" in permissions in plugin.toml.",
                ));
            }
            let events = &mut reg.borrow_mut().events;
            if events.iter().any(|(e, _)| *e == event) {
                return Err(mlua::Error::runtime(format!(
                    "bonsai.on(\"{event}\") is registered twice."
                )));
            }
            events.push((event, f));
            Ok(())
        })?,
    )?;
    let (reg, l) = (registered.clone(), loading.clone());
    bonsai.set(
        "every",
        lua.create_function(move |_, (seconds, f): (f64, Function)| {
            if !l.get() {
                return Err(mlua::Error::runtime(
                    "bonsai.every only works while the plugin loads.",
                ));
            }
            if !(seconds.fract() == 0.0 && seconds >= MIN_EVERY as f64 && seconds <= MAX_EVERY as f64) {
                return Err(mlua::Error::runtime(format!(
                    "bonsai.every takes whole seconds from {MIN_EVERY} to {MAX_EVERY}."
                )));
            }
            let timers = &mut reg.borrow_mut().timers;
            if timers.len() >= MAX_TIMERS {
                return Err(mlua::Error::runtime(format!(
                    "A plugin can have at most {MAX_TIMERS} timers."
                )));
            }
            timers.push((Duration::from_secs(seconds as u64), f));
            Ok(())
        })?,
    )?;
    bonsai.set("ui", view::ui_table(lua)?)?;
    bonsai.set("api", crate::API_VERSION)?;
    bonsai.set("id", manifest.id.as_str())?;
    g.raw_set("bonsai", bonsai)?;
    Ok(())
}

fn command_spec(spec: &Table) -> mlua::Result<(Command, Function, Option<Function>)> {
    let id: String = spec.get("id")?;
    if !is_plugin_id(&id) {
        return Err(mlua::Error::runtime(format!(
            "Command id \"{id}\": use lowercase letters, digits and dashes."
        )));
    }
    let title: String = spec.get("title")?;
    let title = title.trim().to_owned();
    if title.is_empty() || title.chars().count() > 120 {
        return Err(mlua::Error::runtime(format!(
            "Command \"{id}\" needs a title of up to 120 characters."
        )));
    }
    let keywords = match spec.get::<Option<Table>>("keywords")? {
        Some(t) => t
            .sequence_values::<String>()
            .take(20)
            .collect::<mlua::Result<Vec<_>>>()?,
        None => Vec::new(),
    };
    let arg = match spec.get::<Option<String>>("arg")? {
        None => Arg::None,
        Some(a) => Arg::parse(&a).ok_or_else(|| {
            mlua::Error::runtime(format!(
                "Command \"{id}\": arg is none, text, user, game, item or account, not \"{a}\"."
            ))
        })?,
    };
    let context = spec.get::<Option<String>>("context")?;
    if let Some(c) = &context
        && !matches!(c.as_str(), "user" | "game" | "item" | "group")
    {
        return Err(mlua::Error::runtime(format!(
            "Command \"{id}\": context is user, game, item or group, not \"{c}\"."
        )));
    }
    let run: Function = spec
        .get::<Option<Function>>("run")?
        .ok_or_else(|| mlua::Error::runtime(format!("Command \"{id}\" has no run function.")))?;
    let event = spec.get::<Option<Function>>("event")?;
    Ok((
        Command {
            id,
            title,
            keywords,
            arg,
            context,
        },
        run,
        event,
    ))
}

/// Reads what a handler returned into an [`Outcome`], refusing undeclared actions.
fn outcome(manifest: &Manifest, value: Value) -> Result<Outcome, Error> {
    let t = match value {
        Value::Nil => return Ok(Outcome::Nothing),
        Value::Table(t) => t,
        other => {
            return Err(Error::Script(format!(
                "A handler returns nil or a table, not a {}.",
                other.type_name()
            )));
        }
    };
    let get = |k: &str| t.get::<Value>(k).map_err(script);
    if let Value::Table(list) = get("actions")? {
        let mut out = Vec::new();
        for item in list.sequence_values::<Table>() {
            let item = item.map_err(script)?;
            let id: String = item.get("id").map_err(script)?;
            if !manifest.declares(&id) {
                return Err(Error::Undeclared(id));
            }
            let arg = match item.get::<Value>("arg").map_err(script)? {
                Value::Nil => None,
                v => Some(arg_text(&v).map_err(script)?),
            };
            out.push(Action { id, arg });
        }
        return Ok(Outcome::Actions(out));
    }
    if let Value::String(s) = get("toast")? {
        return Ok(Outcome::Toast(s.to_str().map_err(script)?.to_owned()));
    }
    if let Value::Table(o) = get("open")? {
        let kind: String = o.get("kind").map_err(script)?;
        if !matches!(kind.as_str(), "user" | "game" | "item" | "group") {
            return Err(Error::Script(format!(
                "open.kind is user, game, item or group, not \"{kind}\"."
            )));
        }
        let id: f64 = o.get("id").map_err(script)?;
        if !(id >= 1.0 && id.fract() == 0.0 && id < (1u64 << 53) as f64) {
            return Err(Error::Script("open.id must be a whole number.".into()));
        }
        return Ok(Outcome::Open { kind, id: id as u64 });
    }
    if let Value::Table(a) = get("alert")? {
        return Ok(Outcome::Alert {
            title: a.get("title").map_err(script)?,
            body: a
                .get::<Option<String>>("body")
                .map_err(script)?
                .unwrap_or_default(),
            command: a.get("command").map_err(script)?,
        });
    }
    match get("view")? {
        Value::Nil => {}
        v => return Ok(Outcome::View(view::parse(&v).map_err(script)?)),
    }
    if get("close")? == Value::Boolean(true) {
        return Ok(Outcome::Close);
    }
    Err(Error::Script(
        "A handler returns nil or a table with actions, toast, open, alert, view or close.".into(),
    ))
}

fn arg_text(v: &Value) -> mlua::Result<String> {
    match v {
        Value::String(s) => Ok(s.to_str()?.to_owned()),
        Value::Integer(n) => Ok(n.to_string()),
        Value::Number(n) if n.fract() == 0.0 && n.abs() < (1u64 << 53) as f64 => Ok(format!("{n:.0}")),
        other => Err(mlua::Error::runtime(format!(
            "An action's arg is a string or a whole number, not a {}.",
            other.type_name()
        ))),
    }
}

/// Reads `rel` from the plugin folder, refusing anything that resolves outside it (symlinks).
fn read_module(root: &Path, rel: &str) -> Result<String, Error> {
    let path: PathBuf = root.join(rel);
    let real = path.canonicalize().map_err(|e| Error::Io(rel.to_owned(), e))?;
    if !real.starts_with(root) {
        return Err(Error::Script(format!("{rel} is outside the plugin folder.")));
    }
    std::fs::read_to_string(&real).map_err(|e| Error::Io(rel.to_owned(), e))
}

/// Maps a finished call to our errors. A trip of the clock wins even if the plugin caught it.
fn check<T>(
    clock: &Clock,
    lua: &Lua,
    limits: Limits,
    limit: Duration,
    ran: mlua::Result<T>,
) -> Result<T, Error> {
    if clock.tripped.get() {
        return Err(Error::Timeout(limit.as_millis()));
    }
    ran.map_err(|e| match root(&e) {
        mlua::Error::MemoryError(_) => {
            // Free what the failed call left behind before the next one.
            lua.gc_collect().ok();
            Error::Memory(limits.memory >> 20)
        }
        _ => script(e),
    })
}

fn root(e: &mlua::Error) -> &mlua::Error {
    match e {
        mlua::Error::CallbackError { cause, .. } => root(cause),
        mlua::Error::WithContext { cause, .. } => root(cause),
        e => e,
    }
}

/// A Luau error as one line: `main.luau:3: message`, without the traceback.
fn script(e: mlua::Error) -> Error {
    let msg = match root(&e) {
        mlua::Error::RuntimeError(m) | mlua::Error::SyntaxError { message: m, .. } => m.clone(),
        other => other.to_string(),
    };
    let line = msg.lines().next().unwrap_or_default();
    Error::Script(line.strip_prefix("runtime error: ").unwrap_or(line).to_owned())
}
