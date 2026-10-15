//! Buying with Robux: game passes, catalog items and bundles from their creator, and resellers'
//! copies of limiteds and collectibles.
//!
//! Every purchase is one user click on a confirm dialog that showed the price. That price goes
//! to Roblox as the expected price, so Roblox refuses the purchase if it changed in between, and
//! we check it against fresh sale details first so the user gets a clear message. A challenge
//! (2-step verification) is never answered here: the user is sent to the website.

use rbx_api::generated::{apis, economy};
use rbx_api::{AccountId, Client, Error};

use crate::Core;
use crate::cache::Kind;
use crate::fmt;
use crate::launcher::uuid_v4;

/// Robux, as `expectedCurrency`.
const ROBUX: i32 = 1;

/// Kinds that can change when something is bought.
const BOUGHT: &[&str] = &[
    "owns",
    "game_passes",
    "item",
    "items",
    "resellers",
    "collectibles",
    "tradable",
];

/// Whose copy of an item to buy.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Seller {
    /// A new copy from the creator.
    Creator,
    /// The cheapest resale listing right now.
    Cheapest,
    /// This resale listing (`collectibleItemInstanceId`).
    Listing(String),
}

/// A resale listing as the purchase needs it.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
struct Listing {
    instance_id: String,
    product_id: String,
    seller_id: i64,
    seller_type: String,
    price: i64,
}

/// What Roblox said about a purchase, from either purchase endpoint.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub(crate) struct Outcome {
    pub purchased: bool,
    pub pending: bool,
    pub reason: String,
    pub error_message: String,
    pub purchase_result: String,
    pub failure_reason: String,
    pub minutes: Option<i64>,
}

impl From<apis::undocumented::CollectiblePurchaseResult> for Outcome {
    fn from(r: apis::undocumented::CollectiblePurchaseResult) -> Self {
        Outcome {
            purchased: r.purchased.unwrap_or(false),
            pending: r.pending.unwrap_or(false),
            reason: r.reason.unwrap_or_default(),
            error_message: r.error_message.unwrap_or_default(),
            purchase_result: r.purchase_result.unwrap_or_default(),
            failure_reason: r.failure_reason.unwrap_or_default(),
            minutes: r.expiration_time_in_minutes,
        }
    }
}

impl From<apis::undocumented::GamePassPurchaseResult> for Outcome {
    fn from(r: apis::undocumented::GamePassPurchaseResult) -> Self {
        Outcome {
            purchased: r.purchased.unwrap_or(false),
            reason: r.reason.unwrap_or_default(),
            failure_reason: r.failure_reason.unwrap_or_default(),
            minutes: r.expiration_time_in_minutes,
            ..Default::default()
        }
    }
}

impl Outcome {
    /// `Ok(pending)` when bought, else why not in plain words. Same order of checks as the site.
    pub(crate) fn result(&self) -> Result<bool, Error> {
        let msg = |s: &str| Err(Error::Message(s.to_owned()));
        if !self.failure_reason.is_empty() {
            return match self.minutes.filter(|m| *m > 0) {
                Some(m) => Err(Error::Message(format!(
                    "Roblox has paused purchases on this account for {m} min."
                ))),
                None => msg("Roblox has paused purchases on this account for now."),
            };
        }
        if self.purchased {
            return Ok(self.pending);
        }
        let any =
            |s: &str| [&self.reason, &self.error_message, &self.purchase_result].contains(&&s.to_owned());
        if any("TwoStepVerificationRequired") {
            msg("Roblox wants you to verify this purchase. Buy it on the website instead.")
        } else if any("InsufficientBalance") || any("InsufficientFunds") {
            msg("You don't have enough Robux.")
        } else if any("Flooded") {
            msg("Too many purchases. Wait a minute and try again.")
        } else if any("AlreadyOwned") {
            msg("You already own this.")
        } else if any("QuantityLimitExceeded") {
            msg("You already own as many copies as Roblox allows.")
        } else {
            let said = [&self.error_message, &self.reason, &self.purchase_result]
                .into_iter()
                .find(|s| !s.is_empty());
            match said {
                Some(s) => Err(Error::Message(format!("Roblox said: {s}"))),
                None => msg("Roblox didn't complete the purchase."),
            }
        }
    }
}

