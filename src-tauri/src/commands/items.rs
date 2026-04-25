use tauri::State;
use aet_shared::models::calculations::{CraftingContext, CraftingLocation, CraftingResult};
use aet_shared::models::items::{ItemEntity, ItemRegistry};
use aet_shared::models::prices::{PriceMap};
use crate::calculations::crafting_calculations;
use crate::loader::{save_prices};
use crate::state::AppState;




#[tauri::command]
pub fn fetch_all_prices(state: State<AppState>) -> Result<PriceMap, String> {
    let Ok(prices) = state.prices.read()
    else { return Err("Price lock is poisoned".to_string()) };
    Ok(prices.clone())
}

#[tauri::command]
pub fn fetch_all_items(state: State<AppState>) -> ItemRegistry {
    state.items.clone()
}

#[tauri::command]
pub fn fetch_item(state: State<AppState>, unique_name: String) -> Result<ItemEntity, String> {
    match state.items.items.get(&unique_name){
        Some(item) => {Ok(item.clone())},
        None => {Err("Item not found".to_string())}
    }
}


#[tauri::command]
pub fn update_item_price(handle: tauri::AppHandle,state: State<'_, AppState>, unique_name: String, new_price: u32)-> Result<(), String> {
    {
        {
            let Ok(mut data) = state.prices.write()
            else { return Err("RwLock is poisoned".to_string()) };
            match data.get_mut(&unique_name) {
                Some(prices) => {prices.current = new_price}
                None => {}
            }
        }
    }
    save_prices(&handle,&state)
}

#[tauri::command]
pub fn calculate_crafting(state: State<'_, AppState>, unique_name: &str, location: CraftingLocation,use_focus: bool,usage_fee: u32,is_premium: bool,amount: u32) -> Option<CraftingResult> {
    let prices = state.prices.read().unwrap();
    let user = state.user.lock().unwrap();
    let item_registry = &state.items;

    let item_entity = item_registry.items.get(&unique_name.to_string())?;

    let context = CraftingContext {
        item: item_entity,
        amount,
        prices: &prices,
        user_specs: &user.specializations,
        location,
        usage_fee,
        use_focus,
        is_premium,
    };

    crafting_calculations(&context)

}