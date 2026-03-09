use daemon::fetch_hotkeys;
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager};
use std::time::Duration;

fn main() {
    let all_keys = fetch_hotkeys::load_keys();
    let manager = GlobalHotKeyManager::new().unwrap();

    for each_key in all_keys {
        if let Ok(_event) = manager.register(each_key) {
            println!("Key Logger is active");
        } else {
            println!("Failed in setting up Hotkey");
        }
    }

    let receiver = GlobalHotKeyEvent::receiver();

    loop {
        if let Ok(event) = receiver.try_recv() {
            println!("tray event: {event:?}");
        }
        std::thread::sleep(Duration::from_millis(100));
    }
}
