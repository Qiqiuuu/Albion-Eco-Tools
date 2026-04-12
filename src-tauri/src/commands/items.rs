use std::fs;
use chrono::{DateTime, Utc};
use tauri::State;
use aet_shared::models::items::{Item, ItemEntity, ItemRegistry};
use aet_shared::models::prices::ItemPrice;
use crate::loader::load_prices;
use crate::state::AppState;

#[tauri::command]
pub fn fetch_items_by_category(state: State<AppState>,category: Item) -> (Option<DateTime<Utc>>, Vec<ItemEntity>) {
    let filtered = state.items.items.values()
        .filter(|entity| entity.category == category)
        .cloned()
        .collect();
    (state.items.last_price_update,filtered)
}

#[tauri::command]
pub fn refresh_prices(state: State<AppState>) -> ItemPrice {
    let fresh = load_prices();
    let mut prices = state.prices.write().unwrap();
    *prices = fresh.clone();
    fresh
}

#[tauri::command]
pub fn fetch_all_prices(state: State<AppState>) -> ItemPrice {
    state.prices.read().unwrap().clone()
}

pub fn fetch_all_items(state: State<AppState>) -> ItemRegistry {
    state.items.clone()
}