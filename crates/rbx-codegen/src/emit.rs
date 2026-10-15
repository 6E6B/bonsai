//! One spec in, one Rust module out.

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fmt::Write as _;

use serde_json::{Map, Value};

use crate::naming::{self, RESERVED_TYPES, doc, pascal, snake};
use crate::spec::Spec;

/// One operation, for `coverage.json`.
#[derive(Clone)]
pub struct OpInfo {
    pub host: String,
    pub module: String,
    pub version: String,
    pub method: String,
    pub path: String,
    pub function: String,
    pub deprecated: bool,
    pub summary: String,
}

pub struct Output {
    pub code: String,
    pub ops: Vec<OpInfo>,
}

const METHODS: &[&str] = &["get", "post", "put", "patch", "delete"];
const MANAGED_HEADERS: &[&str] = &[
    "x-csrf-token",
    "cookie",
    "rblx-challenge-id",
    "rblx-challenge-type",
    "rblx-challenge-metadata",
];

struct Gen {
    defs: Map<String, Value>,
    def_names: BTreeMap<String, String>,
    /// lowercase names already in use in this module
    taken: HashSet<String>,
    types: Vec<String>,
    /// enum signature -> type name
    enums: HashMap<String, String>,
    /// definitions whose type has been emitted (or is being emitted)
    emitted: HashSet<String>,
}

pub fn generate(spec: &Spec) -> Output {
    let defs = spec
        .doc
        .get("definitions")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let names: Vec<String> = defs.keys().cloned().collect();
    let def_names = naming::definition_names(&names);
    let taken = def_names.values().map(|s| s.to_lowercase()).collect();
    let mut g = Gen {
        defs,
        def_names,
        taken,
        types: Vec::new(),
        enums: HashMap::new(),
        emitted: HashSet::new(),
    };
    let def_keys: Vec<String> = g.def_names.keys().cloned().collect();
    for k in &def_keys {
        g.emit_definition(k);
    }
    let mut fns = String::new();
    let mut ops = Vec::new();
    let mut fn_names: HashSet<String> = HashSet::new();
    let paths = spec
        .doc
        .get("paths")
        .and_then(Value::as_object)
        .cloned()
        .unwrap_or_default();
    let mut path_keys: Vec<&String> = paths.keys().collect();
    path_keys.sort();
    for path in path_keys {
        let item = &paths[path];
        let shared: Vec<Value> = item
            .get("parameters")
            .and_then(Value::as_array)
            .cloned()
            .unwrap_or_default();
        for m in METHODS {
            let Some(op) = item.get(*m) else { continue };
            let name = fn_name(m, path, op, &mut fn_names);
            let deprecated = op.get("deprecated").and_then(Value::as_bool).unwrap_or(false);
            let summary = op
                .get("summary")
                .and_then(Value::as_str)
                .unwrap_or("")
                .to_string();
            g.emit_op(&mut fns, m, path, op, &shared, &name, deprecated, &summary);
            ops.push(OpInfo {
                host: spec.host.clone(),
                module: spec.module.clone(),
                version: spec.version.clone(),
                method: m.to_uppercase(),
                path: path.clone(),
                function: name,
                deprecated,
                summary,
            });
        }
    }
    let mut code = String::new();
    let _ = writeln!(
        code,
        "//! `{}` {} — generated from `{}`.",
        spec.host, spec.version, spec.source
    );
    code.push_str(
        "//!\n//! Do not edit. Change the spec or `rbx-codegen` and run `cargo xtask codegen`.\n\n",
    );
    code.push_str("#![allow(deprecated, unused_imports, clippy::all)]\n\n");
    code.push_str("use std::collections::HashMap;\n\nuse serde::{Deserialize, Serialize};\n\n");
    code.push_str("use crate::rt::{Client, FilePart, Method, Request, Result};\n\n");
    let _ = writeln!(
        code,
        "/// Host for every operation in this module.\npub const HOST: &str = \"{}\";\n",
        spec.host
    );
    for t in &g.types {
        code.push_str(t);
        code.push('\n');
    }
    code.push_str(&fns);
    Output { code, ops }
}

