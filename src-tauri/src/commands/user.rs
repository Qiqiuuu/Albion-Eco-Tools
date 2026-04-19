use tauri::{State};
use aet_shared::models::config::{ActiveTab, AveragePrice, Cities};
use aet_shared::models::items::TrackedFood;
use aet_shared::models::specializations::{CategoryId, SpecId};
use aet_shared::models::user::{UserData};
use crate::loader::save_user;
use crate::state::AppState;


macro_rules! with_user {
    ($state:expr, |$data:ident| $block:block) => {{
        let Ok(mut $data) = $state.user.lock()
        else { return Err("User lock is poisoned".to_string()) };
        $block
    }};
}


#[tauri::command]
pub fn get_player_data(state: State<'_, AppState>) -> Result<UserData, String> {
    let Ok(user) = state.user.lock()
    else { return Err("User lock is poisoned".to_string()) };
    
    Ok(user.clone())
        
}

#[tauri::command]
pub fn get_tracked_foods(state: State<'_, AppState>) -> Result<Vec<TrackedFood>, String> {
    let Ok(user) = state.user.lock()
    else { return Err("User lock is poisoned".to_string()) };
    
    Ok(user.tracked_foods.clone())
}


#[tauri::command]
pub fn update_player_specs(handle: tauri::AppHandle,state: State<'_, AppState>,updated_spec_id :SpecId, level: u32)-> Result<(), String> {
    with_user!(state, |data| {data.set_spec_level(updated_spec_id, level)});
    save_user(&handle,&state)
}

#[tauri::command]
pub fn update_player_mastery(handle: tauri::AppHandle,state: State<'_, AppState>, cat_id: CategoryId, level: u32)-> Result<(), String> {
    with_user!(state, |data| {data.set_mastery_level(cat_id, level)});
    save_user(&handle,&state)
}

#[tauri::command]
pub fn update_active_tab(handle: tauri::AppHandle,state: State<'_, AppState>, new_active_tab: ActiveTab)-> Result<(), String> {
    with_user!(state, |data| {data.active_tab = new_active_tab});
    save_user(&handle,&state)
}

#[tauri::command]
pub fn update_active_category(handle: tauri::AppHandle,state: State<'_, AppState>, new_active_category: CategoryId)-> Result<(), String> {
    with_user!(state, |data| {data.active_category = new_active_category});
    save_user(&handle,&state)
}
#[tauri::command]
pub fn update_premium(handle: tauri::AppHandle,state: State<'_, AppState>, new_premium: bool)-> Result<(), String> {
    with_user!(state, |data| {data.use_premium=new_premium});
    save_user(&handle,&state)
}
#[tauri::command]
pub fn update_focus(handle: tauri::AppHandle,state: State<'_, AppState>, new_focus: bool)-> Result<(), String> {
    with_user!(state, |data| {data.use_focus=new_focus});
    save_user(&handle,&state)
}
#[tauri::command]
pub fn update_silver_fee(handle: tauri::AppHandle,state: State<'_, AppState>, new_silver_fee: u32)-> Result<(), String> {
    with_user!(state, |data| {data.silver_fee = new_silver_fee});
    save_user(&handle,&state)
}

#[tauri::command]
pub fn update_city(handle: tauri::AppHandle,state: State<'_, AppState>, new_city: Cities)-> Result<(), String> {
    with_user!(state, |data|{data.city = new_city});
    save_user(&handle,&state)
}

#[tauri::command]
pub fn update_avg(handle: tauri::AppHandle,state: State<'_, AppState>, new_avg: AveragePrice)-> Result<(), String> {
    with_user!(state, |data|{data.avg=new_avg});
    save_user(&handle,&state)
}

#[tauri::command]
pub fn add_tracked_food(handle: tauri::AppHandle,state: State<'_, AppState>, new_food: TrackedFood)-> Result<(), String> {
    with_user!(state, |data| {data.tracked_foods.push(new_food)});
    save_user(&handle,&state)
}

#[tauri::command]
pub fn remove_tracked_food(handle: tauri::AppHandle,state: State<'_, AppState>, food_to_remove: TrackedFood)-> Result<(), String> {
    with_user!(state, |data| {
        let Some(index) = data.tracked_foods
            .iter()
            .position(|x| x == &food_to_remove)
        else { return Err("Tracked food not found".to_string()) };


        data.tracked_foods.remove(index);
    });
    save_user(&handle,&state)
}


