use std::sync::{Mutex, RwLock};
use aet_shared::models::items::ItemRegistry;
use aet_shared::models::prices::ItemPrice;
use aet_shared::models::user::UserData;

pub struct AppState {
    pub items: ItemRegistry,
    pub prices: RwLock<ItemPrice>,
    pub user: Mutex<UserData>,
}

