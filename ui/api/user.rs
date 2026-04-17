use tauri_sys::core::invoke;
use aet_shared::models::user::UserData;
use aet_shared::models::config::{ActiveTab, AveragePrice, Cities};
use aet_shared::models::specializations::{CategoryId, SpecId};


pub async fn fetch_player_data() -> UserData {
    invoke::<UserData>("get_player_data", &()).await

}


pub async fn send_specs_update(spec_id: SpecId, level: u32) {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { updated_spec_id: SpecId, level: u32 }

    invoke::<()>("update_player_specs",
                 &Args { updated_spec_id: spec_id, level }).await;
}

pub async fn send_active_tab_update(new_tab: ActiveTab) {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { new_active_tab: ActiveTab }
    invoke::<()>("update_active_tab", &Args { new_active_tab: new_tab }).await;
}

pub async fn send_active_category_update(new_category: CategoryId) {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { new_active_category: CategoryId }
    invoke::<()>("update_active_category", &Args { new_active_category: new_category }).await;
}

pub async fn send_premium_update(premium_updated: bool) {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { new_premium: bool }
    invoke::<()>("update_premium", &Args { new_premium: premium_updated }).await;
}

pub async fn send_silver_fee_update(silver_fee_updated: u32) {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { new_silver_fee: u32 }
    invoke::<()>("update_silver_fee", &Args { new_silver_fee: silver_fee_updated }).await;
}

pub async fn send_focus_update(focus_updated: bool) {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { new_focus: bool }
    invoke::<()>("update_focus", &Args { new_focus: focus_updated }).await;
}
pub async fn send_city_update(city_updated: Cities) {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { new_city: Cities }
    invoke::<()>("update_city", &Args { new_city:  city_updated}).await;
}
pub async fn send_avg_update(avg_updated: AveragePrice) {
    #[derive(serde::Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Args { new_avg: AveragePrice }
    invoke::<()>("update_avg", &Args { new_avg: avg_updated }).await;
}
