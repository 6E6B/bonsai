//! The captcha window behind the challenge dialog: Roblox's Arkose puzzle in a separate window,
//! solved by the user. Nothing here solves it.
//!
//! - Like the sign-in window ([`crate::signin`]) it has no capability, so it can't call IPC,
//!   uses an incognito data store and may only load Roblox and Arkose.
//! - It opens Roblox's sign-in page, whose content policy lets Arkose's script run, and swaps
//!   the page for the puzzle before the page's own scripts start.
//! - When the puzzle is solved, the page navigates to `bonsai-captcha://done?token=…`.
//!   [`on_navigation`] stops that navigation, hands the token to bonsai-core and closes the window.
//!   The token is never logged.
//!
//! Closing the window leaves the challenge dialog open, so the user can reopen the puzzle or
//! cancel.

use std::sync::Arc;

use bonsai_core::{Core, links};
use tauri::webview::NewWindowResponse;
use tauri::{AppHandle, Manager, Url, WebviewUrl, WebviewWindowBuilder};

const LABEL: &str = "captcha";
const SCHEME: &str = "bonsai-captcha";

/// Open the puzzle for challenge `seq`, or bring its window forward.
pub fn open(app: &AppHandle, core: &Arc<Core>, seq: u64) -> Result<(), String> {
    let (key, blob) = core
        .captcha_params(seq)
        .ok_or_else(|| "This check has closed.".to_owned())?;
    let page: Url = links::sign_in_page()
        .parse()
        .map_err(|e| format!("bad captcha page address: {e}"))?;
    if let Some(w) = app.get_webview_window(LABEL) {
        if w.title().is_ok_and(|t| t == title(seq)) {
            return w.set_focus().map_err(|e| e.to_string());
        }
        close(app);
    }
    let script = script(&links::captcha_script(&key), &blob);
    let app = app.clone();
    let core = core.clone();
    // Building a webview from a synchronous command deadlocks on Windows.
    std::thread::spawn(move || {
        if let Err(e) = build(&app, &core, seq, page, script) {
            tracing::warn!("opening the captcha window: {e}");
            core.captcha_failed(seq, "Couldn't open the puzzle.");
        }
    });
    Ok(())
}

/// Close the window if it is open.
pub fn close(app: &AppHandle) {
    if let Some(w) = app.get_webview_window(LABEL) {
        let _ = w.clear_all_browsing_data();
        let _ = w.destroy();
    }
}

/// The window title doubles as the challenge it belongs to.
fn title(seq: u64) -> String {
    format!("Roblox puzzle #{seq}")
}

fn build(app: &AppHandle, core: &Arc<Core>, seq: u64, page: Url, script: String) -> tauri::Result<()> {
    let for_nav = core.clone();
    let for_close = app.clone();
    WebviewWindowBuilder::new(app, LABEL, WebviewUrl::External(page))
        .title(title(seq))
        .inner_size(440.0, 560.0)
        .min_inner_size(360.0, 420.0)
        .incognito(true)
        .initialization_script(script)
        .on_navigation(move |url| {
            if url.scheme() != SCHEME {
                return allowed(url);
            }
            match reply(url) {
                Reply::Done(token) => for_nav.captcha_solved(seq, token),
                Reply::Failed => for_nav.captcha_failed(seq, "The puzzle didn't load. Try again."),
                Reply::Unknown => return false,
            }
            let app = for_close.clone();
            // Destroying the window from inside its own navigation callback isn't safe.
            std::thread::spawn(move || close(&app));
            false
        })
        .on_new_window(|_, _| NewWindowResponse::Deny)
        .build()?;
    Ok(())
}

fn allowed(url: &Url) -> bool {
    match url.scheme() {
        "about" | "blob" | "data" => true,
        "https" => url.host_str().is_some_and(links::sign_in_window_allows),
        _ => false,
    }
}

#[derive(Debug, PartialEq)]
enum Reply {
    Done(String),
    Failed,
    Unknown,
}

fn reply(url: &Url) -> Reply {
    match url.host_str() {
        Some("done") => url
            .query_pairs()
            .find(|(k, _)| k == "token")
            .map(|(_, v)| v.into_owned())
            .filter(|t| !t.is_empty())
            .map_or(Reply::Unknown, Reply::Done),
        Some("error") => Reply::Failed,
        _ => Reply::Unknown,
    }
}

/// Replaces the page with Arkose's puzzle and reports back through [`SCHEME`] navigations.
fn script(src: &str, blob: &str) -> String {
    let src = serde_json::to_string(src).unwrap_or_default();
    let blob = serde_json::to_string(blob).unwrap_or_default();
    format!(
        r##"(() => {{
  if (window.top !== window) return;
  const reply = (what, query) => {{ location.href = "{SCHEME}://" + what + "?" + new URLSearchParams(query); }};
  window.bonsaiCaptcha = (e) => {{
    e.setConfig({{
      selector: "#bonsai-captcha",
      mode: "inline",
      data: {{ blob: {blob} }},
      onReady: () => e.run(),
      onCompleted: (r) => reply("done", {{ token: r.token }}),
      onError: () => reply("error", {{}}),
      onFailed: () => reply("error", {{}}),
    }});
  }};
  const mount = () => {{
    document.documentElement.innerHTML =
      '<head><meta charset="utf-8"><title></title></head>' +
      '<body style="margin:0;display:grid;place-items:center;min-height:100vh;background:#111">' +
      '<div id="bonsai-captcha"></div></body>';
    const s = document.createElement("script");
    s.src = {src};
    s.async = true;
    s.dataset.callback = "bonsaiCaptcha";
    s.onerror = () => reply("error", {{}});
    document.head.appendChild(s);
  }};
  if (document.readyState === "loading") document.addEventListener("DOMContentLoaded", mount, {{ once: true }});
  else mount();
}})();"##
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn replies() {
        let u = |s: &str| s.parse::<Url>().unwrap();
        assert_eq!(
            reply(&u("bonsai-captcha://done?token=a%7Cb")),
            Reply::Done("a|b".into())
        );
        assert_eq!(reply(&u("bonsai-captcha://done?token=")), Reply::Unknown);
        assert_eq!(reply(&u("bonsai-captcha://error?")), Reply::Failed);
        assert_eq!(reply(&u("bonsai-captcha://other")), Reply::Unknown);
    }

    #[test]
    fn script_escapes_its_inputs() {
        let s = script("https://arkoselabs.roblox.com/v2/K/api.js", "a\"</script>b");
        assert!(s.contains(r#"blob: "a\"</script>b""#));
        assert!(s.contains(r#"s.src = "https://arkoselabs.roblox.com/v2/K/api.js";"#));
    }
}
