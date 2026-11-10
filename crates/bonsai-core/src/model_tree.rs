//! The decoded instance tree of a downloadable model, with every property as display text.
//! Read only: scripts are never executed and referenced assets are not downloaded. Results are
//! private to the account allowed to read the model.

use std::collections::HashMap;

use bonsai_api::{AccountId, Client, Error};
use rbx_dom_weak::WeakDom;
use rbx_dom_weak::types::{
    CFrame, Color3, Color3uint8, ContentType, PhysicalProperties, Ref, Variant, Vector2, Vector3,
};
use rbx_reflection::{DataType, ReflectionDatabase};
use serde::{Deserialize, Serialize};

use crate::Core;
use crate::dependencies::{
    MAX_ID, MAX_INSTANCES, content_property, decode, download, failure, processed_format, reference,
};
use crate::res::{Key, Res};

const MAX_DEPTH: u16 = 128;
/// Longer values (script sources, long strings) are cut. Copying copies the cut text.
const MAX_VALUE: usize = 4096;
/// Total display text across all properties, so a pathological model can't fill the cache.
const MAX_TEXT: usize = 8 * 1024 * 1024;

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ModelProp {
    pub name: String,
    /// The value's type: `Vector3`, `Content`, or the enum's name for enums.
    pub kind: String,
    pub value: String,
    /// An asset the value points at, for content properties.
    pub asset: Option<u64>,
    /// The node a `Ref` (or an object `Content`) points at.
    pub target: Option<u32>,
    /// `#rrggbb` for colors.
    pub color: Option<String>,
    pub truncated: bool,
    /// From the instance's Attributes rather than a class property.
    pub attribute: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct ModelNode {
    pub parent: Option<u32>,
    pub depth: u16,
    pub class: String,
    pub name: String,
    pub children: u32,
    pub props: Vec<ModelProp>,
}

/// Nodes in depth-first order: each node's descendants follow it directly.
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
pub struct ModelTree {
    pub nodes: Vec<ModelNode>,
}

impl ModelTree {
    /// `Hat/Handle/Attachment`, like the dependency list's paths.
    pub fn path(&self, node: u32) -> String {
        let mut parts = Vec::new();
        let mut at = Some(node);
        while let Some(i) = at {
            let Some(n) = self.nodes.get(i as usize) else {
                break;
            };
            parts.push(n.name.as_str());
            at = n.parent;
        }
        parts.reverse();
        parts.join("/")
    }
}

fn too_large() -> Error {
    failure("This model is too large to inspect.")
}

fn num32(v: f32) -> String {
    if v == 0.0 { "0".into() } else { v.to_string() }
}

fn v2(v: Vector2) -> String {
    format!("{}, {}", num32(v.x), num32(v.y))
}

fn v3(v: Vector3) -> String {
    format!("{}, {}, {}", num32(v.x), num32(v.y), num32(v.z))
}

fn round3(v: f32) -> f32 {
    (v * 1000.0).round() / 1000.0
}

/// Position plus Studio's Orientation (degrees, applied Y, X, Z).
fn cframe(c: CFrame) -> String {
    let m = c.orientation;
    let x = (-m.y.z).clamp(-1.0, 1.0).asin();
    let (y, z) = if m.y.z.abs() < 0.9999 {
        (m.x.z.atan2(m.z.z), m.y.x.atan2(m.y.y))
    } else {
        ((-m.z.x).atan2(m.x.x), 0.0)
    };
    let deg = |r: f32| num32(round3(r.to_degrees()) + 0.0);
    format!(
        "Position {} · Orientation {}, {}, {}",
        v3(c.position),
        deg(x),
        deg(y),
        deg(z)
    )
}

fn hex(c: Color3uint8) -> String {
    format!("#{:02x}{:02x}{:02x}", c.r, c.g, c.b)
}

fn to_u8(c: Color3) -> Color3uint8 {
    let f = |v: f32| (v.clamp(0.0, 1.0) * 255.0).round() as u8;
    Color3uint8::new(f(c.r), f(c.g), f(c.b))
}

fn rgb(c: Color3uint8) -> String {
    format!("{}, {}, {}", c.r, c.g, c.b)
}

fn bytes(n: usize) -> String {
    if n == 1 {
        "1 byte".into()
    } else {
        format!("{n} bytes")
    }
}

struct Ctx<'a> {
    db: &'static ReflectionDatabase<'static>,
    index: &'a HashMap<Ref, u32>,
    paths: &'a [String],
}

