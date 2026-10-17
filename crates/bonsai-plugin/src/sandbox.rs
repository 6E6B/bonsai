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
use crate::{Error, Manifest, is_plugin_id};

/// Most commands a plugin may register.
const MAX_COMMANDS: usize = 100;

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
    /// A line from `print`, for the developer log.
    fn log(&self, plugin: &str, line: &str);
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
#[derive(Clone, Debug, PartialEq, Eq)]
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

pub struct Plugin {
    lua: Lua,
    manifest: Rc<Manifest>,
    commands: Vec<Command>,
    handlers: HashMap<String, Function>,
    clock: Rc<Clock>,
    limits: Limits,
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

        let registered = Rc::new(RefCell::new(Vec::<(Command, Function)>::new()));
        let loading = Rc::new(Cell::new(true));
        install_globals(&lua, &manifest, host, &root, &clock, &registered, &loading).map_err(script)?;
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

        let mut commands = Vec::new();
        let mut handlers = HashMap::new();
        for (cmd, f) in registered.take() {
            handlers.insert(cmd.id.clone(), f);
            commands.push(cmd);
        }
        Ok(Plugin {
            lua,
            manifest,
            commands,
            handlers,
            clock,
            limits,
        })
    }

    pub fn manifest(&self) -> &Manifest {
        &self.manifest
    }

    pub fn commands(&self) -> &[Command] {
        &self.commands
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
        let ctx = self.ctx_table(ctx).map_err(script)?;
        self.clock.start(self.limits.call);
        let ran = f.call::<Value>((ctx, arg));
        self.clock.stop();
        let value = check(&self.clock, &self.lua, self.limits, self.limits.call, ran)?;
        outcome(&self.manifest, value)
    }

    fn ctx_table(&self, ctx: Ctx) -> mlua::Result<Table> {
        let t = self.lua.create_table()?;
        t.set("account", ctx.account as f64)?;
        t.set("now", ctx.now as f64)?;
        let run: Function = self.lua.named_registry_value("bonsai.run")?;
        // `ctx.run(id, arg)` acts as this context's account.
        let account = ctx.account;
        let bound = self
            .lua
            .create_function(move |_, (id, arg): (String, Option<Value>)| {
                run.call::<()>((account, id, arg))
            })?;
        t.set("run", bound)?;
        t.set_readonly(true);
        Ok(t)
    }
}

fn install_globals(
    lua: &Lua,
    manifest: &Rc<Manifest>,
    host: Rc<dyn Host>,
    root: &Path,
    clock: &Rc<Clock>,
    registered: &Rc<RefCell<Vec<(Command, Function)>>>,
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
    let (reg, loading) = (registered.clone(), loading.clone());
    bonsai.set(
        "command",
        lua.create_function(move |_, spec: Table| {
            if !loading.get() {
                return Err(mlua::Error::runtime(
                    "bonsai.command only works while the plugin loads.",
                ));
            }
            let (cmd, run) = command_spec(&spec)?;
            let mut reg = reg.borrow_mut();
            if reg.len() >= MAX_COMMANDS {
                return Err(mlua::Error::runtime(format!(
                    "A plugin can have at most {MAX_COMMANDS} commands."
                )));
            }
            if reg.iter().any(|(c, _)| c.id == cmd.id) {
                return Err(mlua::Error::runtime(format!(
                    "Command \"{}\" is registered twice.",
                    cmd.id
                )));
            }
            reg.push((cmd, run));
            Ok(())
        })?,
    )?;
    bonsai.set("api", crate::API_VERSION)?;
    bonsai.set("id", manifest.id.as_str())?;
    g.raw_set("bonsai", bonsai)?;
    Ok(())
}

fn command_spec(spec: &Table) -> mlua::Result<(Command, Function)> {
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
    Ok((
        Command {
            id,
            title,
            keywords,
            arg,
            context,
        },
        run,
    ))
}

/// Reads what a handler returned into an [`Outcome`], refusing undeclared actions.
fn outcome(manifest: &Manifest, value: Value) -> Result<Outcome, Error> {
    let t = match value {
        Value::Nil => return Ok(Outcome::Nothing),
        Value::Table(t) => t,
        other => {
            return Err(Error::Script(format!(
                "A command returns nil or a table, not a {}.",
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
    Err(Error::Script(
        "A command returns nil or a table with actions, toast, open or alert.".into(),
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
