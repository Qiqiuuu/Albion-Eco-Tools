use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use aet_shared::models::items::ItemRegistry;
use aet_shared::models::prices::{ItemPrice};
use aet_shared::models::user::UserData;
use crate::loader;
use crate::state::AppState;

fn get_config_path(handle: &tauri::AppHandle, json: &str) -> Result<PathBuf, String> {
    handle.path()
        .app_config_dir()
        .map(|p| p.join(format!("{}.json", json)))
        .map_err(|e| e.to_string())
}

pub fn save_json<T: Serialize>(handle: &tauri::AppHandle, data: &T,json: &str) -> Result<(), String> {
    let path = get_config_path(handle, json)?;

    if let Some(parent) = path.parent() {
        fs::create_dir_all(parent).map_err(|e| e.to_string())?;
    }

    let json = serde_json::to_string_pretty(data).map_err(|e| e.to_string())?;
    fs::write(path, json).map_err(|e| e.to_string())
}


pub fn load_json<T: DeserializeOwned + Default>(handle: &tauri::AppHandle,json: &str) -> T {
    let path = get_config_path(handle,json).ok();

    path.and_then(|p| fs::read_to_string(p).ok())
        .and_then(|json| serde_json::from_str(&json).ok())
        .unwrap_or_default()
}


pub fn load_item_registry() -> ItemRegistry {
    let data = fs::read_to_string("resources/items.json")
        .unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn load_prices() -> ItemPrice {
    let data = fs::read_to_string("resources/prices.json")
        .unwrap_or_default();
    serde_json::from_str(&data).unwrap_or_default()
}

pub fn load_player_specs(handle: AppHandle) -> UserData {
    load_json::<UserData>(&handle, "player_data")
}

pub fn save_player_specs(handle: AppHandle, state: State<'_, AppState>) -> Result<(), String> {
    let data = state.user.lock().unwrap();
    save_json(&handle, &*data, "spec")
}