impl Ctx<'_> {
    fn enum_name(&self, class: &str, property: &str) -> Option<&'static str> {
        let class = self.db.classes.get(class)?;
        self.db
            .superclasses_iter(class)
            .find_map(|c| c.properties.get(property))
            .and_then(|p| match p.data_type {
                DataType::Enum(name) => Some(name),
                _ => None,
            })
    }

    fn enum_item(&self, ty: &str, value: u32) -> Option<String> {
        let e = self.db.enums.get(ty)?;
        let (name, _) = e.items.iter().find(|(_, v)| **v == value)?;
        Some(format!("Enum.{ty}.{name}"))
    }

    fn reference(&self, r: &Ref, prop: &mut ModelProp) {
        match self.index.get(r) {
            Some(&i) => {
                prop.target = Some(i);
                prop.value = self.paths[i as usize].clone();
            }
            None if r.is_none() => prop.value = "None".into(),
            None => prop.value = "Outside this model".into(),
        }
    }

    /// Fills `prop` from `value`. `class` is None for attributes, which have no reflection data.
    fn show(&self, class: Option<&str>, name: &str, value: &Variant, prop: &mut ModelProp) {
        prop.kind = format!("{:?}", value.ty());
        prop.value = match value {
            Variant::String(s) => {
                if class.is_some() && content_property(name) {
                    prop.asset = reference(s);
                }
                s.clone()
            }
            Variant::BinaryString(b) => {
                let b: &[u8] = b.as_ref();
                match std::str::from_utf8(b) {
                    Ok(s)
                        if !s
                            .chars()
                            .any(|c| c.is_control() && c != '\n' && c != '\t' && c != '\r') =>
                    {
                        s.to_owned()
                    }
                    _ => bytes(b.len()),
                }
            }
            Variant::SharedString(s) => bytes(s.data().len()),
            Variant::Bool(b) => b.to_string(),
            Variant::Int32(v) => v.to_string(),
            Variant::Int64(v) => v.to_string(),
            Variant::Float32(v) => num32(*v),
            Variant::Float64(v) => {
                if *v == 0.0 {
                    "0".into()
                } else {
                    v.to_string()
                }
            }
            Variant::Vector2(v) => v2(*v),
            Variant::Vector3(v) => v3(*v),
            Variant::Vector2int16(v) => format!("{}, {}", v.x, v.y),
            Variant::Vector3int16(v) => format!("{}, {}, {}", v.x, v.y, v.z),
            Variant::CFrame(c) => cframe(*c),
            Variant::OptionalCFrame(c) => c.map(cframe).unwrap_or_else(|| "None".into()),
            Variant::Color3(c) => {
                let c = to_u8(*c);
                prop.color = Some(hex(c));
                rgb(c)
            }
            Variant::Color3uint8(c) => {
                prop.color = Some(hex(*c));
                rgb(*c)
            }
            Variant::BrickColor(b) => {
                prop.color = Some(hex(b.to_color3uint8()));
                b.to_string()
            }
            Variant::ColorSequence(s) => s
                .keypoints
                .iter()
                .map(|k| format!("{}: {}", num32(k.time), rgb(to_u8(k.color))))
                .collect::<Vec<_>>()
                .join("; "),
            Variant::NumberSequence(s) => s
                .keypoints
                .iter()
                .map(|k| {
                    if k.envelope == 0.0 {
                        format!("{}: {}", num32(k.time), num32(k.value))
                    } else {
                        format!("{}: {} ± {}", num32(k.time), num32(k.value), num32(k.envelope))
                    }
                })
                .collect::<Vec<_>>()
                .join("; "),
            Variant::NumberRange(r) => format!("{} to {}", num32(r.min), num32(r.max)),
            Variant::Rect(r) => format!("{} to {}", v2(r.min), v2(r.max)),
            Variant::UDim(u) => format!("{}, {}", num32(u.scale), u.offset),
            Variant::UDim2(u) => format!(
                "{{{}, {}}}, {{{}, {}}}",
                num32(u.x.scale),
                u.x.offset,
                num32(u.y.scale),
                u.y.offset
            ),
            Variant::Ray(r) => format!("Origin {} · Direction {}", v3(r.origin), v3(r.direction)),
            Variant::Region3(r) => format!("{} to {}", v3(r.min), v3(r.max)),
            Variant::Region3int16(r) => format!(
                "{}, {}, {} to {}, {}, {}",
                r.min.x, r.min.y, r.min.z, r.max.x, r.max.y, r.max.z
            ),
            Variant::Enum(e) => {
                let ty = class.and_then(|c| self.enum_name(c, name));
                if let Some(ty) = ty {
                    prop.kind = ty.into();
                }
                ty.and_then(|ty| self.enum_item(ty, e.to_u32()))
                    .unwrap_or_else(|| e.to_u32().to_string())
            }
            Variant::EnumItem(e) => {
                prop.kind = e.ty.clone();
                self.enum_item(&e.ty, e.value)
                    .unwrap_or_else(|| format!("Enum.{}.{}", e.ty, e.value))
            }
            Variant::Ref(r) => {
                self.reference(r, prop);
                return;
            }
            Variant::ContentId(c) => {
                prop.asset = reference(c.as_str());
                c.as_str().to_owned()
            }
            Variant::Content(c) => match c.value() {
                ContentType::Uri(uri) => {
                    prop.asset = reference(uri);
                    uri.clone()
                }
                ContentType::Object(r) => {
                    self.reference(r, prop);
                    return;
                }
                _ => "None".into(),
            },
            Variant::PhysicalProperties(PhysicalProperties::Custom(p)) => format!(
                "Density {} · Friction {} · Elasticity {} · Friction weight {} · Elasticity weight {}",
                num32(p.density()),
                num32(p.friction()),
                num32(p.elasticity()),
                num32(p.friction_weight()),
                num32(p.elasticity_weight())
            ),
            Variant::PhysicalProperties(_) => "Default".into(),
            Variant::Faces(f) => format!("{f:?}"),
            Variant::Axes(a) => format!("{a:?}"),
            Variant::Font(f) => format!("{} · {:?} · {:?}", f.family, f.weight, f.style),
            Variant::Tags(t) => t.iter().collect::<Vec<_>>().join(", "),
            Variant::UniqueId(u) => u.to_string(),
            Variant::SecurityCapabilities(s) => s.bits().to_string(),
            Variant::MaterialColors(_) => "Terrain material colors".into(),
            Variant::Attributes(a) => {
                let n = a.iter().count();
                if n == 1 {
                    "1 attribute".into()
                } else {
                    format!("{n} attributes")
                }
            }
            other => format!("{other:?}"),
        };
        if prop.value.len() > MAX_VALUE {
            let mut end = MAX_VALUE;
            while !prop.value.is_char_boundary(end) {
                end -= 1;
            }
            prop.value.truncate(end);
            prop.truncated = true;
        }
    }
}

