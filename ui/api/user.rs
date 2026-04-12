use tauri_sys::core::invoke;
use aet_shared::models::user::UserData;
use serde::{Serialize};

#[derive(Serialize)]
struct LevelUpdateArgs {
    name: String,
    level: u32,
}

pub async fn fetch_player_specs() -> UserData {
    invoke::<UserData>("get_player_specs", &()).await

}

pub async fn send_level_update(name: String, level: u32) {
    let args = LevelUpdateArgs { name, level };
    invoke::<()>("update_player_specs", &args).await;
}