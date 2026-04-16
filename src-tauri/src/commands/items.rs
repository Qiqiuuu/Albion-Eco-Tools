
use tauri::State;
use aet_shared::models::calculations::{CraftingContext, CraftingLocation, CraftingResult};
use aet_shared::models::items::{ItemRegistry};
use aet_shared::models::prices::{ItemPrice, PriceMap};
use crate::calculations::crafting_calculations;
use crate::state::AppState;



#[tauri::command]
pub fn fetch_all_prices(state: State<AppState>) -> PriceMap {
    state.prices.read().unwrap().clone()
}

#[tauri::command]
pub fn fetch_all_items(state: State<AppState>) -> ItemRegistry {
    state.items.clone()
}

#[tauri::command]
pub fn calculate_crafting(state: State<'_, AppState>, unique_name: &str, location: CraftingLocation,use_focus: bool,usage_fee: u32,is_premium: bool) -> Option<CraftingResult> {
    let prices = state.prices.read().unwrap();
    let user = state.user.lock().unwrap();
    let item_registry = &state.items;

    let item_entity = item_registry.items.get(&unique_name.to_string())?;

    let context = CraftingContext {
        item: item_entity,
        prices: &prices,
        user_specs: &user.specializations,
        location,
        usage_fee,
        use_focus,
        is_premium,
    };

    crafting_calculations(&context)

}