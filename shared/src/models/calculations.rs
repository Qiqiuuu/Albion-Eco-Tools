use std::collections::HashMap;
use std::panic::Location;
use serde::{Deserialize, Serialize};
use crate::models::items::ItemEntity;
use crate::models::prices::ItemPrice;
use crate::models::specializations::Specializations;




pub struct CraftingContext<'a> {
    pub item: &'a ItemEntity,
    pub prices: &'a HashMap<String, ItemPrice>,
    pub location: &'a CraftingLocation,
    pub usage_fee: f64,
    pub use_focus: bool,
    pub is_premium: bool,
    pub user_specs: &'a [Specializations],
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CraftingResult {
    pub raw_item_cost: f64,
    pub actual_cost: f64,
    pub crafting_tax: f64,
    pub market_value: f64,
    pub market_tax_total: f64,
    pub net_profit: f64,
    pub profit_margin: f64,
    pub focus_efficiency: f64,
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
            (Self::RoyalCity, false) => 0.153,
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