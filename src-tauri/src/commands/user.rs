use tauri::{State};
use aet_shared::models::config::{ActiveTab, AveragePrice, Cities};
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
#[tauri::command]
pub fn update_premium(handle: tauri::AppHandle,state: State<'_, AppState>, new_premium: bool)-> Result<(), String> {
    {
        let mut data = state.user.lock().unwrap();
        data.use_premium=new_premium;
    }
    save_user(&handle,&state)
}
#[tauri::command]
pub fn update_focus(handle: tauri::AppHandle,state: State<'_, AppState>, new_focus: bool)-> Result<(), String> {
    {
        let mut data = state.user.lock().unwrap();
        data.use_focus=new_focus;
    }
    save_user(&handle,&state)
}
#[tauri::command]
pub fn update_silver_fee(handle: tauri::AppHandle,state: State<'_, AppState>, new_silver_fee: u32)-> Result<(), String> {
    {
        let mut data = state.user.lock().unwrap();
        data.silver_fee = new_silver_fee;
    }
    save_user(&handle,&state)
}

#[tauri::command]
pub fn update_city(handle: tauri::AppHandle,state: State<'_, AppState>, new_city: Cities)-> Result<(), String> {
    {
        let mut data = state.user.lock().unwrap();
        data.city = new_city;
    }
    save_user(&handle,&state)
}

#[tauri::command]
pub fn update_avg(handle: tauri::AppHandle,state: State<'_, AppState>, new_avg: AveragePrice)-> Result<(), String> {
    {
        let mut data = state.user.lock().unwrap();
        data.avg=new_avg;
    }
    save_user(&handle,&state)
}
