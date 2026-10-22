//! Local fuzzy index for the command bar: commands, accounts, friends, recent and favorite
//! entities, open tabs and anything cached. Remote results are merged in by the caller.

use std::collections::HashMap;

use bonsai_api::AccountId;
use nucleo_matcher::pattern::{CaseMatching, Normalization, Pattern};
use nucleo_matcher::{Config, Matcher, Utf32Str};
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

#[derive(Clone, Debug, PartialEq)]
pub struct Hit {
    pub entry: Entry,
    pub score: u32,
}

/// The local index. Each source (commands, friends, tabs...) is replaced as a whole when it
/// changes, so rebuilding is incremental per source.
#[derive(Default)]
pub struct Index {
    sources: HashMap<&'static str, Vec<Entry>>,
}

impl Index {
    pub fn set_source(&mut self, name: &'static str, entries: Vec<Entry>) {
        self.sources.insert(name, entries);
    }

    pub fn len(&self) -> usize {
        self.sources.values().map(Vec::len).sum()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Fuzzy search. Results are grouped (groups ordered by their best hit), best match first.
    pub fn search(&self, query: &str, scope: Scope, per_group: usize) -> Vec<Hit> {
        let q = query.trim();
        let mut matcher = Matcher::new(Config::DEFAULT);
        let pattern = Pattern::parse(q, CaseMatching::Ignore, Normalization::Smart);
        let typo = TypoQuery::new(q);
        let mut buf = Vec::new();
        let mut hits: Vec<Hit> = Vec::new();
        for e in self.sources.values().flatten() {
            if !e.group.in_scope(scope) {
                continue;
            }
            let score = if q.is_empty() {
                Some(e.boost)
            } else {
                let mut best: Option<u32> = None;
                for text in std::iter::once(&e.title).chain(e.aliases.iter()) {
                    let s = pattern
                        .score(Utf32Str::new(text, &mut buf), &mut matcher)
                        .or_else(|| typo.as_ref().and_then(|typo| typo.score(text)));
                    best = best.max(s);
                }
                best.map(|s| s + e.boost)
            };
            if let Some(score) = score {
                hits.push(Hit {
                    entry: e.clone(),
                    score,
                });
            }
        }
        // Stable order: score, then title, so remote rows slotting in do not reshuffle ties.
        hits.sort_by(|a, b| {
            b.score
                .cmp(&a.score)
                .then_with(|| a.entry.title.cmp(&b.entry.title))
        });
        let mut best_per_group: Vec<(Group, u32)> = Vec::new();
        let mut counts: HashMap<Group, usize> = HashMap::new();
        let mut kept = Vec::new();
        for h in hits {
            let c = counts.entry(h.entry.group).or_default();
            if *c >= per_group {
                continue;
            }
            *c += 1;
            if !best_per_group.iter().any(|(g, _)| *g == h.entry.group) {
                best_per_group.push((h.entry.group, h.score));
            }
            kept.push(h);
        }
        if q.is_empty() {
            // Empty state keeps the fixed group order.
            best_per_group.sort_by_key(|(g, _)| *g);
        }
        let mut out = Vec::with_capacity(kept.len());
        for (g, _) in best_per_group {
            out.extend(kept.iter().filter(|h| h.entry.group == g).cloned());
        }
        out
    }
}

/// Typo tolerance: a word of the text within one edit (two for long queries) of the query.
/// Prepare the normalized query once per search, not once for every indexed entry.
struct TypoQuery {
    query: Vec<char>,
    max_distance: usize,
}

impl TypoQuery {
    fn new(query: &str) -> Option<Self> {
        let query: Vec<char> = query.to_lowercase().chars().collect();
        (query.len() >= 4).then_some(Self {
            max_distance: if query.len() >= 8 { 2 } else { 1 },
            query,
        })
    }

    fn score(&self, text: &str) -> Option<u32> {
        let lowercase = text.to_lowercase();
        let words = std::iter::once(lowercase.as_str())
            .chain(lowercase.split(|c: char| !c.is_alphanumeric()));
        words
            .filter(|word| !word.is_empty())
            .filter_map(|word| {
                let chars: Vec<char> = word.chars().collect();
                let prefix = &chars[..chars.len().min(self.query.len())];
                let mut distance = bounded_osa_distance(&self.query, prefix, self.max_distance);
                // The full word can beat its prefix when extra leading characters
                // are insertions. Only compare it separately when the word is longer.
                if chars.len() > self.query.len()
                    && let Some(full) =
                        bounded_osa_distance(&self.query, &chars, self.max_distance)
                {
                    distance = Some(distance.map_or(full, |d| d.min(full)));
                }
                distance.map(|d| 20u32.saturating_sub(d as u32 * 8))
            })
            .max()
    }
}

/// Optimal string alignment distance with an edit limit. Store only the three
/// rows needed for adjacent transpositions instead of allocating an n*m matrix.
/// The length bound also avoids computing distances that cannot be a typo hit.
fn bounded_osa_distance(a: &[char], b: &[char], max_distance: usize) -> Option<usize> {
    if a.len().abs_diff(b.len()) > max_distance {
        return None;
    }
    let mut previous_previous = vec![0; b.len() + 1];
    let mut previous: Vec<usize> = (0..=b.len()).collect();
    let mut current = vec![0; b.len() + 1];

    for i in 1..=a.len() {
        current[0] = i;
        for j in 1..=b.len() {
            let cost = usize::from(a[i - 1] != b[j - 1]);
            let mut distance = (previous[j] + 1)
                .min(current[j - 1] + 1)
                .min(previous[j - 1] + cost);
            if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                distance = distance.min(previous_previous[j - 2] + 1);
            }
            current[j] = distance;
        }
        std::mem::swap(&mut previous_previous, &mut previous);
        std::mem::swap(&mut previous, &mut current);
    }

