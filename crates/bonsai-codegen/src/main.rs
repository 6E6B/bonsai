//! `cargo xtask codegen [--check] [--strict]`
//!
//! Reads every `vendor/roblox-api-tracker/<host>/v*.json` and `specs/undocumented/*.yaml` and writes
//! `crates/bonsai-api/src/generated/` plus `coverage.json`.
//!
//! - `--check`: fail if the committed output differs from what would be generated.
//! - `--strict`: fail if an operation the app uses was removed or became deprecated.

mod emit;
mod naming;
mod spec;

use std::collections::{BTreeMap, BTreeSet};
use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use anyhow::{Context, Result, bail};
use serde_json::{Value, json};

fn main() {
    if let Err(e) = run() {
        eprintln!("error: {e:#}");
        std::process::exit(1);
    }
}

fn run() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match args.first().map(String::as_str) {
        Some("codegen") => {}
        _ => bail!("usage: cargo xtask codegen [--check] [--strict]"),
    }
    let check = args.iter().any(|a| a == "--check");
    let strict = args.iter().any(|a| a == "--strict");
    let root = repo_root()?;
    let out_dir = root.join("crates/bonsai-api/src/generated");
    let files = build(&root)?;
    if check {
        let mut stale = Vec::new();
        for (rel, content) in &files {
            let p = root.join(rel);
            if std::fs::read_to_string(&p)
                .ok()
                .map(|s| s.replace("\r\n", "\n"))
                .as_deref()
                != Some(content.as_str())
            {
                stale.push(rel.clone());
            }
        }
        let expected: BTreeSet<PathBuf> = files.keys().map(|k| root.join(k)).collect();
        for p in walk(&out_dir) {
            if !expected.contains(&p) {
                stale.push(p.strip_prefix(&root).unwrap_or(&p).display().to_string());
            }
        }
        if !stale.is_empty() {
            bail!(
                "generated code is stale, run `cargo xtask codegen`:\n  {}",
                stale.join("\n  ")
            );
        }
        println!("generated code is up to date ({} files)", files.len());
    } else {
        let old_coverage = std::fs::read_to_string(root.join("coverage.json")).ok();
        if strict && let Some(old) = &old_coverage {
            strict_check(old, &files["coverage.json"])?;
        }
        if out_dir.exists() {
            std::fs::remove_dir_all(&out_dir)?;
        }
        for (rel, content) in &files {
            let p = root.join(rel);
            if let Some(d) = p.parent() {
                std::fs::create_dir_all(d)?;
            }
            std::fs::write(&p, content)?;
        }
        println!("wrote {} files", files.len());
    }
    Ok(())
}

fn repo_root() -> Result<PathBuf> {
    let here = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    here.ancestors()
        .find(|p| p.join("AGENTS.md").exists() && p.join("crates").exists())
        .map(Path::to_path_buf)
        .context("cannot find the repository root")
}

fn walk(dir: &Path) -> Vec<PathBuf> {
    let mut out = Vec::new();
    let Ok(rd) = std::fs::read_dir(dir) else {
        return out;
    };
    for e in rd.flatten() {
        let p = e.path();
        if p.is_dir() {
            out.extend(walk(&p))
        } else {
            out.push(p)
        }
    }
    out
}