/// Builds the tree. Fails, rather than returning part of it, past the instance, depth or text
/// budgets.
pub(crate) fn build(dom: &WeakDom) -> Result<ModelTree, Error> {
    // First pass: order, parents and paths, so Ref properties can name any node.
    let mut order: Vec<(Ref, Option<u32>, u16)> = Vec::new();
    let mut index: HashMap<Ref, u32> = HashMap::new();
    let mut paths: Vec<String> = Vec::new();
    let mut pending: Vec<(Ref, Option<u32>, u16)> = dom
        .root()
        .children()
        .iter()
        .rev()
        .map(|r| (*r, None, 0))
        .collect();
    while let Some((referent, parent, depth)) = pending.pop() {
        if order.len() >= MAX_INSTANCES as usize || depth > MAX_DEPTH {
            return Err(too_large());
        }
        let Some(instance) = dom.get_by_ref(referent) else {
            continue;
        };
        let at = order.len() as u32;
        let name: String = instance.name.chars().take(120).collect();
        let path = match parent {
            Some(p) => format!("{}/{name}", paths[p as usize]),
            None => name,
        };
        if path.len() > 2048 {
            return Err(failure("This model's instance paths are too long to inspect."));
        }
        paths.push(path);
        index.insert(referent, at);
        order.push((referent, parent, depth));
        for child in instance.children().iter().rev() {
            pending.push((*child, Some(at), depth + 1));
        }
    }
    let ctx = Ctx {
        db: rbx_reflection_database::get_bundled(),
        index: &index,
        paths: &paths,
    };
    let mut text = 0usize;
    let mut nodes: Vec<ModelNode> = Vec::with_capacity(order.len());
    for (referent, parent, depth) in &order {
        let Some(instance) = dom.get_by_ref(*referent) else {
            continue;
        };
        let class = instance.class.as_str();
        let mut props = vec![ModelProp {
            name: "Name".into(),
            kind: "String".into(),
            value: instance.name.clone(),
            ..Default::default()
        }];
        let mut attributes = Vec::new();
        let mut names: Vec<_> = instance.properties.keys().collect();
        names.sort_by(|a, b| a.as_str().cmp(b.as_str()));
        for name in names {
            let value = &instance.properties[name];
            if let Variant::Attributes(a) = value {
                for (key, v) in a.iter() {
                    let mut prop = ModelProp {
                        name: key.clone(),
                        attribute: true,
                        ..Default::default()
                    };
                    ctx.show(None, key, v, &mut prop);
                    attributes.push(prop);
                }
                continue;
            }
            let mut prop = ModelProp {
                name: name.to_string(),
                ..Default::default()
            };
            ctx.show(Some(class), name.as_str(), value, &mut prop);
            props.push(prop);
        }
        props.extend(attributes);
        text += props.iter().map(|p| p.value.len() + p.name.len()).sum::<usize>();
        if text > MAX_TEXT {
            return Err(too_large());
        }
        nodes.push(ModelNode {
            parent: *parent,
            depth: *depth,
            class: class.to_owned(),
            name: instance.name.chars().take(120).collect(),
            children: instance.children().len() as u32,
            props,
        });
    }
    Ok(ModelTree { nodes })
}

