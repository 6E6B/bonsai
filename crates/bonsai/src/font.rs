//! `font://` serves the Google Fonts that `rbx-core` downloaded for themes, so the webview loads
//! them from disk with no network access. The UI registers them with the `FontFace` API in
//! `ui/src/lib/theme/fonts.ts`.
//!
//! URL path: `/<slug>/<weight>.ttf`, as in `AppearanceView::fonts`. Anything else is refused,
//! so a path can't leave the fonts folder.

use std::sync::Arc;

use rbx_core::Core;
use rbx_core::fonts::valid_file;
use tauri::http::{Request, Response, StatusCode, header};
use tauri::{Manager, Runtime, UriSchemeContext, UriSchemeResponder};

pub const SCHEME: &str = "font";

pub fn handle<R: Runtime>(ctx: UriSchemeContext<'_, R>, req: Request<Vec<u8>>, res: UriSchemeResponder) {
    let Some(rel) = rel_path(req.uri().path()) else {
        return res.respond(status(StatusCode::BAD_REQUEST));
    };
    let Some(dir) = ctx
        .app_handle()
        .try_state::<Arc<Core>>()
        .map(|c| c.paths().fonts_dir())
    else {
        return res.respond(status(StatusCode::SERVICE_UNAVAILABLE));
    };
    tauri::async_runtime::spawn_blocking(move || {
        let r = match std::fs::read(dir.join(rel)) {
            Ok(bytes) => Response::builder()
                .header(header::CONTENT_TYPE, "font/ttf")
                // A path always holds the same file until the cache is cleared.
                .header(header::CACHE_CONTROL, "max-age=3600")
                .body(bytes)
                .unwrap_or_else(|_| status(StatusCode::INTERNAL_SERVER_ERROR)),
            Err(_) => status(StatusCode::NOT_FOUND),
        };
        res.respond(r);
    });
}

/// The file path from the URL path, or None when it isn't one of ours. `convertFileSrc`
/// percent-encodes the whole path, so the `/` arrives as `%2F`. A valid path holds no other
/// escapes.
fn rel_path(uri: &str) -> Option<String> {
    let rel = uri
        .trim_start_matches('/')
        .replace("%2F", "/")
        .replace("%2f", "/");
    valid_file(&rel).then_some(rel)
}

fn status(code: StatusCode) -> Response<Vec<u8>> {
    let mut r = Response::new(Vec::new());
    *r.status_mut() = code;
    r
}

#[cfg(test)]
mod tests {
    use super::rel_path;

    #[test]
    fn reads_encoded_and_plain_paths_only() {
        // What `convertFileSrc("lato/400.ttf", "font")` sends.
        assert_eq!(rel_path("/lato%2F400.ttf").as_deref(), Some("lato/400.ttf"));
        assert_eq!(
            rel_path("/inter-tight/700.ttf").as_deref(),
            Some("inter-tight/700.ttf")
        );
        assert_eq!(rel_path("/..%2F..%2Fsecret.ttf"), None);
        assert_eq!(rel_path("/lato%2F400.ttf%00"), None);
        assert_eq!(rel_path("/lato"), None);
    }
}
