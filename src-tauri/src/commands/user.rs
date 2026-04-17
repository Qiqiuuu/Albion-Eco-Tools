use tauri::{State};
use aet_shared::models::config::ActiveTab;
use aet_shared::models::specializations::{CategoryId, SpecId};
use aet_shared::models::user::{UserData};
use crate::loader::save_user;
use crate::state::AppState;



#[tauri::command]
pub fn get_player_data(state: State<'_, AppState>) -> UserData {
    state.user.lock().unwrap().clone()
}


#[tauri::command]
pub fn update_player_specs(handle: tauri::AppHandle,state: State<'_, AppState>,updated_spec_id :SpecId, level: u32)-> Result<(), String> {
    {
        let mut data = state.user.lock().unwrap();
        data.set_spec_level(updated_spec_id, level);
    }
    save_user(&handle,&state)
}

#[tauri::command]
pub fn update_player_mastery(handle: tauri::AppHandle,state: State<'_, AppState>, cat_id: CategoryId, level: u32)-> Result<(), String> {
    {
        let mut data = state.user.lock().unwrap();
        data.set_mastery_level(cat_id, level);
    }

    save_user(&handle,&state)
}

#[tauri::command]
pub fn update_active_tab(handle: tauri::AppHandle,state: State<'_, AppState>, new_active_tab: ActiveTab)-> Result<(), String> {
    {
        let mut data = state.user.lock().unwrap();
        data.active_tab = new_active_tab;
    }
    save_user(&handle,&state)
}

#[tauri::command]
pub fn update_active_category(handle: tauri::AppHandle,state: State<'_, AppState>, new_active_category: CategoryId)-> Result<(), String> {
    {
        let mut data = state.user.lock().unwrap();
        data.active_category = new_active_category;
    }
    save_user(&handle,&state)
}
