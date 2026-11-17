//! Playtime log: how long your own accounts spend in each experience, recorded from their own
//! presence while Bonsai runs. Kept per account in the local cache. Nothing in here writes to
//! Roblox; the only requests are the presence reads the background loop already makes.

use std::collections::HashMap;

use bonsai_api::AccountId;
use serde::{Deserialize, Serialize};

use parking_lot::Mutex;

use crate::cache::{Kind, now};
use crate::core::{Core, Inner};
use crate::store::{Presence, PresenceKind};

pub(crate) const PLAYTIME: &str = "playtime";

/// Sightings in the same experience at most this far apart join one session. Presence is read
/// every two minutes, so this allows one missed read.
pub const GAP_SECS: i64 = 5 * 60;

/// Sessions are kept this long after they end.
pub const KEEP_SECS: i64 = 400 * DAY;

/// At most this many sessions per account; the oldest go first.
const MAX: usize = 5000;

/// Held while a change to the log is made and saved, so the cache gets the same order of
/// changes as the store.
static SAVE: Mutex<()> = Mutex::new(());

const DAY: i64 = 24 * 60 * 60;
pub const WEEK: i64 = 7 * DAY;

/// A stretch of time one account was seen in one experience, in unix seconds.
#[derive(Clone, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Session {
    pub universe_id: u64,
    /// The experience's root place, to open or join it.
    #[serde(default)]
    pub place_id: u64,
    pub start: i64,
    pub end: i64,
    /// The account was seen out of the game after `end`, so the next sighting starts anew.
    #[serde(default, skip_serializing_if = "std::ops::Not::not")]
    pub closed: bool,
}

/// Drop sessions that ended more than [`KEEP_SECS`] before `at`. True when anything went.
pub fn prune(list: &mut Vec<Session>, at: i64) -> bool {
    let n = list.len();
    list.retain(|s| at - s.end < KEEP_SECS);
    list.len() != n
}

/// A session last seen at `end` was over by `at`: it ended somewhere in between, so credit half
/// of that time, when it is short enough to be one missed or regular read.
fn finish(last: &mut Session, at: i64) {
    if at > last.end && at - last.end <= GAP_SECS {
        last.end += (at - last.end) / 2;
    }
}

/// Note the account's own presence `p` at `at`. In an experience, it extends the last session
/// when that is the same experience and recent enough, or starts a new one. Out of one, it
/// closes the last session. True when the list changed.
pub fn note(list: &mut Vec<Session>, p: &Presence, at: i64) -> bool {
    let mut changed = prune(list, at);
    let universe_id = match (p.kind, p.universe_id) {
        (PresenceKind::InGame, Some(u)) if u > 0 => u,
        // A hidden or unknown experience says nothing either way.
        (PresenceKind::InGame, _) => return changed,
        _ => {
            if let Some(last) = list.last_mut().filter(|l| !l.closed) {
                finish(last, at);
                last.closed = true;
                changed = true;
            }
            return changed;
        }
    };
    if let Some(last) = list.last_mut()
        && !last.closed
        && last.universe_id == universe_id
        && at >= last.end
        && at - last.end <= GAP_SECS
    {
        if at == last.end {
            return changed;
        }
        last.end = at;
        return true;
    }
    // Moved straight to another experience: the last one ended on the way.
    if let Some(last) = list
        .last_mut()
        .filter(|l| !l.closed && l.universe_id != universe_id)
    {
        finish(last, at);
        last.closed = true;
    }
    list.push(Session {
        universe_id,
        place_id: p.root_place_id.or(p.place_id).unwrap_or(0),
        start: at,
        end: at,
        closed: false,
    });
    if list.len() > MAX {
        list.drain(..list.len() - MAX);
    }
    true
}

/// The start of the week (Monday 00:00 local) holding `t`, where local time is `offset` seconds
/// ahead of UTC.
pub fn week_start(t: i64, offset: i64) -> i64 {
    let days = (t + offset).div_euclid(DAY);
    // 1970-01-01 was a Thursday, three days after a Monday.
    let monday = days - (days + 3).rem_euclid(7);
    monday * DAY - offset
}

/// A span of the log to sum up.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Range {
    /// Since Monday.
    Week,
    /// This week and the three before.
    Month,
    /// This week and the eleven before.
    Quarter,
    /// This week and the 51 before.
    Year,
    All,
}

impl Range {
    pub const ALL: [Range; 5] = [Range::Week, Range::Month, Range::Quarter, Range::Year, Range::All];

