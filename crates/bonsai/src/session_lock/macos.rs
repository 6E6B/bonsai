use super::{Callback, Event};
use block2::RcBlock;
use objc2::{MainThreadMarker, rc::Retained, runtime::ProtocolObject};
use objc2_app_kit::{
    NSWorkspace, NSWorkspaceDidWakeNotification, NSWorkspaceSessionDidResignActiveNotification,
    NSWorkspaceWillSleepNotification,
};
use objc2_foundation::{NSDistributedNotificationCenter, NSNotification, NSObjectProtocol, NSString};
use std::{cell::RefCell, ptr::NonNull};

thread_local! {
    // The observers and copied blocks live until the app's main thread exits.
    static OBSERVERS: RefCell<Vec<Retained<ProtocolObject<dyn NSObjectProtocol>>>> = const { RefCell::new(Vec::new()) };
}

pub fn start(callback: Callback) {
    if MainThreadMarker::new().is_none() {
        callback(Event::Unavailable(
            "The macOS session monitor requires the main thread.",
        ));
        return;
    }
    let distributed = NSDistributedNotificationCenter::defaultCenter();
    let notify = callback.clone();
    let block = RcBlock::new(move |_: NonNull<NSNotification>| fire(&notify));
    // The inherited block API copies a Send-capable block. No Objective-C pointers
    // are captured or dereferenced by it; delivery uses the application's run loop.
    let observer = unsafe {
        distributed.addObserverForName_object_queue_usingBlock(
            Some(&NSString::from_str("com.apple.screenIsLocked")),
            None,
            None,
            &block,
        )
    };
    OBSERVERS.with(|observers| observers.borrow_mut().push(observer));
    let center = NSWorkspace::sharedWorkspace().notificationCenter();
    // Session switching and both sides of sleep are conservative lock boundaries.
    for name in unsafe {
        [
            NSWorkspaceSessionDidResignActiveNotification,
            NSWorkspaceWillSleepNotification,
            NSWorkspaceDidWakeNotification,
        ]
    } {
        let notify = callback.clone();
        let block = RcBlock::new(move |_: NonNull<NSNotification>| fire(&notify));
        let observer =
            unsafe { center.addObserverForName_object_queue_usingBlock(Some(name), None, None, &block) };
        OBSERVERS.with(|observers| observers.borrow_mut().push(observer));
    }
}

fn fire(callback: &Callback) {
    // Never unwind into Objective-C notification delivery.
    let _ = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| callback(Event::Lock)));
}