fn fn_name(method: &str, path: &str, op: &Value, used: &mut HashSet<String>) -> String {
    let base = if let Some(n) = op.get("x-rust-name").and_then(Value::as_str) {
        n.to_string()
    } else {
        let mut parts = vec![method.to_string()];
        for seg in path.split('/').filter(|s| !s.is_empty()) {
            let is_version =
                seg.len() >= 2 && seg.starts_with('v') && seg[1..].chars().all(|c| c.is_ascii_digit());
            if is_version {
                continue;
            }
            if let Some(p) = seg.strip_prefix('{').and_then(|s| s.strip_suffix('}')) {
                parts.push(format!("by_{}", snake(p).trim_end_matches('_')));
            } else {
                parts.push(snake(seg).trim_end_matches('_').to_string());
            }
        }
        parts.join("_")
    };
    let mut name = base.clone();
    if used.contains(&name) {
        // Same path shape under two versions: keep the version to disambiguate.
        let v: String = path
            .split('/')
            .find(|s| s.starts_with('v') && s[1..].chars().all(|c| c.is_ascii_digit()) && s.len() >= 2)
            .unwrap_or("x")
            .to_string();
        name = format!("{base}_{v}");
        let mut i = 2;
        while used.contains(&name) {
            name = format!("{base}_{i}");
            i += 1;
        }
    }
    used.insert(name.clone());
    name
}

struct Param {
    rust: String,
    wire: String,
    ty: String,
    required: bool,
    loc: String,
    doc: String,
}

impl Gen {
    fn unique_type(&mut self, base: &str) -> String {
        let mut base = pascal(base);
        if RESERVED_TYPES.contains(&base.as_str()) {
            base.push_str("Model");
        }
        let mut cand = base.clone();
        let mut i = 2;
        while self.taken.contains(&cand.to_lowercase()) {
            cand = format!("{base}{i}");
            i += 1;
        }
        self.taken.insert(cand.to_lowercase());
        cand
    }

    fn emit_definition(&mut self, key: &str) {
        if !self.emitted.insert(key.to_string()) {
            return;
        }
        let Some(schema) = self.defs.get(key).cloned() else {
            return;
        };
        let name = self.def_names[key].clone();
        let is_object = schema.get("properties").is_some()
            || (schema.get("type").and_then(Value::as_str) == Some("object")
                && schema
                    .get("additionalProperties")
                    .is_none_or(|a| a == &Value::Bool(false)));
        if is_object && schema.get("enum").is_none() {
            self.emit_struct(&name, &schema, Some(key));
        } else if let Some(e) = self.enum_type(&schema, Some(&name)) {
            let _ = e;
        } else {
            let ty = self.type_of(&schema, &name, "Item");
            let d = doc(key);
            self.types.push(format!("/// `{d}`\npub type {name} = {ty};\n"));
        }
    }

    fn emit_struct(&mut self, name: &str, schema: &Value, spec_name: Option<&str>) {
        let required: HashSet<String> = schema
            .get("required")
            .and_then(Value::as_array)
            .map(|a| a.iter().filter_map(|v| v.as_str().map(str::to_string)).collect())
            .unwrap_or_default();
        let props = schema
            .get("properties")
            .and_then(Value::as_object)
            .cloned()
            .unwrap_or_default();
        let mut fields = String::new();
        let mut used: HashSet<String> = HashSet::new();
        for (wire, p) in &props {
            let mut rust = snake(wire);
            let mut i = 2;
            while !used.insert(rust.clone()) {
                rust = format!("{}{i}", snake(wire));
                i += 1;
            }
            let mut ty = self.type_of(p, name, wire);
            if ty == name {
                ty = format!("Box<{ty}>");
            }
            if let Some(d) = p.get("description").and_then(Value::as_str) {
                let _ = writeln!(fields, "    /// {}", doc(d));
            }
            if wire != &rust {
                let _ = writeln!(fields, "    #[serde(rename = \"{}\")]", wire.replace('"', "\\\""));
            }
            // An integer enum with no names in the spec: Roblox has switched some of these to
            // strings on the wire. Decode anything unexpected as `None` instead of failing.
            let loose_int = p.get("enum").is_some()
                && matches!(p.get("type").and_then(Value::as_str), Some("integer" | "number"))
                && !ty.starts_with(char::is_uppercase);
            if required.contains(wire) {
                let _ = writeln!(fields, "    pub {rust}: {ty},");
            } else {
                if loose_int {
                    fields.push_str("    #[serde(deserialize_with = \"crate::rt::lenient\")]\n");
                }
                fields.push_str("    #[serde(skip_serializing_if = \"Option::is_none\")]\n");
                let _ = writeln!(fields, "    pub {rust}: Option<{ty}>,");
            }
        }
        let mut s = String::new();
        if let Some(sn) = spec_name {
            let _ = writeln!(s, "/// `{}`", doc(sn));
        }
        if let Some(d) = schema.get("description").and_then(Value::as_str) {
            if spec_name.is_some() {
                s.push_str("///\n");
            }
            let _ = writeln!(s, "/// {}", doc(d));
        }
        s.push_str(
            "#[derive(Debug, Clone, Default, PartialEq, Serialize, Deserialize)]\n#[serde(default)]\n",
        );
        let _ = writeln!(s, "pub struct {name} {{\n{fields}}}");
        self.types.push(s);
    }

