use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::Mutex;
use tauri::{Manager, State};

// App Settings
#[derive(Serialize, Deserialize, Clone)]
pub struct Settings {
    pub server_url: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            server_url: "192.168.1.17:5882".to_string(),
        }
    }
}

pub struct AppState {
    pub settings: Mutex<Settings>,
    pub settings_path: PathBuf,
}

impl AppState {
    fn save_settings(&self) -> Result<(), String> {
        let settings = self.settings.lock().unwrap();
        let json = serde_json::to_string_pretty(&*settings)
            .map_err(|e| format!("Failed to serialize settings: {}", e))?;
        fs::write(&self.settings_path, json)
            .map_err(|e| format!("Failed to write settings: {}", e))?;
        Ok(())
    }

    fn load_settings(path: &PathBuf) -> Settings {
        fs::read_to_string(path)
            .ok()
            .and_then(|json| serde_json::from_str(&json).ok())
            .unwrap_or_default()
    }
}

#[tauri::command]
fn get_settings(state: State<AppState>) -> Settings {
    state.settings.lock().unwrap().clone()
}

#[tauri::command]
fn update_settings(state: State<AppState>, settings: Settings) -> Result<(), String> {
    *state.settings.lock().unwrap() = settings;
    state.save_settings()
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[derive(Serialize, Deserialize)]
struct MoveCommand {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize)]
struct RobotStatus {
    position: Position,
    x_status: String,
    y_status: String,
    z_status: String,
    status: String,
}

#[derive(Serialize, Deserialize)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[tauri::command]
async fn connect(state: State<'_, AppState>) -> Result<RobotStatus, String> {
    let server_url = state.settings.lock().unwrap().server_url.clone();
    let client = reqwest::Client::new();
    let url = format!("http://{}/api/position", server_url);

    match client.get(&url).send().await {
        Ok(response) => {
            if response.status().is_success() {
                response
                    .json::<RobotStatus>()
                    .await
                    .map_err(|e| format!("Failed to parse response: {}", e))
            } else {
                Err(format!(
                    "Server responded with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Err(format!("Failed to connect: {}", e)),
    }
}

#[tauri::command]
async fn send_go_command(
    state: State<'_, AppState>,
    x: f64,
    y: f64,
    z: f64,
) -> Result<String, String> {
    let server_url = state.settings.lock().unwrap().server_url.clone();
    let client = reqwest::Client::new();
    let command = MoveCommand { x, y, z };

    let url = format!("http://{}/api/move", server_url);

    match client.post(&url).json(&command).send().await {
        Ok(response) => {
            if response.status().is_success() {
                Ok(format!("Command sent successfully to {}", url))
            } else {
                Err(format!(
                    "Server responded with status: {}",
                    response.status()
                ))
            }
        }
        Err(e) => Err(format!("Failed to send command: {}", e)),
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            let app_data_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to get app data dir");
            fs::create_dir_all(&app_data_dir).expect("Failed to create app data dir");

            let settings_path = app_data_dir.join("settings.json");
            let settings = AppState::load_settings(&settings_path);

            app.manage(AppState {
                settings: Mutex::new(settings),
                settings_path,
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            greet,
            get_settings,
            update_settings,
            connect,
            send_go_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
