//! Plugin views: a tree of our own components that a command returns as `{ view = ... }`. The
//! app draws it with its theme and keyboard handling; plugins never send HTML. Buttons, inputs
//! and table rows name an action, which comes back to the command's `event` function. See
//! `docs/plugins.md#views`.

use mlua::{Lua, Table, Value};

/// Most nodes in one view, counting every row, column, text and control.
pub const MAX_NODES: usize = 500;
/// How deep rows and columns may nest.
pub const MAX_DEPTH: usize = 8;
/// Most rows and columns in one table.
pub const MAX_ROWS: usize = 500;
pub const MAX_COLUMNS: usize = 12;
/// Longest text node, and longest label, cell or input value.
const MAX_TEXT: usize = 2000;
const MAX_LABEL: usize = 120;
const MAX_CELL: usize = 200;
const MAX_VALUE: usize = 500;

/// The component names `bonsai.ui` makes and a view may use.
pub const KINDS: [&str; 8] = [
    "text", "heading", "row", "column", "stat", "table", "input", "button",
];

/// One part of a view.
#[derive(Clone, Debug, PartialEq)]
pub enum Node {
    Text {
        text: String,
        muted: bool,
        mono: bool,
    },
    Heading(String),
    /// Side by side, wrapping when they don't fit.
    Row(Vec<Node>),
    /// One under another.
    Column(Vec<Node>),
    Stat {
        label: String,
        value: Cell,
    },
    Table {
        columns: Vec<TableColumn>,
        rows: Vec<Vec<Cell>>,
        /// Sent with the row's number when a row is picked.
        action: Option<String>,
    },
    Input {
        /// Its value comes back under this key in `e.values`.
        key: String,
        label: String,
        value: String,
        placeholder: String,
        /// Sent when Enter is pressed in it.
        action: Option<String>,
    },
    Button {
        label: String,
        action: String,
        style: ButtonStyle,
    },
}

/// A table cell or stat value. Numbers are drawn in the user's number format.
#[derive(Clone, Debug, PartialEq)]
pub enum Cell {
    Text(String),
    Number(f64),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct TableColumn {
    pub label: String,
    /// Right-aligned, for numbers.
    pub end: bool,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ButtonStyle {
    Default,
    Primary,
    Danger,
}

/// What the user did in a view, for the command's `event(ctx, e)`.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct ViewEvent {
    pub action: String,
    /// Each input's current text, by key.
    pub values: Vec<(String, String)>,
    /// The picked table row, from 1.
    pub row: Option<usize>,
}

/// Action names: lowercase letters, digits, `_` and `-`, at most 64.
pub fn is_action(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_lowercase() || b.is_ascii_digit() || b == b'_' || b == b'-')
}

/// Reads a view from what a handler returned.
pub(crate) fn parse(v: &Value) -> mlua::Result<Node> {
    let mut p = Parser::default();
    p.node(v, 0)
}

#[derive(Default)]
struct Parser {
    nodes: usize,
    keys: Vec<String>,
}

fn err(msg: impl Into<String>) -> mlua::Error {
    mlua::Error::runtime(format!("view: {}", msg.into()))
}

fn clip(s: &str, max: usize) -> String {
    s.chars().take(max).collect()
}

/// A string or number field as text.
fn text_of(v: &Value) -> mlua::Result<Option<String>> {
    Ok(match v {
        Value::Nil => None,
        Value::String(s) => Some(s.to_str()?.to_owned()),
        Value::Integer(n) => Some(n.to_string()),
        Value::Number(n) if n.fract() == 0.0 && n.abs() < (1u64 << 53) as f64 => Some(format!("{n:.0}")),
        Value::Number(n) => Some(n.to_string()),
        Value::Boolean(b) => Some(b.to_string()),
        other => return Err(err(format!("expected text, got a {}", other.type_name()))),
    })
}

