use arboard::Clipboard;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant};
use tauri::Emitter;

// macOS-specific imports for core-graphics
#[cfg(target_os = "macos")]
use core_graphics::event::{CGEvent, CGEventFlags, CGEventTapLocation};
#[cfg(target_os = "macos")]
use core_graphics::event_source::{CGEventSource, CGEventSourceStateID};

// Import rdev for Windows/Linux
#[cfg(not(target_os = "macos"))]
use rdev::{simulate, EventType, Key as RdevKey, SimulateError};

// Import Enigo for Linux fallback
#[cfg(target_os = "linux")]
use enigo::{
    Direction::{Click, Press, Release},
    Enigo, Key, Keyboard, Settings,
};

// Global flag to prevent concurrent text insertions
static IS_INSERTING: AtomicBool = AtomicBool::new(false);

// Guard to ensure IS_INSERTING is reset even if the function returns early
struct InsertionGuard;

impl Drop for InsertionGuard {
    fn drop(&mut self) {
        IS_INSERTING.store(false, Ordering::SeqCst);
    }
}

/// Main entry point for inserting transcribed text at cursor position
#[tauri::command]
pub async fn insert_text(app: tauri::AppHandle, text: String) -> Result<(), String> {
    // Check if already inserting text
    if IS_INSERTING.swap(true, Ordering::SeqCst) {
        log::warn!("Text insertion already in progress, skipping duplicate request");
        return Err("Text insertion already in progress".to_string());
    }

    // Ensure we reset the flag on exit
    let _guard = InsertionGuard;

    log::info!("=== TEXT INSERTION START ===");
    log::info!("Text length: {} chars", text.len());

    // Check accessibility permission on macOS
    #[cfg(target_os = "macos")]
    let has_accessibility_permission = {
        use crate::commands::permissions::check_accessibility_permission;
        check_accessibility_permission().await?
    };

    #[cfg(not(target_os = "macos"))]
    let has_accessibility_permission = true;

    // Move to a blocking task since clipboard and keyboard operations are synchronous
    let app_clone = app.clone();
    tokio::task::spawn_blocking(move || {
        insert_text_impl(text, has_accessibility_permission, Some(app_clone))
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

/// Copy plain text to the system clipboard without attempting to paste
#[tauri::command]
pub async fn copy_text_to_clipboard(text: String) -> Result<(), String> {
    tokio::task::spawn_blocking(move || {
        let mut clipboard =
            Clipboard::new().map_err(|e| format!("Failed to initialize clipboard: {}", e))?;
        clipboard
            .set_text(&text)
            .map_err(|e| format!("Failed to set clipboard: {}", e))?;
        Ok(())
    })
    .await
    .map_err(|e| format!("Task failed: {}", e))?
}

/// Implementation of text insertion with multiple fallback strategies
fn insert_text_impl(
    text: String,
    has_accessibility_permission: bool,
    app_handle: Option<tauri::AppHandle>,
) -> Result<(), String> {
    let start_time = Instant::now();

    // Step 1: Initialize clipboard and set text
    let mut clipboard =
        Clipboard::new().map_err(|e| format!("Failed to initialize clipboard: {}", e))?;

    clipboard
        .set_text(&text)
        .map_err(|e| format!("Failed to set clipboard: {}", e))?;

    log::debug!("Clipboard set with {} chars", text.len());

    // Skip clipboard verification in release builds - it's an extra read + compare
    // that adds latency without providing value in production
    #[cfg(debug_assertions)]
    {
        match clipboard.get_text() {
            Ok(content) => {
                if content == text {
                    log::debug!("Clipboard content verified");
                } else {
                    log::warn!("Clipboard content mismatch!");
                }
            }
            Err(e) => {
                log::warn!("Could not verify clipboard: {}", e);
            }
        }
    }

    // Step 2: Check if we have permission to paste
    if !has_accessibility_permission {
        log::warn!("No accessibility permission - text copied to clipboard but cannot auto-paste");
        return Err("No accessibility permission - text copied to clipboard. Please paste manually or grant accessibility permission.".to_string());
    }

    // Step 3: Wait for focus to leave Verity (macOS only)
    #[cfg(target_os = "macos")]
    {
        if let Err(e) = wait_for_focus_restoration(500) {
            log::warn!("Focus restoration warning: {}", e);
            // Continue anyway - the target app might still receive the paste
        }
    }

    // Step 4: Release any stuck modifier keys before pasting
    #[cfg(target_os = "macos")]
    {
        if let Err(e) = release_all_modifier_keys() {
            log::warn!("Failed to release modifier keys: {}", e);
        }
    }

    // Step 5: Small delay for system stability
    thread::sleep(Duration::from_millis(50));

    // Step 6: Try paste strategies with retries
    let mut last_error: Option<String> = None;

    for attempt in 1..=3 {
        log::info!("Paste attempt {}/3", attempt);

        // Strategy 1: Platform-specific primary method
        #[cfg(target_os = "macos")]
        {
            match paste_with_core_graphics() {
                Ok(_) => {
                    log::info!(
                        "=== TEXT INSERTION SUCCESS (core-graphics) in {}ms ===",
                        start_time.elapsed().as_millis()
                    );
                    return Ok(());
                }
                Err(e) => {
                    log::warn!("core-graphics paste failed: {}", e);
                    last_error = Some(e);
                }
            }
        }

        #[cfg(not(target_os = "macos"))]
        {
            match paste_with_rdev() {
                Ok(_) => {
                    log::info!(
                        "=== TEXT INSERTION SUCCESS (rdev) in {}ms ===",
                        start_time.elapsed().as_millis()
                    );
                    return Ok(());
                }
                Err(e) => {
                    log::warn!("rdev paste failed: {}", e);
                    last_error = Some(e);
                }
            }
        }

        // Strategy 2: AppleScript fallback (macOS only)
        #[cfg(target_os = "macos")]
        {
            match paste_with_applescript() {
                Ok(_) => {
                    log::info!(
                        "=== TEXT INSERTION SUCCESS (AppleScript) in {}ms ===",
                        start_time.elapsed().as_millis()
                    );
                    return Ok(());
                }
                Err(e) => {
                    log::warn!("AppleScript paste failed: {}", e);
                    last_error = Some(e);
                }
            }
        }

        // Increasing delay between retries
        if attempt < 3 {
            let delay = 50 * attempt as u64;
            log::debug!("Retry delay: {}ms", delay);
            thread::sleep(Duration::from_millis(delay));
        }
    }

    // Strategy 3: Type characters as last resort (macOS only)
    #[cfg(target_os = "macos")]
    {
        log::info!("Trying character-by-character typing as last resort");
        match type_with_applescript(&text) {
            Ok(_) => {
                log::info!(
                    "=== TEXT INSERTION SUCCESS (type characters) in {}ms ===",
                    start_time.elapsed().as_millis()
                );
                return Ok(());
            }
            Err(e) => {
                log::error!("Character typing failed: {}", e);
                last_error = Some(e);
            }
        }
    }

    // All strategies failed - notify user
    let error_msg = last_error.unwrap_or_else(|| "Unknown paste error".to_string());
    log::error!("All paste strategies failed: {}", error_msg);

    if let Some(app) = app_handle {
        let _ = app.emit("paste-error", "Paste failed - text copied to clipboard");
    }

    // Don't return error - text is still in clipboard for manual paste
    Ok(())
}

// ============================================================================
// macOS-specific implementations
// ============================================================================

#[cfg(target_os = "macos")]
fn get_frontmost_app() -> Result<String, String> {
    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg("tell application \"System Events\" to get name of first process whose frontmost is true")
        .output()
        .map_err(|e| format!("Failed to run osascript: {}", e))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("osascript failed: {}", stderr));
    }

    String::from_utf8(output.stdout)
        .map(|s| s.trim().to_string())
        .map_err(|e| format!("Invalid UTF-8: {}", e))
}

#[cfg(target_os = "macos")]
fn wait_for_focus_restoration(max_wait_ms: u64) -> Result<(), String> {
    let start = Instant::now();
    let check_interval = Duration::from_millis(10);

    log::debug!("Waiting for focus to leave Verity (max {}ms)", max_wait_ms);

    while start.elapsed().as_millis() < max_wait_ms as u128 {
        match get_frontmost_app() {
            Ok(app) => {
                if app != "Verity" && app != "verity" {
                    log::debug!(
                        "Focus restored to '{}' after {}ms",
                        app,
                        start.elapsed().as_millis()
                    );
                    return Ok(());
                }
            }
            Err(e) => {
                log::debug!("Could not get frontmost app: {}", e);
                // Continue checking - might be temporary
            }
        }
        thread::sleep(check_interval);
    }

    log::warn!(
        "Timeout waiting for focus restoration after {}ms",
        max_wait_ms
    );
    Err("Timeout waiting for focus restoration".to_string())
}

#[cfg(target_os = "macos")]
fn release_all_modifier_keys() -> Result<(), String> {
    log::debug!("Releasing all modifier keys");

    // Virtual keycodes for modifier keys on macOS
    const KEY_COMMAND_LEFT: u16 = 55;
    const KEY_COMMAND_RIGHT: u16 = 54;
    const KEY_SHIFT_LEFT: u16 = 56;
    const KEY_SHIFT_RIGHT: u16 = 60;
    const KEY_OPTION_LEFT: u16 = 58;
    const KEY_OPTION_RIGHT: u16 = 61;
    const KEY_CONTROL_LEFT: u16 = 59;
    const KEY_CONTROL_RIGHT: u16 = 62;

    let modifiers = [
        KEY_COMMAND_LEFT,
        KEY_COMMAND_RIGHT,
        KEY_SHIFT_LEFT,
        KEY_SHIFT_RIGHT,
        KEY_OPTION_LEFT,
        KEY_OPTION_RIGHT,
        KEY_CONTROL_LEFT,
        KEY_CONTROL_RIGHT,
    ];

    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|_| "Failed to create event source")?;

    for keycode in modifiers {
        // Send key up event for each modifier
        if let Ok(event) = CGEvent::new_keyboard_event(source.clone(), keycode, false) {
            event.post(CGEventTapLocation::HID);
        }
    }

    // Small delay to let the system process the releases
    thread::sleep(Duration::from_millis(10));

    Ok(())
}

