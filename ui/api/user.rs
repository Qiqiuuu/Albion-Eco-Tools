use tauri_sys::core::invoke;
use aet_shared::models::user::UserData;
use aet_shared::models::config::ActiveTab;
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
