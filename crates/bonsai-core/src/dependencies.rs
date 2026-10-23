//! References in downloadable Roblox models. A scan never executes scripts or follows the
//! referenced assets recursively. Results are private to the account allowed to read the model.

use std::collections::{BTreeMap, BTreeSet};

use bonsai_api::generated::{assetdelivery, economy};
use bonsai_api::{AccountId, Client, Error};
use futures::{StreamExt, stream};
use rbx_dom_weak::{WeakDom, types::Variant};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::res::{Key, Res};

const MAX_MODEL: usize = 16 * 1024 * 1024;
const MAX_EXPANDED: usize = 64 * 1024 * 1024;
const MAX_INSTANCES: u32 = 20_000;
const MAX_DEPENDENCIES: usize = 256;
const MAX_REFERENCES: usize = 4096;
const MAX_ID: u64 = (1 << 53) - 1;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Usage {
    pub class: String,
    pub path: String,
    pub property: String,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Dependency {
    pub id: u64,
    pub uses: Vec<Usage>,
    /// Missing metadata does not hide a reference.
    pub name: Option<String>,
    pub asset_type: i32,
    pub creator: String,
    pub creator_id: u64,
    pub creator_is_group: bool,
    pub creator_verified: bool,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct Dependencies {
    pub items: Vec<Dependency>,
    /// One representation was unavailable; the other still yielded a valid model.
    pub note: Option<String>,
}

fn failure(message: &str) -> Error {
    Error::Message(message.into())
}

/// Only asset URIs and Roblox asset URLs count. A number in a script, instance name or an
/// unrelated URL is not a dependency. Keep ids exact across IPC's JS-number boundary.
fn reference(value: &str) -> Option<u64> {
    let value = value.trim();
    let lower = value.to_ascii_lowercase();
    let id = if let Some(id) = lower.strip_prefix("rbxassetid://") {
        id.parse().ok()?
    } else {
        let url = url::Url::parse(value).ok()?;
        if !matches!(url.scheme(), "http" | "https")
            || !url
                .host_str()
                .is_some_and(|h| h == "roblox.com" || h.ends_with(".roblox.com"))
            || !matches!(
                url.path().trim_end_matches('/'),
                "/asset" | "/asset/" | "/v1/asset"
            )
        {
            return None;
        }
        url.query_pairs()
            .find(|(key, _)| key.eq_ignore_ascii_case("id"))?
            .1
            .parse()
            .ok()?
    };
    (id > 0 && id <= MAX_ID).then_some(id)
}

/// Guard binary chunk sizes before a decoder allocates or decompresses them. rbx-dom handles
/// the actual LZ4/Zstd and property formats; this is only a resource budget, not a second parser.
fn binary_budget(bytes: &[u8]) -> Result<(), Error> {
    let word = |at| -> Result<u32, Error> {
        let b = bytes
            .get(at..at + 4)
            .ok_or_else(|| failure("The model is truncated."))?;
        Ok(u32::from_le_bytes([b[0], b[1], b[2], b[3]]))
    };
    if bytes.len() < 32 || word(16)? > MAX_INSTANCES || word(20)? > MAX_INSTANCES {
        return Err(failure("This model is too large to inspect."));
    }
    let mut at = 32;
    let mut expanded = 0usize;
    while at < bytes.len() {
        let compressed = word(at + 4)? as usize;
        let size = word(at + 8)? as usize;
        if compressed > 0 && compressed < 4 {
            return Err(failure("The model is truncated."));
        }
        expanded = expanded.saturating_add(size);
        if expanded > MAX_EXPANDED {
            return Err(failure("This model is too large to inspect."));
        }
        at = at
            .checked_add(16 + if compressed == 0 { size } else { compressed })
            .filter(|end| *end <= bytes.len())
            .ok_or_else(|| failure("The model is truncated."))?;
    }
    Ok(())
}

fn parse(bytes: &[u8], own_id: u64) -> Result<Vec<Dependency>, Error> {
    if bytes.len() > MAX_MODEL {
        return Err(failure("This model is too large to inspect."));
    }
    let dom: WeakDom = if bytes.starts_with(b"<roblox!") {
        binary_budget(bytes)?;
        rbx_binary::from_reader(bytes).map_err(|_| failure("Couldn't read this binary model."))?
    } else {
        rbx_xml::from_reader(
            bytes,
            rbx_xml::DecodeOptions::new().property_behavior(rbx_xml::DecodePropertyBehavior::ReadUnknown),
        )
        .map_err(|_| failure("This asset isn't a readable Roblox model."))?
    };
    let mut found: BTreeMap<u64, BTreeSet<Usage>> = BTreeMap::new();
    let mut pending = vec![(dom.root_ref(), String::new(), 0)];
    let mut count = 0;
    let mut references = 0;
    while let Some((referent, parent, depth)) = pending.pop() {
        count += 1;
        if count > MAX_INSTANCES || depth > 128 {
            return Err(failure("This model is too large to inspect."));
        }
        let Some(instance) = dom.get_by_ref(referent) else {
            continue;
        };
        let name: String = instance.name.chars().take(120).collect();
        let path = if parent.is_empty() {
            name
        } else {
            format!("{parent}/{name}")
        };
        if path.len() > 2048 {
            return Err(failure("This model's instance paths are too long to inspect."));
        }
        for (property, value) in &instance.properties {
            let value = match value {
                Variant::ContentId(v) => Some(v.as_str()),
                Variant::Content(v) => v.as_uri(),
                // Some historical models serialize content properties as strings. Do not
                // interpret script sources, names or arbitrary StringValues as dependencies.
                Variant::String(v) if content_property(property.as_str()) => Some(v.as_str()),
                _ => None,
            };
            if let Some(id) = value.and_then(reference).filter(|id| *id != own_id) {
                references += 1;
                if references > MAX_REFERENCES {
                    return Err(failure("This model has too many references to inspect."));
                }
                found.entry(id).or_default().insert(Usage {
                    class: instance.class.to_string(),
                    path: path.clone(),
                    property: property.to_string(),
                });
                if found.len() > MAX_DEPENDENCIES {
                    return Err(failure("This model has too many dependencies to inspect."));
                }
            }
        }
        for child in instance.children() {
            pending.push((
                *child,
                if referent == dom.root_ref() {
                    String::new()
                } else {
                    path.clone()
                },
                depth + 1,
            ));
        }
    }
    Ok(found
        .into_iter()
        .map(|(id, uses)| Dependency {
            id,
            uses: uses.into_iter().collect(),
            ..Default::default()
        })
        .collect())
}

fn content_property(name: &str) -> bool {
    matches!(
        name,
        "MeshId"
            | "TextureID"
            | "TextureId"
            | "Texture"
            | "ColorMap"
            | "NormalMap"
            | "MetalnessMap"
            | "RoughnessMap"
            | "ShirtTemplate"
            | "PantsTemplate"
            | "Graphic"
            | "SoundId"
            | "AnimationId"
            | "Image"
            | "Video"
            | "CageMeshId"
            | "ReferenceMeshId"
    )
}

fn processed_format(asset_type: i32) -> Option<&'static str> {
    match asset_type {
        17 | 79 => Some("avatar_meshpart_head"),
        8 | 41..=47 | 57 | 58 => Some("avatar_meshpart_accessory"),
        _ => None,
    }
}

async fn model(client: &Client, id: u64, format: &str) -> Result<Vec<Dependency>, Error> {
    let params = assetdelivery::v2::GetAssetIdByAssetIdParams {
        roblox_asset_format: format.into(),
        ..Default::default()
    };
    let response = assetdelivery::v2::get_asset_id_by_asset_id(client, id as i64, &params).await?;
    let location = response
        .locations
        .unwrap_or_default()
        .into_iter()
        .find_map(|l| l.location)
        .ok_or_else(|| {
            failure(
                "Roblox won't serve this model to this account. It may be private, moderated or unavailable.",
            )
        })?;
    let bytes = client.fetch_cdn_limited(&location, MAX_MODEL).await?;
    tokio::task::spawn_blocking(move || parse(&bytes, id))
        .await
        .map_err(|_| failure("Couldn't inspect this model."))?
}

async fn scan(client: &Client, id: u64, asset_type: i32) -> Result<Dependencies, Error> {
    let source = model(client, id, "").await;
    let (mut items, note) = if let Some(format) = processed_format(asset_type) {
        match (source, model(client, id, format).await) {
            (Ok(mut a), Ok(b)) => {
                for item in b {
                    if let Some(existing) = a.iter_mut().find(|v| v.id == item.id) {
                        existing.uses.extend(item.uses);
                        existing.uses.sort();
                        existing.uses.dedup();
                    } else {
                        a.push(item);
                    }
                }
                (a, None)
            }
            (Ok(a), Err(_)) => (
                a,
                Some("Processed model unavailable. Showing references from the source model.".into()),
            ),
            (Err(_), Ok(b)) => (
                b,
                Some("Source model unavailable. Showing references from the processed model.".into()),
            ),
            (Err(e), Err(_)) => return Err(e),
        }
    } else {
        (source?, None)
    };
    if items.len() > MAX_DEPENDENCIES {
        return Err(failure("This model has too many dependencies to inspect."));
    }
    if items.iter().map(|item| item.uses.len()).sum::<usize>() > MAX_REFERENCES {
        return Err(failure("This model has too many references to inspect."));
    }
    // Metadata is optional. Keep the references even when economy denies access or returns 404.
    items = stream::iter(items)
        .map(|mut item| async move {
            if let Ok(info) = economy::undocumented::get_asset_economy_details(client, item.id as i64).await {
                item.name = info.name.filter(|n| !n.is_empty());
                item.asset_type = info.asset_type_id.unwrap_or_default();
                let creator = info.creator.unwrap_or_default();
                item.creator = creator.name.unwrap_or_default();
                item.creator_id = creator.id.unwrap_or_default().max(0) as u64;
                item.creator_is_group = creator.creator_type.as_deref() == Some("Group");
                item.creator_verified = creator.has_verified_badge.unwrap_or_default();
            }
            item
        })
        .buffer_unordered(6)
        .collect()
        .await;
    items.sort_by_key(|item| item.id);
    Ok(Dependencies { items, note })
}

impl Core {
    /// Cached scan, isolated by account because model access depends on that account.
    pub fn item_dependencies(&self, account: AccountId, id: u64, asset_type: i32) -> Res<Dependencies> {
        self.inner.res(
            Key::private(account, "item_dependencies_v1", format!("{id}:{asset_type}")),
            15 * 60,
            move |f| async move {
                if id == 0 || id > MAX_ID {
                    return Err(failure("Invalid asset ID."));
                }
                scan(&f.client, id, asset_type).await
            },
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use rbx_dom_weak::{InstanceBuilder, types::ContentId};

    #[tokio::test]
    async fn replays_source_processed_and_unavailable_metadata() {
        let replay = bonsai_api::replay::Replay::start(&["dependencies"]).await;
        let result = scan(&replay.client(), 1000100, 8).await.unwrap();
        assert_eq!(result.items.len(), 2);
        assert!(result.note.is_none());
        assert_eq!(result.items[0].name.as_deref(), Some("Example mesh"));
        assert!(result.items[0].creator_is_group);
        assert!(result.items[0].creator_verified);
        assert_eq!(result.items[0].uses.len(), 2);
        assert_eq!(result.items[1].name, None);
        assert_eq!(result.items[1].uses.len(), 2);
        assert_eq!(replay.requests().await.len(), 6);
    }

    #[tokio::test]
    async fn either_representation_can_still_be_inspected() {
        for unavailable in [0, 2] {
            let mut fixture = bonsai_api::replay::load("dependencies");
            fixture.interactions[unavailable].response.body = serde_json::json!({
                "errors": [{"message": "Unavailable"}]
            });
            // The corresponding CDN response is deliberately never requested.
            let replay = bonsai_api::replay::Replay::from_fixture(&fixture).await;
            let result = scan(&replay.client(), 1000100, 8).await.unwrap();
            assert_eq!(result.items.len(), 2);
            assert!(result.note.is_some());
            assert_eq!(replay.requests().await.len(), 5);
        }
    }

    #[tokio::test]
    #[ignore = "live Roblox API, never run in CI"]
    async fn live_accessory_dependencies() {
        let client = Client::builder(AccountId::ANONYMOUS).build().unwrap();
        let result = scan(&client, 1365767, 8).await.unwrap();
        assert!(
            result
                .items
                .iter()
                .any(|d| d.uses.iter().any(|u| u.property.contains("Mesh")))
        );
        assert!(
            result
                .items
                .iter()
                .any(|d| d.uses.iter().any(|u| u.property.contains("Texture")))
        );
        assert!(result.items.iter().all(|d| d.id != 1365767));
    }

    #[test]
    fn only_exact_asset_references() {
        for value in [
            "rbxassetid://42",
            " RBXASSETID://42 ",
            "https://www.roblox.com/asset/?ID=42",
            "http://roblox.com/asset?id=42&version=2",
        ] {
            assert_eq!(reference(value), Some(42), "{value}");
        }
        for value in [
            "rbxassetid://0",
            "rbxassetid://9007199254740992",
            "rbxassetid://42junk",
            "rbxasset://textures/face.png",
            "https://evil.test/asset?id=42",
            "https://roblox.com.evil.test/asset?id=42",
            "https://roblox.com/users/42?id=42",
            "print('rbxassetid://42')",
            "42",
        ] {
            assert_eq!(reference(value), None, "{value}");
        }
    }

    fn model_dom() -> WeakDom {
        WeakDom::new(
            InstanceBuilder::new("DataModel").with_child(
                InstanceBuilder::new("Accessory")
                    .with_name("Hat")
                    .with_child(
                        InstanceBuilder::new("MeshPart")
                            .with_name("Handle")
                            .with_property("MeshId", ContentId::from("rbxassetid://42"))
                            .with_property("TextureID", ContentId::from("rbxassetid://43"))
                            .with_child(
                                InstanceBuilder::new("SurfaceAppearance")
                                    .with_property("ColorMap", ContentId::from("rbxassetid://43"))
                                    .with_property(
                                        "NormalMap",
                                        ContentId::from("https://www.roblox.com/asset/?id=44"),
                                    ),
                            ),
                    )
                    .with_child(InstanceBuilder::new("Script").with_property("Source", "rbxassetid://999")),
            ),
        )
    }

    fn assert_model(items: &[Dependency]) {
        assert_eq!(items.iter().map(|i| i.id).collect::<Vec<_>>(), [42, 43, 44]);
        assert_eq!(items[1].uses.len(), 2);
        assert!(
            items[1]
                .uses
                .iter()
                .any(|u| u.path == "Hat/Handle" && u.class == "MeshPart")
        );
    }

    #[test]
    fn binary_and_xml_have_the_same_references() {
        let dom = model_dom();
        let mut xml = Vec::new();
        rbx_xml::to_writer_default(&mut xml, &dom, dom.root().children()).unwrap();
        assert_model(&parse(&xml, 1).unwrap());
        for compression in [
            rbx_binary::CompressionType::None,
            rbx_binary::CompressionType::Lz4,
            rbx_binary::CompressionType::Zstd,
        ] {
            let mut binary = Vec::new();
            rbx_binary::Serializer::new()
                .compression_type(compression)
                .serialize(&mut binary, &dom, dom.root().children())
                .unwrap();
            assert_model(&parse(&binary, 1).unwrap());
            assert!(!parse(&binary, 43).unwrap().iter().any(|d| d.id == 43));
        }
    }

    #[test]
    fn empty_model_is_distinct_from_unreadable_data() {
        assert!(parse(b"<roblox version=\"4\"></roblox>", 1).unwrap().is_empty());
        assert!(parse(b"version 5.00\n", 1).is_err());
        assert!(parse(b"<roblox><Item", 1).is_err());
        assert!(parse(b"<roblox!", 1).is_err());
    }

    #[test]
    fn oversized_binary_headers_and_chunks_are_rejected_before_decoding() {
        let mut bytes = vec![0u8; 48];
        bytes[..8].copy_from_slice(b"<roblox!");
        bytes[20..24].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(binary_budget(&bytes).is_err());
        bytes[20..24].copy_from_slice(&1u32.to_le_bytes());
        bytes[40..44].copy_from_slice(&u32::MAX.to_le_bytes());
        assert!(binary_budget(&bytes).is_err());
    }

    #[test]
    fn processed_formats_do_not_replace_layered_clothing() {
        assert_eq!(processed_format(8), Some("avatar_meshpart_accessory"));
        assert_eq!(processed_format(79), Some("avatar_meshpart_head"));
        assert_eq!(processed_format(64), None);
        assert_eq!(processed_format(3), None);
    }
}
