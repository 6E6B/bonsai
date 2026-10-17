//! Fixture plugins against a fake host: escapes fail, limits trip, undeclared actions are refused.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::cell::RefCell;
use std::path::Path;
use std::rc::Rc;
use std::time::Duration;

use bonsai_plugin::{Action, Arg, Ctx, Error, Host, Limits, Manifest, Outcome, Plugin};

#[derive(Default)]
struct FakeHost {
    runs: RefCell<Vec<(u64, String, Option<String>)>>,
    logs: RefCell<Vec<String>>,
    /// How long `run` takes, to check host time isn't counted.
    delay: Duration,
    fail: Option<String>,
}

impl Host for FakeHost {
    fn run(&self, _plugin: &str, account: u64, action: &str, arg: Option<&str>) -> Result<(), String> {
        std::thread::sleep(self.delay);
        if let Some(e) = &self.fail {
            return Err(e.clone());
        }
        self.runs
            .borrow_mut()
            .push((account, action.to_owned(), arg.map(str::to_owned)));
        Ok(())
    }
    fn log(&self, _plugin: &str, line: &str) {
        self.logs.borrow_mut().push(line.to_owned());
    }
}

const MANIFEST: &str = r#"
id = "test"
entry = "main.luau"
actions = ["user.unfriend", "user.decline_friend"]
"#;

const CTX: Ctx = Ctx {
    account: 7,
    now: 1_800_000_000,
};

fn limits() -> Limits {
    Limits {
        call: Duration::from_millis(50),
        load: Duration::from_millis(250),
        memory: 8 << 20,
    }
}

struct Fixture {
    _dir: tempfile::TempDir,
    host: Rc<FakeHost>,
    result: Result<Plugin, Error>,
}

fn write(dir: &Path, files: &[(&str, &str)]) {
    for (name, text) in files {
        let path = dir.join(name);
        std::fs::create_dir_all(path.parent().unwrap()).unwrap();
        std::fs::write(path, text).unwrap();
    }
}

fn load_with(main: &str, files: &[(&str, &str)], host: FakeHost) -> Fixture {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("test");
    write(&dir, &[("main.luau", main)]);
    write(&dir, files);
    let host = Rc::new(host);
    let manifest = Manifest::parse(MANIFEST).unwrap();
    let result = Plugin::load(&dir, manifest, host.clone(), limits());
    Fixture {
        _dir: tmp,
        host,
        result,
    }
}

fn load(main: &str) -> Result<Plugin, Error> {
    load_with(main, &[], FakeHost::default()).result
}

/// A plugin with one command `c` whose body is `body`.
fn command(body: &str) -> Plugin {
    let main = format!(
        "bonsai.command {{ id = 'c', title = 'C', arg = 'text', run = function(ctx, arg)\n{body}\nend }}"
    );
    load(&main).unwrap()
}

fn run(body: &str) -> Result<Outcome, Error> {
    command(body).run_command("c", CTX, None)
}

fn script_error(r: Result<impl std::fmt::Debug, Error>) -> String {
    match r {
        Err(Error::Script(m)) => m,
        other => panic!("expected a script error, got {other:?}"),
    }
}

#[test]
fn registers_and_runs_commands() {
    let p = load(
        r#"
        bonsai.command {
          id = "unfriend-inactive",
          title = "Unfriend people offline more than 6 months",
          keywords = { "friends", "cleanup" },
          arg = "user",
          context = "user",
          run = function(ctx, arg)
            return { actions = { { id = "user.unfriend", arg = arg }, { id = "user.decline_friend", arg = "12" } } }
          end,
        }
        bonsai.command { id = "hello", title = "Hello", run = function(ctx) return { toast = "hi " .. ctx.account } end }
        "#,
    )
    .unwrap();
    let cmds = p.commands();
    assert_eq!(cmds.len(), 2);
    assert_eq!(cmds[0].keywords, vec!["friends", "cleanup"]);
    assert_eq!(
        (cmds[0].arg, cmds[0].context.as_deref()),
        (Arg::User, Some("user"))
    );
    assert_eq!(
        p.run_command("unfriend-inactive", CTX, Some("156")).unwrap(),
        Outcome::Actions(vec![
            Action {
                id: "user.unfriend".into(),
                arg: Some("156".into())
            },
            Action {
                id: "user.decline_friend".into(),
                arg: Some("12".into())
            },
        ])
    );
    assert_eq!(
        p.run_command("hello", CTX, None).unwrap(),
        Outcome::Toast("hi 7".into())
    );
    assert!(matches!(
        p.run_command("nope", CTX, None),
        Err(Error::NoCommand(_))
    ));
    assert!(matches!(
        p.run_command("unfriend-inactive", CTX, Some("bob")),
        Err(Error::Script(_))
    ));
}

