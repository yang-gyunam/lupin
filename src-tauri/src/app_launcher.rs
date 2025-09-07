use std::process::Command;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppConfig {
    pub name: String,
    pub path: String,
    pub args: Vec<String>,
    pub icon: Option<String>,
}

impl AppConfig {
    pub fn get_default_apps() -> Vec<AppConfig> {
        #[cfg(target_os = "macos")]
        {
            vec![
                AppConfig {
                    name: "Visual Studio Code".to_string(),
                    path: "/Applications/Visual Studio Code.app/Contents/MacOS/Electron".to_string(),
                    args: vec![],
                    icon: Some("vscode.png".to_string()),
                },
                AppConfig {
                    name: "Chrome".to_string(),
                    path: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome".to_string(),
                    args: vec!["--new-window".to_string()],
                    icon: Some("chrome.png".to_string()),
                },
                AppConfig {
                    name: "Terminal".to_string(),
                    path: "/System/Applications/Utilities/Terminal.app/Contents/MacOS/Terminal".to_string(),
                    args: vec![],
                    icon: Some("terminal.png".to_string()),
                },
                AppConfig {
                    name: "Slack".to_string(),
                    path: "/Applications/Slack.app/Contents/MacOS/Slack".to_string(),
                    args: vec![],
                    icon: Some("slack.png".to_string()),
                },
                AppConfig {
                    name: "Notion".to_string(),
                    path: "/Applications/Notion.app/Contents/MacOS/Notion".to_string(),
                    args: vec![],
                    icon: Some("notion.png".to_string()),
                },
            ]
        }
        
        #[cfg(target_os = "windows")]
        {
            vec![
                AppConfig {
                    name: "Visual Studio Code".to_string(),
                    path: "C:\\Program Files\\Microsoft VS Code\\Code.exe".to_string(),
                    args: vec![],
                    icon: Some("vscode.png".to_string()),
                },
                AppConfig {
                    name: "Chrome".to_string(),
                    path: "C:\\Program Files\\Google\\Chrome\\Application\\chrome.exe".to_string(),
                    args: vec!["--new-window".to_string()],
                    icon: Some("chrome.png".to_string()),
                },
                AppConfig {
                    name: "Notepad++".to_string(),
                    path: "C:\\Program Files\\Notepad++\\notepad++.exe".to_string(),
                    args: vec![],
                    icon: Some("notepad.png".to_string()),
                },
                AppConfig {
                    name: "PowerShell".to_string(),
                    path: "C:\\Windows\\System32\\WindowsPowerShell\\v1.0\\powershell.exe".to_string(),
                    args: vec![],
                    icon: Some("powershell.png".to_string()),
                },
            ]
        }
        
        #[cfg(target_os = "linux")]
        {
            vec![
                AppConfig {
                    name: "Visual Studio Code".to_string(),
                    path: "/usr/bin/code".to_string(),
                    args: vec![],
                    icon: Some("vscode.png".to_string()),
                },
                AppConfig {
                    name: "Chrome".to_string(),
                    path: "/usr/bin/google-chrome".to_string(),
                    args: vec!["--new-window".to_string()],
                    icon: Some("chrome.png".to_string()),
                },
                AppConfig {
                    name: "Terminal".to_string(),
                    path: "/usr/bin/gnome-terminal".to_string(),
                    args: vec![],
                    icon: Some("terminal.png".to_string()),
                },
            ]
        }
    }
    
    pub fn launch(&self) -> Result<(), String> {
        Command::new(&self.path)
            .args(&self.args)
            .spawn()
            .map_err(|e| format!("Failed to launch {}: {}", self.name, e))?;
        
        Ok(())
    }
    
    pub fn is_installed(&self) -> bool {
        std::path::Path::new(&self.path).exists()
    }
}