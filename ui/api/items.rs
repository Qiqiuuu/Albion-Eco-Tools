
use tauri_sys::core::invoke;
use aet_shared::models::items::{Item, ItemEntity, ItemRegistry};
use chrono::{DateTime, Utc};
use aet_shared::models::prices::{ItemPrice};

pub async fn fetch_items_by_category(category: Item) {
    invoke::<(Option<DateTime<Utc>>, Vec<ItemEntity>)>(
        "fetch_items_by_category",
        &serde_json::json!({ "category": category })
    ).await;
}

pub async fn fetch_all_items() -> ItemRegistry{
    invoke::<ItemRegistry>("fetch_all_items", &()).await
}

pub async fn fetch_all_prices() -> ItemPrice {
    invoke::<ItemPrice>("fetch_all_prices", &()).await
}
