use tauri_sys::core::invoke;
use aet_shared::models::items::{ItemRegistry};
use aet_shared::models::calculations::{CraftingLocation, CraftingResult};
use aet_shared::models::prices::{PriceMap};


pub async fn fetch_all_items() -> ItemRegistry{
    invoke::<ItemRegistry>("fetch_all_items", &()).await
}

pub async fn fetch_all_prices() -> PriceMap {
    invoke::<PriceMap>("fetch_all_prices", &()).await
}

pub async fn send_item_price_update(unique_name: String,new_price: u32){
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { unique_name: String ,
                  new_price: u32}
    invoke::<()>("update_item_price", &Args{unique_name,new_price}).await
}

pub async fn calculate_crafting(
    unique_name: &str,
    location: CraftingLocation,
    use_focus: bool,
    usage_fee: u32,
    is_premium: bool,
) -> Option<CraftingResult> {

    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args<'a> {
        unique_name: &'a str,
        location: CraftingLocation,
        use_focus: bool,
        usage_fee: u32,
        is_premium: bool,
    }


    invoke::<Option<CraftingResult>>(
        "calculate_crafting",
        &Args {
            unique_name,
            location,
            use_focus,
            usage_fee,
            is_premium,
        },
    ).await
}