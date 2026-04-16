use serde::{Serialize, de::DeserializeOwned};
use std::fs;
use std::path::PathBuf;
use tauri::{AppHandle, Manager, State};
use aet_shared::models::items::ItemRegistry;
use aet_shared::models::prices::{ItemPrice, PriceMap};
use aet_shared::models::user::UserData;
use crate::state::AppState;


const USER_FILE:   &str = "user";
const ITEMS_FILE:  &str = "items";
const PRICES_FILE: &str = "prices";

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

pub fn load_user(handle: &AppHandle) -> UserData {
    load_json(handle, USER_FILE)
}

pub fn load_item_registry(handle: &AppHandle) -> ItemRegistry {
    load_json(handle, ITEMS_FILE)
}

pub fn load_prices(handle: &AppHandle) -> PriceMap {
    load_json(handle, PRICES_FILE)
}


pub fn save_user(handle: &AppHandle, state: &State<AppState>) -> Result<(), String> {
    let data = state.user.lock().unwrap();
    save_json(handle, &*data, USER_FILE)
}

pub fn save_prices(handle: &AppHandle, state: &State<AppState>) -> Result<(), String> {
    let data = state.prices.read().unwrap();
    save_json(handle, &*data, PRICES_FILE)
}
