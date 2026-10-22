//! Game playlists: named lists of experiences an account keeps in this app. They live per
//! account in the local cache, survive "clear cache" and never go to Roblox.

use bonsai_api::AccountId;
use serde::{Deserialize, Serialize};

use crate::Core;

const KEY: &str = "playlists";
pub const NAME_MAX: usize = 60;
pub const MAX_GAMES: usize = 500;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Playlist {
    pub id: u64,
    pub name: String,
    #[serde(default)]
    pub games: Vec<PlaylistGame>,
}

/// An experience in a playlist, by place id like every game command. The universe and name are
/// what was known when it was added, so the list shows something before details load.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PlaylistGame {
    pub place_id: u64,
    #[serde(default)]
    pub universe_id: u64,
    #[serde(default)]
    pub name: String,
}

fn clean_name(name: &str) -> Result<String, String> {
    let n: String = name.trim().chars().take(NAME_MAX).collect();
    if n.is_empty() {
        return Err("Name the playlist.".to_owned());
    }
    Ok(n)
}

fn find(all: &mut [Playlist], id: u64) -> Result<&mut Playlist, String> {
    all.iter_mut()
        .find(|p| p.id == id)
        .ok_or_else(|| "No such playlist.".to_owned())
}

/// Adds a playlist named `name` and returns its id.
pub fn create(all: &mut Vec<Playlist>, name: &str) -> Result<u64, String> {
    let name = clean_name(name)?;
    let id = all.iter().map(|p| p.id).max().unwrap_or(0) + 1;
    all.push(Playlist {
        id,
        name,
        games: Vec::new(),
    });
    Ok(id)
}

pub fn rename(all: &mut [Playlist], id: u64, name: &str) -> Result<(), String> {
    let name = clean_name(name)?;
    find(all, id)?.name = name;
    Ok(())
}

pub fn delete(all: &mut Vec<Playlist>, id: u64) -> Result<(), String> {
    let before = all.len();
    all.retain(|p| p.id != id);
    if all.len() == before {
        return Err("No such playlist.".to_owned());
    }
    Ok(())
}

/// Appends `game` unless the playlist already has it. False when it was already there.
pub fn add(all: &mut [Playlist], id: u64, game: PlaylistGame) -> Result<bool, String> {
    let p = find(all, id)?;
    if p.games.iter().any(|g| g.place_id == game.place_id) {
        return Ok(false);
    }
    if p.games.len() >= MAX_GAMES {
        return Err(format!("A playlist holds up to {MAX_GAMES} experiences."));
    }
    p.games.push(game);
    Ok(true)
}

pub fn remove(all: &mut [Playlist], id: u64, place_id: u64) -> Result<(), String> {
    let p = find(all, id)?;
    let before = p.games.len();
    p.games.retain(|g| g.place_id != place_id);
    if p.games.len() == before {
        return Err("That experience isn't in this playlist.".to_owned());
    }
    Ok(())
}

impl Core {
    /// The playlists `account` keeps, oldest first.
    pub fn playlists(&self, account: AccountId) -> Vec<Playlist> {
        if account.is_anonymous() {
            return Vec::new();
        }
        self.local(account, KEY)
    }

    /// Runs `f` on `account`'s playlists and saves them when it succeeds.
    pub fn edit_playlists<T>(
        &self,
        account: AccountId,
        f: impl FnOnce(&mut Vec<Playlist>) -> Result<T, String>,
    ) -> Result<T, String> {
        if account.is_anonymous() {
            return Err("Sign in to keep playlists.".to_owned());
        }
        let mut all = self.playlists(account);
        let out = f(&mut all)?;
        self.set_local(account, KEY, &all);
        Ok(out)
    }

    /// Adds the experience at `place_id` to a playlist, filling in its name from what is cached.
    pub fn add_to_playlist(&self, account: AccountId, id: u64, place_id: u64) -> Result<bool, String> {
        let game = {
            let s = self.store();
            let g = s.game_for_place(place_id);
            PlaylistGame {
                place_id,
                universe_id: g.map_or(0, |g| g.universe_id),
                name: g.map(|g| g.name.clone()).unwrap_or_default(),
            }
        };
        self.edit_playlists(account, |all| add(all, id, game))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn game(place_id: u64) -> PlaylistGame {
        PlaylistGame {
            place_id,
            ..Default::default()
        }
    }

    #[test]
    fn create_gives_new_ids_and_needs_a_name() {
        let mut all = Vec::new();
        assert_eq!(create(&mut all, " Obbies "), Ok(1));
        assert_eq!(create(&mut all, "Horror"), Ok(2));
        assert_eq!(all[0].name, "Obbies");
        assert!(create(&mut all, "   ").is_err());
        delete(&mut all, 1).unwrap();
        // Ids are not reused while a later one exists.
        assert_eq!(create(&mut all, "Again"), Ok(3));
        assert_eq!(
            create(&mut all, &"x".repeat(100)).map(|_| all.last().unwrap().name.len()),
            Ok(NAME_MAX)
        );
    }

    #[test]
    fn add_skips_duplicates_and_remove_takes_out() {
        let mut all = Vec::new();
        let id = create(&mut all, "Fav").unwrap();
        assert_eq!(add(&mut all, id, game(10)), Ok(true));
        assert_eq!(add(&mut all, id, game(20)), Ok(true));
        assert_eq!(add(&mut all, id, game(10)), Ok(false));
        assert_eq!(all[0].games.len(), 2);
        remove(&mut all, id, 10).unwrap();
        assert_eq!(all[0].games, vec![game(20)]);
        assert!(remove(&mut all, id, 10).is_err());
        assert!(add(&mut all, 99, game(1)).is_err());
    }

    #[test]
    fn rename_and_delete_unknown_fail() {
        let mut all = Vec::new();
        let id = create(&mut all, "A").unwrap();
        rename(&mut all, id, "B").unwrap();
        assert_eq!(all[0].name, "B");
        assert!(rename(&mut all, 5, "C").is_err());
        assert!(rename(&mut all, id, "").is_err());
        assert!(delete(&mut all, 5).is_err());
    }
}
