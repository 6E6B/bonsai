//! `bg://` serves the app background picture that `bonsai-core` copied into
//! `Paths::backgrounds_dir()`, so the webview shows it with no file or network access.
//!
//! URL path: `/<16 hex>.<png|jpg|gif|webp>`, as in `BackgroundView::file`. Anything else is
//! refused, so a path can't leave the folder.

use std::sync::Arc;

use bonsai_core::Core;
use bonsai_core::backgrounds::{content_type, valid_file};
use tauri::http::{Request, Response, StatusCode, header};
use tauri::{Manager, Runtime, UriSchemeContext, UriSchemeResponder};

pub const SCHEME: &str = "bg";

pub fn handle<R: Runtime>(ctx: UriSchemeContext<'_, R>, req: Request<Vec<u8>>, res: UriSchemeResponder) {
    let name = req.uri().path().trim_start_matches('/').to_owned();
    if !valid_file(&name) {
        return res.respond(status(StatusCode::BAD_REQUEST));
    }
    let Some(dir) = ctx
        .app_handle()
        .try_state::<Arc<Core>>()
        .map(|c| c.paths().backgrounds_dir())
    else {
        return res.respond(status(StatusCode::SERVICE_UNAVAILABLE));
    };
    tauri::async_runtime::spawn_blocking(move || {
        let r = match std::fs::read(dir.join(&name)) {
            Ok(bytes) => Response::builder()
                .header(header::CONTENT_TYPE, content_type(&name))
                // The name is a hash of the contents.
                .header(header::CACHE_CONTROL, "max-age=31536000, immutable")
                .body(bytes)
                .unwrap_or_else(|_| status(StatusCode::INTERNAL_SERVER_ERROR)),
            Err(_) => status(StatusCode::NOT_FOUND),
        };
        res.respond(r);
    });
}

fn status(code: StatusCode) -> Response<Vec<u8>> {
    let mut r = Response::new(Vec::new());
    *r.status_mut() = code;
    r
}
