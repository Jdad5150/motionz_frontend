use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;
use tauri::{Emitter, Manager, State};
use tokio::sync::Mutex;
use tokio_tungstenite::{connect_async, tungstenite::Message};

const POLL_INTERVAL_MS: u64 = 10; // 100Hz

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
    pub polling_active: Arc<AtomicBool>,
}

impl AppState {
    async fn save_settings(&self) -> Result<(), String> {
        let settings = self.settings.lock().await;
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
async fn get_settings(state: State<'_, AppState>) -> Result<Settings, String> {
    Ok(state.settings.lock().await.clone())
}

#[tauri::command]
async fn update_settings(state: State<'_, AppState>, settings: Settings) -> Result<(), String> {
    *state.settings.lock().await = settings;
    state.save_settings().await
}

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

#[derive(Serialize, Deserialize)]
struct MoveCommand {
    x: f64,
    y: f64,
    z: f64,
}

#[derive(Serialize, Deserialize, Clone)]
struct RobotStatus {
    position: Position,
    x_status: String,
    y_status: String,
    z_status: String,
    status: String,
}

#[derive(Serialize, Deserialize, Clone)]
struct Position {
    x: f64,
    y: f64,
    z: f64,
}

#[tauri::command]
async fn connect(
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle,
) -> Result<RobotStatus, String> {
    // Stop any existing polling
    state.polling_active.store(false, Ordering::SeqCst);
    tokio::time::sleep(Duration::from_millis(POLL_INTERVAL_MS * 2)).await;

    let server_url = state.settings.lock().await.server_url.clone();
    let ws_url = format!("ws://{}/ws", server_url);

    let (ws_stream, _) = connect_async(&ws_url)
        .await
        .map_err(|e| format!("Failed to connect to WebSocket: {}", e))?;

    let (mut write, mut read) = ws_stream.split();

    // Get initial status
    write
        .send(Message::Text("ping".into()))
        .await
        .map_err(|e| format!("Failed to send ping: {}", e))?;

    let initial_status: RobotStatus = loop {
        match read.next().await {
            Some(Ok(Message::Text(text))) => {
                break serde_json::from_str(&text)
                    .map_err(|e| format!("Failed to parse response: {}", e))?;
            }
            Some(Ok(Message::Close(_))) => {
                return Err("Connection closed".to_string());
            }
            Some(Err(e)) => {
                return Err(format!("WebSocket error: {}", e));
            }
            None => {
                return Err("No response received".to_string());
            }
            _ => continue,
        }
    };

    // Start polling loop
    let polling_active = state.polling_active.clone();
    polling_active.store(true, Ordering::SeqCst);

    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_millis(POLL_INTERVAL_MS));

        while polling_active.load(Ordering::SeqCst) {
            interval.tick().await;

            if write.send(Message::Text("ping".into())).await.is_err() {
                break;
            }

            match read.next().await {
                Some(Ok(Message::Text(text))) => {
                    if let Ok(status) = serde_json::from_str::<RobotStatus>(&text) {
                        let _ = app_handle.emit("robot-status", status);
                    }
                }
                Some(Ok(Message::Close(_))) | Some(Err(_)) | None => {
                    break;
                }
                _ => continue,
            }
        }

        polling_active.store(false, Ordering::SeqCst);
        let _ = app_handle.emit("robot-disconnected", ());
    });

    Ok(initial_status)
}

#[tauri::command]
async fn disconnect(state: State<'_, AppState>) -> Result<(), String> {
    state.polling_active.store(false, Ordering::SeqCst);
    Ok(())
}

#[tauri::command]
async fn send_go_command(
    state: State<'_, AppState>,
    x: f64,
    y: f64,
    z: f64,
) -> Result<String, String> {
    let server_url = state.settings.lock().await.server_url.clone();
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
                polling_active: Arc::new(AtomicBool::new(false)),
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            get_settings,
            update_settings,
            connect,
            disconnect,
            send_go_command
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