#[cfg(target_os = "macos")]
fn paste_with_core_graphics() -> Result<(), String> {
    log::debug!("Attempting paste with core-graphics CGEventPost");

    // Virtual keycode for 'V' on macOS
    const KEY_V: u16 = 9;

    let source = CGEventSource::new(CGEventSourceStateID::HIDSystemState)
        .map_err(|_| "Failed to create event source")?;

    // Create key down event with Command modifier
    let key_down = CGEvent::new_keyboard_event(source.clone(), KEY_V, true)
        .map_err(|_| "Failed to create key down event")?;
    key_down.set_flags(CGEventFlags::CGEventFlagCommand);
    key_down.post(CGEventTapLocation::HID);

    // Small delay between key down and key up
    thread::sleep(Duration::from_millis(30));

    // Create key up event with Command modifier
    let key_up = CGEvent::new_keyboard_event(source, KEY_V, false)
        .map_err(|_| "Failed to create key up event")?;
    key_up.set_flags(CGEventFlags::CGEventFlagCommand);
    key_up.post(CGEventTapLocation::HID);

    // Wait for paste to complete
    thread::sleep(Duration::from_millis(50));

    log::debug!("core-graphics paste completed");
    Ok(())
}

#[cfg(target_os = "macos")]
fn paste_with_applescript() -> Result<(), String> {
    log::debug!("Attempting paste with AppleScript");

    let script = r#"
        tell application "System Events"
            keystroke "v" using {command down}
        end tell
    "#;

    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(script)
        .output()
        .map_err(|e| format!("Failed to run osascript: {}", e))?;

    if output.status.success() {
        log::debug!("AppleScript paste completed");
        // Wait for paste to complete
        thread::sleep(Duration::from_millis(50));
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("AppleScript failed: {}", error))
    }
}

