use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::specializations::{SpecKind, Specializations};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "type", content = "sub_type", rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Item {
    Artifact,
    CityResource,
    Consumable(Consumable),
    Crafting(Crafting),
    Equipment,
    Farmable,
    Furniture,
    LuxuryGood,
    Material,
    NonTradable,
    Other,
    Product(Product),
    Token,
    Tome,
    Trophy,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Crafting {
    RefinedResource,
    Resource,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Station {
    Butcher,
    Smelter,
    WarriorsForge,
    Weaver,
    Cook,
    HuntersLodge,
    Lumbermill,
    Tanner,
    Toolmaker,
    Stonemason,
    RepairStation,
    AlchemistLab,
    Mill,
    Saddler,
    Farm,
    HerbGarden,
    Pasture,
    Kennel,
    MagesTower,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Product {
    AnimalProduct,
    Crop,
    Herb,
    Meat,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Consumable {
    Food,
    Potion,
    Fish,
    TomeOfInsight,
    Map,
    BagOfSilver,
    Vanity,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Tier {
    #[serde(rename = "1")]
    T1,
    #[serde(rename = "2")]
    T2,
    #[serde(rename = "3")]
    T3,
    #[serde(rename = "4")]
    T4,
    #[serde(rename = "5")]
    T5,
    #[serde(rename = "6")]
    T6,
    #[serde(rename = "7")]
    T7,
    #[serde(rename = "8")]
    T8,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Enchantment {
    #[serde(rename = "0")]
    Common,
    #[serde(rename = "1")]
    Uncommon,
    #[serde(rename = "2")]
    Rare,
    #[serde(rename = "3")]
    Exceptional,
    #[serde(rename = "4")]
    Pristine,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum Quality {
    #[serde(rename = "1")]
    Normal,
    #[serde(rename = "2")]
    Good,
    #[serde(rename = "3")]
    Outstanding,
    #[serde(rename = "4")]
    Excellent,
    #[serde(rename = "5")]
    Masterpiece,
}

#[derive(Debug, Serialize, Deserialize, Clone,Default)]
pub struct ItemRegistry {
    pub items: HashMap<String, ItemEntity>,
    pub last_price_update: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct ItemEntity {
    pub unique_name: String,
    pub name: String,
    pub tier: Tier,
    pub enchantment: Enchantment,
    pub quality: Quality,
    pub value: u32,
    pub category: Item,
    pub recipes: Option<Vec<Recipe>>,
    pub station: Option<Station>,
    pub base_focus: Option<u32>,
    pub specialization: Option<SpecKind>,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Recipe {
    pub output_count: u8,
    pub ingredients: Vec<Ingredient>,
}

impl Recipe {
    pub fn new(output_count: u8, ingredients: Vec<Ingredient>) -> Self {
        Self { output_count, ingredients }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Ingredient {
    pub unique_name: String,
    pub count: u32,
}

impl ItemEntity {
    pub fn new(unique_name: &str, name: &str, tier: Tier, value: u32) -> Self {
        Self {
            unique_name: unique_name.to_string(),
            name: name.to_string(),
            tier,
            enchantment: Enchantment::Common,
            quality: Quality::Normal,
            value,
            base_focus: None,
            category: Item::Other,
            specialization: None,
            recipes: None,
            station: None,
        }
    }
    pub fn with_category(mut self, category: Item) -> Self {
        self.category = category;
        self
    }

    pub fn with_station(mut self, station: Station) -> Self {
        self.station = station.into();
        self
    }

    pub fn with_recipe(mut self, recipe: Recipe) -> Self {
        self.recipes = Some(vec![recipe]);
        self
    }

    pub fn with_specialization(mut self, spec: SpecKind) -> Self {
        self.specialization = Some(spec);
        self
    }

    pub fn with_base_focus(mut self, base_focus: u32) -> Self {
        self.base_focus = Some(base_focus);
        self
    }

    pub fn with_enchantment(mut self, enchantment: Enchantment) -> Self {
        self.enchantment = enchantment;
        self
    }
}
