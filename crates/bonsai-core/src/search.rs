//! Command bar search types shared with Roblox search (`remote.rs`): scopes, result groups and
//! what a result opens. Ranking happens in the UI (`ui/src/lib/commands`).

use bonsai_api::AccountId;
use serde::{Deserialize, Serialize};

use crate::commands::CommandId;
use crate::resolve::Pasted;

/// Narrowing prefix typed at the start of the query.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Scope {
    All,
    /// `>`
    Commands,
    /// `@`
    Users,
    /// `#`
    Communities,
    /// `g:`
    Experiences,
    /// `i:`
    Items,
    /// `inv:`
    Inventory,
    /// `acc:`
    Accounts,
}

/// Split a scope prefix off the query.
pub fn parse_scope(input: &str) -> (Scope, &str) {
    let s = input.trim_start();
    for (p, scope) in [
        ("inv:", Scope::Inventory),
        ("acc:", Scope::Accounts),
        ("g:", Scope::Experiences),
        ("i:", Scope::Items),
        (">", Scope::Commands),
        ("@", Scope::Users),
        ("#", Scope::Communities),
    ] {
        if let Some(rest) = s.strip_prefix(p) {
            return (scope, rest.trim_start());
        }
    }
    (Scope::All, s)
}

/// Result groups, in default order.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum Group {
    Links,
    Tabs,
    Accounts,
    Friends,
    Recent,
    Experiences,
    Users,
    Communities,
    Items,
    Inventory,
    Commands,
}

impl Group {
    pub fn title(self) -> &'static str {
        match self {
            Group::Links => "Pasted link",
            Group::Tabs => "Open tabs",
            Group::Accounts => "Your accounts",
            Group::Friends => "Friends",
            Group::Recent => "Recent",
            Group::Experiences => "Experiences",
            Group::Users => "People",
            Group::Communities => "Communities",
            Group::Items => "Marketplace",
            Group::Inventory => "Your inventory",
            Group::Commands => "Commands",
        }
    }

    pub fn in_scope(self, s: Scope) -> bool {
        match s {
            Scope::All => true,
            Scope::Commands => self == Group::Commands,
            Scope::Users => matches!(self, Group::Users | Group::Friends),
            Scope::Communities => self == Group::Communities,
            Scope::Experiences => self == Group::Experiences,
            Scope::Items => self == Group::Items,
            Scope::Inventory => self == Group::Inventory,
            Scope::Accounts => self == Group::Accounts,
        }
    }
}

/// What a row opens.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Target {
    Command(CommandId),
    Account(AccountId),
    User(u64),
    Experience {
        universe_id: u64,
        place_id: u64,
    },
    Place(u64),
    Group(u64),
    Asset(u64),
    Bundle(u64),
    Tab(String),
    Pasted(PastedKey),
    /// A plain argument value (theme name, color, settings section).
    Value(String),
}

/// Hashable wrapper so pasted targets can sit in the index.
#[derive(Clone, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct PastedKey(pub String);

impl From<&Pasted> for PastedKey {
    fn from(p: &Pasted) -> Self {
        PastedKey(format!("{p:?}"))
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct Entry {
    pub group: Group,
    pub title: String,
    pub subtitle: String,
    /// Other names to match: username, aliases, nicknames.
    pub aliases: Vec<String>,
    pub target: Target,
    /// Added to the score: favorites, friends in game, recency.
    pub boost: u32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prefixes() {
        assert_eq!(parse_scope("> switch"), (Scope::Commands, "switch"));
        assert_eq!(parse_scope("@bob"), (Scope::Users, "bob"));
        assert_eq!(parse_scope("#club"), (Scope::Communities, "club"));
        assert_eq!(parse_scope("g: adopt"), (Scope::Experiences, "adopt"));
        assert_eq!(parse_scope("i:hat"), (Scope::Items, "hat"));
        assert_eq!(parse_scope("inv:hat"), (Scope::Inventory, "hat"));
        assert_eq!(parse_scope("acc:alt"), (Scope::Accounts, "alt"));
        assert_eq!(parse_scope("adopt"), (Scope::All, "adopt"));
    }
}
