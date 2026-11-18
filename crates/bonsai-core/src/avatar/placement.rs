//! Moving, turning and sizing a worn rigid accessory (hats, face, neck, shoulder, front and back
//! accessories), within the bounds Roblox publishes in the avatar rules.
//!
//! Roblox keeps the adjustment in the worn asset's `meta` as `position`, `rotation` (degrees)
//! and `scale` vectors. Which asset types may be adjusted, and how far, comes from
//! `accessoryRefinementTypes` and `accessoryRefinement{Lower,Upper}Bounds` in
//! `GET /v1/avatar-rules`.

use std::collections::HashMap;

use bonsai_api::generated::avatar;
use serde::{Deserialize, Serialize};

/// Two values closer than this are the same. Roblox stores 1.11 as 1.1100000000000001 and may
/// hand back single-precision values.
const EPS: f64 = 1e-4;

/// One of the three adjustments.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum Group {
    Position,
    Rotation,
    Scale,
}

impl Group {
    pub const ALL: [Group; 3] = [Group::Position, Group::Rotation, Group::Scale];

    pub fn key(self) -> &'static str {
        match self {
            Group::Position => "position",
            Group::Rotation => "rotation",
            Group::Scale => "scale",
        }
    }

    pub fn parse(s: &str) -> Option<Group> {
        match s.trim().to_ascii_lowercase().as_str() {
            "position" | "pos" | "p" | "move" | "offset" => Some(Group::Position),
            "rotation" | "rot" | "r" | "turn" | "angle" => Some(Group::Rotation),
            "scale" | "size" | "s" => Some(Group::Scale),
            _ => None,
        }
    }

    /// The value of every axis when nothing is adjusted.
    pub fn neutral(self) -> f64 {
        match self {
            Group::Scale => 1.0,
            _ => 0.0,
        }
    }

    /// Edited values are rounded to this: studs, degrees, or a scale factor.
    pub fn step(self) -> f64 {
        match self {
            Group::Position => 0.01,
            Group::Rotation => 0.1,
            Group::Scale => 0.01,
        }
    }

    /// Short names of the X, Y and Z axes as the editor shows them.
    pub fn axis_labels(self) -> [&'static str; 3] {
        match self {
            Group::Position => ["Side", "Height", "Depth"],
            Group::Rotation => ["Tilt", "Turn", "Roll"],
            Group::Scale => ["Width", "Height", "Depth"],
        }
    }
}

const AXES: [char; 3] = ['x', 'y', 'z'];

/// Where an accessory sits relative to where it attaches. Rotation is in degrees.
#[derive(Clone, Copy, Debug, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct Placement {
    pub position: [f64; 3],
    pub rotation: [f64; 3],
    pub scale: [f64; 3],
}

impl Default for Placement {
    fn default() -> Self {
        Placement {
            position: [0.0; 3],
            rotation: [0.0; 3],
            scale: [1.0; 3],
        }
    }
}

impl Placement {
    pub fn get(&self, g: Group) -> [f64; 3] {
        match g {
            Group::Position => self.position,
            Group::Rotation => self.rotation,
            Group::Scale => self.scale,
        }
    }

    fn get_mut(&mut self, g: Group) -> &mut [f64; 3] {
        match g {
            Group::Position => &mut self.position,
            Group::Rotation => &mut self.rotation,
            Group::Scale => &mut self.scale,
        }
    }

    fn values(&self) -> impl Iterator<Item = f64> + '_ {
        self.position
            .iter()
            .chain(&self.rotation)
            .chain(&self.scale)
            .copied()
    }

    pub fn is_finite(&self) -> bool {
        self.values().all(f64::is_finite)
    }

    /// Equal within rounding.
    pub fn same(&self, other: &Placement) -> bool {
        self.values()
            .zip(other.values())
            .all(|(a, b)| (a - b).abs() < EPS)
    }

    pub fn is_default(&self) -> bool {
        self.same(&Placement::default())
    }

    /// Reflected across the avatar's middle, for the accessory on the other side.
    pub fn mirrored(mut self) -> Placement {
        self.position[0] = -self.position[0];
        self.rotation[1] = -self.rotation[1];
        self.rotation[2] = -self.rotation[2];
        self
    }
}

/// How far one asset type may be adjusted.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
#[serde(default)]
pub struct PlacementRule {
    pub type_id: i64,
    pub min: Placement,
    pub max: Placement,
}

