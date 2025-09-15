use tauri::{
    AppHandle, Manager, Emitter,
    menu::{Menu, MenuItemBuilder, PredefinedMenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

#[cfg(target_os = "macos")]
use cocoa::appkit::NSApplication;

pub fn create_tray(app: &AppHandle) -> tauri::Result<()> {
    // Create tray menu
    let menu = Menu::new(app)?;
    
    // Add menu items
    let show_item = MenuItemBuilder::with_id("show", "Show Lupin")
        .build(app)?;
    let hide_item = MenuItemBuilder::with_id("hide", "Hide")
        .build(app)?;
    let separator = PredefinedMenuItem::separator(app)?;
    let start_automation = MenuItemBuilder::with_id("start_automation", "Start Automation")
        .build(app)?;
    let stop_automation = MenuItemBuilder::with_id("stop_automation", "Stop Automation")
        .build(app)?;
    let separator2 = PredefinedMenuItem::separator(app)?;
    let quit_item = MenuItemBuilder::with_id("quit", "Quit Lupin")
        .build(app)?;
    
    menu.append(&show_item)?;
    menu.append(&hide_item)?;
    menu.append(&separator)?;
    menu.append(&start_automation)?;
    menu.append(&stop_automation)?;
    menu.append(&separator2)?;
    menu.append(&quit_item)?;
    
    // Build tray with menu
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("Lupin - Anti-surveillance Tool")
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(move |app, event| {
            match event.id.as_ref() {
                "show" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                        
                        // Show in Dock on macOS when window is shown
                        #[cfg(target_os = "macos")]
                        {
                            use cocoa::appkit::{NSApp, NSApplicationActivationPolicy};
                            unsafe {
                                let app = NSApp();
                                app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
                            }
                        }
                    }
                }
                "hide" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.hide();
                        
                        // Hide from Dock on macOS when window is hidden
                        #[cfg(target_os = "macos")]
                        {
                            use cocoa::appkit::{NSApp, NSApplicationActivationPolicy};
                            unsafe {
                                let app = NSApp();
                                app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
                            }
                        }
                    }
                }
                "start_automation" => {
                    // Emit event to frontend
                    let _ = app.emit("tray-start-automation", ());
                }
                "stop_automation" => {
                    // Emit event to frontend
                    let _ = app.emit("tray-stop-automation", ());
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            match event {
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                } => {
                    // Single click - show/hide window
                    let app = tray.app_handle();
                    if let Some(window) = app.get_webview_window("main") {
                        if window.is_visible().unwrap_or(false) {
                            let _ = window.hide();
                            
                            // Hide from Dock on macOS
                            #[cfg(target_os = "macos")]
                            {
                                use cocoa::appkit::{NSApp, NSApplicationActivationPolicy};
                                unsafe {
                                    let app = NSApp();
                                    app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
                                }
                            }
                        } else {
                            let _ = window.show();
                            let _ = window.set_focus();
                            
                            // Show in Dock on macOS
                            #[cfg(target_os = "macos")]
                            {
                                use cocoa::appkit::{NSApp, NSApplicationActivationPolicy};
                                unsafe {
                                    let app = NSApp();
                                    app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
                                }
                            }
                        }
                    }
                }
                TrayIconEvent::DoubleClick {
                    button: MouseButton::Left,
                    ..
                } => {
                    // Double click - show window
                    let app = tray.app_handle();
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }
                _ => {}
            }
        })
        .build(app)?;
    
    Ok(())
}

#[allow(dead_code)]
pub fn update_tray_tooltip(_app: &AppHandle, is_active: bool) {
    let status = if is_active { "Active" } else { "Inactive" };
    let _tooltip = format!("Lupin - {}", status);
    
    // Note: In Tauri v2, we need to get the tray instance
    // This is a simplified version - in production you'd store the tray reference
    // and update it directly
}