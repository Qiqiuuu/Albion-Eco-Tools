use tauri::{AppHandle, State};
use aet_shared::models::user::{UserData};
use crate::loader;
use crate::state::AppState;




#[tauri::command]
pub fn get_player_specs(state: State<'_, AppState>) -> UserData {
    state.user.lock().unwrap().clone()
}

#[tauri::command]
pub fn update_player_specs(state: State<'_, AppState>, name: String, level: u32) {
    let mut data = state.user.lock().unwrap();
    data.specializations.insert(name, level);
}