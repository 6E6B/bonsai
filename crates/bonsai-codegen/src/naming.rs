//! Turning .NET-style model names and JSON keys into Rust identifiers.

use std::collections::{BTreeMap, HashMap, HashSet};

use heck::{ToSnakeCase, ToUpperCamelCase};

/// Names a generated type must never take.
pub const RESERVED_TYPES: &[&str] = &[
    "Option", "Result", "Vec", "String", "Box", "Self", "Request", "Client", "Value", "HashMap", "Error",
    "Method", "Default", "Ok", "Err", "Some", "None", "FilePart", "Response",
];

const RUST_KEYWORDS: &[&str] = &[
    "as", "break", "const", "continue", "crate", "else", "enum", "extern", "false", "fn", "for", "if",
    "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub", "ref", "return", "self", "static",
    "struct", "super", "trait", "true", "type", "unsafe", "use", "where", "while", "async", "await", "dyn",
    "abstract", "become", "box", "do", "final", "macro", "override", "priv", "typeof", "unsized", "virtual",
    "yield", "try", "gen", "client", "params", "body", "r",
];

fn clean(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { ' ' })
        .collect()
}

pub fn pascal(s: &str) -> String {
    let p = clean(s).to_upper_camel_case();
    if p.is_empty() {
        "X".into()
    } else if p.starts_with(|c: char| c.is_ascii_digit()) {
        format!("N{p}")
    } else {
        p
    }
}

pub fn snake(s: &str) -> String {
    let p = clean(s).to_snake_case();
    let p = if p.is_empty() { "x".into() } else { p };
    let p = if p.starts_with(|c: char| c.is_ascii_digit()) {
        format!("n{p}")
    } else {
        p
    };
    if RUST_KEYWORDS.contains(&p.as_str()) {
        format!("{p}_")
    } else {
        p
    }
}

/// Split `A.B.Page[C.D, E.F]` into base segments and generic arguments.
fn parse(name: &str) -> (Vec<String>, Vec<String>) {
    let (base, args) = match name.find('[') {
        Some(i) => {
            let inner = name[i + 1..].strip_suffix(']').unwrap_or(&name[i + 1..]);
            let mut args = Vec::new();
            let mut depth = 0;
            let mut cur = String::new();
            for c in inner.chars() {
                match c {
                    '[' => {
                        depth += 1;
                        cur.push(c)
                    }
                    ']' => {
                        depth -= 1;
                        cur.push(c)
                    }
                    ',' if depth == 0 => args.push(std::mem::take(&mut cur).trim().to_string()),
                    _ => cur.push(c),
                }
            }
            if !cur.trim().is_empty() {
                args.push(cur.trim().to_string());
            }
            (&name[..i], args)
        }
        None => (name, Vec::new()),
    };
    // Drop wrapper words that only add noise once the suffix is long enough to be unique.
    let segs = base
        .split(['.', '+'])
        .filter(|s| !s.is_empty() && !matches!(*s, "Roblox" | "Api" | "Models" | "Model"))
        .map(pascal)
        .collect();
    (segs, args)
}

fn display(name: &str, k: usize) -> String {
    let (segs, args) = parse(name);
    let k = k.min(segs.len()).max(1);
    let mut s: String = segs[segs.len().saturating_sub(k)..].concat();
    for a in args {
        s.push_str(&display(&a, 1));
    }
    if s.is_empty() { "Model".into() } else { s }
}

