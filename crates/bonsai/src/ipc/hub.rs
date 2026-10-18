//! UI-facing hub views. All network and filesystem work runs through Core's worker runtime.
use super::{Id, Invocation};
use bonsai_core::{
    Core,
    commands::CommandId,
    hub::{Publish, Target},
};
use serde::Serialize;
use specta::Type;
use std::sync::Arc;
use tauri::State;

#[derive(Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct HubEntry {
    pub id: String,
    pub kind: String,
    pub owner_id: String,
    pub name: String,
    pub description: String,
    pub tags: Vec<String>,
    pub base: Option<String>,
    pub version: String,
    pub sha256: String,
    pub document: Option<String>,
    pub permissions: Vec<String>,
    pub api_version: Option<u32>,
    pub size: Id,
}
#[derive(Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct HubSubmission {
    pub id: String,
    pub kind: String,
    pub entry_id: String,
    pub version: String,
    pub status: String,
    pub reason: Option<String>,
}
#[derive(Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct HubView {
    pub configured: bool,
    pub loading: bool,
    pub busy: bool,
    pub signing_in: bool,
    pub error: Option<String>,
    pub user_id: Option<String>,
    pub user_name: Option<String>,
    pub items: Vec<HubEntry>,
    pub next_offset: Option<u32>,
    pub submissions: Vec<HubSubmission>,
    pub revoked: Vec<String>,
}
#[tauri::command]
#[specta::specta]
pub fn hub_catalog(core: State<'_, Arc<Core>>, query: String) -> Result<HubView, String> {
    let query = serde_json::from_str(&query).map_err(|_| "Invalid hub query.".to_owned())?;
    let s = core.hub_snapshot(query);
    let items = s
        .catalog
        .page
        .items
        .into_iter()
        .map(|e| HubEntry {
            id: e.id,
            kind: e.kind.path().unwrap_or("unknown").into(),
            owner_id: e.owner_id,
            name: e.name,
            description: e.description,
            tags: e.tags,
            base: e.base,
            version: e.version,
            sha256: e.sha256,
            document: e.document,
            permissions: e.permissions,
            api_version: e.api_version,
            size: Id(e.size as u64),
        })
        .collect();
    let submissions = s
        .submissions
        .into_iter()
        .map(|r| HubSubmission {
            id: r.id,
            kind: r.kind.path().unwrap_or("unknown").into(),
            entry_id: r.entry_id,
            version: r.version,
            status: r.status,
            reason: r.reason,
        })
        .collect();
    Ok(HubView {
        configured: s.configured,
        loading: s.catalog.loading,
        busy: s.busy,
        signing_in: s.signing_in,
        error: s.error.or(s.catalog.error),
        user_id: s.user.as_ref().map(|u| u.id.clone()),
        user_name: s.user.map(|u| u.name),
        items,
        next_offset: s.catalog.page.next_offset,
        submissions,
        revoked: s.revoked,
    })
}

pub fn run(core: &Arc<Core>, id: CommandId, inv: &Invocation) -> Option<Result<(), String>> {
    let input = || {
        inv.arg
            .as_deref()
            .ok_or_else(|| "Choose hub content first.".to_owned())
    };
    let target = || serde_json::from_str::<Target>(input()?).map_err(|_| "Invalid hub content.".to_owned());
    Some(match id {
        CommandId::HubRefresh => {
            core.refresh_hub();
            Ok(())
        }
        CommandId::HubSignIn => core.hub_sign_in(),
        CommandId::HubSignOut => core.hub_sign_out(),
        CommandId::HubInstall => target().and_then(|t| core.hub_install(t)),
        CommandId::HubPublishTheme => input()
            .and_then(|s| serde_json::from_str::<Publish>(s).map_err(|_| "Invalid publication.".to_owned()))
            .and_then(|p| core.hub_publish_theme(p)),
        CommandId::HubPublishPlugin => input()
            .and_then(|s| serde_json::from_str::<Publish>(s).map_err(|_| "Invalid publication.".to_owned()))
            .and_then(|p| core.hub_publish_plugin(p)),
        CommandId::HubDelete => target().and_then(|t| core.hub_delete(t)),
        CommandId::HubUnlist => target().and_then(|t| core.hub_unlist(t)),
        CommandId::HubSubmissions => core.hub_submissions(),
        CommandId::HubRevocations => core.hub_check_revocations(),
        CommandId::HubReport => input()
            .and_then(|s| {
                serde_json::from_str::<serde_json::Value>(s).map_err(|_| "Invalid report.".to_owned())
            })
            .and_then(|v| {
                let t = serde_json::from_value::<Target>(v.clone())
                    .map_err(|_| "Invalid report target.".to_owned())?;
                let reason = v.get("reason").and_then(|s| s.as_str()).unwrap_or_default();
                core.hub_report(t, reason.into())
            }),
        _ => return None,
    })
}

#[derive(Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct HubVersion {
    pub version: String,
    pub sha256: String,
    pub created_at: Id,
    pub permissions: Vec<String>,
}
#[derive(Clone, Serialize, Type)]
#[serde(rename_all = "camelCase")]
pub struct HubHistory {
    pub loading: bool,
    pub error: Option<String>,
    pub versions: Vec<HubVersion>,
}
#[tauri::command]
#[specta::specta]
pub fn hub_history(core: State<'_, Arc<Core>>, target: String) -> Result<HubHistory, String> {
    let target = serde_json::from_str(&target).map_err(|_| "Invalid hub target.".to_owned())?;
    let h = core.hub_history(target);
    Ok(HubHistory {
        loading: h.loading,
        error: h.error,
        versions: h
            .versions
            .into_iter()
            .map(|v| HubVersion {
                version: v.version,
                sha256: v.sha256,
                created_at: Id(v.created_at),
                permissions: v.permissions,
            })
            .collect(),
    })
}