impl PlacementRule {
    pub fn range(&self, g: Group, axis: usize) -> (f64, f64) {
        (self.min.get(g)[axis], self.max.get(g)[axis])
    }

    pub fn contains(&self, p: &Placement) -> bool {
        Group::ALL.iter().all(|&g| {
            (0..3).all(|i| {
                let (lo, hi) = self.range(g, i);
                let v = p.get(g)[i];
                v >= lo - EPS && v <= hi + EPS
            })
        })
    }

    /// `p` with every axis moved inside the bounds.
    pub fn clamp(&self, mut p: Placement) -> Placement {
        for g in Group::ALL {
            for i in 0..3 {
                let (lo, hi) = self.range(g, i);
                let v = &mut p.get_mut(g)[i];
                *v = round(v.clamp(lo, hi));
            }
        }
        p
    }
}

fn round(v: f64) -> f64 {
    (v * 10_000.0).round() / 10_000.0
}

/// A number for messages: no trailing zeros.
fn num(v: f64) -> String {
    let s = format!("{:.4}", round(v));
    let s = s.trim_end_matches('0').trim_end_matches('.');
    if s == "-0" { "0".into() } else { s.into() }
}

fn axis_name(g: Group, axis: usize) -> String {
    format!("{} {}", g.key(), AXES[axis].to_ascii_uppercase())
}

/// The adjustment stored in an asset's meta. `None` when it has none, which Roblox treats as
/// the default placement. A missing axis is the default for it.
pub fn from_meta(meta: Option<&avatar::v1::AssetMetaModelV1>) -> Option<Placement> {
    let m = meta?;
    if m.position.is_none() && m.rotation.is_none() && m.scale.is_none() {
        return None;
    }
    let d = Placement::default();
    let pick = |v: Option<f64>, d: f64| v.filter(|v| v.is_finite()).unwrap_or(d);
    let p = Placement {
        position: m
            .position
            .as_ref()
            .map_or(d.position, |v| [pick(v.x, 0.0), pick(v.y, 0.0), pick(v.z, 0.0)]),
        rotation: m
            .rotation
            .as_ref()
            .map_or(d.rotation, |v| [pick(v.x, 0.0), pick(v.y, 0.0), pick(v.z, 0.0)]),
        scale: m
            .scale
            .as_ref()
            .map_or(d.scale, |v| [pick(v.x, 1.0), pick(v.y, 1.0), pick(v.z, 1.0)]),
    };
    Some(p)
}

/// `meta` with the placement written in. Layer order and puffiness are kept, the format
/// version is set when missing, and every axis is written out, including defaults, so Roblox
/// never falls back to a value we did not mean.
pub fn to_meta(meta: Option<avatar::v1::AssetMetaModelV1>, p: Placement) -> avatar::v1::AssetMetaModelV1 {
    let mut m = meta.unwrap_or_default();
    m.version.get_or_insert(1);
    let [x, y, z] = p.position.map(round);
    m.position = Some(avatar::v1::AssetPosition {
        x: Some(x),
        y: Some(y),
        z: Some(z),
    });
    let [x, y, z] = p.rotation.map(round);
    m.rotation = Some(avatar::v1::AssetRotation {
        x: Some(x),
        y: Some(y),
        z: Some(z),
    });
    let [x, y, z] = p.scale.map(round);
    m.scale = Some(avatar::v1::AssetScale {
        x: Some(x),
        y: Some(y),
        z: Some(z),
    });
    m
}

/// The bounds of each adjustable asset type. A type is left out unless Roblox lists it as
/// adjustable and gives complete, ordered bounds for it.
pub fn rules(r: &avatar::v1::AvatarRulesModel) -> Vec<PlacementRule> {
    let lower: HashMap<String, avatar::v1::AccessoryRefinementModel> =
        super::convert(r.accessory_refinement_lower_bounds.as_ref()).unwrap_or_default();
    let upper: HashMap<String, avatar::v1::AccessoryRefinementModel> =
        super::convert(r.accessory_refinement_upper_bounds.as_ref()).unwrap_or_default();
    let mut out: Vec<PlacementRule> = r
        .accessory_refinement_types
        .iter()
        .flatten()
        .filter_map(|t| {
            let name = t.name()?;
            let rule = PlacementRule {
                type_id: t.value()?,
                min: bound(lower.get(name)?)?,
                max: bound(upper.get(name)?)?,
            };
            let ordered = rule.min.values().zip(rule.max.values()).all(|(lo, hi)| lo <= hi);
            ordered.then_some(rule)
        })
        .collect();
    out.sort_by_key(|r| r.type_id);
    out.dedup_by_key(|r| r.type_id);
    out
}

