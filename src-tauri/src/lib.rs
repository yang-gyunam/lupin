mod automation;
mod process_disguise;
mod app_launcher;
mod tray;

use automation::{AutomationConfig, AutomationEngine};
use process_disguise::ProcessDisguise;
use app_launcher::AppConfig;
use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::Manager;

struct AppState {
    automation_engine: Arc<Mutex<Option<Arc<AutomationEngine>>>>,
}

#[tauri::command]
async fn start_automation(
    config: AutomationConfig, 
    state: tauri::State<'_, AppState>
) -> Result<(), String> {
    let mut engine_lock = state.automation_engine.lock().await;
    
    // Create new engine
    let engine = Arc::new(AutomationEngine::new(config));
    let engine_clone = Arc::clone(&engine);
    
    // Start automation in background
    tokio::spawn(async move {
        engine_clone.run().await;
    });
    
    // Store engine reference
    *engine_lock = Some(engine);
    
    Ok(())
}

#[tauri::command]
async fn stop_automation(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let engine_lock = state.automation_engine.lock().await;
    if let Some(engine) = engine_lock.as_ref() {
        engine.stop().await;
    }
    Ok(())
}

#[tauri::command]
fn get_disguises() -> Vec<ProcessDisguise> {
    ProcessDisguise::get_available_disguises()
}

#[tauri::command]
fn set_process_name(name: String) -> Result<(), String> {
    ProcessDisguise::set_process_name(&name);
    Ok(())
}

#[tauri::command]
fn get_available_apps() -> Vec<AppConfig> {
    AppConfig::get_default_apps()
}

#[tauri::command]
fn launch_app(app: AppConfig) -> Result<(), String> {
    app.launch()
}

#[tauri::command]
fn check_app_installed(app: AppConfig) -> bool {
    app.is_installed()
}

#[tauri::command]
fn get_system_language() -> String {
    // Get system language using environment variables
    // First try LANG, then LC_ALL, then default to en
    std::env::var("LANG")
        .or_else(|_| std::env::var("LC_ALL"))
        .or_else(|_| std::env::var("LANGUAGE"))
        .unwrap_or_else(|_| "en_US".to_string())
}

#[tauri::command]
fn show_in_dock() {
    #[cfg(target_os = "macos")]
    {
        use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};
        unsafe {
            let app = NSApp();
            app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyRegular);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let app_state = AppState {
        automation_engine: Arc::new(Mutex::new(None)),
    };
    
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .manage(app_state)
        .invoke_handler(tauri::generate_handler![
            start_automation,
            stop_automation,
            get_disguises,
            set_process_name,
            get_available_apps,
            launch_app,
            check_app_installed,
            get_system_language,
            show_in_dock
        ])
        .setup(|app| {
            // Set activation policy to accessory on startup (hide from Dock)
            #[cfg(target_os = "macos")]
            {
                use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};
                unsafe {
                    let app = NSApp();
                    app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
                }
            }
            
            // Create system tray
            tray::create_tray(&app.handle())?;
            
            // Setup window close behavior - hide instead of exit
            if let Some(window) = app.get_webview_window("main") {
                let window_clone = window.clone();
                window.on_window_event(move |event| {
                    if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                        // Prevent default close behavior
                        api.prevent_close();
                        // Hide the window instead
                        let _ = window_clone.hide();
                        
                        // Hide from Dock on macOS when window is hidden
                        #[cfg(target_os = "macos")]
                        {
                            use cocoa::appkit::{NSApp, NSApplication, NSApplicationActivationPolicy};
                            unsafe {
                                let app = NSApp();
                                app.setActivationPolicy_(NSApplicationActivationPolicy::NSApplicationActivationPolicyAccessory);
                            }
                        }
                    }
                });
            }
            
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}