use serde::{Deserialize, Serialize};
use crate::models::items::ItemEntity;
use crate::models::prices::{PriceMap};
use crate::models::specializations::Category;

pub struct CraftingContext<'a> {
    pub item: &'a ItemEntity,
    pub amount: u32,
    pub prices: &'a PriceMap,
    pub user_specs: &'a [Category],
    pub location: CraftingLocation,
    pub usage_fee: u32,
    pub use_focus: bool,
    pub is_premium: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingResult {
    pub raw_item_cost: f64,
    pub profit: f64,
    pub profit_margin: f64,
    pub focus_cost: f64,
    pub silver_per_focus: f64,
    pub tax: f64,
    pub station_tax: f64
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum CraftingLocation {
    RoyalCity,
    RoyalCityWithBonus,
    Island,
    IslandWithBonus,
}

impl CraftingLocation {
    pub fn get_rr(&self, use_focus: bool) -> f64 {
        match (self, use_focus) {
            (Self::RoyalCity, false) => 0.152,
            (Self::RoyalCity, true) => 0.435,
            (Self::RoyalCityWithBonus, false) => 0.248,
            (Self::RoyalCityWithBonus, true) => 0.479,
            (Self::Island, false) => 0.13,
            (Self::Island, true) => 0.425,
            _ => 0.0,
        }
    }
}
pub enum Tax{
    TaxRate,
    Setup
}