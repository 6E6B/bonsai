//! Native destinations and clipboard integration for the shared rendered-image commands.

use super::*;
use bonsai_core::thumbs::{ThumbKey, ThumbKind};
use tauri_plugin_clipboard_manager::ClipboardExt;

#[derive(Deserialize)]
struct ImageTarget {
    key: String,
    #[serde(default)]
    name: String,
}

fn target(core: &Core, inv: &Invocation) -> Option<(ThumbKey, String)> {
    if let Some(arg) = inv.arg.as_deref() {
        if let Ok(target) = serde_json::from_str::<ImageTarget>(arg) {
            return Some((ThumbKey::parse(&target.key)?, target.name));
        }
        return Some((ThumbKey::parse(arg)?, String::new()));
    }
    let (kind, id) = inv.context.as_deref()?.split_once(':')?;
    let id = id.parse().ok()?;
    let kind = match kind {
        "user" => ThumbKind::Headshot,
        "item" => ThumbKind::Asset,
        "bundle" => ThumbKind::Bundle,
        "group" => ThumbKind::GroupIcon,
        "game" => {
            return Some((
                ThumbKey::large(ThumbKind::GameIcon, universe_of(core, id)?),
                String::new(),
            ));
        }
        _ => return None,
    };
    Some((ThumbKey::large(kind, id), String::new()))
}

pub(super) fn run(app: &AppHandle, core: &Arc<Core>, inv: &Invocation, id: CommandId) -> Result<(), String> {
    let (key, name) = target(core, inv)
        .ok_or_else(|| "Select an image, or open an item, profile or experience first.".to_owned())?;
    let core = core.clone();
    let app = app.clone();
    // Resolving, decoding, clipboard writes and dialogs never run on the IPC/main thread.
    tauri::async_runtime::spawn(async move {
        let image = match core.thumbnail_image(key).await {
            Ok(image) => image,
            Err(e) => return core.notice(e, true),
        };
        let worker_core = core.clone();
        let result = tauri::async_runtime::spawn_blocking(move || -> Result<(), String> {
            if core.accounts_locked() {
                return Err("Unlock Bonsai to access images.".into());
            }
            match id {
                CommandId::CopyImageUrl => {
                    let url = image
                        .url
                        .ok_or_else(|| "The cached image has no public URL.".to_owned())?;
                    app.clipboard()
                        .write_text(url)
                        .map_err(|e| format!("Could not copy image URL: {e}"))?;
                    core.notice("Copied image URL.", false);
                }
                CommandId::CopyImage => {
                    let rgba = image.image.rgba()?;
                    let image =
                        tauri::image::Image::new_owned(rgba.as_raw().clone(), rgba.width(), rgba.height());
                    app.clipboard()
                        .write_image(&image)
                        .map_err(|e| format!("Could not copy image: {e}"))?;
                    core.notice("Copied image.", false);
                }
                CommandId::SaveImage => {
                    let ext = image.image.extension();
                    let stem = bonsai_core::template::file_stem(&name, key.id, false);
                    let mut dialog = rfd::FileDialog::new()
                        .set_title("Save rendered thumbnail")
                        .set_file_name(format!("{stem} thumbnail.{ext}"))
                        .add_filter(format!("{} image", ext.to_uppercase()), &[ext]);
                    if let Some(window) = app.get_webview_window("main") {
                        dialog = dialog.set_parent(&window);
                    }
                    let Some(path) = dialog.save_file() else {
                        return Ok(());
                    };
                    if core.accounts_locked() {
                        return Err("Unlock Bonsai to access images.".into());
                    }
                    // Keep the exact destination chosen in the native dialog, including its
                    // overwrite confirmation. The default name/filter match the actual bytes.
                    image.image.save(&path)?;
                    core.notice(format!("Saved thumbnail to {}", path.display()), false);
                }
                _ => unreachable!("image command"),
            }
            Ok(())
        })
        .await;
        match result {
            Ok(Ok(())) => {}
            Ok(Err(e)) => worker_core.notice(e, true),
            Err(e) => worker_core.notice(format!("Could not export image: {e}"), true),
        }
    });
    Ok(())
}