    pub fn name(self) -> &'static str {
        match self {
            Range::Week => "week",
            Range::Month => "month",
            Range::Quarter => "quarter",
            Range::Year => "year",
            Range::All => "all",
        }
    }

    pub fn parse(s: &str) -> Option<Range> {
        Range::ALL.into_iter().find(|r| r.name() == s.trim())
    }

    fn weeks(self) -> Option<i64> {
        match self {
            Range::Week => Some(1),
            Range::Month => Some(4),
            Range::Quarter => Some(12),
            Range::Year => Some(52),
            Range::All => None,
        }
    }

    /// Where the range starts, for the week holding `at`. `None` means the whole log.
    pub fn start(self, at: i64, offset: i64) -> Option<i64> {
        self.weeks().map(|n| week_start(at, offset) - (n - 1) * WEEK)
    }
}

/// Time in one experience over a range.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct ExperienceTotal {
    pub universe_id: u64,
    pub place_id: u64,
    pub secs: i64,
    pub sessions: u32,
    pub last_played: i64,
    /// Seconds per account, most first.
    pub accounts: Vec<(AccountId, i64)>,
}

/// The log summed over a range.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Breakdown {
    pub secs: i64,
    pub sessions: u32,
    /// Most played first.
    pub experiences: Vec<ExperienceTotal>,
    /// Seconds per account, most first; accounts with none are left out.
    pub accounts: Vec<(AccountId, i64)>,
    /// Seconds per week, oldest first, every week of the range (from the first session for
    /// [`Range::All`]) including empty ones.
    pub weeks: Vec<(i64, i64)>,
}

fn sort_desc(m: HashMap<AccountId, i64>) -> Vec<(AccountId, i64)> {
    let mut v: Vec<_> = m.into_iter().filter(|(_, s)| *s > 0).collect();
    v.sort_by(|a, b| b.1.cmp(&a.1).then(a.0.0.cmp(&b.0.0)));
    v
}

/// Sums `logs` over `from..=to` (all of it before `to` when `from` is `None`), splitting
/// sessions at week starts. Sessions are clipped to the range; a session counts when any of
/// it falls inside.
pub fn breakdown(logs: &[(AccountId, &[Session])], from: Option<i64>, to: i64, offset: i64) -> Breakdown {
    let mut out = Breakdown::default();
    let mut games: HashMap<u64, (ExperienceTotal, HashMap<AccountId, i64>)> = HashMap::new();
    let mut accounts: HashMap<AccountId, i64> = HashMap::new();
    let mut weeks: HashMap<i64, i64> = HashMap::new();
    let mut first = to;
    for (account, list) in logs {
        for s in list.iter() {
            let a = from.map_or(s.start, |f| s.start.max(f));
            let b = s.end.min(to);
            if b < a {
                continue;
            }
            first = first.min(a);
            let secs = b - a;
            out.secs += secs;
            out.sessions += 1;
            *accounts.entry(*account).or_default() += secs;
            let (g, per) = games.entry(s.universe_id).or_insert_with(|| {
                (
                    ExperienceTotal {
                        universe_id: s.universe_id,
                        ..Default::default()
                    },
                    HashMap::new(),
                )
            });
            g.secs += secs;
            g.sessions += 1;
            if b >= g.last_played {
                g.last_played = b;
                if s.place_id > 0 {
                    g.place_id = s.place_id;
                }
            }
            *per.entry(*account).or_default() += secs;
            let mut t = a;
            while t < b {
                let w = week_start(t, offset);
                let next = (w + WEEK).min(b);
                *weeks.entry(w).or_default() += next - t;
                t = next;
            }
        }
    }
    out.accounts = sort_desc(accounts);
    out.experiences = games
        .into_values()
        .map(|(mut g, per)| {
            g.accounts = sort_desc(per);
            g
        })
        .collect();
    out.experiences.sort_by(|a, b| {
        b.secs
            .cmp(&a.secs)
            .then(b.last_played.cmp(&a.last_played))
            .then(a.universe_id.cmp(&b.universe_id))
    });
    let last = week_start(to, offset);
    let mut w = week_start(from.unwrap_or(first), offset);
    while w <= last {
        out.weeks.push((w, weeks.get(&w).copied().unwrap_or(0)));
        w += WEEK;
    }
    out
}

