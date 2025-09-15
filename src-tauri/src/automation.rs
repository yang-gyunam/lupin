use rand::{thread_rng, Rng};
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
use tokio::time::sleep;
use serde::{Deserialize, Serialize};
use enigo::{Enigo, Settings, Coordinate, Direction, Button, Keyboard, Mouse};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationConfig {
    pub enabled: bool,
    pub min_interval_ms: u64,
    pub max_interval_ms: u64,
    pub mouse_movement_range: i32,
    pub enable_clicks: bool,
    pub enable_keyboard: bool,
    pub keyboard_text: Option<String>,
    pub active_apps: Vec<String>,
}

impl Default for AutomationConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            min_interval_ms: 30000,  // 30 seconds
            max_interval_ms: 840000,  // 14 minutes
            mouse_movement_range: 5,
            enable_clicks: false,
            enable_keyboard: false,
            keyboard_text: None,
            active_apps: vec![],
        }
    }
}

pub struct AutomationEngine {
    config: Arc<Mutex<AutomationConfig>>,
    text_position: Arc<Mutex<usize>>,  // Track current position in text
}

impl AutomationEngine {
    pub fn new(config: AutomationConfig) -> Self {
        Self {
            config: Arc::new(Mutex::new(config)),
            text_position: Arc::new(Mutex::new(0)),
        }
    }

    pub async fn run(&self) {
        loop {
            // Generate random values before await
            let (interval, mouse_range) = {
                let config = self.config.lock().await;
                if !config.enabled {
                    break;
                }
                
                let mut rng = thread_rng();
                let interval = rng.gen_range(config.min_interval_ms..=config.max_interval_ms);
                (interval, config.mouse_movement_range)
            };
            
            // Sleep with generated interval
            sleep(Duration::from_millis(interval)).await;
            
            // Re-check if still enabled after sleep
            let config = self.config.lock().await;
            if !config.enabled {
                break;
            }
            
            // Perform automation action
            self.perform_action(
                mouse_range,
                config.enable_clicks,
                config.enable_keyboard,
                config.keyboard_text.clone()
            ).await;
        }
    }

    async fn perform_action(&self, mouse_range: i32, enable_clicks: bool, enable_keyboard: bool, keyboard_text: Option<String>) {
        // Check if any action is enabled
        if !enable_clicks && !enable_keyboard {
            println!("No actions enabled, skipping");
            return;
        }
        
        // Determine action before creating non-Send types
        let action = {
            let mut rng = thread_rng();
            // If only keyboard is enabled, always do keyboard action
            if enable_keyboard && !enable_clicks {
                2
            }
            // If only clicks are enabled, choose between mouse move or click
            else if enable_clicks && !enable_keyboard {
                rng.gen_range(0..2)
            }
            // If both are enabled, choose from all actions
            else {
                rng.gen_range(0..3)
            }
        };
        
        // Handle keyboard text position if needed
        let mut text_position = if action == 2 && enable_keyboard {
            Some(self.text_position.lock().await)
        } else {
            None
        };
        
        // Now create non-Send types after all awaits
        let mut enigo = match Enigo::new(&Settings::default()) {
            Ok(e) => e,
            Err(e) => {
                println!("Failed to initialize Enigo: {:?}", e);
                return;
            }
        };
        
        let mut rng = thread_rng();
        
        match action {
            0 => {
                // Move mouse smoothly to a random position
                let target_dx = rng.gen_range(-mouse_range..=mouse_range) as i32;
                let target_dy = rng.gen_range(-mouse_range..=mouse_range) as i32;
                
                // Smooth movement with multiple small steps
                Self::smooth_mouse_move(&mut enigo, target_dx, target_dy);
                
                println!("Smoothly moved mouse by ({}, {})", target_dx, target_dy);
            },
            1 if enable_clicks => {
                // Perform a click
                if let Err(e) = enigo.button(Button::Left, Direction::Click) {
                    println!("Failed to click mouse: {:?}", e);
                } else {
                    println!("Clicked mouse");
                }
            },
            2 if enable_keyboard => {
                // Type text sequentially from the configured text
                if let Some(ref text) = keyboard_text {
                    if !text.is_empty() {
                        // Use the pre-locked position
                        if let Some(ref mut pos) = text_position {
                            Self::type_text_sequentially(&mut enigo, text, pos);
                        }
                    } else {
                        println!("Keyboard text is empty, typing default character");
                        // Default behavior: type a random character
                        let chars = vec![' ', 'a', 'e', 'i', 'o', 'u'];
                        let random_char = chars[rng.gen_range(0..chars.len())];
                        if let Err(e) = enigo.text(&random_char.to_string()) {
                            println!("Failed to type character: {:?}", e);
                        } else {
                            println!("Typed character: {}", random_char);
                        }
                    }
                } else {
                    println!("No keyboard text configured, typing default character");
                    // Default behavior: type a random character
                    let chars = vec![' ', 'a', 'e', 'i', 'o', 'u'];
                    let random_char = chars[rng.gen_range(0..chars.len())];
                    if let Err(e) = enigo.text(&random_char.to_string()) {
                        println!("Failed to type character: {:?}", e);
                    } else {
                        println!("Typed character: {}", random_char);
                    }
                }
            },
            _ => {
                // Default to mouse movement
                let dx = rng.gen_range(-mouse_range..=mouse_range) as i32;
                let dy = rng.gen_range(-mouse_range..=mouse_range) as i32;
                
                Self::smooth_mouse_move(&mut enigo, dx, dy);
                
                println!("Moved mouse by ({}, {})", dx, dy);
            }
        }
    }

