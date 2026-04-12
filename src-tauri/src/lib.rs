
pub mod commands;

mod loader;
mod state;
pub mod calculations;

use aet_shared::models::user::UserData;
use crate::commands::items::{fetch_items_by_category, refresh_prices};
use crate::commands::user::{get_player_specs, update_player_specs};
use crate::loader::{load_item_registry, load_prices};
use crate::state::AppState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let state = AppState {
        items: load_item_registry(),
        prices: std::sync::RwLock::new(load_prices()),
        user: std::sync::Mutex::new(UserData::default()),
    };

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .manage(state)
        .invoke_handler(tauri::generate_handler![
            get_player_specs,
            update_player_specs,
            fetch_items_by_category,
            refresh_prices
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
