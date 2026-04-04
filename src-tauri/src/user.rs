use std::collections::HashMap;
use std::sync::Mutex;
use serde::{Deserialize, Serialize};
use tauri::State;

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub struct UserData {
    pub specializations: HashMap<String, u32>,
}

pub struct AppState(pub Mutex<UserData>);

#[tauri::command]
pub fn get_player_data(state: State<AppState>) -> UserData {
    state.0.lock().unwrap().clone()
}


#[tauri::command]
pub fn update_level(state: State<AppState>, name: String, level: u32) {
    let mut user = state.0.lock().unwrap();
    user.specializations.insert(name, level);
}

