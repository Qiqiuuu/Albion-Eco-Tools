use std::fs;
use tauri::Manager;

#[tauri::command]
pub fn save_data(handle: tauri::AppHandle, json: String) -> Result<(), String> {
    let path = handle.path().app_config_dir().map_err(|e| e.to_string())?.join("data.json");
    fs::create_dir_all(path.parent().unwrap()).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}

#[tauri::command]
pub fn load_data(handle: tauri::AppHandle) -> Result<String, String> {
    let path = handle.path().app_config_dir().map_err(|e| e.to_string())?.join("data.json");
    if !path.exists() { return Ok("{}".to_string()); }
    fs::read_to_string(path).map_err(|e| e.to_string())
}