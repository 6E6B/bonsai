//! Your own account: what Roblox has on file (email, phone, birthday, Premium), privacy
//! settings, the display name, private servers across experiences, and the local disk cache.

use rbx_api::generated::{accountinformation, accountsettings, apis, auth, games, premiumfeatures, users};
use rbx_api::{AccountId, Error};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::res::{Key, Page, Res};
use crate::store::UserInfo;

const INFO_AGE: i64 = 300;
const LIST_AGE: i64 = 300;
const PIN_AGE: i64 = 60;

/// What Roblox has on file for the signed-in account. Parts Roblox didn't answer stay empty.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AccountInfo {
    /// Masked by Roblox (`t*****@example.com`); empty when none is set.
    pub email: String,
    pub email_verified: bool,
    /// `+1 555…`; empty when none is set.
    pub phone: String,
    pub phone_verified: bool,
    /// (year, month, day).
    pub birthdate: Option<(i32, i32, i32)>,
    pub premium: Option<bool>,
}

/// One privacy setting and the values this account may pick.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PrivacySetting {
    /// Roblox's key, like `whoCanSeeMyInventory`.
    pub key: String,
    pub value: String,
    pub options: Vec<PrivacyOption>,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PrivacyOption {
    pub value: String,
    /// False when Roblox needs something first (a parent's consent, age verification).
    pub allowed: bool,
}

/// Which private servers to list.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ServersTab {
    /// Ones you own.
    Owned,
    /// Ones others let you join.
    Invited,
}

/// A private server in the list across experiences.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MyServer {
    pub id: u64,
    pub name: String,
    pub active: bool,
    pub universe_id: u64,
    pub place_id: u64,
    pub game: String,
    pub owner_id: u64,
    /// Robux a month; `None` when free.
    pub price: Option<i64>,
    pub renews: bool,
    /// ISO date, or empty.
    pub expires: String,
}

/// Bytes on disk.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct CacheUsage {
    pub data: u64,
    pub images: u64,
}

/// An option the account can pick: no requirement, or one it meets by picking it.
fn allowed(requirement: Option<&str>) -> bool {
    matches!(requirement, None | Some("SelfUpdateSetting"))
}

/// Settings with a plain string value, sorted by key. Time windows, limits and settings with
/// nothing to pick are left out.
pub(crate) fn privacy_list(r: apis::undocumented::UserSettingsAndOptions) -> Vec<PrivacySetting> {
    let mut out: Vec<PrivacySetting> = r
        .into_iter()
        .filter_map(|(key, s)| {
            let value = s.current_value.as_ref()?.as_str()?.to_owned();
            let options: Vec<PrivacyOption> = s
                .options
                .unwrap_or_default()
                .into_iter()
                .filter_map(|o| {
                    Some(PrivacyOption {
                        value: o.option?.option_value?,
                        allowed: allowed(o.requirement.as_deref()),
                    })
                })
                .collect();
            Some(PrivacySetting { key, value, options })
        })
        .collect();
    out.sort_by(|a, b| a.key.cmp(&b.key));
    out
}

/// The account PIN, which Roblox asks for before changing settings or trading when it is on.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PinStatus {
    pub enabled: bool,
    /// Unix seconds when the PIN locks again; 0 while locked.
    pub unlocked_until: i64,
}

/// `unlockedUntil` from the PIN API as unix seconds. The field is a bare number; small values
/// are read as seconds left (what the site counts down), large ones as a time.
fn pin_deadline(v: Option<f64>, now: i64) -> i64 {
    match v {
        Some(n) if n > 1e12 => (n / 1000.0) as i64,
        Some(n) if n > 1e9 => n as i64,
        Some(n) if n > 0.0 => now + n.ceil() as i64,
        _ => 0,
    }
}

