use std::sync::{Mutex, RwLock};
use aet_shared::models::items::ItemRegistry;
use aet_shared::models::prices::{PriceMap};
use aet_shared::models::user::UserData;

pub struct AppState {
    pub items: ItemRegistry,
    pub prices: RwLock<PriceMap>,
    pub user: Mutex<UserData>,
}

