use super::{Callback, Event};
use glib::variant::ToVariant;
use gtk::{gio, glib};
use std::collections::HashMap;

const LOGIN: &str = "org.freedesktop.login1";
const SESSION: &str = "org.freedesktop.login1.Session";

pub fn start(callback: Callback) {
    std::thread::spawn(move || {
        let context = glib::MainContext::new();
        let result = context.with_thread_default(|| watch(&callback, &context));
        if !matches!(result, Ok(Ok(()))) {
            callback(Event::Unavailable(
                "Could not monitor Linux session notifications.",
            ));
        }
    });
}

fn watch(callback: &Callback, context: &glib::MainContext) -> Result<(), glib::Error> {
    // Keep connections alive for the lifetime of this GLib event loop. GIO resolves
    // well-known sender names, so another user's session cannot impersonate login1.
    let session = gio::bus_get_sync(gio::BusType::Session, gio::Cancellable::NONE)?;
    let mut desktop_found = false;
    for (name, path) in [
        ("org.freedesktop.ScreenSaver", "/ScreenSaver"),
        ("org.gnome.ScreenSaver", "/org/gnome/ScreenSaver"),
    ] {
        desktop_found |= has_owner(&session, name);
        let notify = callback.clone();
        session.signal_subscribe(
            Some(name),
            Some(name),
            Some("ActiveChanged"),
            Some(path),
            None,
            gio::DBusSignalFlags::NONE,
            move |_, _, _, _, _, parameters| {
                if screen_locked(parameters) {
                    notify(Event::Lock);
                }
            },
        );
    }
    // Some ScreenSaver implementations use the freedesktop object path instead.
    let notify = callback.clone();
    session.signal_subscribe(
        Some("org.freedesktop.ScreenSaver"),
        Some("org.freedesktop.ScreenSaver"),
        Some("ActiveChanged"),
        Some("/org/freedesktop/ScreenSaver"),
        None,
        gio::DBusSignalFlags::NONE,
        move |_, _, _, _, _, parameters| {
            if screen_locked(parameters) {
                notify(Event::Lock);
            }
        },
    );
    on_closed(&session, callback);

    let system = gio::bus_get_sync(gio::BusType::System, gio::Cancellable::NONE).ok();
    let mut own_session = false;
    if let Some(system) = &system {
        let notify = callback.clone();
        system.signal_subscribe(
            Some(LOGIN),
            Some("org.freedesktop.login1.Manager"),
            Some("PrepareForSleep"),
            Some("/org/freedesktop/login1"),
            None,
            gio::DBusSignalFlags::NONE,
            // Both suspend and resume lock. Resume covers a signal delivered late.
            move |_, _, _, _, _, parameters| {
                if parameters.get::<(bool,)>().is_some() {
                    notify(Event::Lock);
                }
            },
        );
        let reply = system.call_sync(
            Some(LOGIN),
            "/org/freedesktop/login1",
            "org.freedesktop.login1.Manager",
            "GetSessionByPID",
            Some(&(std::process::id(),).to_variant()),
            None,
            gio::DBusCallFlags::NONE,
            3000,
            gio::Cancellable::NONE,
        );
        if let Ok(reply) = reply
            && let Some((path,)) = reply.get::<(glib::variant::ObjectPath,)>()
        {
            let path = path.as_str();
            own_session = true;
            let notify = callback.clone();
            system.signal_subscribe(
                Some(LOGIN),
                Some(SESSION),
                Some("Lock"),
                Some(path),
                None,
                gio::DBusSignalFlags::NONE,
                move |_, _, _, _, _, _| notify(Event::Lock),
            );
            let notify = callback.clone();
            system.signal_subscribe(
                Some(LOGIN),
                Some("org.freedesktop.DBus.Properties"),
                Some("PropertiesChanged"),
                Some(path),
                Some(SESSION),
                gio::DBusSignalFlags::NONE,
                move |_, _, _, _, _, parameters| {
                    if session_locked(parameters) {
                        notify(Event::Lock);
                    }
                },
            );
        }
        on_closed(system, callback);
    }
    if !desktop_found && !own_session {
        callback(Event::Unavailable(
            "No supported desktop lock service was detected.",
        ));
    } else if system.is_none() {
        callback(Event::Unavailable("System sleep notifications are unavailable."));
    }
    glib::MainLoop::new(Some(context), false).run();
    Ok(())
}

fn on_closed(connection: &gio::DBusConnection, callback: &Callback) {
    let notify = callback.clone();
    connection.connect_closed(move |_, _, _| {
        notify(Event::Unavailable(
            "The system lock notification connection closed.",
        ));
        notify(Event::Lock);
    });
}

fn has_owner(connection: &gio::DBusConnection, name: &str) -> bool {
    connection
        .call_sync(
            Some("org.freedesktop.DBus"),
            "/org/freedesktop/DBus",
            "org.freedesktop.DBus",
            "NameHasOwner",
            Some(&(name,).to_variant()),
            None,
            gio::DBusCallFlags::NONE,
            3000,
            gio::Cancellable::NONE,
        )
        .ok()
        .and_then(|r| r.get::<(bool,)>())
        .is_some_and(|(yes,)| yes)
}

fn screen_locked(parameters: &glib::Variant) -> bool {
    parameters.get::<(bool,)>() == Some((true,))
}
fn session_locked(parameters: &glib::Variant) -> bool {
    let Some((interface, changed, _)) =
        parameters.get::<(String, HashMap<String, glib::Variant>, Vec<String>)>()
    else {
        return false;
    };
    interface == SESSION
        && (changed.get("LockedHint").and_then(|v| v.get::<bool>()) == Some(true)
            || changed.get("Active").and_then(|v| v.get::<bool>()) == Some(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn desktop_unlock_and_malformed_signals_never_unlock_the_vault() {
        assert!(screen_locked(&(true,).to_variant()));
        assert!(!screen_locked(&(false,).to_variant()));
        assert!(!screen_locked(&("true",).to_variant()));
        for (property, value, expected) in [
            ("LockedHint", true, true),
            ("LockedHint", false, false),
            ("Active", false, true),
            ("Active", true, false),
            ("IdleHint", true, false),
        ] {
            let changed = HashMap::from([(property.to_owned(), value.to_variant())]);
            assert_eq!(
                session_locked(&(SESSION, &changed, Vec::<String>::new()).to_variant()),
                expected
            );
            assert!(!session_locked(
                &("another.interface", changed, Vec::<String>::new()).to_variant()
            ));
        }
    }
}