fn price_changed(now: Option<i64>, expected: i64) -> Result<(), Error> {
    match now {
        Some(p) if p == expected => Ok(()),
        Some(p) => Err(Error::Message(format!(
            "The price is now {}. Look again before buying.",
            price_text(p)
        ))),
        None => Err(Error::Message("It's not for sale any more.".into())),
    }
}

fn price_text(p: i64) -> String {
    if p == 0 { "free".into() } else { fmt::robux(p) }
}

/// The cheapest listing, or the one with `instance`, from the first page of resellers.
async fn listing(c: &Client, collectible_id: &str, instance: Option<&str>) -> Result<Listing, Error> {
    let p = apis::undocumented::GetItemResellersParams {
        limit: Some(30),
        cursor: None,
    };
    let page = apis::undocumented::get_item_resellers(c, collectible_id, &p).await?;
    let found = page
        .data
        .unwrap_or_default()
        .into_iter()
        .find(|r| instance.is_none_or(|i| r.collectible_item_instance_id.as_deref() == Some(i)));
    let Some(r) = found else {
        return Err(Error::Message(match instance {
            Some(_) => "That copy was sold or taken off sale.".into(),
            None => "Nobody is selling this right now.".into(),
        }));
    };
    let s = r.seller.unwrap_or_default();
    Ok(Listing {
        instance_id: r.collectible_item_instance_id.unwrap_or_default(),
        product_id: r.collectible_product_id.unwrap_or_default(),
        seller_id: s.seller_id.unwrap_or(0),
        seller_type: s.seller_type.unwrap_or_else(|| "User".into()),
        price: r.price.unwrap_or(0),
    })
}

impl Core {
    /// Buy a game pass. Only call after the user confirmed `price` in the UI.
    pub fn buy_pass(&self, account: AccountId, product_id: u64, name: String, price: i64) {
        self.inner
            .mutate(account, "buy the pass", "", BOUGHT, move |f| async move {
                let body = apis::undocumented::GamePassPurchaseRequest {
                    expected_price: Some(price),
                    idempotency_key: Some(uuid_v4()),
                };
                let r =
                    apis::undocumented::post_purchase_game_pass(&f.client, product_id as i64, &body).await?;
                Outcome::from(r).result()?;
                f.inner.toast(Some(account), format!("Bought {name}."), false);
                refresh_robux(&f.inner, account, &f.client).await;
                Ok(())
            });
    }