    /// If the schema is an enum we can name, emit it (once) and return its type name.
    fn enum_type(&mut self, schema: &Value, preferred: Option<&str>) -> Option<String> {
        let values = schema.get("enum")?.as_array()?;
        let ty = schema.get("type").and_then(Value::as_str).unwrap_or("");
        let desc = schema.get("description").and_then(Value::as_str).unwrap_or("");
        let (kind, variants): (&str, Vec<(String, i64, String)>) = if ty == "string" {
            let v: Vec<(String, i64, String)> = values
                .iter()
                .enumerate()
                .filter_map(|(i, v)| v.as_str().map(|s| (s.to_string(), i as i64, s.to_string())))
                .collect();
            ("str", v)
        } else {
            let named = naming::enum_names(desc);
            if named.is_empty() {
                return None;
            }
            ("int", named.into_iter().map(|(n, v)| (n.clone(), v, n)).collect())
        };
        if variants.is_empty() {
            return None;
        }
        let sig = format!(
            "{kind}:{}",
            variants
                .iter()
                .map(|(n, v, _)| format!("{n}={v}"))
                .collect::<Vec<_>>()
                .join(",")
        );
        if let Some(n) = self.enums.get(&sig) {
            return Some(n.clone());
        }
        let name = match preferred {
            Some(p) if self.def_names.values().any(|d| d == p) => p.to_string(),
            Some(p) => self.unique_type(p),
            None => self.unique_type("Enum"),
        };
        self.enums.insert(sig, name.clone());
        let mut body = String::new();
        let mut used = HashSet::new();
        for (n, v, wire) in &variants {
            let mut id = pascal(n);
            if id == "Unknown" {
                id = "UnknownValue".into();
            }
            let base = id.clone();
            let mut i = 2;
            while !used.insert(id.clone()) {
                id = format!("{base}{i}");
                i += 1;
            }
            let _ = writeln!(body, "    {id} = {v} => \"{}\",", wire.replace('"', "\\\""));
        }
        let d = if desc.is_empty() {
            String::new()
        } else {
            format!("    /// {}\n", doc(desc))
        };
        self.types.push(format!(
            "crate::api_enum! {{\n    {kind}\n{d}    {name} {{\n{body}    }}\n}}\n"
        ));
        Some(name)
    }

