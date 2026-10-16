//! The sign-in window behind `account.sign_in_browser`: Roblox's own sign-in page in a
//! separate window, so the user signs in (captcha and two-step included) on roblox.com itself.
//!
//! - It has no capability in `capabilities/`, so Tauri rejects any IPC from it, and
//!   [`crate::run`] also drops invokes from any webview but the main one.
//! - It uses an incognito data store: nothing is shared with the main webview or kept after
//!   the window closes.
//! - It may only load Roblox and the captcha provider ([`links::sign_in_window_allows`]).
//!   Roblox links that open a new window go to the system browser.
//! - After each page load, Rust reads `.ROBLOSECURITY` from the window's cookie store and hands
//!   it to the same check-and-confirm step as a pasted cookie. The cookie never reaches the UI.
//!
//! The session it creates is bound to a key that roblox.com keeps in the window's IndexedDB,
//! which goes away with the window. That's accepted: Roblox doesn't enforce bound sessions
//! now. Don't seed or tamper with that IndexedDB. If enforcement returns, mark these accounts
//! "sign in again" and point users to quick sign-in.

use std::sync::Arc;

use bonsai_core::{Core, Secret, links};
use tauri::webview::{NewWindowResponse, PageLoadEvent};
use tauri::{AppHandle, Manager, Url, WebviewUrl, WebviewWindow, WebviewWindowBuilder, WindowEvent};

const LABEL: &str = "signin";
const COOKIE: &str = ".ROBLOSECURITY";

/// Open the window, or bring it forward if it is already open.
pub fn open(app: &AppHandle, core: &Arc<Core>) -> Result<(), String> {
    let page: Url = links::sign_in_page()
        .parse()
        .map_err(|e| format!("bad sign-in page address: {e}"))?;
    core.browser_sign_in_started();
    if let Some(w) = app.get_webview_window(LABEL) {
        return w.set_focus().map_err(|e| e.to_string());
    }
    let app = app.clone();
    let core = core.clone();
    // Building a webview from a synchronous command deadlocks on Windows, so build it off
    // the main thread.
    std::thread::spawn(move || {
        if let Err(e) = build(&app, &core, page) {
            tracing::warn!("opening the sign-in window: {e}");
            core.browser_sign_in_closed();
        }
    });
    Ok(())
}

/// Close the window if it is open. Its data store goes with it.
pub fn close(app: &AppHandle) {
    if let Some(w) = app.get_webview_window(LABEL) {
        let _ = w.clear_all_browsing_data();
        let _ = w.destroy();
    }
}

fn build(app: &AppHandle, core: &Arc<Core>, page: Url) -> tauri::Result<()> {
    let for_links = core.clone();
    let for_load = core.clone();
    let w = WebviewWindowBuilder::new(app, LABEL, WebviewUrl::External(page))
        .title("Sign in to Roblox")
        .inner_size(480.0, 720.0)
        .min_inner_size(360.0, 480.0)
        .incognito(true)
        .on_navigation(allowed)
        .on_new_window(move |url, _| {
            if matches!(url.scheme(), "http" | "https")
                && let Err(e) = for_links.open_web(url.as_str())
            {
                tracing::debug!("sign-in window link not opened: {e}");
            }
            NewWindowResponse::Deny
        })
        .on_page_load(move |w, p| {
            if p.event() == PageLoadEvent::Finished {
                let core = for_load.clone();
                // Reading cookies waits on the webview's event loop; never do it on that
                // thread (it deadlocks on Windows).
                std::thread::spawn(move || take_session(&w, &core));
            }
        })
        .build()?;
    let core = core.clone();
    w.on_window_event(move |e| {
        if let WindowEvent::Destroyed = e {
            core.browser_sign_in_closed();
        }
    });
    Ok(())
}

/// Pages and frames the window may load. Blank and inline frames are fine; other sites are not.
fn allowed(url: &Url) -> bool {
    match url.scheme() {
        "about" | "blob" | "data" => true,
        "https" => url.host_str().is_some_and(links::sign_in_window_allows),
        _ => {
            tracing::debug!("sign-in window blocked a {} link", url.scheme());
            false
        }
    }
}

/// Once the page has a session, hand it to bonsai-core and close the window.
fn take_session(w: &WebviewWindow, core: &Core) {
    let Ok(site) = links::sign_in_page().parse::<Url>() else {
        return;
    };
    let cookies = match w.cookies_for_url(site) {
        Ok(c) => c,
        Err(e) => {
            tracing::warn!("reading the sign-in window's cookies: {e}");
            return;
        }
    };
    let Some(session) = cookies
        .iter()
        .find(|c| c.name() == COOKIE && !c.value().is_empty())
        .map(|c| Secret::new(c.value()))
    else {
        return;
    };
    drop(cookies);
    if core.submit_session(session) {
        let _ = w.clear_all_browsing_data();
        let _ = w.destroy();
    }
}