fn bound(m: &avatar::v1::AccessoryRefinementModel) -> Option<Placement> {
    let p = m.position.as_ref()?;
    let r = m.rotation.as_ref()?;
    let s = m.scale.as_ref()?;
    let p = Placement {
        position: [p.x_position?, p.y_position?, p.z_position?],
        rotation: [r.x_rotation?, r.y_rotation?, r.z_rotation?],
        scale: [s.x_scale?, s.y_scale?, s.z_scale?],
    };
    p.is_finite().then_some(p)
}

/// A change to one accessory's placement.
#[derive(Clone, Debug, PartialEq)]
pub enum Adjust {
    /// Set these axes to these values.
    Set(Vec<(Group, usize, f64)>),
    /// Back to the default placement, for one group or all of it.
    Reset(Option<Group>),
    /// Take another worn accessory's placement, reflected when `mirrored`.
    Copy { from: u64, mirrored: bool },
}

impl Adjust {
    /// The other accessory this copies from.
    pub fn source(&self) -> Option<u64> {
        match self {
            Adjust::Copy { from, .. } => Some(*from),
            _ => None,
        }
    }
}

/// Parse `<item id> <what>` where `<what>` is one or more `<axis> <value>` pairs
/// (`position.x 0.1`, `ry -5`, `scale 110%`), `reset [position|rotation|scale]`, or
/// `match <item id>` / `mirror <item id>`.
///
/// An axis is a group (`position`, `rotation`, `scale` or `p`, `r`, `s`), optionally followed
/// by `.x`, `.y` or `.z`; a group alone sets all three. Scale takes a factor or a percentage.
pub fn parse(text: &str) -> Result<(u64, Adjust), String> {
    const USAGE: &str = "Give an item id, then values like \"position.y 0.1 rotation.x -5 scale 110%\", \"reset\", or \"match\" or \"mirror\" and another item id.";
    let mut words = text.split_whitespace();
    let id = words.next().and_then(|w| w.parse::<u64>().ok()).ok_or(USAGE)?;
    let rest: Vec<&str> = words.collect();
    let Some(first) = rest.first() else {
        return Err(USAGE.into());
    };
    let first = first.to_ascii_lowercase();
    match first.as_str() {
        "reset" | "default" | "clear" => {
            let group = match rest.get(1) {
                Some(g) => Some(Group::parse(g).ok_or(USAGE)?),
                None => None,
            };
            if rest.len() > 2 {
                return Err(USAGE.into());
            }
            return Ok((id, Adjust::Reset(group)));
        }
        "match" | "copy" | "same" | "mirror" => {
            let from = rest
                .get(1)
                .and_then(|w| w.parse::<u64>().ok())
                .filter(|_| rest.len() == 2)
                .ok_or(USAGE)?;
            if from == id {
                return Err("Pick a different item to copy from.".into());
            }
            return Ok((
                id,
                Adjust::Copy {
                    from,
                    mirrored: first == "mirror",
                },
            ));
        }
        _ => {}
    }
    if !rest.len().is_multiple_of(2) {
        return Err(USAGE.into());
    }
    let mut set: Vec<(Group, usize, f64)> = Vec::new();
    for pair in rest.chunks(2) {
        let (group, axes) = parse_axis(pair[0]).ok_or(USAGE)?;
        let value = parse_value(pair[1], group).ok_or_else(|| format!("\"{}\" is not a number.", pair[1]))?;
        for axis in axes {
            set.retain(|&(g, a, _)| (g, a) != (group, axis));
            set.push((group, axis, value));
        }
    }
    Ok((id, Adjust::Set(set)))
}

/// `position.x`, `pos-x`, `px`, `rotation`, `s` and the like.
fn parse_axis(word: &str) -> Option<(Group, Vec<usize>)> {
    let w = word.to_ascii_lowercase();
    let w = w.trim_end_matches(':');
    let (group, axis) = if let Some((g, a)) = w.split_once(['.', '-', '_']) {
        (g, a)
    } else if let Some(g) = Group::parse(w) {
        return Some((g, vec![0, 1, 2]));
    } else if w.len() == 2 {
        w.split_at(1)
    } else {
        return None;
    };
    let group = Group::parse(group)?;
    let axis = AXES
        .iter()
        .position(|&c| axis.len() == 1 && axis.starts_with(c))?;
    Some((group, vec![axis]))
}