impl Inner {
    /// A fresh presence for `account` itself: add it to the playtime log, unless that is off.
    pub(crate) fn note_playtime(&self, account: AccountId, p: &Presence) {
        if account.is_anonymous() || !self.settings.read().playtime_log {
            return;
        }
        let _save = SAVE.lock();
        let snapshot = {
            let mut s = self.store.write();
            let d = s.data.entry(account).or_default();
            if !note(&mut d.playtime, p, now()) {
                return;
            }
            d.playtime.clone()
        };
        self.cache.put(Kind::Local, account, PLAYTIME, &snapshot);
    }
}

impl Core {
    /// Your accounts' playtime over `range`, where local time is `offset` seconds ahead of UTC.
    /// `account` narrows the experiences and weeks to one account; the per-account totals
    /// always cover all of them. Asks for any experience names that are missing.
    pub fn playtime(&self, range: Range, account: Option<AccountId>, offset: i64) -> Breakdown {
        let at = now();
        let logs: Vec<(AccountId, Vec<Session>)> = {
            let s = self.store();
            s.accounts
                .accounts
                .iter()
                .filter(|a| !a.id.is_anonymous())
                .map(|a| {
                    let list = s.data.get(&a.id).map(|d| d.playtime.clone()).unwrap_or_default();
                    (a.id, list)
                })
                .collect()
        };
        let all: Vec<(AccountId, &[Session])> = logs.iter().map(|(a, l)| (*a, l.as_slice())).collect();
        let from = range.start(at, offset);
        let mut out = match account {
            Some(one) => {
                let mine: Vec<_> = all.iter().filter(|(a, _)| *a == one).cloned().collect();
                let mut b = breakdown(&mine, from, at, offset);
                b.accounts = breakdown(&all, from, at, offset).accounts;
                b
            }
            None => breakdown(&all, from, at, offset),
        };
        if range == Range::All && out.weeks.is_empty() {
            out.weeks.push((week_start(at, offset), 0));
        }
        let ids: Vec<u64> = out.experiences.iter().map(|g| g.universe_id).collect();
        self.want_universes(&ids);
        out
    }