async fn load(client: &Client, id: u64, format: &'static str) -> Result<ModelTree, Error> {
    let bytes = download(client, id, format).await?;
    tokio::task::spawn_blocking(move || build(&decode(&bytes)?))
        .await
        .map_err(|_| failure("Couldn't inspect this model."))?
}

/// Whether Roblox also serves a processed (MeshPart) form of this asset type.
pub fn has_processed(asset_type: i32) -> bool {
    processed_format(asset_type).is_some()
}

impl Core {
    /// The source model, or with `processed` the avatar MeshPart form for accessories and heads.
    /// Cached per account because model access depends on the account.
    pub fn item_model(
        &self,
        account: AccountId,
        id: u64,
        asset_type: i32,
        processed: bool,
    ) -> Res<ModelTree> {
        let format = if processed {
            processed_format(asset_type)
        } else {
            Some("")
        };
        self.inner.res(
            Key::private(
                account,
                "item_model_v1",
                format!(
                    "{id}:{}",
                    format.map_or("none", |f| if f.is_empty() { "source" } else { f })
                ),
            ),
            15 * 60,
            move |f| async move {
                if id == 0 || id > MAX_ID {
                    return Err(failure("Invalid asset ID."));
                }
                let format = format.ok_or_else(|| failure("This item has no processed model."))?;
                load(&f.client, id, format).await
            },
        )
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use rbx_dom_weak::InstanceBuilder;
    use rbx_dom_weak::types::{Attributes, ContentId, Enum, Matrix3};

    fn model() -> WeakDom {
        let mut dom = WeakDom::new(InstanceBuilder::new("DataModel"));
        let root = dom.root_ref();
        let hat = dom.insert(root, InstanceBuilder::new("Accessory").with_name("Hat"));
        let handle = dom.insert(
            hat,
            InstanceBuilder::new("Part")
                .with_name("Handle")
                .with_property("Material", Enum::from_u32(256))
                .with_property("Color", Color3::new(1.0, 0.0, 0.5))
                .with_property("Size", Vector3::new(1.0, 2.5, 0.0))
                .with_property(
                    "CFrame",
                    CFrame::new(Vector3::new(0.0, 1.0, 0.0), Matrix3::identity()),
                ),
        );
        dom.insert(
            handle,
            InstanceBuilder::new("SpecialMesh")
                .with_property("MeshId", ContentId::from("rbxassetid://42"))
                .with_property("TextureId", ContentId::from("rbxasset://textures/x.png")),
        );
        let mut attributes = Attributes::new();
        attributes.insert("Tilt".into(), Variant::Float64(1.5));
        let weld = dom.insert(
            hat,
            InstanceBuilder::new("Weld")
                .with_property("Part0", handle)
                .with_property("Attributes", attributes),
        );
        dom.insert(
            weld,
            InstanceBuilder::new("Script").with_property("Source", "x".repeat(MAX_VALUE + 10)),
        );
        dom
    }

    fn prop<'a>(node: &'a ModelNode, name: &str) -> &'a ModelProp {
        node.props.iter().find(|p| p.name == name).expect(name)
    }

    fn check(tree: &ModelTree) {
        let names: Vec<_> = tree.nodes.iter().map(|n| (n.class.as_str(), n.depth)).collect();
        assert_eq!(
            names,
            [
                ("Accessory", 0),
                ("Part", 1),
                ("SpecialMesh", 2),
                ("Weld", 1),
                ("Script", 2)
            ]
        );
        assert_eq!(tree.nodes[0].children, 2);
        assert_eq!(tree.nodes[3].parent, Some(0));
        assert_eq!(tree.path(2), "Hat/Handle/SpecialMesh");

        let handle = &tree.nodes[1];
        assert_eq!(handle.props[0].name, "Name");
        assert_eq!(handle.props[0].value, "Handle");
        let material = prop(handle, "Material");
        assert_eq!(material.kind, "Material");
        assert_eq!(material.value, "Enum.Material.Plastic");
        let color = prop(handle, "Color");
        assert_eq!(color.value, "255, 0, 128");
        assert_eq!(color.color.as_deref(), Some("#ff0080"));
        assert_eq!(prop(handle, "Size").value, "1, 2.5, 0");
        assert_eq!(
            prop(handle, "CFrame").value,
            "Position 0, 1, 0 · Orientation 0, 0, 0"
        );

        // rbx-dom migrates SpecialMesh's legacy ids to Content properties, as Studio does.
        let mesh = &tree.nodes[2];
        assert_eq!(prop(mesh, "MeshContent").kind, "Content");
        assert_eq!(prop(mesh, "MeshContent").asset, Some(42));
        assert_eq!(prop(mesh, "TextureContent").asset, None);

        let weld = &tree.nodes[3];
        let part0 = prop(weld, "Part0");
        assert_eq!(part0.target, Some(1));
        assert_eq!(part0.value, "Hat/Handle");
        let tilt = prop(weld, "Tilt");
        assert!(tilt.attribute);
        assert_eq!(tilt.value, "1.5");
        assert_eq!(weld.props.last().unwrap().name, "Tilt");

        let source = prop(&tree.nodes[4], "Source");
        assert!(source.truncated);
        assert_eq!(source.value.len(), MAX_VALUE);
    }

    #[test]
    fn binary_and_xml_decode_to_the_same_tree() {
        let dom = model();
        let mut xml = Vec::new();
        rbx_xml::to_writer_default(&mut xml, &dom, dom.root().children()).unwrap();
        check(&build(&decode(&xml).unwrap()).unwrap());
        let mut binary = Vec::new();
        rbx_binary::to_writer(&mut binary, &dom, dom.root().children()).unwrap();
        check(&build(&decode(&binary).unwrap()).unwrap());
    }

    #[test]
    fn deep_models_are_rejected() {
        let mut dom = WeakDom::new(InstanceBuilder::new("DataModel"));
        let mut at = dom.root_ref();
        for _ in 0..=MAX_DEPTH + 1 {
            at = dom.insert(at, InstanceBuilder::new("Folder"));
        }
        assert!(build(&dom).is_err());
    }

    #[test]
    fn orientation_matches_studio() {
        // 90 degrees about Y.
        let c = CFrame::new(
            Vector3::new(0.0, 0.0, 0.0),
            Matrix3::new(
                Vector3::new(0.0, 0.0, 1.0),
                Vector3::new(0.0, 1.0, 0.0),
                Vector3::new(-1.0, 0.0, 0.0),
            ),
        );
        assert_eq!(cframe(c), "Position 0, 0, 0 · Orientation 0, 90, 0");
    }

    #[tokio::test]
    async fn replays_source_and_processed() {
        let mut fixture = bonsai_api::replay::load("dependencies");
        let processed = fixture.interactions.split_off(2);
        let replay = bonsai_api::replay::Replay::from_fixture(&fixture).await;
        let tree = load(&replay.client(), 1000100, "").await.unwrap();
        assert_eq!(tree.nodes[0].class, "Accessory");
        assert!(tree.nodes.iter().any(|n| n.class == "SpecialMesh"));

        fixture.interactions = processed;
        let replay = bonsai_api::replay::Replay::from_fixture(&fixture).await;
        let tree = load(&replay.client(), 1000100, "avatar_meshpart_accessory")
            .await
            .unwrap();
        assert!(tree.nodes.iter().any(|n| n.class == "MeshPart"));
    }
}
