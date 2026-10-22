//! Tier 3 without a click: what each Robux or trade action counts against, the daily budgets the
//! user sets per plugin and account, and the price a plugin's purchase comes to before it runs.
//! See `docs/plugins.md#limits`.

use std::sync::Arc;

use serde::{Deserialize, Serialize};

use crate::AccountId;
use crate::commands::{CommandId, Tier};
use crate::core::{Core, Inner};
use crate::fmt;
use crate::market::{ItemKind, ItemRef};

use super::ProposedAction;
use super::reads::{pages, wait};

/// How far back budgets count, in seconds.
pub(super) const DAY: i64 = 24 * 3600;
/// Most passes looked through for the one a plugin buys.
const PASSES: usize = 500;

/// What a plugin may spend on one account in 24 hours without a click. Zero is none: until the
/// user sets one, tier 3 asks each time.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct Budget {
    /// The account's own Robux.
    #[serde(default)]
    pub robux: u32,
    #[serde(default)]
    pub trades: u32,
    /// Robux paid out of the funds of communities the account manages, all together.
    #[serde(default)]
    pub payouts: u32,
}

/// What a plugin spent without a click on one account in the last 24 hours.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct Spent {
    pub robux: i64,
    pub trades: u32,
    pub payouts: i64,
}

impl Spent {
    pub(super) fn add(&mut self, c: Charge) {
        match c {
            Charge::Robux(n) => self.robux += n,
            Charge::Trade => self.trades += 1,
            Charge::Payout(n) => self.payouts += n,
        }
    }

    pub(super) fn plus(self, o: Spent) -> Spent {
        Spent {
            robux: self.robux + o.robux,
            trades: self.trades + o.trades,
            payouts: self.payouts + o.payouts,
        }
    }
}

/// What one tier 3 action counts against.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Charge {
    Robux(i64),
    Trade,
    /// Robux from a community's funds, not the account's.
    Payout(i64),
}

/// Why a tier 3 action that counts `charge` can't run without a click, having `spent` this much
/// of `budget`, or `None` when it can. Actions no budget covers always ask.
pub fn budget_refusal(charge: Option<Charge>, budget: Budget, spent: Spent) -> Option<String> {
    match charge {
        None => Some("This needs you to confirm each time.".into()),
        Some(Charge::Robux(_)) if budget.robux == 0 => {
            Some("It has no Robux budget on this account. Set one in Settings > Plugins.".into())
        }
        Some(Charge::Robux(n)) if spent.robux + n > i64::from(budget.robux) => Some(format!(
            "Over its budget of {} a day.",
            fmt::robux(budget.robux.into())
        )),
        Some(Charge::Trade) if budget.trades == 0 => {
            Some("It has no trade budget on this account. Set one in Settings > Plugins.".into())
        }
        Some(Charge::Trade) if spent.trades >= budget.trades => {
            Some(format!("Over its budget of {} a day.", trades(budget.trades)))
        }
        Some(Charge::Payout(_)) if budget.payouts == 0 => {
            Some("It has no payout budget on this account. Set one in Settings > Plugins.".into())
        }
        Some(Charge::Payout(n)) if spent.payouts + n > i64::from(budget.payouts) => Some(format!(
            "Over its payout budget of {} a day.",
            fmt::robux(budget.payouts.into())
        )),
        Some(_) => None,
    }
}

fn trades(n: u32) -> String {
    if n == 1 {
        "1 trade".into()
    } else {
        format!("{n} trades")
    }
}

/// A tier 3 action that spends Robux, from the argument a plugin gives it. Each names the most
/// it may cost; it runs at the price Roblox asks now, and only when that is no more.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) enum Spend {
    /// `market.buy` and `market.buy_as`: `item:<id> <most>`, `bundle:<id> <most>` or `<id>
    /// <most>`. `market.buy_resale` takes the same and buys the cheapest reseller's copy.
    Item { item: ItemRef, resale: bool, most: i64 },
    /// `game.buy_pass`: `<place> <pass> <most>`.
    Pass { place: u64, pass: u64, most: i64 },
    /// `game.buy_private_server`: `<place> <most> <name>`.
    Server { place: u64, most: i64, name: String },
    /// `group.payout`: `group:<id> <user id> <amount>`, from the community's funds.
    Payout { group: u64, user: u64, amount: i64 },
}