#[cfg(target_os = "macos")]
fn type_with_applescript(text: &str) -> Result<(), String> {
    log::debug!("Attempting to type characters with AppleScript");

    // Escape special characters for AppleScript string
    let escaped_text = text
        .replace('\\', "\\\\")
        .replace('"', "\\\"")
        .replace('\n', "\\n")
        .replace('\r', "\\r")
        .replace('\t', "\\t");

    // Use keystroke to type the text character by character
    // This is slower but works in places where paste is blocked
    let script = format!(
        r#"
        tell application "System Events"
            keystroke "{}"
        end tell
    "#,
        escaped_text
    );

    let output = std::process::Command::new("osascript")
        .arg("-e")
        .arg(&script)
        .output()
        .map_err(|e| format!("Failed to run osascript: {}", e))?;

    if output.status.success() {
        log::debug!("AppleScript keystroke completed");
        Ok(())
    } else {
        let error = String::from_utf8_lossy(&output.stderr);
        Err(format!("AppleScript keystroke failed: {}", error))
    }
}

// ============================================================================
// Windows/Linux implementations
// ============================================================================

#[cfg(not(target_os = "macos"))]
fn send_key_event(event_type: &EventType) -> Result<(), SimulateError> {
    match simulate(event_type) {
        Ok(()) => {
            // Let the OS catch up
            thread::sleep(Duration::from_millis(50));
            Ok(())
        }
        Err(e) => {
            log::error!("Failed to send event {:?}: {:?}", event_type, e);
            Err(e)
        }
    }
}

