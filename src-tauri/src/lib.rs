mod user;
mod data;

use std::sync::Mutex;
use crate::user::{AppState, UserData,get_player_data,update_level};
use crate::data::{load_data, save_data};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(AppState(Mutex::new(UserData::default())))
        .invoke_handler(tauri::generate_handler![
            get_player_data,
            update_level,
            save_data,
            load_data])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