fn cell(v: &Value) -> mlua::Result<Cell> {
    Ok(match v {
        Value::Nil => Cell::Text(String::new()),
        Value::Integer(n) => Cell::Number(*n as f64),
        Value::Number(n) if n.is_finite() => Cell::Number(*n),
        other => Cell::Text(clip(&text_of(other)?.unwrap_or_default(), MAX_CELL)),
    })
}

fn action(t: &Table, what: &str) -> mlua::Result<Option<String>> {
    match text_of(&t.get::<Value>("action")?)? {
        None => Ok(None),
        Some(a) if is_action(&a) => Ok(Some(a)),
        Some(a) => Err(err(format!(
            "{what} action \"{a}\": use lowercase letters, digits, _ and -."
        ))),
    }
}

impl Parser {
    fn count(&mut self) -> mlua::Result<()> {
        self.nodes += 1;
        if self.nodes > MAX_NODES {
            return Err(err(format!("at most {MAX_NODES} parts.")));
        }
        Ok(())
    }

    fn node(&mut self, v: &Value, depth: usize) -> mlua::Result<Node> {
        if depth > MAX_DEPTH {
            return Err(err(format!("rows and columns nest at most {MAX_DEPTH} deep.")));
        }
        self.count()?;
        let t = match v {
            Value::String(s) => {
                return Ok(Node::Text {
                    text: clip(&s.to_str()?, MAX_TEXT),
                    muted: false,
                    mono: false,
                });
            }
            Value::Table(t) => t,
            other => {
                return Err(err(format!(
                    "each part is a table from bonsai.ui or a string, not a {}.",
                    other.type_name()
                )));
            }
        };
        let kind = text_of(&t.get::<Value>("type")?)?.unwrap_or_default();
        // `ui.text("hi")` keeps the text in `text`; `ui.text { "hi", muted = true }` in [1].
        let body = || -> mlua::Result<String> {
            let v = match t.get::<Value>("text")? {
                Value::Nil => t.raw_get::<Value>(1)?,
                v => v,
            };
            Ok(text_of(&v)?.unwrap_or_default())
        };
        let label = |k: &str| -> mlua::Result<String> {
            Ok(clip(
                &text_of(&t.get::<Value>(k)?)?.unwrap_or_default(),
                MAX_LABEL,
            ))
        };
        Ok(match kind.as_str() {
            "text" => Node::Text {
                text: clip(&body()?, MAX_TEXT),
                muted: t.get::<Option<bool>>("muted")?.unwrap_or(false),
                mono: t.get::<Option<bool>>("mono")?.unwrap_or(false),
            },
            "heading" => Node::Heading(clip(&body()?, MAX_LABEL)),
            "row" | "column" => {
                let mut children = Vec::new();
                for c in t.sequence_values::<Value>() {
                    children.push(self.node(&c?, depth + 1)?);
                }
                if kind == "row" {
                    Node::Row(children)
                } else {
                    Node::Column(children)
                }
            }
            "stat" => Node::Stat {
                label: label("label")?,
                value: cell(&t.get::<Value>("value")?)?,
            },
            "table" => self.table(t)?,
            "input" => {
                let key = text_of(&t.get::<Value>("key")?)?.unwrap_or_default();
                if !is_action(&key) {
                    return Err(err(format!(
                        "input key \"{key}\": use lowercase letters, digits, _ and -."
                    )));
                }
                if self.keys.contains(&key) {
                    return Err(err(format!("two inputs have the key \"{key}\".")));
                }
                self.keys.push(key.clone());
                Node::Input {
                    label: label("label")?,
                    value: clip(
                        &text_of(&t.get::<Value>("value")?)?.unwrap_or_default(),
                        MAX_VALUE,
                    ),
                    placeholder: label("placeholder")?,
                    action: action(t, "input")?,
                    key,
                }
            }
            "button" => {
                let text = label("label")?;
                if text.trim().is_empty() {
                    return Err(err("a button needs a label."));
                }
                let style = match text_of(&t.get::<Value>("style")?)?.as_deref() {
                    None | Some("default") => ButtonStyle::Default,
                    Some("primary") => ButtonStyle::Primary,
                    Some("danger") => ButtonStyle::Danger,
                    Some(s) => {
                        return Err(err(format!(
                            "button style is default, primary or danger, not \"{s}\"."
                        )));
                    }
                };
                Node::Button {
                    action: action(t, "button")?
                        .ok_or_else(|| err(format!("button \"{text}\" needs an action.")))?,
                    label: text,
                    style,
                }
            }
            "" => return Err(err("a part has no type. Make parts with bonsai.ui.")),
            k => {
                return Err(err(format!("no part \"{k}\". Parts are {}.", KINDS.join(", "))));
            }
        })
    }