fn parse_value(word: &str, group: Group) -> Option<f64> {
    let w = word.trim();
    let v = if let Some(p) = w.strip_suffix('%') {
        if group != Group::Scale {
            return None;
        }
        p.parse::<f64>().ok()? / 100.0
    } else if group == Group::Rotation {
        w.trim_end_matches("deg")
            .trim_end_matches('°')
            .parse::<f64>()
            .ok()?
    } else {
        w.parse::<f64>().ok()?
    };
    v.is_finite().then_some(v)
}

/// The placement `adjust` leads to from `current`, inside `rule`. `source` is the placement of
/// the accessory a copy takes from.
///
/// A value set by hand outside the bounds is an error that names them. A copied placement, and
/// axes left alone that the bounds no longer allow, are moved inside instead.
pub fn apply(
    current: Placement,
    adjust: &Adjust,
    source: Option<Placement>,
    rule: &PlacementRule,
) -> Result<Placement, String> {
    let next = match adjust {
        Adjust::Set(values) => {
            if values.is_empty() {
                return Err("Nothing to change.".into());
            }
            let mut p = current;
            for &(g, axis, v) in values {
                if !v.is_finite() || axis > 2 {
                    return Err("Not a number.".into());
                }
                let (lo, hi) = rule.range(g, axis);
                if v < lo - EPS || v > hi + EPS {
                    return Err(format!(
                        "Roblox allows {} from {} to {}.",
                        axis_name(g, axis),
                        num(lo),
                        num(hi)
                    ));
                }
                let step = g.step();
                p.get_mut(g)[axis] = round((v / step).round() * step);
            }
            p
        }
        Adjust::Reset(None) => Placement::default(),
        Adjust::Reset(Some(g)) => {
            let mut p = current;
            *p.get_mut(*g) = [g.neutral(); 3];
            p
        }
        Adjust::Copy { mirrored, .. } => {
            let s = source.ok_or("That item is not worn.")?;
            if *mirrored { s.mirrored() } else { s }
        }
    };
    if !next.is_finite() {
        return Err("Not a number.".into());
    }
    Ok(rule.clamp(next))
}

/// Describe where Roblox put an accessory when it differs from what was asked.
pub fn describe(p: &Placement) -> String {
    Group::ALL
        .iter()
        .map(|&g| {
            let v = p.get(g);
            format!("{} {}, {}, {}", g.key(), num(v[0]), num(v[1]), num(v[2]))
        })
        .collect::<Vec<_>>()
        .join("; ")
}

#[cfg(test)]
mod tests {
    use super::*;

    fn rule(type_id: i64, rx: f64) -> PlacementRule {
        PlacementRule {
            type_id,
            min: Placement {
                position: [-0.25; 3],
                rotation: [-rx, -30.0, -30.0],
                scale: [0.8; 3],
            },
            max: Placement {
                position: [0.25; 3],
                rotation: [rx, 30.0, 30.0],
                scale: [1.2; 3],
            },
        }
    }

    fn refinement(p: f64, r: [f64; 3], s: f64) -> serde_json::Value {
        serde_json::json!({
            "position": {"xPosition": p, "yPosition": p, "zPosition": p},
            "rotation": {"xRotation": r[0], "yRotation": r[1], "zRotation": r[2]},
            "scale": {"xScale": s, "yScale": s, "zScale": s},
        })
    }

    #[test]
    fn reads_bounds_from_the_rules() {
        let r: avatar::v1::AvatarRulesModel = serde_json::from_value(serde_json::json!({
            "accessoryRefinementTypes": [8, 46, 41, 45],
            "accessoryRefinementLowerBounds": {
                "Hat": refinement(-0.25, [-30.0, -30.0, -30.0], 0.8),
                "BackAccessory": refinement(-0.25, [-15.0, -30.0, -30.0], 0.8),
                "HairAccessory": refinement(-0.25, [-30.0, -30.0, -30.0], 0.8),
                "FrontAccessory": {"position": {"xPosition": -0.25}},
            },
            "accessoryRefinementUpperBounds": {
                "Hat": refinement(0.25, [30.0, 30.0, 30.0], 1.2),
                "BackAccessory": refinement(0.25, [15.0, 30.0, 30.0], 1.2),
                "HairAccessory": refinement(-0.5, [30.0, 30.0, 30.0], 1.2),
                "FrontAccessory": refinement(0.25, [15.0, 30.0, 30.0], 1.2),
                "WaistAccessory": refinement(0.25, [30.0, 30.0, 30.0], 1.2),
            },
        }))
        .unwrap();
        let got = rules(&r);
        // Hair: upper below lower. Front: incomplete lower. Waist: not listed as adjustable.
        assert_eq!(got, vec![rule(8, 30.0), rule(46, 15.0)]);
    }