    /// Rust type for a schema. `owner` and `field` name any inline types we have to create.
    fn type_of(&mut self, s: &Value, owner: &str, field: &str) -> String {
        if let Some(r) = s.get("$ref").and_then(Value::as_str) {
            let key = r.trim_start_matches("#/definitions/");
            return match self.def_names.get(key).cloned() {
                Some(n) => {
                    self.emit_definition(key);
                    n
                }
                None => "serde_json::Value".into(),
            };
        }
        if s.get("enum").is_some()
            && let Some(n) = self.enum_type(s, Some(&format!("{owner}{}", pascal(field))))
        {
            return n;
        }
        match s.get("type").and_then(Value::as_str) {
            Some("integer") => match s.get("format").and_then(Value::as_str) {
                Some("int32") => "i32".into(),
                _ => "i64".into(),
            },
            Some("number") => "f64".into(),
            Some("boolean") => "bool".into(),
            Some("string") => "String".into(),
            Some("file") => "FilePart".into(),
            Some("array") => {
                let inner = s
                    .get("items")
                    .map(|i| self.type_of(i, owner, field))
                    .unwrap_or("serde_json::Value".into());
                format!("Vec<{inner}>")
            }
            Some("object") | None => {
                if s.get("properties")
                    .and_then(Value::as_object)
                    .is_some_and(|p| !p.is_empty())
                {
                    let name = self.unique_type(&format!("{owner}{}", pascal(field)));
                    self.emit_struct(&name, s, None);
                    name
                } else if let Some(ap) = s.get("additionalProperties").filter(|a| a.is_object()) {
                    let inner = self.type_of(ap, owner, field);
                    format!("HashMap<String, {inner}>")
                } else {
                    "serde_json::Value".into()
                }
            }
            Some(_) => "serde_json::Value".into(),
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn emit_op(
        &mut self,
        out: &mut String,
        method: &str,
        path: &str,
        op: &Value,
        shared: &[Value],
        name: &str,
        deprecated: bool,
        summary: &str,
    ) {
        let fn_pascal = pascal(name);
        let mut raw: Vec<Value> = shared.to_vec();
        raw.extend(
            op.get("parameters")
                .and_then(Value::as_array)
                .cloned()
                .unwrap_or_default(),
        );
        let mut params: Vec<Param> = Vec::new();
        let mut used: HashSet<String> = HashSet::new();
        let mut body_ty: Option<String> = None;
        for p in &raw {
            let loc = p.get("in").and_then(Value::as_str).unwrap_or("query").to_string();
            let wire = p.get("name").and_then(Value::as_str).unwrap_or("x").to_string();
            if loc == "header" && MANAGED_HEADERS.contains(&wire.to_lowercase().as_str()) {
                continue;
            }
            if loc == "body" {
                let schema = p.get("schema").cloned().unwrap_or(Value::Null);
                body_ty = Some(self.type_of(&schema, &fn_pascal, "Body"));
                continue;
            }
            let mut rust = snake(&wire);
            let mut i = 2;
            while !used.insert(rust.clone()) {
                rust = format!("{}{i}", snake(&wire));
                i += 1;
            }
            let required = loc == "path" || p.get("required").and_then(Value::as_bool).unwrap_or(false);
            let mut ty = self.type_of(p, &fn_pascal, &wire);
            // trades `/v1/trades/{tradeStatusType}` is typed integer but its pattern only allows
            // letters: a number routes to `/v1/trades/{tradeId}` instead. Send the name.
            if loc == "path"
                && p.get("type").and_then(Value::as_str) == Some("integer")
                && p.get("pattern").and_then(Value::as_str) == Some("^[A-Za-z]*$")
            {
                ty = "String".into();
            }
            let d = p
                .get("description")
                .and_then(Value::as_str)
                .map(doc)
                .unwrap_or_default();
            params.push(Param {
                rust,
                wire,
                ty,
                required,
                loc,
                doc: d,
            });
        }
        // Placeholders the spec forgot to declare still need an argument.
        let mut rest = path;
        while let Some(i) = rest.find('{') {
            let Some(j) = rest[i..].find('}') else { break };
            let pname = rest[i + 1..i + j].to_string();
            if !params
                .iter()
                .any(|p| p.loc == "path" && p.wire.eq_ignore_ascii_case(&pname))
            {
                let mut rust = snake(&pname);
                while !used.insert(rust.clone()) {
                    rust.push('_');
                }
                params.push(Param {
                    rust,
                    wire: pname,
                    ty: "String".into(),
                    required: true,
                    loc: "path".into(),
                    doc: String::new(),
                });
            }
            rest = &rest[i + j + 1..];
        }
        let resp_schema = op.get("responses").and_then(Value::as_object).and_then(|r| {
            let mut codes: Vec<&String> = r.keys().filter(|k| k.starts_with('2')).collect();
            codes.sort();
            codes.into_iter().find_map(|c| r[c].get("schema").cloned())
        });
        let ret = resp_schema.map(|s| self.type_of(&s, &fn_pascal, "Response"));

        // Optional inputs go in a params struct with Default, so new optional fields never break callers.
        let extra: Vec<&Param> = params.iter().filter(|p| p.loc != "path").collect();
        let params_ty = if extra.is_empty() {
            None
        } else {
            let n = self.unique_type(&format!("{fn_pascal}Params"));
            let mut s = format!(
                "/// Query, header and form inputs for [`{name}`].\n#[derive(Debug, Clone, Default)]\npub struct {n} {{\n"
            );
            for p in &extra {
                if !p.doc.is_empty() {
                    let _ = writeln!(s, "    /// {}", p.doc);
                }
                let ty = if p.required {
                    p.ty.clone()
                } else {
                    format!("Option<{}>", p.ty)
                };
                let _ = writeln!(s, "    pub {}: {ty},", p.rust);
            }
            s.push_str("}\n");
            self.types.push(s);
            Some(n)
        };

        let mut args = Vec::new();
        let mut call = Vec::new();
        for p in params.iter().filter(|p| p.loc == "path") {
            let ty = if p.ty == "String" {
                "&str".to_string()
            } else {
                p.ty.clone()
            };
            args.push(format!("{}: {ty}", p.rust));
            call.push(p.rust.clone());
        }
        if let Some(t) = &params_ty {
            args.push(format!("params: &{t}"));
            call.push("params".into());
        }
        if let Some(t) = &body_ty {
            args.push(format!("body: &{t}"));
            call.push("body".into());
        }
        let mut fmt_path = String::new();
        let mut fmt_args = Vec::new();
        let mut rest = path;
        while let Some(i) = rest.find('{') {
            fmt_path.push_str(&rest[..i]);
            let Some(j) = rest[i..].find('}') else { break };
            let pname = &rest[i + 1..i + j];
            let rust = params
                .iter()
                .find(|p| p.loc == "path" && p.wire.eq_ignore_ascii_case(pname))
                .map(|p| p.rust.clone());
            match rust {
                Some(r) => {
                    fmt_path.push_str("{}");
                    fmt_args.push(format!("crate::rt::path_value(&{r})"));
                }
                None => fmt_path.push_str(&rest[i..i + j + 1]),
            }
            rest = &rest[i + j + 1..];
        }
        fmt_path.push_str(rest);
        let path_expr = if fmt_args.is_empty() {
            format!("\"{fmt_path}\".to_string()")
        } else {
            format!("format!(\"{fmt_path}\", {})", fmt_args.join(", "))
        };

        let mut docs = format!("/// `{} {}`\n", method.to_uppercase(), path);
        if !summary.is_empty() {
            let _ = write!(docs, "///\n/// {}\n", doc(summary));
        }
        if let Some(d) = op.get("description").and_then(Value::as_str) {
            let _ = write!(docs, "///\n/// {}\n", doc(d));
        }
        if let Some(src) = op.get("x-captured").and_then(Value::as_str) {
            let _ = write!(docs, "///\n/// Undocumented; captured {}.\n", doc(src));
        }
        let dep = if deprecated {
            "#[deprecated(note = \"marked deprecated by Roblox\")]\n"
        } else {
            ""
        };
        let m = method.to_uppercase();
        let _ = write!(
            out,
            "{docs}{dep}pub fn {name}_request({}) -> Result<Request> {{\n    let {mutable}r = Request::new(Method::{m}, HOST, {path_expr});\n",
            args.join(", "),
            mutable = if extra.is_empty() && body_ty.is_none() {
                ""
            } else {
                "mut "
            },
        );
        for p in &extra {
            let f = match p.loc.as_str() {
                "header" => "header",
                "formData" if p.ty == "FilePart" => "form_file",
                "formData" => "form_text",
                _ => "query",
            };
            let _ = writeln!(out, "    r.{f}(\"{}\", &params.{});", p.wire, p.rust);
        }
        if body_ty.is_some() {
            out.push_str("    r.json(body)?;\n");
        }
        out.push_str("    Ok(r)\n}\n\n");
        let ret_ty = ret.clone().unwrap_or_else(|| "()".into());
        let finish = if ret.is_some() { ".json()" } else { ".unit()" };
        let mut client_args = vec!["client: &Client".to_string()];
        client_args.extend(args.iter().cloned());
        let _ = write!(
            out,
            "{docs}{dep}pub async fn {name}({}) -> Result<{ret_ty}> {{\n    client.send({name}_request({})?).await?{finish}\n}}\n\n",
            client_args.join(", "),
            call.join(", ")
        );
    }
}
