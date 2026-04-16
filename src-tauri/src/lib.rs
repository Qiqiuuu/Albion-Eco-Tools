
pub mod commands;

mod loader;
mod state;
pub mod calculations;

use std::sync::{Mutex, RwLock};
use tauri::Manager;
use crate::commands::items::{fetch_all_items, fetch_all_prices};
use crate::commands::user::{get_player_data, update_player_specs};
use crate::loader::{load_item_registry, load_prices, load_user};
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let handle = app.handle();
            let state = AppState {
                items:  load_item_registry(handle),
                prices: RwLock::new(load_prices(handle)),
                user:   Mutex::new(load_user(handle)),
            };
            app.manage(state);
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            get_player_data,
            update_player_specs,
            fetch_all_items,
            fetch_all_prices
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