impl Spend {
    /// `None` for an action that doesn't spend Robux; an error when the argument doesn't say
    /// what and for how much.
    pub(super) fn parse(id: CommandId, arg: Option<&str>) -> Option<Result<Spend, String>> {
        use CommandId as C;
        let words: Vec<&str> = arg.unwrap_or_default().split_whitespace().collect();
        let id_of = |s: &str| s.parse::<u64>().ok().filter(|n| *n > 0);
        let robux = |s: &str| s.parse::<i64>().ok().filter(|n| *n >= 0);
        Some(match id {
            C::BuyItem | C::BuyItemAs | C::BuyResale => match words[..] {
                [item, most] => item_ref(item).zip(robux(most)).map(|(item, most)| Spend::Item {
                    item,
                    resale: id == C::BuyResale,
                    most,
                }),
                _ => None,
            }
            .ok_or_else(|| "Give an item and the most to pay, like item:1028606 50.".to_owned()),
            C::BuyPass => match words[..] {
                [place, pass, most] => match (id_of(place), id_of(pass), robux(most)) {
                    (Some(place), Some(pass), Some(most)) => Some(Spend::Pass { place, pass, most }),
                    _ => None,
                },
                _ => None,
            }
            .ok_or_else(|| "Give a place, a pass and the most to pay, like 1818 2001 25.".to_owned()),
            C::BuyPrivateServer => match words[..] {
                [place, most, ref name @ ..] if !name.is_empty() => {
                    id_of(place).zip(robux(most)).map(|(place, most)| Spend::Server {
                        place,
                        most,
                        name: name.join(" ").chars().take(50).collect(),
                    })
                }
                _ => None,
            }
            .ok_or_else(|| "Give a place, the most to pay and a name, like 1818 100 Friends.".to_owned()),
            C::PayOut => match words[..] {
                [group, user, amount] => {
                    let group = group.strip_prefix("group:").and_then(id_of);
                    let amount = robux(amount).filter(|n| *n > 0);
                    match (group, id_of(user), amount) {
                        (Some(group), Some(user), Some(amount)) => {
                            Some(Spend::Payout { group, user, amount })
                        }
                        _ => None,
                    }
                }
                _ => None,
            }
            .ok_or_else(|| "Give a community, a user id and an amount, like group:7 156 100.".to_owned()),
            _ => return None,
        })
    }

    /// The most it can cost. A batch is checked against this before anything runs.
    fn most(&self) -> i64 {
        match self {
            Spend::Item { most, .. } | Spend::Pass { most, .. } | Spend::Server { most, .. } => *most,
            Spend::Payout { amount, .. } => *amount,
        }
    }
}

/// `item:1`, `bundle:1` or a bare asset id.
fn item_ref(s: &str) -> Option<ItemRef> {
    let (bundle, id) = match s.strip_prefix("bundle:") {
        Some(id) => (true, id),
        None => (false, s.strip_prefix("item:").unwrap_or(s)),
    };
    let id = id.parse().ok().filter(|n| *n > 0)?;
    Some(if bundle {
        ItemRef::bundle(id)
    } else {
        ItemRef::asset(id)
    })
}

fn item_key(i: ItemRef) -> String {
    match i.kind {
        ItemKind::Asset => format!("item:{}", i.id),
        ItemKind::Bundle => format!("bundle:{}", i.id),
    }
}

/// What tier 3 action `a` counts against without a click, before its price is known: the most it
/// may cost, or a trade. `None` for actions no budget covers.
pub(super) fn charge(a: &ProposedAction) -> Option<Charge> {
    if a.tier != Tier::Value {
        return None;
    }
    if a.id == CommandId::AcceptTrade {
        return Some(Charge::Trade);
    }
    match Spend::parse(a.id, a.arg.as_deref()) {
        Some(Ok(Spend::Payout { amount, .. })) => Some(Charge::Payout(amount)),
        Some(Ok(s)) => Some(Charge::Robux(s.most())),
        _ => None,
    }
}