impl Core {
    /// Whether the account PIN is on and unlocked.
    pub fn pin_status(&self, account: AccountId) -> Res<PinStatus> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner
            .res(Key::private(account, "pin", ""), PIN_AGE, move |f| async move {
                let r = auth::v1::get_account_pin(&f.client).await?;
                Ok(PinStatus {
                    enabled: r.is_enabled.unwrap_or(false),
                    unlocked_until: pin_deadline(r.unlocked_until, crate::cache::now()),
                })
            })
    }

    /// Unlock the account PIN for a while, from the PIN dialog.
    pub fn unlock_pin(&self, account: AccountId, pin: &str) -> Result<(), String> {
        let pin = pin.trim().to_owned();
        if pin.len() != 4 || !pin.chars().all(|c| c.is_ascii_digit()) {
            return Err("Enter the 4-digit PIN.".into());
        }
        self.inner.mutate(
            account,
            "unlock the PIN",
            "PIN unlocked.",
            &["pin"],
            move |f| async move {
                let body = auth::v1::AccountPinRequest {
                    pin: Some(pin),
                    reauthentication_token: None,
                };
                auth::v1::post_account_pin_unlock(&f.client, &body).await?;
                Ok(())
            },
        );
        Ok(())
    }

    /// Lock the account PIN again before it times out.
    pub fn lock_pin(&self, account: AccountId) {
        self.inner.mutate(
            account,
            "lock the PIN",
            "PIN locked.",
            &["pin"],
            move |f| async move {
                auth::v1::post_account_pin_lock(&f.client).await?;
                Ok(())
            },
        );
    }

    /// End every other session of the account (other browsers, devices and apps). Roblox
    /// answers with a new cookie for this one, which the client saves.
    pub fn sign_out_others(&self, account: AccountId) {
        self.inner.mutate(
            account,
            "sign out other sessions",
            "Signed out of all other sessions.",
            &[],
            move |f| async move {
                auth::v2::post_logoutfromallsessionsandreauthenticate(&f.client, &Default::default()).await?;
                Ok(())
            },
        );
    }

    /// Email, phone, birthday and Premium for one of your accounts.
    pub fn account_info(&self, account: AccountId) -> Res<AccountInfo> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "account_info", ""),
            INFO_AGE,
            move |f| async move {
                let c = &f.client;
                let (email, phone, birth, premium) = tokio::join!(
                    accountsettings::v1::get_email(c),
                    accountinformation::v1::get_phone(c),
                    users::v1::get_birthdate(c),
                    premiumfeatures::undocumented::get_has_premium(c, account.0 as i64),
                );
                // Fail only when nothing came back, so one blocked part doesn't hide the rest.
                if let (Err(e), Err(_), Err(_), Err(_)) = (&email, &phone, &birth, &premium) {
                    return Err(Error::Message(e.user_message()));
                }
                let mut info = AccountInfo::default();
                if let Ok(e) = email {
                    info.email = e.email_address.unwrap_or_default();
                    info.email_verified = e.verified.unwrap_or(false);
                }
                if let Ok(p) = phone {
                    let parts = [
                        p.prefix.map(|x| format!("+{}", x.trim_start_matches('+'))),
                        p.phone,
                    ];
                    info.phone = parts.into_iter().flatten().collect::<Vec<_>>().join(" ");
                    info.phone_verified = p.is_verified.unwrap_or(false);
                }
                if let Ok(b) = birth
                    && let (Some(y), Some(m), Some(d)) = (b.birth_year, b.birth_month, b.birth_day)
                {
                    info.birthdate = Some((y, m, d));
                }
                info.premium = premium.ok();
                Ok(info)
            },
        )
    }

    /// Privacy settings of one of your accounts.
    pub fn privacy(&self, account: AccountId) -> Res<Vec<PrivacySetting>> {
        if account.is_anonymous() {
            return Res::default();
        }
        self.inner.res(
            Key::private(account, "privacy", ""),
            INFO_AGE,
            move |f| async move {
                let r = apis::undocumented::get_user_settings_and_options(&f.client).await?;
                Ok(privacy_list(r))
            },
        )
    }

    /// Change one privacy setting. Shows the new value at once.
    pub fn set_privacy(&self, account: AccountId, key: &str, value: &str) -> Result<(), String> {
        let list = self
            .privacy(account)
            .data
            .ok_or_else(|| "Privacy settings are still loading.".to_owned())?;
        let setting = list
            .iter()
            .find(|s| s.key == key)
            .ok_or_else(|| "Unknown privacy setting.".to_owned())?;
        let option = setting
            .options
            .iter()
            .find(|o| o.value == value)
            .ok_or_else(|| "Not an option for this setting.".to_owned())?;
        if !option.allowed {
            return Err("Roblox doesn't let this account pick that. Change it on the website.".into());
        }
        if setting.value == value {
            return Ok(());
        }
        let (key, value) = (key.to_owned(), value.to_owned());
        {
            let (key, value) = (key.clone(), value.clone());
            self.inner
                .res
                .update::<Vec<PrivacySetting>>(&Key::private(account, "privacy", ""), move |v| {
                    if let Some(s) = v.iter_mut().find(|s| s.key == key) {
                        s.value = value;
                    }
                });
        }
        self.inner.repaint();
        self.inner.mutate(
            account,
            "change the setting",
            "",
            &["privacy"],
            move |f| async move {
                let body = std::collections::HashMap::from([(key, value)]);
                let req = apis::undocumented::post_user_settings_request(&body)?;
                f.client.send(req).await?;
                Ok(())
            },
        );
        Ok(())
    }

    /// Change the display name. Roblox checks it first and allows a change once a week.
    pub fn set_display_name(&self, account: AccountId, name: &str) -> Result<(), String> {
        let name = name.trim().to_owned();
        if name.is_empty() {
            return Err("Give a display name.".into());
        }
        self.inner.mutate(
            account,
            "change the display name",
            "Display name changed.",
            &[],
            move |f| async move {
                let id = account.0 as i64;
                let p = users::v1::GetUsersByUserIdDisplayNamesValidateParams {
                    display_name: name.clone(),
                };
                users::v1::get_users_by_user_id_display_names_validate(&f.client, id, &p)
                    .await
                    .map_err(cooldown)?;
                let body = users::v1::SetDisplayNameRequest {
                    new_display_name: Some(name.clone()),
                };
                let req = users::v1::patch_users_by_user_id_display_names_request(id, &body)?;
                f.client.send(req).await.map_err(cooldown)?;
                {
                    let mut s = f.inner.store.write();
                    if let Some(a) = s.accounts.get_mut(account) {
                        a.display_name = name.clone();
                    }
                    if let Some(u) = s.users.get_mut(&account.0) {
                        u.display_name = name;
                    }
                }
                f.inner.save_accounts();
                Ok(())
            },
        );
        Ok(())
    }

    /// Private servers you own or can join, across every experience.
    pub fn my_private_servers(&self, account: AccountId, tab: ServersTab, more: bool) -> Res<Page<MyServer>> {
        if account.is_anonymous() {
            return Res::default();
        }
        let n = match tab {
            ServersTab::Owned => 0,
            ServersTab::Invited => 1,
        };
        self.inner.paged(
            Key::private(account, "my_private_servers", n),
            LIST_AGE,
            more,
            move |f, cursor| async move {
                let p = games::undocumented::GetMyPrivateServersParams {
                    private_servers_tab: Some(n),
                    items_per_page: Some(50),
                    cursor,
                };
                let r = games::undocumented::get_my_private_servers(&f.client, &p).await?;
                let mut users = Vec::new();
                let items = r
                    .data
                    .unwrap_or_default()
                    .into_iter()
                    .filter_map(|s| {
                        let owner_id = s.owner_id.unwrap_or(0).max(0) as u64;
                        let owner = s.owner_name.clone().unwrap_or_default();
                        if owner_id > 0 && !owner.is_empty() {
                            users.push(UserInfo {
                                id: owner_id,
                                name: owner.clone(),
                                display_name: owner,
                                verified: false,
                            });
                        }
                        Some(MyServer {
                            id: s.private_server_id? as u64,
                            name: s.name.unwrap_or_default(),
                            active: s.active.unwrap_or(false),
                            universe_id: s.universe_id.unwrap_or(0).max(0) as u64,
                            place_id: s.place_id.unwrap_or(0).max(0) as u64,
                            game: s.universe_name.unwrap_or_default(),
                            owner_id,
                            price: s.price_in_robux.filter(|p| *p > 0),
                            renews: s.will_renew.unwrap_or(false),
                            expires: s.expiration_date.unwrap_or_default(),
                        })
                    })
                    .collect();
                f.note_users(users);
                Ok(Page {
                    items,
                    next: r.next_page_cursor.filter(|c| !c.is_empty()),
                })
            },
        )
    }

    /// Reload the Home feed now (the refresh button).
    pub fn reload_home(&self, account: AccountId) {
        {
            let mut s = self.inner.store.write();
            let h = &mut s.data.entry(account).or_default().home;
            // `ensure_home` reloads anything older than ten minutes, or with an error over a
            // minute old: make it both.
            h.fetched_at = 0;
            h.error = None;
        }
        self.inner.res.invalidate(account, "favorite_games");
        self.inner.res.invalidate(account, "home_feed");
        self.ensure_home(account);
    }

    /// Measure the disk cache at most every ten seconds. The result shows up in
    /// `StoreData::cache_usage`. Cheap to call on every fetch.
    pub fn measure_cache(&self) {
        if !self.inner.due("cache_usage", 10) {
            return;
        }
        let inner = self.inner.clone();
        self.inner.rt.spawn_blocking(move || {
            let usage = CacheUsage {
                data: inner.cache.size_bytes(),
                images: inner.disk.size_bytes(),
            };
            inner.store.write().cache_usage = Some(usage);
            inner.repaint();
        });
    }

    /// Delete cached pages and images. Local data (drafts, pinned chats, wishlist, recent games)
    /// stays.
    pub fn clear_cache(&self) {
        let inner = self.inner.clone();
        self.inner.rt.spawn_blocking(move || {
            inner.cache.clear_fetched();
            inner.disk.clear();
            inner.res.clear();
            inner.ledgers.clear();
            inner.store.write().thumbs.clear();
            inner.thumb_budget.lock().clear();
            inner.toast(None, "Cache cleared.", false);
            let usage = CacheUsage {
                data: inner.cache.size_bytes(),
                images: inner.disk.size_bytes(),
            };
            inner.store.write().cache_usage = Some(usage);
            inner.repaint();
        });
    }
}