#[test]
fn other_outcomes() {
    assert_eq!(run("return nil").unwrap(), Outcome::Nothing);
    assert_eq!(
        run("return { open = { kind = 'user', id = 1 } }").unwrap(),
        Outcome::Open {
            kind: "user".into(),
            id: 1
        }
    );
    assert_eq!(
        run("return { alert = { title = 'T', body = 'B', command = 'plugin.test.c' } }").unwrap(),
        Outcome::Alert {
            title: "T".into(),
            body: "B".into(),
            command: Some("plugin.test.c".into())
        }
    );
    script_error(run("return { open = { kind = 'file', id = 1 } }"));
    script_error(run("return 5"));
    script_error(run("return { what = 1 }"));
}

#[test]
fn bad_registrations_fail_to_load() {
    for main in [
        "bonsai.command { id = 'Bad', title = 'x', run = function() end }",
        "bonsai.command { id = 'a', title = '', run = function() end }",
        "bonsai.command { id = 'a', title = 'x' }",
        "bonsai.command { id = 'a', title = 'x', arg = 'file', run = function() end }",
        "bonsai.command { id = 'a', title = 'x', context = 'trade', run = function() end }",
        "for i = 1, 2 do bonsai.command { id = 'a', title = 'x', run = function() end } end",
        "for i = 1, 101 do bonsai.command { id = 'a' .. i, title = 'x', run = function() end } end",
        "error('boom')",
        "local x = ",
    ] {
        assert!(matches!(load(main), Err(Error::Script(_))), "{main}");
    }
    // Only while loading.
    let p = command("bonsai.command { id = 'late', title = 'x', run = function() end }");
    assert!(script_error(p.run_command("c", CTX, None)).contains("while the plugin loads"));
}

#[test]
fn folder_must_match_id() {
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("other");
    write(&dir, &[("main.luau", "")]);
    let m = Manifest::parse(MANIFEST).unwrap();
    assert!(matches!(
        Plugin::load(&dir, m, Rc::new(FakeHost::default()), limits()),
        Err(Error::Manifest(_))
    ));
}

#[test]
fn dangerous_globals_are_gone() {
    for name in [
        "os",
        "io",
        "debug",
        "getfenv",
        "setfenv",
        "loadstring",
        "load",
        "dofile",
        "loadfile",
        "package",
        "ffi",
        "jit",
        "collectgarbage_full",
    ] {
        assert_eq!(
            run(&format!("return {{ toast = type({name}) }}")).unwrap(),
            Outcome::Toast("nil".into()),
            "{name}"
        );
    }
}

#[test]
fn libraries_and_metatables_are_read_only() {
    for body in [
        "string.upper = function() end",
        "getmetatable('').__index = {}",
        "getmetatable('').__index.upper = nil",
        "table.insert = nil",
        "math.pi = 3",
        "bonsai.command = nil",
        "bonsai.id = 'someone-else'",
        "ctx.account = 1",
        "ctx.run = nil",
        "setmetatable(string, {})",
    ] {
        let err = script_error(run(body));
        assert!(err.contains("readonly"), "{body}: {err}");
    }
    // Globals a script sets don't leak into the shared ones.
    let p = command("x = (x or 0) + 1 return { toast = tostring(x) }");
    assert_eq!(p.run_command("c", CTX, None).unwrap(), Outcome::Toast("1".into()));
    assert_eq!(p.run_command("c", CTX, None).unwrap(), Outcome::Toast("2".into()));
}

#[test]
fn require_stays_in_the_folder() {
    let f = load_with(
        r#"
        local util = require("lib/util")
        bonsai.command { id = "c", title = "C", run = function() return { toast = util.hi() } end }
        bonsai.command { id = "up", title = "Up", arg = "text", run = function(ctx, m) return require(m) end }
        "#,
        &[("lib/util.luau", "return { hi = function() return 'hi' end }")],
        FakeHost::default(),
    );
    let p = f.result.unwrap();
    assert_eq!(
        p.run_command("c", CTX, None).unwrap(),
        Outcome::Toast("hi".into())
    );
    for path in [
        "../secret",
        "/etc/passwd",
        "lib/../main",
        "lib\\util",
        "Lib/Util",
        "missing",
    ] {
        assert!(
            matches!(p.run_command("up", CTX, Some(path)), Err(Error::Script(_))),
            "{path}"
        );
    }
}

#[cfg(unix)]
#[test]
fn require_refuses_symlinks_out() {
    let tmp = tempfile::tempdir().unwrap();
    std::fs::write(tmp.path().join("secret.luau"), "return { toast = 'leaked' }").unwrap();
    let dir = tmp.path().join("test");
    write(&dir, &[("main.luau", "local s = require('secret')")]);
    std::os::unix::fs::symlink(tmp.path().join("secret.luau"), dir.join("secret.luau")).unwrap();
    let m = Manifest::parse(MANIFEST).unwrap();
    let err = Plugin::load(&dir, m, Rc::new(FakeHost::default()), limits())
        .err()
        .unwrap();
    assert!(err.to_string().contains("outside the plugin folder"), "{err}");
}