/// What a proposed action may cost, for the action dialog: "Up to R$ 50", "R$ 100" or "1 trade".
pub fn cost_label(a: &ProposedAction) -> Option<String> {
    match Spend::parse(a.id, a.arg.as_deref()) {
        Some(Ok(Spend::Payout { amount, .. })) => Some(fmt::robux(amount)),
        Some(Ok(s)) => Some(format!("Up to {}", fmt::robux(s.most()))),
        _ => charge(a).map(|_| trades(1)),
    }
}

/// Which budgets an action key needs: (Robux, trades, payouts).
fn budgeted(key: &str) -> (bool, bool, bool) {
    use CommandId as C;
    match CommandId::from_key(key) {
        Some(C::BuyItem | C::BuyItemAs | C::BuyResale | C::BuyPass | C::BuyPrivateServer) => {
            (true, false, false)
        }
        Some(C::AcceptTrade) => (false, true, false),
        Some(C::PayOut) => (false, false, true),
        _ => (false, false, false),
    }
}

impl Inner {
    /// The budget `plugin` has on `account`.
    pub(super) fn budget(&self, plugin: &str, account: AccountId) -> Budget {
        let p = self.plugins.lock();
        p.saved
            .plugins
            .get(plugin)
            .and_then(|s| s.budgets.get(&account).copied())
            .unwrap_or_default()
    }

    /// What `s` costs `account` now, and the argument that buys it at exactly that price. Waits
    /// for the same cached details the screens use, so call it only off the main thread.
    pub(super) fn price(self: &Arc<Self>, account: AccountId, s: &Spend) -> Result<(i64, String), String> {
        let at_most = |p: i64, most: i64| {
            if p > most {
                Err(format!(
                    "It costs {}, more than the {} it may pay.",
                    fmt::robux(p),
                    fmt::robux(most)
                ))
            } else {
                Ok(())
            }
        };
        match s {
            Spend::Item { item, resale, most } => {
                let d = wait(|| self.item(*item))?.ok_or("This item doesn't exist.")?;
                let c = &d.card;
                if c.collectible_id.is_empty() {
                    return Err("This item can only be bought on the website.".into());
                }
                let p = if *resale {
                    c.lowest_resale
                        .filter(|p| *p > 0 && c.limited.resellable())
                        .ok_or("Nobody is selling this right now.")?
                } else {
                    c.buy_offer().map(|(_, p)| p).ok_or("It's not for sale.")?
                };
                at_most(p, *most)?;
                Ok((p, format!("{} {p}", item_key(*item))))
            }
            Spend::Pass { place, pass, most } => {
                let u = self.universe(*place)?;
                let passes = pages(PASSES, |more| self.game_passes(account, u, more))?;
                let found = passes
                    .iter()
                    .find(|p| p.id == *pass)
                    .ok_or("That pass isn't in this experience's store.")?;
                if found.owned {
                    return Err("This account already owns this pass.".into());
                }
                let p = found.price.ok_or("This pass is off sale.")?;
                if found.product_id == 0 {
                    return Err("This pass can only be bought on the website.".into());
                }
                at_most(p, *most)?;
                Ok((p, format!("{place} {pass} {p}")))
            }
            Spend::Server { place, most, name } => {
                let u = self.universe(*place)?;
                let o = wait(|| self.private_server_offer(account, u, *place))?.unwrap_or_default();
                let p = o
                    .price
                    .filter(|_| o.enabled && o.can_create)
                    .ok_or("You can't buy a private server there.")?;
                at_most(p, *most)?;
                Ok((p, format!("{place} {p} {name}")))
            }
            Spend::Payout { group, user, amount } => Ok((*amount, format!("group:{group} {user} {amount}"))),
        }
    }

    fn universe(&self, place: u64) -> Result<u64, String> {
        self.read_game(place)?;
        self.store
            .read()
            .place_universe
            .get(&place)
            .copied()
            .ok_or_else(|| "No experience has that place.".to_owned())
    }
}

/// One account's budget for one plugin, with what it spent in the last 24 hours.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PluginBudget {
    pub account: AccountId,
    pub budget: Budget,
    pub spent: Spent,
    /// It may run actions that spend Robux, accept trades, or pay out community funds.
    pub robux: bool,
    pub trades: bool,
    pub payouts: bool,
}

/// The budget dialog's plugin and account, while it is open.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PendingBudget {
    pub plugin: String,
    pub plugin_name: String,
    pub current: PluginBudget,
}

