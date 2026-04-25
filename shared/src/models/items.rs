use std::collections::HashMap;

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use crate::models::specializations::{SpecId};

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
impl Tier{
    pub fn badge(&self) -> (&'static str, &'static str) {
        match self {
            Tier::T1 => ("tier-badge t1", "T1"),
            Tier::T2 => ("tier-badge t2", "T2"),
            Tier::T3 => ("tier-badge t3", "T3"),
            Tier::T4 => ("tier-badge t4", "T4"),
            Tier::T5 => ("tier-badge t5", "T5"),
            Tier::T6 => ("tier-badge t6", "T6"),
            Tier::T7 => ("tier-badge t7", "T7"),
            Tier::T8 => ("tier-badge t8", "T8"),
        }
    }
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
impl Enchantment {
    pub fn to_u8(&self) -> u8 {
        match self {
            Enchantment::Common      => 0,
            Enchantment::Uncommon    => 1,
            Enchantment::Rare        => 2,
            Enchantment::Exceptional => 3,
            Enchantment::Pristine    => 4,
        }
    }

    pub fn from_u8(value: u8) -> Self {
        match value {
            1 => Enchantment::Uncommon,
            2 => Enchantment::Rare,
            3 => Enchantment::Exceptional,
            4 => Enchantment::Pristine,
            _ => Enchantment::Common,
        }
    }

    pub fn badge(&self) -> (&'static str, &'static str) {
        match self {
            Enchantment::Uncommon    => ("enchant-badge e1", ".1"),
            Enchantment::Rare        => ("enchant-badge e2", ".2"),
            Enchantment::Exceptional => ("enchant-badge e3", ".3"),
            Enchantment::Pristine    => ("enchant-badge e4", ".4"),
            Enchantment::Common      => ("", ""),
        }
    }
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

impl ItemRegistry {
    pub fn get_item_entity_by_name_and_enchant(&self, name: &str, enchantment: Enchantment) -> &ItemEntity {
        self.items
            .values()
            .find(|item| item.name == name && item.enchantment == enchantment)
            .unwrap()
    }
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
    pub specialization: Option<SpecId>,
}
impl ItemEntity {
    pub fn get_img(&self) -> String {
        format!("https://render.albiononline.com/v1/item/{}.png", &self.unique_name)
    }
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

impl Ingredient {
    pub fn new(unique_name: String, count: u32) -> Self {
        Self { unique_name, count }
    }
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct TrackedFood {
    pub item: ItemEntity,
    pub quantity: u32,
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

    pub fn with_specialization(mut self, spec: SpecId) -> Self {
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