    pub async fn stop(&self) {
        let mut config = self.config.lock().await;
        config.enabled = false;
        println!("Automation engine stopped");
    }

    #[allow(dead_code)]
    pub async fn is_running(&self) -> bool {
        let config = self.config.lock().await;
        config.enabled
    }
    
    /// Smoothly move the mouse from current position to target position
    fn smooth_mouse_move(enigo: &mut Enigo, target_dx: i32, target_dy: i32) {
        use std::thread;
        use std::time::Duration;
        use rand::Rng;
        
        let mut rng = thread_rng();
        
        // Calculate the number of steps based on distance
        let distance = ((target_dx.pow(2) + target_dy.pow(2)) as f64).sqrt();
        let steps = (distance / 1.5).max(15.0).min(80.0) as i32; // More steps for smoother movement
        
        // Add slight curve to the path for more natural movement
        let curve_factor = rng.gen_range(-0.3..0.3); // Random curve
        
        let mut current_x = 0.0;
        let mut current_y = 0.0;
        
        for i in 0..steps {
            let progress = i as f64 / steps as f64;
            
            // Use easing function for natural acceleration/deceleration
            // This creates an S-curve motion (slow-fast-slow)
            let eased_progress = if progress < 0.5 {
                2.0 * progress * progress
            } else {
                1.0 - 2.0 * (1.0 - progress) * (1.0 - progress)
            };
            
            // Calculate target position for this step with slight curve
            let target_x = target_dx as f64 * eased_progress;
            let target_y = target_dy as f64 * eased_progress;
            
            // Add subtle curve to the path
            let curve_offset = (progress * std::f64::consts::PI).sin() * curve_factor * distance;
            let perpendicular_x = -target_dy as f64 / distance * curve_offset;
            let perpendicular_y = target_dx as f64 / distance * curve_offset;
            
            // Calculate movement delta for this step
            let final_x = target_x + perpendicular_x;
            let final_y = target_y + perpendicular_y;
            
            let delta_x = (final_x - current_x) as i32;
            let delta_y = (final_y - current_y) as i32;
            
            // Move the mouse by the delta
            if delta_x != 0 || delta_y != 0 {
                let _ = enigo.move_mouse(delta_x, delta_y, Coordinate::Rel);
                current_x = final_x;
                current_y = final_y;
            }
            
            // Variable delay based on movement phase
            let base_delay = rng.gen_range(1..3); // Base delay in milliseconds
            let random_delay = rng.gen_range(0..2); // Small random variation
            
            // Slower at start and end for more natural movement
            let speed_variation = if i < steps / 4 || i > steps * 3 / 4 {
                2 // Slower at start and end
            } else {
                0 // Faster in the middle
            };
            
            thread::sleep(Duration::from_millis((base_delay + random_delay + speed_variation) as u64));
        }
        
        // Final adjustment to ensure we reach exact target
        let final_x = target_dx - current_x as i32;
        let final_y = target_dy - current_y as i32;
        
        if final_x != 0 || final_y != 0 {
            let _ = enigo.move_mouse(final_x, final_y, Coordinate::Rel);
        }
    }
    
