//! `wikiimg://` serves pictures from wiki articles, which `bonsai-core` fetches from Fandom's
//! image host and caches on disk, so the webview never loads them from Fandom.
//!
//! URL path: an article image's `src` (`WikiImage::src`), a path on that host. `bonsai-core`
//! refuses anything that would leave it, and serves nothing while the wiki is off.

use std::sync::Arc;

use bonsai_core::Core;
use tauri::http::{Request, Response, StatusCode, header};
use tauri::{Manager, Runtime, UriSchemeContext, UriSchemeResponder};

pub const SCHEME: &str = "wikiimg";

pub fn handle<R: Runtime>(ctx: UriSchemeContext<'_, R>, req: Request<Vec<u8>>, res: UriSchemeResponder) {
    let Some(src) = src_of(req.uri().path()) else {
        return res.respond(status(StatusCode::BAD_REQUEST));
    };
    let Some(core) = ctx
        .app_handle()
        .try_state::<Arc<Core>>()
        .map(|c| c.inner().clone())
    else {
        return res.respond(status(StatusCode::SERVICE_UNAVAILABLE));
    };
    let image = core.wiki_image(&src);
    tauri::async_runtime::spawn(async move {
        let r = match image.await {
            Ok(Some(i)) => Response::builder()
                .header(header::CONTENT_TYPE, i.mime)
                // The path names one revision of the file.
                .header(header::CACHE_CONTROL, "max-age=86400")
                .body(i.bytes)
                .unwrap_or_else(|_| status(StatusCode::INTERNAL_SERVER_ERROR)),
            _ => status(StatusCode::NOT_FOUND),
        };
        res.respond(r);
    });
}

/// `convertFileSrc` percent-encodes the whole `src`, its own escapes included, so one decode
/// gives it back.
fn src_of(path: &str) -> Option<String> {
    let raw = path.trim_start_matches('/');
    let b = raw.as_bytes();
    let mut out = Vec::with_capacity(b.len());
    let mut i = 0;
    while i < b.len() {
        if b[i] == b'%' {
            let v = raw
                .get(i + 1..i + 3)
                .and_then(|h| u8::from_str_radix(h, 16).ok())?;
            out.push(v);
            i += 3;
        } else {
            out.push(b[i]);
            i += 1;
        }
    }
    let src = String::from_utf8(out).ok()?;
    (!src.is_empty()).then_some(src)
}

fn status(code: StatusCode) -> Response<Vec<u8>> {
    let mut r = Response::new(Vec::new());
    *r.status_mut() = code;
    r
}

#[cfg(test)]
mod tests {
    use super::src_of;

    #[test]
    fn decodes_once() {
        // What `convertFileSrc("roblox/images/a/a9/B%2C_2026.png/revision/latest?cb=1", "wikiimg")` sends.
        assert_eq!(
            src_of("/roblox%2Fimages%2Fa%2Fa9%2FB%252C_2026.png%2Frevision%2Flatest%3Fcb%3D1").as_deref(),
            Some("roblox/images/a/a9/B%2C_2026.png/revision/latest?cb=1")
        );
        assert_eq!(src_of("/"), None);
        assert_eq!(src_of("/bad%zz"), None);
    }
}
