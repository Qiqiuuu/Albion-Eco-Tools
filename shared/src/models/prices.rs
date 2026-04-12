use std::collections::HashMap;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct CityPrice {
    pub sell_price_min: u64,
    pub buy_price_max: u64,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct ItemPrice {
    pub current: u64,
    pub cities: HashMap<String, CityPrice>,
}

pub type PriceMap = HashMap<String, ItemPrice>;