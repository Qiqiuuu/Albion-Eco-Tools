
use tauri_sys::core::invoke;
use aet_shared::models::items::{Item, ItemEntity, ItemRegistry};
use chrono::{DateTime, Utc};
use aet_shared::models::calculations::{CraftingLocation, CraftingResult};
use aet_shared::models::prices::{ItemPrice, PriceMap};

pub async fn fetch_items_by_category(category: Item) {
    invoke::<(Option<DateTime<Utc>>, Vec<ItemEntity>)>(
        "fetch_items_by_category",
        &serde_json::json!({ "category": category })
    ).await;
}

pub async fn fetch_all_items() -> ItemRegistry{
    invoke::<ItemRegistry>("fetch_all_items", &()).await
}

pub async fn fetch_all_prices() -> PriceMap {
    invoke::<PriceMap>("fetch_all_prices", &()).await
}


pub async fn calculate_crafting(
    unique_name: &str,
    location: CraftingLocation,
    use_focus: bool,
    usage_fee: u32,
    is_premium: bool,
) -> Option<CraftingResult> {

    invoke::<Option<CraftingResult>>(
        "calculate_crafting",
        &serde_json::json!({
            "unique_name": unique_name,
            "location": location,
            "use_focus": use_focus,
            "usage_fee": usage_fee,
            "is_premium": is_premium
        })
    ).await
}