/// Roblox answers a display name change inside the weekly limit with 429 code 5; say that
/// instead of "too many requests".
fn cooldown(e: Error) -> Error {
    match e {
        Error::RateLimited { .. } => {
            Error::Message("You can change your display name once every 7 days.".into())
        }
        e => e,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use apis::undocumented::{UserSettingAndOptions, UserSettingOption, UserSettingOptionValue};

    #[test]
    fn pin_deadline_reads_seconds_left_or_a_time() {
        assert_eq!(pin_deadline(None, 1000), 0);
        assert_eq!(pin_deadline(Some(0.0), 1000), 0);
        assert_eq!(pin_deadline(Some(299.5), 1000), 1300);
        assert_eq!(pin_deadline(Some(1_800_000_000.0), 1000), 1_800_000_000);
        assert_eq!(pin_deadline(Some(1_800_000_000_000.0), 1000), 1_800_000_000);
    }

    fn opt(v: &str, req: Option<&str>) -> UserSettingOption {
        UserSettingOption {
            option: Some(UserSettingOptionValue {
                option_value: Some(v.into()),
                option_type: None,
            }),
            requirement: req.map(Into::into),
        }
    }

    #[test]
    fn privacy_keeps_string_settings_and_marks_allowed_options() {
        let mut m = apis::undocumented::UserSettingsAndOptions::new();
        m.insert(
            "whoCanTradeWithMe".into(),
            UserSettingAndOptions {
                current_value: Some(serde_json::json!("NoOne")),
                options: Some(vec![
                    opt("AllUsers", Some("Inherited")),
                    opt("Friends", Some("SelfUpdateSetting")),
                    opt("NoOne", None),
                ]),
            },
        );
        m.insert(
            "doNotDisturbTimeWindow".into(),
            UserSettingAndOptions {
                current_value: Some(serde_json::json!({ "startTimeMinutes": 1 })),
                options: None,
            },
        );
        m.insert(
            "dailyScreenTimeLimit".into(),
            UserSettingAndOptions {
                current_value: None,
                options: None,
            },
        );
        let l = privacy_list(m);
        assert_eq!(l.len(), 1);
        let s = &l[0];
        assert_eq!((s.key.as_str(), s.value.as_str()), ("whoCanTradeWithMe", "NoOne"));
        let allowed: Vec<bool> = s.options.iter().map(|o| o.allowed).collect();
        assert_eq!(allowed, vec![false, true, true]);
    }
}