    /// Type text sequentially from the current position
    fn type_text_sequentially(enigo: &mut Enigo, text: &str, position: &mut usize) {
        use std::thread;
        use std::time::Duration;
        use rand::Rng;
        
        let mut rng = thread_rng();
        let chars: Vec<char> = text.chars().collect();
        
        if chars.is_empty() {
            return;
        }
        
        // Reset position if we've reached the end
        if *position >= chars.len() {
            *position = 0;
            println!("Reached end of text, wrapping around to beginning");
        }
        
        let start_pos = *position;
        
        // Determine how many characters to type (5-20 characters or until end of sentence/word)
        let base_chars = rng.gen_range(5..=20);
        let mut end_pos = start_pos;
        let max_chars = base_chars.min(chars.len() - start_pos);
        
        // Type at least the minimum, then try to complete the current word
        for i in start_pos..chars.len().min(start_pos + max_chars) {
            end_pos = i + 1;
            
            // If we've typed at least 5 chars and hit a word boundary, consider stopping
            if i >= start_pos + 5 {
                if chars[i] == ' ' || chars[i] == '.' || chars[i] == ',' || 
                   chars[i] == '!' || chars[i] == '?' || chars[i] == '\n' {
                    break;
                }
            }
        }
        
        // Type the selected portion of text
        let mut typed = String::new();
        for i in start_pos..end_pos {
            let ch = chars[i];
            typed.push(ch);
            
            // Type the character
            if let Err(e) = enigo.text(&ch.to_string()) {
                println!("Failed to type character '{}': {:?}", ch, e);
            }
            
            // Natural delay between characters
            // Simulate human typing speed (40-120 WPM)
            let base_delay = rng.gen_range(50..150); // milliseconds
            
            // Add variation for more natural feeling
            let variation = match ch {
                ' ' => rng.gen_range(0..50),  // Slightly longer after spaces
                '.' | '!' | '?' => rng.gen_range(100..300), // Longer pause after sentences
                ',' => rng.gen_range(50..150), // Medium pause after commas
                '\n' => rng.gen_range(200..400), // Pause after line breaks
                _ => 0,
            };
            
            thread::sleep(Duration::from_millis((base_delay + variation) as u64));
        }
        
        // Update position for next time
        *position = end_pos;
        
        println!("Typed text: \"{}\" (position: {} -> {})", typed, start_pos, end_pos);
    }
    
    /// Type text naturally with human-like delays between characters (legacy random version)
    #[allow(dead_code)]
    fn type_text_naturally(enigo: &mut Enigo, text: &str) {
        use std::thread;
        use std::time::Duration;
        use rand::Rng;
        
        let mut rng = thread_rng();
        let chars: Vec<char> = text.chars().collect();
        
        // Pick a random starting position in the text
        let start_pos = rng.gen_range(0..chars.len());
        
        // Determine how many characters to type (1-5 characters or a word)
        let mut end_pos = start_pos;
        let max_chars = rng.gen_range(1..=5).min(chars.len() - start_pos);
        
        // Try to complete a word if we're in the middle of one
        for i in start_pos..chars.len().min(start_pos + max_chars) {
            end_pos = i + 1;
            // Stop at word boundaries (space, punctuation)
            if chars[i] == ' ' || chars[i] == '.' || chars[i] == ',' || chars[i] == '!' || chars[i] == '?' {
                break;
            }
        }
        
        // Type the selected portion of text
        for i in start_pos..end_pos {
            let ch = chars[i];
            
            // Type the character
            if let Err(e) = enigo.text(&ch.to_string()) {
                println!("Failed to type character '{}': {:?}", ch, e);
            }
            
            // Natural delay between characters
            // Simulate human typing speed (40-120 WPM)
            let base_delay = rng.gen_range(50..150); // milliseconds
            
            // Add variation for more natural feeling
            let variation = match ch {
                ' ' => rng.gen_range(0..50),  // Slightly longer after spaces
                '.' | '!' | '?' => rng.gen_range(100..300), // Longer pause after sentences
                ',' => rng.gen_range(50..150), // Medium pause after commas
                _ => 0,
            };
            
            thread::sleep(Duration::from_millis((base_delay + variation) as u64));
        }
        
        println!("Typed text portion: '{}'", &text[start_pos..end_pos]);
    }
}