#[cfg(target_os = "windows")]
fn paste_with_rdev() -> Result<(), String> {
    log::debug!("Attempting paste with rdev on Windows");

    // Initial delay for stability
    thread::sleep(Duration::from_millis(50));

    for attempt in 1..=2 {
        log::debug!("Windows paste attempt {}/2", attempt);

        let result = (|| {
            send_key_event(&EventType::KeyPress(RdevKey::ControlLeft))?;
            send_key_event(&EventType::KeyPress(RdevKey::KeyV))?;
            send_key_event(&EventType::KeyRelease(RdevKey::KeyV))?;
            send_key_event(&EventType::KeyRelease(RdevKey::ControlLeft))?;
            Ok::<(), SimulateError>(())
        })();

        match result {
            Ok(_) => {
                log::debug!("Windows paste completed on attempt {}", attempt);
                return Ok(());
            }
            Err(e) if attempt < 2 => {
                log::warn!(
                    "Windows paste attempt {} failed: {:?}, retrying...",
                    attempt,
                    e
                );
                thread::sleep(Duration::from_millis(50));
            }
            Err(e) => {
                return Err(format!("Windows paste failed: {:?}", e));
            }
        }
    }

    unreachable!()
}

#[cfg(target_os = "linux")]
fn paste_with_rdev() -> Result<(), String> {
    log::debug!("Attempting paste with rdev on Linux");

    let result = (|| {
        send_key_event(&EventType::KeyPress(RdevKey::ControlLeft))?;
        send_key_event(&EventType::KeyPress(RdevKey::KeyV))?;
        send_key_event(&EventType::KeyRelease(RdevKey::KeyV))?;
        send_key_event(&EventType::KeyRelease(RdevKey::ControlLeft))?;
        Ok::<(), SimulateError>(())
    })();

    result.map_err(|e| format!("Linux paste failed: {:?}", e))
}