    #[test]
    fn missing_bounds_allow_nothing() {
        assert!(rules(&avatar::v1::AvatarRulesModel::default()).is_empty());
    }

    #[test]
    fn reads_and_writes_meta() {
        assert_eq!(from_meta(None), None);
        let layered = avatar::v1::AssetMetaModelV1 {
            order: Some(3),
            puffiness: Some(0.5),
            ..Default::default()
        };
        assert_eq!(from_meta(Some(&layered)), None);

        let p = Placement {
            position: [0.1, -0.2, 0.0],
            rotation: [5.0, 0.0, -12.5],
            scale: [1.1, 1.0, 0.9],
        };
        let m = to_meta(Some(layered), p);
        assert_eq!((m.order, m.puffiness, m.version), (Some(3), Some(0.5), Some(1)));
        assert_eq!(from_meta(Some(&m)), Some(p));

        let json = serde_json::to_value(to_meta(None, Placement::default())).unwrap();
        assert_eq!(
            json,
            serde_json::json!({
                "position": {"X": 0.0, "Y": 0.0, "Z": 0.0},
                "rotation": {"X": 0.0, "Y": 0.0, "Z": 0.0},
                "scale": {"X": 1.0, "Y": 1.0, "Z": 1.0},
                "version": 1,
            }),
            "defaults are written out"
        );

        let partial: avatar::v1::AssetMetaModelV1 =
            serde_json::from_value(serde_json::json!({"scale": {"X": 1.1}})).unwrap();
        assert_eq!(
            from_meta(Some(&partial)),
            Some(Placement {
                scale: [1.1, 1.0, 1.0],
                ..Default::default()
            })
        );
    }

    #[test]
    fn keeps_an_existing_version() {
        let m = avatar::v1::AssetMetaModelV1 {
            version: Some(2),
            ..Default::default()
        };
        assert_eq!(to_meta(Some(m), Placement::default()).version, Some(2));
    }

    #[test]
    fn compares_within_rounding() {
        let a = Placement {
            scale: [1.11, 1.0, 1.0],
            ..Default::default()
        };
        let b = Placement {
            scale: [1.11 + 1e-12, 1.000_000_02, 1.0],
            ..Default::default()
        };
        assert!(a.same(&b));
        assert!(!a.same(&Placement::default()));
        assert!(Placement::default().is_default());
    }

    #[test]
    fn mirrors_across_the_middle() {
        let p = Placement {
            position: [0.1, 0.2, 0.3],
            rotation: [1.0, 2.0, 3.0],
            scale: [1.1, 1.0, 0.9],
        };
        let m = p.mirrored();
        assert_eq!(m.position, [-0.1, 0.2, 0.3]);
        assert_eq!(m.rotation, [1.0, -2.0, -3.0]);
        assert_eq!(m.scale, p.scale);
        assert_eq!(m.mirrored(), p);
    }

    #[test]
    fn clamps_into_bounds() {
        let r = rule(46, 15.0);
        let p = Placement {
            position: [1.0, -1.0, 0.1],
            rotation: [20.0, -40.0, 0.0],
            scale: [2.0, 0.1, 1.0],
        };
        assert!(!r.contains(&p));
        let c = r.clamp(p);
        assert_eq!(c.position, [0.25, -0.25, 0.1]);
        assert_eq!(c.rotation, [15.0, -30.0, 0.0]);
        assert_eq!(c.scale, [1.2, 0.8, 1.0]);
        assert!(r.contains(&c));
    }

    #[test]
    fn parses_values() {
        let (id, a) = parse("123 position.x 0.1 ry -5 scale 110% rotation.z 4°").unwrap();
        assert_eq!(id, 123);
        assert_eq!(
            a,
            Adjust::Set(vec![
                (Group::Position, 0, 0.1),
                (Group::Rotation, 1, -5.0),
                (Group::Scale, 0, 1.1),
                (Group::Scale, 1, 1.1),
                (Group::Scale, 2, 1.1),
                (Group::Rotation, 2, 4.0),
            ])
        );
        let (_, a) = parse("1 size-y 0.9 sy 1.1").unwrap();
        assert_eq!(a, Adjust::Set(vec![(Group::Scale, 1, 1.1)]), "last value wins");
    }