    /// Buy an item or bundle from `seller` for `price`. Only call after the user confirmed
    /// `price` in the UI. `collectible_id` is the item's `collectibleItemId`.
    pub fn buy_item(
        &self,
        account: AccountId,
        collectible_id: String,
        name: String,
        price: i64,
        seller: Seller,
    ) {
        self.inner
            .mutate(account, "buy the item", "", BOUGHT, move |f| async move {
                let c = &f.client;
                let mut body = apis::undocumented::CollectiblePurchaseRequest {
                    collectible_item_id: Some(collectible_id.clone()),
                    expected_currency: Some(ROBUX),
                    expected_price: Some(price),
                    expected_purchaser_id: Some(account.0 as i64),
                    expected_purchaser_type: Some("User".into()),
                    idempotency_key: Some(uuid_v4()),
                    ..Default::default()
                };
                let r = match seller {
                    Seller::Creator => {
                        let ids = apis::undocumented::CollectibleItemIds {
                            item_ids: Some(vec![collectible_id.clone()]),
                        };
                        let d = apis::undocumented::post_collectible_item_details(c, &ids)
                            .await?
                            .into_iter()
                            .find(|d| d.collectible_item_id.as_deref() == Some(collectible_id.as_str()))
                            .ok_or_else(|| Error::Message("This item can't be bought.".into()))?;
                        let sold_out = d.units_available_for_consumption == Some(0)
                            && d.lowest_available_resale_item_instance_id.is_some();
                        if sold_out {
                            return Err(Error::Message("Sold out. Buy from a reseller instead.".into()));
                        }
                        if d.sale_location_type.as_deref() == Some("ExperiencesDevApiOnly") {
                            return Err(Error::Message("This is only sold inside experiences.".into()));
                        }
                        price_changed(d.price, price)?;
                        body.collectible_product_id = d.collectible_product_id;
                        body.expected_seller_id = d.creator_id;
                        body.expected_seller_type = d.creator_type;
                        apis::undocumented::post_purchase_item(c, &collectible_id, &body).await?
                    }
                    Seller::Cheapest | Seller::Listing(_) => {
                        let instance = match &seller {
                            Seller::Listing(i) => Some(i.as_str()),
                            _ => None,
                        };
                        let l = listing(c, &collectible_id, instance).await?;
                        price_changed(Some(l.price), price)?;
                        if l.seller_id == account.0 as i64 && l.seller_type == "User" {
                            return Err(Error::Message("That's your own copy.".into()));
                        }
                        body.collectible_item_instance_id = Some(l.instance_id);
                        body.collectible_product_id = Some(l.product_id);
                        body.expected_seller_id = Some(l.seller_id);
                        body.expected_seller_type = Some(l.seller_type);
                        apis::undocumented::post_purchase_resale(c, &collectible_id, &body).await?
                    }
                };
                let pending = Outcome::from(r).result()?;
                let text = if pending {
                    format!("Bought {name}. It can take a minute to show up.")
                } else {
                    format!("Bought {name}.")
                };
                f.inner.toast(Some(account), text, false);
                refresh_robux(&f.inner, account, c).await;
                Ok(())
            });
    }
}

/// Fetch the balance again so the status bar shows what's left.
async fn refresh_robux(inner: &crate::core::Inner, account: AccountId, c: &Client) {
    if let Ok(r) = economy::v1::get_user_currency(c).await {
        let r = r.robux.unwrap_or_default();
        inner.store.write().data.entry(account).or_default().robux = Some(r);
        inner.cache.put(Kind::Robux, account, "robux", &r);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn said(f: impl FnOnce(&mut Outcome)) -> Result<bool, String> {
        let mut o = Outcome::default();
        f(&mut o);
        o.result().map_err(|e| e.user_message())
    }

    #[test]
    fn outcomes_in_plain_words() {
        assert_eq!(said(|o| o.purchased = true), Ok(false));
        assert_eq!(
            said(|o| {
                o.purchased = true;
                o.pending = true;
            }),
            Ok(true)
        );
        assert_eq!(
            said(|o| o.error_message = "InsufficientBalance".into()),
            Err("You don't have enough Robux.".into())
        );
        assert!(
            said(|o| o.reason = "TwoStepVerificationRequired".into())
                .unwrap_err()
                .contains("website")
        );
        assert!(
            said(|o| o.purchase_result = "Flooded".into())
                .unwrap_err()
                .starts_with("Too many")
        );
        // A restriction wins even if Roblox also says purchased.
        assert!(
            said(|o| {
                o.purchased = true;
                o.failure_reason = "Restricted".into();
                o.minutes = Some(30);
            })
            .unwrap_err()
            .contains("30 min")
        );
        assert!(
            said(|o| o.error_message = "QuantityLimitExceeded".into())
                .unwrap_err()
                .starts_with("You already own")
        );
        assert_eq!(said(|_| {}), Err("Roblox didn't complete the purchase.".into()));
        assert_eq!(
            said(|o| o.reason = "SomethingNew".into()),
            Err("Roblox said: SomethingNew".into())
        );
    }

    #[test]
    fn price_must_match_what_was_shown() {
        assert!(price_changed(Some(50), 50).is_ok());
        assert!(
            price_changed(Some(75), 50)
                .unwrap_err()
                .user_message()
                .contains("now")
        );
        assert!(price_changed(None, 50).is_err());
        assert!(price_changed(Some(0), 0).is_ok());
    }
}
