//! `thumb://` serves thumbnails from `bonsai-core`'s cache so the webview loads and caches them
//! like any image, with no base64 over IPC and no network access from the webview.
//!
//! URL path: `/<kind>.<id>.<s|l>`, like `/headshot.1.s`. The UI builds it in
//! `ui/src/lib/thumb.ts`. A missing image is requested and the response waits for it.

use std::sync::Arc;
use std::time::Duration;

use bonsai_core::Core;
use bonsai_core::thumbs::{ThumbKey, ThumbState};
use tauri::http::{Request, Response, StatusCode, header};
use tauri::{Manager, Runtime, UriSchemeContext, UriSchemeResponder};

pub const SCHEME: &str = "thumb";

const POLL: Duration = Duration::from_millis(50);
const WAIT: Duration = Duration::from_secs(15);

pub fn handle<R: Runtime>(ctx: UriSchemeContext<'_, R>, req: Request<Vec<u8>>, res: UriSchemeResponder) {
    if let Some(core) = ctx.app_handle().try_state::<Arc<Core>>()
        && core.accounts_locked()
    {
        return res.respond(
            Response::builder()
                .status(403)
                .body(Vec::new())
                .unwrap_or_default(),
        );
    }

    let Some(key) = parse(req.uri().path()) else {
        return res.respond(status(StatusCode::BAD_REQUEST));
    };
    let Some(core) = ctx
        .app_handle()
        .try_state::<Arc<Core>>()
        .map(|c| c.inner().clone())
    else {
        return res.respond(status(StatusCode::SERVICE_UNAVAILABLE));
    };
    core.want_thumbs(&[key]);
    tauri::async_runtime::spawn(async move {
        let mut waited = Duration::ZERO;
        loop {
            let image = match core.store().thumbs.get(&key) {
                Some(ThumbState::Ready(i)) => Some(i.clone()),
                Some(ThumbState::Unavailable) => {
                    return res.respond(status(StatusCode::NOT_FOUND));
                }
                Some(ThumbState::Loading) => None,
                // Dropped from memory in the meantime; it loads again from disk.
                None => {
                    core.want_thumbs(&[key]);
                    None
                }
            };
            if let Some(image) = image {
                return res.respond(serve(&image));
            }
            if waited >= WAIT {
                return res.respond(status(StatusCode::GATEWAY_TIMEOUT));
            }
            tokio::time::sleep(POLL).await;
            waited += POLL;
        }
    });
}

fn parse(path: &str) -> Option<ThumbKey> {
    ThumbKey::parse(path)
}

fn serve(image: &bonsai_core::thumbs::Image) -> Response<Vec<u8>> {
    Response::builder()
        .header(header::CONTENT_TYPE, image.mime)
        // A key always maps to the same image within a run.
        .header(header::CACHE_CONTROL, "max-age=3600")
        .body(image.bytes.clone())
        .unwrap_or_else(|_| status(StatusCode::INTERNAL_SERVER_ERROR))
}

fn status(code: StatusCode) -> Response<Vec<u8>> {
    let mut r = Response::new(Vec::new());
    *r.status_mut() = code;
    r
}

#[cfg(test)]
mod tests {
    use super::*;
    use bonsai_core::thumbs::ThumbKind;

    #[test]
    fn parses_paths() {
        assert_eq!(
            parse("/headshot.42.s"),
            Some(ThumbKey {
                kind: ThumbKind::Headshot,
                id: 42,
                large: false
            })
        );
        assert_eq!(
            parse("/game_icon.7.l"),
            Some(ThumbKey {
                kind: ThumbKind::GameIcon,
                id: 7,
                large: true
            })
        );
        assert_eq!(parse("/headshot.42"), None);
        assert_eq!(parse("/headshot.x.s"), None);
        assert_eq!(parse("/nope.1.s"), None);
        assert_eq!(parse("/headshot.1.s.extra"), None);
        assert_eq!(parse("/headshot.0.s"), None);
        assert_eq!(parse("/headshot.9007199254740992.s"), None);
    }
}