    fn table(&mut self, t: &Table) -> mlua::Result<Node> {
        let mut columns = Vec::new();
        if let Some(cols) = t.get::<Option<Table>>("columns")? {
            for c in cols.sequence_values::<Value>() {
                let col = match c? {
                    Value::Table(c) => TableColumn {
                        label: clip(
                            &text_of(&c.get::<Value>("label")?)?.unwrap_or_default(),
                            MAX_LABEL,
                        ),
                        end: match text_of(&c.get::<Value>("align")?)?.as_deref() {
                            None | Some("start") => false,
                            Some("end") => true,
                            Some(a) => {
                                return Err(err(format!("column align is start or end, not \"{a}\".")));
                            }
                        },
                    },
                    v => TableColumn {
                        label: clip(&text_of(&v)?.unwrap_or_default(), MAX_LABEL),
                        end: false,
                    },
                };
                columns.push(col);
            }
        }
        if columns.is_empty() || columns.len() > MAX_COLUMNS {
            return Err(err(format!("a table has 1 to {MAX_COLUMNS} columns.")));
        }
        let mut rows = Vec::new();
        if let Some(list) = t.get::<Option<Table>>("rows")? {
            for r in list.sequence_values::<Table>() {
                let r = r?;
                if rows.len() >= MAX_ROWS {
                    return Err(err(format!("a table has at most {MAX_ROWS} rows.")));
                }
                let mut cells = Vec::with_capacity(columns.len());
                for i in 1..=columns.len() {
                    cells.push(cell(&r.raw_get::<Value>(i)?)?);
                }
                rows.push(cells);
            }
        }
        Ok(Node::Table {
            columns,
            rows,
            action: action(t, "table")?,
        })
    }
}

/// `bonsai.ui`: one function per part, each tagging the table (or text) it is given.
pub(crate) fn ui_table(lua: &Lua) -> mlua::Result<Table> {
    let ui = lua.create_table()?;
    for kind in KINDS {
        ui.set(
            kind,
            lua.create_function(move |lua, v: Value| {
                let t = match v {
                    Value::Table(t) => t,
                    Value::String(_) | Value::Integer(_) | Value::Number(_)
                        if matches!(kind, "text" | "heading") =>
                    {
                        let t = lua.create_table()?;
                        t.raw_set("text", v)?;
                        t
                    }
                    Value::Nil => lua.create_table()?,
                    other => {
                        return Err(mlua::Error::runtime(format!(
                            "bonsai.ui.{kind} takes a table, not a {}.",
                            other.type_name()
                        )));
                    }
                };
                t.raw_set("type", kind)?;
                Ok(t)
            })?,
        )?;
    }
    ui.set_readonly(true);
    Ok(ui)
}

/// The event as the Luau table `{ action, values, row }`.
pub(crate) fn event_table(lua: &Lua, e: &ViewEvent) -> mlua::Result<Table> {
    let t = lua.create_table()?;
    t.set("action", e.action.as_str())?;
    let values = lua.create_table()?;
    for (k, v) in &e.values {
        values.set(k.as_str(), v.as_str())?;
    }
    t.set("values", values)?;
    if let Some(r) = e.row {
        t.set("row", r as f64)?;
    }
    Ok(t)
}