    #[test]
    fn parses_resets_and_copies() {
        assert_eq!(parse("5 reset").unwrap(), (5, Adjust::Reset(None)));
        assert_eq!(
            parse("5 reset rotation").unwrap(),
            (5, Adjust::Reset(Some(Group::Rotation)))
        );
        assert_eq!(
            parse("5 match 6").unwrap(),
            (
                5,
                Adjust::Copy {
                    from: 6,
                    mirrored: false
                }
            )
        );
        assert_eq!(
            parse("5 mirror 6").unwrap(),
            (
                5,
                Adjust::Copy {
                    from: 6,
                    mirrored: true
                }
            )
        );
    }

    #[test]
    fn rejects_bad_input() {
        for bad in [
            "",
            "hat",
            "5",
            "5 position.x",
            "5 position.w 1",
            "5 elbow 1",
            "5 position.x 10%",
            "5 position.x nan",
            "5 position.x inf",
            "5 reset elbow",
            "5 match",
            "5 match 5",
            "5 mirror 6 7",
        ] {
            assert!(parse(bad).is_err(), "{bad:?}");
        }
    }

    #[test]
    fn sets_values_inside_bounds() {
        let r = rule(8, 30.0);
        let now = Placement::default();
        let set = |v: Vec<(Group, usize, f64)>| apply(now, &Adjust::Set(v), None, &r);

        let p = set(vec![(Group::Position, 1, 0.123), (Group::Rotation, 0, -12.34)]).unwrap();
        assert_eq!(p.position, [0.0, 0.12, 0.0], "snapped to the step");
        assert_eq!(p.rotation, [-12.3, 0.0, 0.0]);

        assert_eq!(
            set(vec![(Group::Scale, 0, 1.25)]).unwrap_err(),
            "Roblox allows scale X from 0.8 to 1.2."
        );
        assert_eq!(
            set(vec![(Group::Position, 2, -0.3)]).unwrap_err(),
            "Roblox allows position Z from -0.25 to 0.25."
        );
        assert!(set(vec![(Group::Scale, 0, f64::NAN)]).is_err());
        assert!(set(vec![]).is_err());
        assert!(set(vec![(Group::Scale, 0, 1.2)]).is_ok(), "bounds are inclusive");
    }

    #[test]
    fn setting_one_axis_pulls_the_rest_inside_new_bounds() {
        let r = rule(46, 15.0);
        let now = Placement {
            rotation: [25.0, 0.0, 0.0],
            ..Default::default()
        };
        let p = apply(now, &Adjust::Set(vec![(Group::Scale, 1, 1.1)]), None, &r).unwrap();
        assert_eq!(p.rotation[0], 15.0);
        assert_eq!(p.scale, [1.0, 1.1, 1.0]);
    }

    #[test]
    fn resets_and_copies_with_clamping() {
        let back = rule(46, 15.0);
        let now = Placement {
            position: [0.1, 0.1, 0.1],
            rotation: [10.0, 10.0, 10.0],
            scale: [1.1; 3],
        };
        let p = apply(now, &Adjust::Reset(Some(Group::Rotation)), None, &back).unwrap();
        assert_eq!(p.rotation, [0.0; 3]);
        assert_eq!(p.position, now.position);
        assert!(
            apply(now, &Adjust::Reset(None), None, &back)
                .unwrap()
                .is_default()
        );

        let hat = Placement {
            position: [0.2, 0.0, 0.0],
            rotation: [25.0, 5.0, 0.0],
            ..Default::default()
        };
        let copy = |mirrored| apply(now, &Adjust::Copy { from: 2, mirrored }, Some(hat), &back).unwrap();
        assert_eq!(
            copy(false).rotation,
            [15.0, 5.0, 0.0],
            "clamped to the back's bounds"
        );
        assert_eq!(copy(true).position, [-0.2, 0.0, 0.0]);
        assert_eq!(copy(true).rotation, [15.0, -5.0, 0.0]);
        assert!(
            apply(
                now,
                &Adjust::Copy {
                    from: 2,
                    mirrored: false
                },
                None,
                &back
            )
            .is_err()
        );
    }

    #[test]
    fn describes_numbers_plainly() {
        assert_eq!(
            describe(&Placement {
                position: [0.1, -0.0, 0.25],
                ..Default::default()
            }),
            "position 0.1, 0, 0.25; rotation 0, 0, 0; scale 1, 1, 1"
        );
    }
}
