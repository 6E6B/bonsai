//! Serves HTTPS theme backgrounds through the core cache without webview network access.
use bonsai_core::Core;
use std::sync::Arc;
use tauri::http::{Request, Response, StatusCode, header};
use tauri::{Manager, Runtime, UriSchemeContext, UriSchemeResponder};

pub const SCHEME: &str = "bg";

pub fn handle<R: Runtime>(ctx: UriSchemeContext<'_, R>, req: Request<Vec<u8>>, res: UriSchemeResponder) {
    let url = bonsai_core::backgrounds::url_from_query(req.uri().query().unwrap_or_default());
    let Some(url) = url.filter(|u| bonsai_core::backgrounds::validate_url(u).is_ok()) else {
        return res.respond(status(StatusCode::BAD_REQUEST));
    };
    let Some(core) = ctx.app_handle().try_state::<Arc<Core>>().map(|c| Arc::clone(&c)) else {
        return res.respond(status(StatusCode::SERVICE_UNAVAILABLE));
    };
    tauri::async_runtime::spawn(async move {
        let response = match core.background_image(&url).await {
            Ok((bytes, mime)) => Response::builder()
                .header(header::CONTENT_TYPE, mime)
                .header(header::CACHE_CONTROL, "max-age=86400")
                .body(bytes)
                .unwrap_or_else(|_| status(StatusCode::INTERNAL_SERVER_ERROR)),
            Err(error) => {
                core.notice(error, true);
                status(StatusCode::BAD_GATEWAY)
            }
        };
        res.respond(response);
    });
}

fn status(code: StatusCode) -> Response<Vec<u8>> {
    let mut response = Response::new(Vec::new());
    *response.status_mut() = code;
    response
}