/// Map every definition name to a Rust type name: the shortest unique suffix of its dotted name.
pub fn definition_names(names: &[String]) -> BTreeMap<String, String> {
    let mut k: HashMap<&str, usize> = names.iter().map(|n| (n.as_str(), 1)).collect();
    for _ in 0..8 {
        let mut groups: HashMap<String, Vec<&str>> = HashMap::new();
        for n in names {
            groups
                .entry(display(n, k[n.as_str()]).to_lowercase())
                .or_default()
                .push(n);
        }
        let mut changed = false;
        for g in groups.values().filter(|g| g.len() > 1) {
            for n in g {
                let max = parse(n).0.len();
                if let Some(v) = k.get_mut(n)
                    && *v < max
                {
                    *v += 1;
                    changed = true;
                }
            }
        }
        if !changed {
            break;
        }
    }
    let mut out = BTreeMap::new();
    let mut used: HashSet<String> = HashSet::new();
    let mut sorted: Vec<&String> = names.iter().collect();
    sorted.sort();
    for n in sorted {
        let mut base = display(n, k[n.as_str()]);
        if RESERVED_TYPES.contains(&base.as_str()) {
            base.push_str("Model");
        }
        let mut cand = base.clone();
        let mut i = 2;
        while used.contains(&cand.to_lowercase()) {
            cand = format!("{base}{i}");
            i += 1;
        }
        used.insert(cand.to_lowercase());
        out.insert(n.clone(), cand);
    }
    out
}

/// Parse `['PlainText' = 0, 'Link' = 1]` out of an enum description.
pub fn enum_names(desc: &str) -> Vec<(String, i64)> {
    let mut out = Vec::new();
    let b = desc.as_bytes();
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'\''
            && let Some(end) = desc[i + 1..].find('\'')
        {
            {
                let name = &desc[i + 1..i + 1 + end];
                let rest = desc[i + 2 + end..].trim_start();
                if let Some(rest) = rest.strip_prefix('=') {
                    let rest = rest.trim_start();
                    let num: String = rest
                        .chars()
                        .enumerate()
                        .take_while(|(j, c)| c.is_ascii_digit() || (*j == 0 && *c == '-'))
                        .map(|(_, c)| c)
                        .collect();
                    if let Ok(n) = num.parse::<i64>() {
                        out.push((name.to_string(), n));
                    }
                }
                i += end + 2;
                continue;
            }
        }
        i += 1;
    }
    out
}

/// Rust-safe doc text on one line.
pub fn doc(s: &str) -> String {
    s.split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .replace('\\', "\\\\")
        .replace('[', "\\[")
        .replace(']', "\\]")
        .replace('<', "\\<")
        .replace('>', "\\>")
        .replace('`', "'")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn strips_prefix_and_keeps_shortest_unique_suffix() {
        let names = vec![
            "Roblox.Users.Api.Models.UserResponse".to_string(),
            "Roblox.Web.Responses.Users.UserResponse".to_string(),
            "Roblox.Users.Api.GetUserResponse".to_string(),
            "Roblox.Web.WebAPI.Models.ApiPageResponse[Roblox.Web.Responses.Games.GameServerResponse]"
                .to_string(),
        ];
        let m = definition_names(&names);
        assert_eq!(m["Roblox.Users.Api.GetUserResponse"], "GetUserResponse");
        assert_eq!(m["Roblox.Users.Api.Models.UserResponse"], "UsersUserResponse");
        assert_eq!(
            m["Roblox.Web.Responses.Users.UserResponse"],
            "ResponsesUsersUserResponse"
        );
        assert_eq!(
            m["Roblox.Web.WebAPI.Models.ApiPageResponse[Roblox.Web.Responses.Games.GameServerResponse]"],
            "ApiPageResponseGameServerResponse"
        );
    }

    #[test]
    fn parses_enum_descriptions() {
        assert_eq!(
            enum_names("The type ['PlainText' = 0, 'Link' = 1, 'Neg' = -1]"),
            vec![("PlainText".into(), 0), ("Link".into(), 1), ("Neg".into(), -1)]
        );
        assert!(enum_names("no enum here").is_empty());
    }

    #[test]
    fn identifiers() {
        assert_eq!(snake("userId"), "user_id");
        assert_eq!(snake("type"), "type_");
        assert_eq!(snake("2fa"), "n2fa");
        assert_eq!(pascal("game-passes"), "GamePasses");
    }
}
