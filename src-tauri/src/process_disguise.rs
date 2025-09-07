use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessDisguise {
    pub name: String,
    pub description: String,
}

impl ProcessDisguise {
    pub fn get_available_disguises() -> Vec<ProcessDisguise> {
        vec![
            ProcessDisguise {
                name: "system_update".to_string(),
                description: "System Update Service".to_string(),
            },
            ProcessDisguise {
                name: "chrome_helper".to_string(),
                description: "Chrome Helper Process".to_string(),
            },
            ProcessDisguise {
                name: "runtime_broker".to_string(),
                description: "Runtime Broker".to_string(),
            },
            ProcessDisguise {
                name: "software_reporter".to_string(),
                description: "Software Reporter Tool".to_string(),
            },
            ProcessDisguise {
                name: "com.apple.WebKit".to_string(),
                description: "WebKit Service".to_string(),
            },
            ProcessDisguise {
                name: "WindowServer".to_string(),
                description: "Window Server Process".to_string(),
            },
            ProcessDisguise {
                name: "mdworker".to_string(),
                description: "Metadata Server Worker".to_string(),
            },
            ProcessDisguise {
                name: "kernel_task".to_string(),
                description: "Kernel Task".to_string(),
            },
        ]
    }
    
    #[cfg(target_os = "macos")]
    pub fn set_process_name(name: &str) {
        // Process name changing is complex and requires special permissions
        // For now, we'll just log the intention
        println!("Would set process name to: {}", name);
    }
    
    #[cfg(target_os = "windows")]
    pub fn set_process_name(name: &str) {
        // On Windows, we can't directly change process name
        // But we can set the window title which appears in Task Manager
        println!("Would set process name to: {}", name);
    }
    
    #[cfg(target_os = "linux")]
    pub fn set_process_name(name: &str) {
        // Linux process name changing via prctl
        println!("Would set process name to: {}", name);
    }
}