#[test]
fn bytecode_is_not_loaded() {
    let bytecode = mlua::chunk::Compiler::new()
        .compile("return { toast = 'bytecode' }")
        .unwrap();
    let tmp = tempfile::tempdir().unwrap();
    let dir = tmp.path().join("test");
    write(&dir, &[("main.luau", "local m = require('evil')")]);
    std::fs::write(dir.join("evil.luau"), bytecode).unwrap();
    let m = Manifest::parse(MANIFEST).unwrap();
    assert!(Plugin::load(&dir, m, Rc::new(FakeHost::default()), limits()).is_err());
}

#[test]
fn time_limit_trips_even_if_caught() {
    assert!(matches!(run("while true do end"), Err(Error::Timeout(50))));
    assert!(matches!(
        run("for i = 1, 1e9 do pcall(function() while true do end end) end"),
        Err(Error::Timeout(50))
    ));
    assert!(matches!(
        run("local co = coroutine.create(function() while true do end end) coroutine.resume(co)"),
        Err(Error::Timeout(50)) | Err(Error::Script(_))
    ));
    assert!(matches!(load("while true do end"), Err(Error::Timeout(250))));
    // The VM still works afterwards.
    let p = command("if arg == 'spin' then while true do end end return { toast = 'ok' }");
    assert!(matches!(
        p.run_command("c", CTX, Some("spin")),
        Err(Error::Timeout(_))
    ));
    assert_eq!(
        p.run_command("c", CTX, None).unwrap(),
        Outcome::Toast("ok".into())
    );
}

#[test]
fn memory_limit_trips() {
    assert!(matches!(
        run("local s = string.rep('x', 64 * 1024 * 1024)"),
        Err(Error::Memory(8))
    ));
    assert!(matches!(
        run("local t = {} for i = 1, 1e7 do t[i] = { i } end"),
        Err(Error::Memory(8)) | Err(Error::Timeout(_))
    ));
    let p = command(
        "if arg == 'big' then local s = string.rep('x', 64 * 1024 * 1024) end return { toast = 'ok' }",
    );
    assert!(matches!(
        p.run_command("c", CTX, Some("big")),
        Err(Error::Memory(_))
    ));
    assert_eq!(
        p.run_command("c", CTX, None).unwrap(),
        Outcome::Toast("ok".into())
    );
}

#[test]
fn undeclared_actions_are_refused() {
    // Returned: the whole result is refused.
    assert!(matches!(
        run("return { actions = { { id = 'user.unfriend', arg = 1 }, { id = 'market.buy' } } }"),
        Err(Error::Undeclared(a)) if a == "market.buy"
    ));
    // Through ctx.run: a Luau error the plugin can catch, and the host never sees it.
    let f = load_with(
        r#"bonsai.command { id = "c", title = "C", run = function(ctx)
            local ok, err = pcall(ctx.run, "account.sign_out")
            ctx.run("user.unfriend", 156)
            return { toast = tostring(ok) .. " " .. tostring(err) }
        end }"#,
        &[],
        FakeHost::default(),
    );
    let p = f.result.unwrap();
    match p.run_command("c", CTX, None).unwrap() {
        Outcome::Toast(t) => assert!(t.starts_with("false") && t.contains("account.sign_out"), "{t}"),
        other => panic!("{other:?}"),
    }
    assert_eq!(
        *f.host.runs.borrow(),
        vec![(7, "user.unfriend".to_owned(), Some("156".to_owned()))]
    );
}

#[test]
fn host_errors_reach_the_plugin_and_host_time_is_free() {
    let main = r#"bonsai.command { id = "c", title = "C", run = function(ctx)
        for i = 1, 3 do ctx.run("user.unfriend", i) end
        return { toast = "done" }
    end }"#;
    // Three 40 ms host calls go past the 50 ms limit, but only Luau time counts.
    let slow = FakeHost {
        delay: Duration::from_millis(40),
        ..FakeHost::default()
    };
    let f = load_with(main, &[], slow);
    assert_eq!(
        f.result.unwrap().run_command("c", CTX, None).unwrap(),
        Outcome::Toast("done".into())
    );
    assert_eq!(f.host.runs.borrow().len(), 3);

    let failing = FakeHost {
        fail: Some("Automation is off for this account.".into()),
        ..FakeHost::default()
    };
    let f = load_with(main, &[], failing);
    let err = script_error(f.result.unwrap().run_command("c", CTX, None));
    assert!(err.contains("Automation is off"), "{err}");
}

#[test]
fn print_goes_to_the_host_log() {
    let f = load_with("print('loaded', 1, nil, true)", &[], FakeHost::default());
    assert!(f.result.is_ok());
    assert_eq!(*f.host.logs.borrow(), vec!["loaded\t1\tnil\ttrue"]);
}