impl Core {
    /// Budgets on the accounts `id` may use, when it has actions a budget covers.
    pub fn plugin_budgets(&self, id: &str) -> Vec<PluginBudget> {
        let now = crate::cache::now();
        let p = self.inner.plugins.lock();
        let Some(saved) = p.saved.plugins.get(id) else {
            return Vec::new();
        };
        let (robux, trades, payouts) = saved
            .actions
            .iter()
            .map(|k| budgeted(k))
            .fold((false, false, false), |(r, t, p), (r2, t2, p2)| {
                (r || r2, t || t2, p || p2)
            });
        if !robux && !trades && !payouts {
            return Vec::new();
        }
        saved
            .accounts
            .iter()
            .map(|a| PluginBudget {
                account: *a,
                budget: saved.budgets.get(a).copied().unwrap_or_default(),
                spent: p.log.spent(id, *a, now),
                robux,
                trades,
                payouts,
            })
            .collect()
    }

    /// Opens the budget dialog for `id` on `account`.
    pub fn request_budget(&self, id: &str, account: AccountId) -> Result<(), String> {
        let name = {
            let p = self.inner.plugins.lock();
            let e = p
                .entries
                .iter()
                .find(|e| e.info.id == id)
                .ok_or_else(|| format!("No plugin \"{id}\"."))?;
            e.info
                .manifest
                .as_ref()
                .map_or_else(|| id.to_owned(), |m| m.name.clone())
        };
        let current = self.plugin_budgets(id).into_iter().find(|b| b.account == account);
        let Some(current) = current else {
            return Err(if self.plugin_budgets(id).is_empty() {
                format!("{name} has no actions a budget covers.")
            } else {
                format!("{name} may not use this account. Allow it in Settings > Plugins first.")
            });
        };
        self.inner.store.write().plugin_budget = Some(PendingBudget {
            plugin: id.to_owned(),
            plugin_name: name,
            current,
        });
        self.inner.repaint();
        Ok(())
    }

    /// Saves the budget in the dialog. Zero turns that budget off.
    pub fn save_budget(&self, robux: u32, trades: u32, payouts: u32) -> Result<(), String> {
        let Some(pending) = self.store().plugin_budget.clone() else {
            return Err("Nothing to save.".into());
        };
        let account = pending.current.account;
        {
            let mut p = self.inner.plugins.lock();
            let saved = p
                .saved
                .plugins
                .get_mut(&pending.plugin)
                .ok_or("That plugin was removed.")?;
            if !saved.accounts.contains(&account) {
                return Err("It may not use this account any more.".into());
            }
            let b = Budget {
                robux: if pending.current.robux { robux } else { 0 },
                trades: if pending.current.trades { trades } else { 0 },
                payouts: if pending.current.payouts { payouts } else { 0 },
            };
            if b == Budget::default() {
                saved.budgets.remove(&account);
            } else {
                saved.budgets.insert(account, b);
            }
        }
        self.inner.store.write().plugin_budget = None;
        self.saved_plugins_changed();
        Ok(())
    }

