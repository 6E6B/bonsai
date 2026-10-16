//! `rbxasset://` serves Roblox asset files (models, meshes, textures, animations) to the 3D
//! avatar viewer. `rbx-core` fetches and caches them; the webview never touches the network.
//!
//! URL path: `/<account>.<asset id>`, with optional query `f=<Roblox-AssetFormat>` and
//! `r=<contentRepresentationPriorityList, base64url>`. The UI builds it in
//! `ui/src/lib/avatar3d/host.ts`.

use std::sync::Arc;

use rbx_core::assets::AssetReq;
use rbx_core::{AccountId, Core};
use tauri::http::{Request, Response, StatusCode, header};
use tauri::{Manager, Runtime, UriSchemeContext, UriSchemeResponder};

pub const SCHEME: &str = "rbxasset";

pub fn handle<R: Runtime>(ctx: UriSchemeContext<'_, R>, req: Request<Vec<u8>>, res: UriSchemeResponder) {
    // The sign-in window shows roblox.com; only the app's own window may fetch as an account.
    if ctx.webview_label() != "main" {
        return res.respond(status(StatusCode::FORBIDDEN));
    }
    let Some((account, asset)) = parse(req.uri().path(), req.uri().query().unwrap_or("")) else {
        return res.respond(status(StatusCode::BAD_REQUEST));
    };
    let Some(core) = ctx
        .app_handle()
        .try_state::<Arc<Core>>()
        .map(|c| c.inner().clone())
    else {
        return res.respond(status(StatusCode::SERVICE_UNAVAILABLE));
    };
    let file = core.asset_file(account, asset);
    tauri::async_runtime::spawn(async move {
        let r = match file.await {
            Ok(Some(bytes)) => Response::builder()
                .header(header::CONTENT_TYPE, "application/octet-stream")
                // Assets don't change under the same id and variant.
                .header(header::CACHE_CONTROL, "max-age=3600")
                .header(header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                .body(bytes)
                .unwrap_or_else(|_| status(StatusCode::INTERNAL_SERVER_ERROR)),
            _ => status(StatusCode::NOT_FOUND),
        };
        res.respond(r);
    });
}

fn parse(path: &str, query: &str) -> Option<(AccountId, AssetReq)> {
    let (account, id) = path.trim_start_matches('/').split_once('.')?;
    let mut req = AssetReq {
        id: id.parse().ok()?,
        ..Default::default()
    };
    for pair in query.split('&').filter(|p| !p.is_empty()) {
        match pair.split_once('=')? {
            ("f", v) => req.format = v.to_owned(),
            ("r", v) => req.representations = v.replace('-', "+").replace('_', "/"),
            _ => return None,
        }
    }
    let account = AccountId(account.parse().ok()?);
    req.is_valid().then_some((account, req))
}

fn status(code: StatusCode) -> Response<Vec<u8>> {
    let mut r = Response::new(Vec::new());
    *r.status_mut() = code;
    r
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_paths() {
        let (a, r) = parse("/7.123", "").unwrap();
        assert_eq!(a, AccountId(7));
        assert_eq!(r.id, 123);
        assert_eq!(r.format, "");
        let (_, r) = parse("/0.5", "f=avatar_meshpart_head&r=W3s-_0").unwrap();
        assert_eq!(r.format, "avatar_meshpart_head");
        assert_eq!(r.representations, "W3s+/0");
        assert!(parse("/7", "").is_none());
        assert!(parse("/x.1", "").is_none());
        assert!(parse("/7.1", "f=a%20b").is_none());
        assert!(parse("/7.1", "zzz=1").is_none());
    }
}