    (previous[b.len()] <= max_distance).then_some(previous[b.len()])
}

#[cfg(test)]
mod tests {
    use super::*;

    fn e(group: Group, title: &str, target: Target) -> Entry {
        Entry {
            group,
            title: title.into(),
            subtitle: String::new(),
            aliases: vec![],
            target,
            boost: 0,
        }
    }

    fn index() -> Index {
        let mut i = Index::default();
        i.set_source(
            "commands",
            vec![
                e(
                    Group::Commands,
                    "Switch account",
                    Target::Command(CommandId::SwitchAccount),
                ),
                e(Group::Commands, "Go to Home", Target::Command(CommandId::GoHome)),
            ],
        );
        i.set_source(
            "friends",
            vec![Entry {
                aliases: vec!["builderman".into()],
                ..e(Group::Friends, "Builder Man", Target::User(156))
            }],
        );
        i.set_source(
            "recent",
            vec![e(
                Group::Experiences,
                "Adopt Me!",
                Target::Experience {
                    universe_id: 383310974,
                    place_id: 920587237,
                },
            )],
        );
        i
    }

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

    #[test]
    fn fuzzy_aliases_scope_and_typos() {
        let i = index();
        let h = i.search("bldrmn", Scope::All, 5);
        assert_eq!(h[0].entry.target, Target::User(156));
        assert!(i.search("switch", Scope::Users, 5).is_empty());
        assert_eq!(
            i.search("switch", Scope::Commands, 5)[0].entry.target,
            Target::Command(CommandId::SwitchAccount)
        );
        // transposition typo
        let h = i.search("adpot", Scope::All, 5);
        assert!(
            h.iter()
                .any(|h| matches!(h.entry.target, Target::Experience { .. })),
            "{h:?}"
        );
        // empty query keeps group order
        let h = i.search("", Scope::All, 5);
        assert_eq!(h[0].entry.group, Group::Friends);
    }

    #[test]
    fn bounded_osa_matches_reference_on_short_strings() {
        fn reference(a: &[char], b: &[char]) -> usize {
            let mut distances = vec![vec![0; b.len() + 1]; a.len() + 1];
            for (i, row) in distances.iter_mut().enumerate() {
                row[0] = i;
            }
            for (j, cell) in distances[0].iter_mut().enumerate() {
                *cell = j;
            }
            for i in 1..=a.len() {
                for j in 1..=b.len() {
                    let cost = usize::from(a[i - 1] != b[j - 1]);
                    let mut d = (distances[i - 1][j] + 1)
                        .min(distances[i][j - 1] + 1)
                        .min(distances[i - 1][j - 1] + cost);
                    if i > 1 && j > 1 && a[i - 1] == b[j - 2] && a[i - 2] == b[j - 1] {
                        d = d.min(distances[i - 2][j - 2] + 1);
                    }
                    distances[i][j] = d;
                }
            }
            distances[a.len()][b.len()]
        }

        let words = ["", "a", "b", "ab", "ba", "aba", "bba", "abba", "é", "éa"];
        for left in words {
            for right in words {
                let a: Vec<char> = left.chars().collect();
                let b: Vec<char> = right.chars().collect();
                let actual = reference(&a, &b);
                for max in 0..=2 {
                    assert_eq!(
                        bounded_osa_distance(&a, &b, max),
                        (actual <= max).then_some(actual),
                        "{left:?} -> {right:?}, max={max}"
                    );
                }
            }
        }
    }

    fn large_index() -> Index {
        let mut i = index();
        let many: Vec<Entry> = (0..5000)
            .map(|n| e(Group::Users, &format!("player_{n}_name"), Target::User(n)))
            .collect();
        i.set_source("cached", many);
        i
    }

    #[test]
    fn searches_large_index() {
        let i = large_index();
        let hits = i.search("plyr 42", Scope::All, 8);
        assert!(!hits.is_empty());
        let exact = i.search("player_42_name", Scope::Users, 8);
        assert!(exact.iter().any(|h| h.entry.target == Target::User(42)));
    }

    /// Host timing varies too much for a CI assertion. Run explicitly in an idle
    /// release build: cargo test -p bonsai-core --release fast_enough_for_one_frame -- --ignored
    #[test]
    #[ignore = "run manually in release; shared CI runners have unreliable wall-clock timing"]
    fn fast_enough_for_one_frame() {
        let i = large_index();
        let start = std::time::Instant::now();
        let hits = i.search("plyr 42", Scope::All, 8);
        let elapsed = start.elapsed();
        assert!(!hits.is_empty());
        let budget = if cfg!(debug_assertions) { 1000 } else { 16 };
        assert!(elapsed.as_millis() < budget, "{elapsed:?}");
    }

}
