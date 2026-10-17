//! `rbxasset://` serves Roblox asset files (models, meshes, textures, animations) to the 3D
//! avatar viewer, and sound files to the item page's audio player. `bonsai-core` fetches and
//! caches them; the webview never touches the network.
//!
//! URL path: `/<account>.<asset id>`, with optional query `f=<Roblox-AssetFormat>` and
//! `r=<contentRepresentationPriorityList, base64url>`. The UI builds it in
//! `ui/src/lib/avatar3d/host.ts`.

use std::sync::Arc;

use bonsai_core::assets::AssetReq;
use bonsai_core::{AccountId, Core};
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
                .header(header::CONTENT_TYPE, content_type(&bytes))
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

/// Audio files get their real type so `<audio>` plays them; everything else is opaque bytes.
fn content_type(bytes: &[u8]) -> &'static str {
    match bytes {
        [b'O', b'g', b'g', b'S', ..] => "audio/ogg",
        [b'I', b'D', b'3', ..] => "audio/mpeg",
        [0xFF, b, ..] if b & 0xE0 == 0xE0 => "audio/mpeg",
        [b'f', b'L', b'a', b'C', ..] => "audio/flac",
        [b'R', b'I', b'F', b'F', _, _, _, _, b'W', b'A', b'V', b'E', ..] => "audio/wav",
        _ => "application/octet-stream",
    }
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

    #[test]
    fn sniffs_audio() {
        assert_eq!(content_type(b"OggS\0\x02"), "audio/ogg");
        assert_eq!(content_type(b"ID3\x04"), "audio/mpeg");
        assert_eq!(content_type(&[0xFF, 0xFB, 0x90]), "audio/mpeg");
        assert_eq!(content_type(b"RIFF\0\0\0\0WAVEfmt "), "audio/wav");
        assert_eq!(content_type(b"<roblox!"), "application/octet-stream");
        assert_eq!(content_type(b""), "application/octet-stream");
    }
}
