//! Loading specs: tracker Swagger 2.0 JSON, the one OpenAPI 3 file, and hand-written YAML.
//!
//! Everything is normalised into the Swagger 2.0 shape (`definitions`, `#/definitions/` refs,
//! body parameters) so the rest of the generator only knows one format.

use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use serde_json::{Map, Value, json};

/// One spec file, normalised.
pub struct Spec {
    /// e.g. `users.roblox.com`
    pub host: String,
    /// Rust module for the host, e.g. `users`.
    pub module: String,
    /// Rust module for the version, e.g. `v1`.
    pub version: String,
    /// Where it came from, relative to the repo root.
    pub source: String,
    #[allow(dead_code)]
    pub undocumented: bool,
    pub doc: Value,
}

pub fn load_all(root: &Path) -> Result<Vec<Spec>> {
    let mut out = Vec::new();
    let tracker = root.join("vendor/roblox-api-tracker");
    if !tracker.join("endpoints.json").exists() {
        bail!("vendor/roblox-api-tracker is empty; run `git submodule update --init`");
    }
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(&tracker)?
        .filter_map(|e| e.ok().map(|e| e.path()))
        .filter(|p| {
            p.is_dir()
                && !p
                    .file_name()
                    .is_some_and(|n| n.to_string_lossy().starts_with('.'))
        })
        .collect();
    dirs.sort();
    for dir in dirs {
        let folder = file_name(&dir);
        let mut files: Vec<PathBuf> = std::fs::read_dir(&dir)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| {
                let n = file_name(p);
                n.starts_with('v') && n.ends_with(".json")
            })
            .collect();
        files.sort_by_key(|p| version_number(&file_name(p)));
        for f in files {
            let text = std::fs::read_to_string(&f).with_context(|| format!("reading {}", f.display()))?;
            let mut doc: Value =
                serde_json::from_str(&text).with_context(|| format!("parsing {}", f.display()))?;
            if doc.get("openapi").is_some() {
                doc = openapi3_to_swagger2(doc);
            }
            let host = doc
                .get("host")
                .and_then(Value::as_str)
                .map(str::to_string)
                .unwrap_or_else(|| format!("{folder}.roblox.com"));
            let version = file_name(&f).trim_end_matches(".json").to_string();
            out.push(Spec {
                module: ident_module(&folder),
                host,
                version,
                source: rel(root, &f),
                undocumented: false,
                doc,
            });
        }
    }
    let und = root.join("specs/undocumented");
    if und.exists() {
        let mut files: Vec<PathBuf> = std::fs::read_dir(&und)?
            .filter_map(|e| e.ok().map(|e| e.path()))
            .filter(|p| file_name(p).ends_with(".yaml"))
            .collect();
        files.sort();
        for f in files {
            let text = std::fs::read_to_string(&f)?;
            let doc: Value =
                serde_norway::from_str(&text).with_context(|| format!("parsing {}", f.display()))?;
            let host = doc
                .get("host")
                .and_then(Value::as_str)
                .with_context(|| format!("{} has no host", f.display()))?
                .to_string();
            let module = ident_module(host.trim_end_matches(".roblox.com"));
            if out.iter().any(|s| s.module == module && s.host != host) {
                bail!(
                    "{}: module `{module}` is already used by another host",
                    f.display()
                );
            }
            out.push(Spec {
                module,
                host,
                version: "undocumented".into(),
                source: rel(root, &f),
                undocumented: true,
                doc,
            });
        }
    }
    Ok(out)
}

fn file_name(p: &Path) -> String {
    p.file_name()
        .map(|n| n.to_string_lossy().into_owned())
        .unwrap_or_default()
}

fn rel(root: &Path, p: &Path) -> String {
    p.strip_prefix(root)
        .unwrap_or(p)
        .to_string_lossy()
        .replace('\\', "/")
}

fn version_number(name: &str) -> u32 {
    name.trim_start_matches('v')
        .trim_end_matches(".json")
        .parse()
        .unwrap_or(0)
}

fn ident_module(s: &str) -> String {
    let m: String = s
        .chars()
        .map(|c| {
            if c.is_ascii_alphanumeric() {
                c.to_ascii_lowercase()
            } else {
                '_'
            }
        })
        .collect();
    if m.starts_with(|c: char| c.is_ascii_digit()) {
        format!("h{m}")
    } else {
        m
    }
}

/// Convert the OpenAPI 3 shapes the tracker uses into Swagger 2.0.
fn openapi3_to_swagger2(mut doc: Value) -> Value {
    fn fix_refs(v: &mut Value) {
        match v {
            Value::Object(m) => {
                if let Some(Value::String(r)) = m.get_mut("$ref") {
                    *r = r.replace("#/components/schemas/", "#/definitions/");
                }
                // `allOf: [{$ref}]` with a single entry is just a ref.
                if let Some(Value::Array(all)) = m.get("allOf")
                    && all.len() == 1
                    && let Some(r) = all[0].get("$ref").cloned()
                {
                    m.remove("allOf");
                    let r = match r {
                        Value::String(r) => {
                            Value::String(r.replace("#/components/schemas/", "#/definitions/"))
                        }
                        r => r,
                    };
                    m.insert("$ref".into(), r);
                }
                for v in m.values_mut() {
                    fix_refs(v);
                }
            }
            Value::Array(a) => a.iter_mut().for_each(fix_refs),
            _ => {}
        }
    }
    let defs = doc
        .pointer("/components/schemas")
        .cloned()
        .unwrap_or_else(|| json!({}));
    let mut out = Map::new();
    out.insert("swagger".into(), json!("2.0"));
    if let Some(i) = doc.get("info") {
        out.insert("info".into(), i.clone());
    }
    out.insert("definitions".into(), defs);
    let mut paths = doc.get_mut("paths").map(Value::take).unwrap_or_else(|| json!({}));
    if let Some(paths) = paths.as_object_mut() {
        for item in paths.values_mut() {
            let Some(item) = item.as_object_mut() else {
                continue;
            };
            for op in item.values_mut() {
                let Some(op) = op.as_object_mut() else { continue };
                // Parameters keep `schema`; Swagger 2 has the type inline.
                if let Some(Value::Array(params)) = op.get_mut("parameters") {
                    for p in params.iter_mut() {
                        if let Some(p) = p.as_object_mut()
                            && let Some(Value::Object(s)) = p.remove("schema")
                        {
                            for (k, v) in s {
                                p.entry(k).or_insert(v);
                            }
                        }
                    }
                }
                if let Some(rb) = op.remove("requestBody") {
                    let schema = first_content_schema(&rb);
                    let params = op.entry("parameters").or_insert_with(|| json!([]));
                    if let (Some(a), Some(schema)) = (params.as_array_mut(), schema) {
                        a.push(json!({"in": "body", "name": "body", "required": true, "schema": schema}));
                    }
                }
                if let Some(Value::Object(resps)) = op.get_mut("responses") {
                    for r in resps.values_mut() {
                        if let Some(schema) = first_content_schema(r)
                            && let Some(r) = r.as_object_mut()
                        {
                            r.remove("content");
                            r.insert("schema".into(), schema);
                        }
                    }
                }
            }
        }
    }
    out.insert("paths".into(), paths);
    let mut v = Value::Object(out);
    fix_refs(&mut v);
    v
}

fn first_content_schema(v: &Value) -> Option<Value> {
    let content = v.get("content")?.as_object()?;
    content
        .get("application/json")
        .or_else(|| content.values().next())
        .and_then(|c| c.get("schema"))
        .cloned()
}