    pub fn cancel_budget(&self) {
        self.inner.store.write().plugin_budget = None;
        self.inner.repaint();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn action(id: CommandId, arg: &str) -> ProposedAction {
        ProposedAction {
            id,
            arg: Some(arg.into()),
            tier: Tier::Value,
        }
    }

    #[test]
    fn spends_name_what_and_the_most() {
        let p = |id, arg: &str| Spend::parse(id, Some(arg));
        assert_eq!(
            p(CommandId::BuyItem, "item:5 50"),
            Some(Ok(Spend::Item {
                item: ItemRef::asset(5),
                resale: false,
                most: 50
            }))
        );
        assert_eq!(
            p(CommandId::BuyResale, "bundle:7 0"),
            Some(Ok(Spend::Item {
                item: ItemRef::bundle(7),
                resale: true,
                most: 0
            }))
        );
        assert_eq!(
            p(CommandId::BuyPrivateServer, "1818 100 My   server"),
            Some(Ok(Spend::Server {
                place: 1818,
                most: 100,
                name: "My server".into()
            }))
        );
        assert_eq!(
            p(CommandId::PayOut, "group:7 156 100"),
            Some(Ok(Spend::Payout {
                group: 7,
                user: 156,
                amount: 100
            }))
        );
        assert!(matches!(p(CommandId::BuyItem, "item:5"), Some(Err(_))));
        assert!(matches!(p(CommandId::BuyItem, "item:5 -1"), Some(Err(_))));
        assert!(matches!(p(CommandId::BuyPass, "1818 2001"), Some(Err(_))));
        assert!(matches!(p(CommandId::BuyPrivateServer, "1818 100"), Some(Err(_))));
        assert!(matches!(p(CommandId::PayOut, "7 156 100"), Some(Err(_))));
        assert!(matches!(p(CommandId::PayOut, "group:7 156 0"), Some(Err(_))));
        assert_eq!(p(CommandId::AcceptTrade, "1"), None);
        assert_eq!(p(CommandId::DeleteInventoryItem, "1"), None);
    }

    #[test]
    fn charges_and_costs() {
        assert_eq!(
            charge(&action(CommandId::BuyPass, "1 2 25")),
            Some(Charge::Robux(25))
        );
        assert_eq!(charge(&action(CommandId::AcceptTrade, "9")), Some(Charge::Trade));
        assert_eq!(charge(&action(CommandId::DeleteInventoryItem, "9")), None);
        assert_eq!(charge(&action(CommandId::CounterTrade, "9")), None);
        assert_eq!(
            cost_label(&action(CommandId::BuyItem, "5 1200")).as_deref(),
            Some("Up to R$ 1,200")
        );
        assert_eq!(
            cost_label(&action(CommandId::PayOut, "group:1 2 30")).as_deref(),
            Some("R$ 30")
        );
        assert_eq!(
            cost_label(&action(CommandId::AcceptTrade, "9")).as_deref(),
            Some("1 trade")
        );
        assert_eq!(cost_label(&action(CommandId::RenamePrivateServer, "9 x")), None);
    }

    #[test]
    fn budgets_refuse_what_doesnt_fit() {
        let b = Budget {
            robux: 100,
            trades: 2,
            payouts: 50,
        };
        let spent = Spent {
            robux: 60,
            trades: 1,
            payouts: 0,
        };
        assert_eq!(budget_refusal(Some(Charge::Robux(40)), b, spent), None);
        assert!(
            budget_refusal(Some(Charge::Robux(41)), b, spent)
                .unwrap()
                .contains("R$ 100 a day")
        );
        assert_eq!(budget_refusal(Some(Charge::Trade), b, spent), None);
        let spent = Spent {
            robux: 0,
            trades: 2,
            payouts: 0,
        };
        assert!(
            budget_refusal(Some(Charge::Trade), b, spent)
                .unwrap()
                .contains("2 trades a day")
        );
        // No budget set is none at all, even for something free.
        let none = Budget::default();
        assert!(
            budget_refusal(Some(Charge::Robux(0)), none, Spent::default())
                .unwrap()
                .contains("no Robux budget")
        );
        assert!(
            budget_refusal(Some(Charge::Trade), none, Spent::default())
                .unwrap()
                .contains("no trade budget")
        );
        assert!(budget_refusal(None, b, Spent::default()).is_some());
    }

    #[test]
    fn payouts_have_their_own_budget() {
        let b = Budget {
            robux: 1000,
            trades: 0,
            payouts: 50,
        };
        let payout = action(CommandId::PayOut, "group:7 156 40");
        assert_eq!(charge(&payout), Some(Charge::Payout(40)));
        // Robux spent from the account leaves the payout budget alone, and the other way round.
        let spent = Spent {
            robux: 900,
            trades: 0,
            payouts: 10,
        };
        assert_eq!(budget_refusal(Some(Charge::Payout(40)), b, spent), None);
        assert!(
            budget_refusal(Some(Charge::Payout(41)), b, spent)
                .unwrap()
                .contains("payout budget of R$ 50")
        );
        assert_eq!(budget_refusal(Some(Charge::Robux(100)), b, spent), None);
        let none = Budget { payouts: 0, ..b };
        assert!(
            budget_refusal(Some(Charge::Payout(1)), none, Spent::default())
                .unwrap()
                .contains("no payout budget")
        );
    }
}