    /// Deletes the playtime log of every account.
    pub fn clear_playtime(&self) {
        let _save = SAVE.lock();
        let ids: Vec<AccountId> = {
            let mut s = self.inner.store.write();
            s.data
                .iter_mut()
                .filter(|(_, d)| !d.playtime.is_empty())
                .map(|(id, d)| {
                    d.playtime.clear();
                    *id
                })
                .collect()
        };
        for id in ids {
            self.inner
                .cache
                .put(Kind::Local, id, PLAYTIME, &Vec::<Session>::new());
        }
        drop(_save);
        self.inner.repaint();
        self.inner.toast(None, "Playtime log cleared.", false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn in_game(universe: u64) -> Presence {
        Presence {
            kind: PresenceKind::InGame,
            place_id: Some(universe * 10),
            root_place_id: Some(universe * 10),
            universe_id: Some(universe),
            job_id: None,
            last_location: String::new(),
        }
    }

    fn online() -> Presence {
        Presence {
            kind: PresenceKind::Online,
            ..Default::default()
        }
    }

    fn session(universe_id: u64, start: i64, end: i64) -> Session {
        Session {
            universe_id,
            place_id: universe_id * 10,
            start,
            end,
            closed: false,
        }
    }

    #[test]
    fn sightings_close_together_make_one_session() {
        let mut l = Vec::new();
        assert!(note(&mut l, &in_game(1), 1000));
        assert!(note(&mut l, &in_game(1), 1120));
        assert!(!note(&mut l, &in_game(1), 1120));
        assert!(note(&mut l, &in_game(1), 1240));
        assert_eq!(l, vec![session(1, 1000, 1240)]);
    }

    #[test]
    fn a_long_gap_or_another_game_starts_a_new_session() {
        let mut l = Vec::new();
        note(&mut l, &in_game(1), 1000);
        note(&mut l, &in_game(1), 1000 + GAP_SECS + 1);
        note(&mut l, &in_game(2), 1000 + GAP_SECS + 60);
        assert_eq!(l.len(), 3);
        assert_eq!(l[2].universe_id, 2);
        assert_eq!(l[2].place_id, 20);
    }

    #[test]
    fn leaving_the_game_closes_the_session() {
        let mut l = Vec::new();
        note(&mut l, &in_game(1), 1000);
        note(&mut l, &in_game(1), 1100);
        assert!(note(&mut l, &online(), 1200));
        assert!(!note(&mut l, &online(), 1300));
        note(&mut l, &in_game(1), 1320);
        assert_eq!(l.len(), 2);
        assert!(l[0].closed);
        // Last seen at 1100, gone at 1200: half the gap counts.
        assert_eq!((l[0].start, l[0].end), (1000, 1150));
        assert_eq!((l[1].start, l[1].end), (1320, 1320));
    }

    #[test]
    fn one_sighting_counts_until_halfway_to_the_next_read() {
        let mut l = Vec::new();
        note(&mut l, &in_game(1), 1000);
        note(&mut l, &in_game(2), 1120);
        note(&mut l, &online(), 1240);
        assert_eq!((l[0].start, l[0].end, l[0].closed), (1000, 1060, true));
        assert_eq!((l[1].start, l[1].end, l[1].closed), (1120, 1180, true));
        // A gap longer than one missed read says nothing about when it ended.
        note(&mut l, &in_game(3), 2000);
        note(&mut l, &online(), 2000 + GAP_SECS + 1);
        assert_eq!(l[2].end, 2000);
    }

    #[test]
    fn hidden_experience_is_ignored() {
        let mut l = Vec::new();
        let hidden = Presence {
            universe_id: None,
            ..in_game(1)
        };
        assert!(!note(&mut l, &hidden, 1000));
        assert!(l.is_empty());
    }

    #[test]
    fn old_sessions_are_pruned() {
        let mut l = vec![session(1, 0, 10), session(2, KEEP_SECS, KEEP_SECS + 10)];
        assert!(prune(&mut l, KEEP_SECS + 20));
        assert_eq!(l.len(), 1);
        assert_eq!(l[0].universe_id, 2);
    }

    #[test]
    fn weeks_start_on_local_monday() {
        // 2024-01-03 12:00 UTC, a Wednesday; Monday 2024-01-01 00:00 UTC is 1704067200.
        let wed = 1_704_283_200;
        assert_eq!(week_start(wed, 0), 1_704_067_200);
        // Two hours ahead of UTC, the local Monday starts two hours earlier in UTC.
        assert_eq!(week_start(wed, 7200), 1_704_067_200 - 7200);
        // Sunday 23:00 local belongs to the week before.
        let sunday_late = 1_704_067_200 - 3600;
        assert_eq!(week_start(sunday_late, 0), 1_704_067_200 - WEEK);
        assert_eq!(week_start(1_704_067_200, 0), 1_704_067_200);
    }

    #[test]
    fn range_starts_whole_weeks_back() {
        let wed = 1_704_283_200;
        let monday = 1_704_067_200;
        assert_eq!(Range::Week.start(wed, 0), Some(monday));
        assert_eq!(Range::Month.start(wed, 0), Some(monday - 3 * WEEK));
        assert_eq!(Range::All.start(wed, 0), None);
        assert_eq!(Range::parse("quarter"), Some(Range::Quarter));
        assert_eq!(Range::parse("nope"), None);
    }

    #[test]
    fn breakdown_sums_by_experience_account_and_week() {
        let monday = 1_704_067_200;
        let a = vec![
            session(1, monday - 1800, monday + 1800), // crosses into this week
            session(2, monday + 3600, monday + 3600 + 600),
        ];
        let b = vec![session(1, monday + 7200, monday + 7200 + 1200)];
        let logs = [(AccountId(7), a.as_slice()), (AccountId(8), b.as_slice())];
        let to = monday + DAY;

        let all = breakdown(&logs, None, to, 0);
        assert_eq!(all.secs, 3600 + 600 + 1200);
        assert_eq!(all.sessions, 3);
        assert_eq!(
            all.weeks,
            vec![(monday - WEEK, 1800), (monday, 1800 + 600 + 1200)]
        );
        assert_eq!(all.experiences[0].universe_id, 1);
        assert_eq!(all.experiences[0].secs, 4800);
        assert_eq!(
            all.experiences[0].accounts,
            vec![(AccountId(7), 3600), (AccountId(8), 1200)]
        );
        assert_eq!(all.experiences[0].last_played, monday + 8400);
        assert_eq!(all.accounts, vec![(AccountId(7), 4200), (AccountId(8), 1200)]);

        let week = breakdown(&logs, Some(monday), to, 0);
        assert_eq!(week.secs, 1800 + 600 + 1200);
        assert_eq!(week.weeks, vec![(monday, 3600)]);
    }

    #[test]
    fn empty_range_still_lists_its_weeks() {
        let monday = 1_704_067_200;
        let b = breakdown(&[], Some(monday - WEEK), monday + 10, 0);
        assert_eq!(b.weeks, vec![(monday - WEEK, 0), (monday, 0)]);
        assert!(b.experiences.is_empty());
    }
}