/// Every output file, relative to the repo root, already formatted.
fn build(root: &Path) -> Result<BTreeMap<String, String>> {
    let specs = spec::load_all(root)?;
    let mut files = BTreeMap::new();
    let mut hosts: BTreeMap<String, (String, Vec<String>)> = BTreeMap::new();
    let mut ops = Vec::new();
    for s in &specs {
        let out = emit::generate(s);
        let code = rustfmt(&out.code).with_context(|| format!("formatting output for {}", s.source))?;
        files.insert(
            format!("crates/bonsai-api/src/generated/{}/{}.rs", s.module, s.version),
            code,
        );
        hosts
            .entry(s.module.clone())
            .or_insert_with(|| (s.host.clone(), Vec::new()))
            .1
            .push(s.version.clone());
        ops.extend(out.ops);
    }
    let mut root_mod = String::from(
        "//! Generated Roblox API modules, one per host and version.\n//!\n//! Do not edit. Run `cargo xtask codegen`.\n\n",
    );
    for (module, (host, versions)) in &hosts {
        root_mod.push_str(&format!("pub mod {module};\n"));
        let mut m = format!("//! `{host}`\n\n");
        for v in versions {
            m.push_str(&format!("pub mod {v};\n"));
        }
        files.insert(
            format!("crates/bonsai-api/src/generated/{module}/mod.rs"),
            rustfmt(&m)?,
        );
    }
    files.insert(
        "crates/bonsai-api/src/generated/mod.rs".into(),
        rustfmt(&root_mod)?,
    );

    // Usage: the UI reaches the API only through bonsai-core, so that is where we look.
    let mut sources = String::new();
    for p in walk(&root.join("crates/bonsai-core/src")) {
        if p.extension().is_some_and(|e| e == "rs") {
            sources.push_str(&std::fs::read_to_string(&p).unwrap_or_default());
            sources.push('\n');
        }
    }
    let used = |o: &emit::OpInfo| {
        let a = format!("{}::{}::{}", o.module, o.version, o.function);
        let b = format!("{a}_request");
        contains_ident(&sources, &a) || contains_ident(&sources, &b)
    };
    let list: Vec<Value> = ops
        .iter()
        .map(|o| {
            json!({
                "host": o.host,
                "module": format!("{}::{}", o.module, o.version),
                "method": o.method,
                "path": o.path,
                "function": o.function,
                "deprecated": o.deprecated,
                "used": used(o),
                "summary": o.summary,
            })
        })
        .collect();
    let used_n = list.iter().filter(|v| v["used"] == json!(true)).count();
    let cov = json!({
        "total": list.len(),
        "used": used_n,
        "operations": list,
    });
    let mut text = serde_json::to_string_pretty(&cov)?;
    text.push('\n');
    files.insert("coverage.json".into(), text);
    Ok(files)
}

fn contains_ident(hay: &str, needle: &str) -> bool {
    let mut start = 0;
    while let Some(i) = hay[start..].find(needle) {
        let at = start + i;
        let end = at + needle.len();
        let next = hay[end..].chars().next();
        if !next.is_some_and(|c| c.is_ascii_alphanumeric() || c == '_') {
            return true;
        }
        start = end;
    }
    false
}

fn strict_check(old: &str, new: &str) -> Result<()> {
    let old: Value = serde_json::from_str(old)?;
    let new: Value = serde_json::from_str(new)?;
    let key = |v: &Value| format!("{} {} {}", v["host"], v["method"], v["path"]);
    let empty = Vec::new();
    let new_ops: BTreeMap<String, &Value> = new["operations"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .map(|v| (key(v), v))
        .collect();
    let mut problems = Vec::new();
    for o in old["operations"]
        .as_array()
        .unwrap_or(&empty)
        .iter()
        .filter(|v| v["used"] == json!(true))
    {
        match new_ops.get(&key(o)) {
            None => problems.push(format!("removed: {}", key(o))),
            Some(n) if n["deprecated"] == json!(true) && o["deprecated"] != json!(true) => {
                problems.push(format!("newly deprecated: {}", key(o)))
            }
            _ => {}
        }
    }
    if problems.is_empty() {
        Ok(())
    } else {
        bail!("operations in use changed:\n  {}", problems.join("\n  "))
    }
}

fn rustfmt(code: &str) -> Result<String> {
    let mut child = Command::new("rustfmt")
        .args(["--edition", "2024", "--emit", "stdout", "--quiet"])
        .arg("--config")
        .arg("max_width=110")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .context("running rustfmt (rustup component add rustfmt)")?;
    child
        .stdin
        .take()
        .context("rustfmt stdin")?
        .write_all(code.as_bytes())?;
    let out = child.wait_with_output()?;
    if !out.status.success() {
        let err = String::from_utf8_lossy(&out.stderr);
        let mut dump = std::env::temp_dir();
        dump.push("bonsai-codegen-failed.rs");
        let _ = std::fs::write(&dump, code);
        bail!("rustfmt failed ({}): {err}", dump.display());
    }
    Ok(String::from_utf8(out.stdout)?)
}
