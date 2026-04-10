use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    Trophy
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Crafting {
    RefinedResource,
    Resource
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
    MagesTower
}



#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Product {
    AnimalProduct,
    Crop,
    Herb,
    Meat,
    None
}


#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum Consumable{
    Food,
    Potion,
    Fish,
    TomeOfInsight,
    Map,
    BagOfSilver,
    Vanity,
    None
}


#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Tier {
    #[serde(rename = "1")] T1,
    #[serde(rename = "2")] T2,
    #[serde(rename = "3")] T3,
    #[serde(rename = "4")] T4,
    #[serde(rename = "5")] T5,
    #[serde(rename = "6")] T6,
    #[serde(rename = "7")] T7,
    #[serde(rename = "8")] T8,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Enchantment {
    #[serde(rename = "0")] Common,
    #[serde(rename = "1")] Uncommon,
    #[serde(rename = "2")] Rare,
    #[serde(rename = "3")] Exceptional,
    #[serde(rename = "4")] Pristine,
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Quality {
    #[serde(rename = "1")] Normal,
    #[serde(rename = "2")] Good,
    #[serde(rename = "3")] Outstanding,
    #[serde(rename = "4")] Excellent,
    #[serde(rename = "5")] Masterpiece,
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemRegistry {
    pub items: HashMap<String, ItemEntity>,
    pub last_price_update: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ItemEntity {
    pub unique_name: String,
    pub name: String,
    pub tier: Tier,
    pub enchantment: Enchantment,
    pub quality: Quality,
    pub item_value: u32,
    pub item_category: Item,
    pub recipes: Option<Vec<Recipe>>,
    pub station: Option<Station>,
    pub prices: HashMap<String, CityPrice>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Recipe {
    pub output_count: u8,
    pub ingredients: Vec<Ingredient>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Ingredient {
    pub unique_name: String,
    pub count: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CityPrice {
    pub sell_price_min: u64,
    pub buy_price_max: u64,
    pub updated_at: DateTime<Utc>,
}



impl ItemEntity {
    fn new(
        id: &str,
        name: &str,
        tier: Tier,
        enchantment: Enchantment,
        category: Item,
        value: u32,
        recipe: Option<Recipe>,
        station: Option<Station>
    ) -> Self {
        Self {
            unique_name: id.to_string(),
            name: name.to_string(),
            tier,
            enchantment,
            quality: Quality::Normal,
            item_value: value,
            item_category: category,
            recipes: recipe.map(|r| vec![r]),
            station,
            prices: HashMap::new(),
        }
    }
}


fn ing(id: &str, count: u32) -> Ingredient {
    Ingredient { unique_name: id.to_string(), count }
}

fn food_enchants(
    base_id: &str,
    name: &str,
    tier: Tier,
    value: u32,
    output_count: u8,
    base_ingredients: Vec<Ingredient>,
    station: Station,
    sauce_counts: [u32; 3],
) -> Vec<ItemEntity> {
    let sauces = [
        ("T1_FISHSAUCE_LEVEL1", sauce_counts[0]),
        ("T1_FISHSAUCE_LEVEL2", sauce_counts[1]),
        ("T1_FISHSAUCE_LEVEL3", sauce_counts[2]),
    ];

    let mut variants = vec![
        ItemEntity::new(base_id, name, tier, Enchantment::Common, Item::Consumable(Consumable::Food), value, Some(Recipe { output_count, ingredients: base_ingredients.clone() }), Some(station.clone())),
    ];

    for i in 0..3 {
        let mut ingredients = base_ingredients.clone();
        ingredients.push(ing(sauces[i].0, sauces[i].1));
        let enchant = match i { 0 => Enchantment::Uncommon, 1 => Enchantment::Rare, _ => Enchantment::Exceptional };
        variants.push(ItemEntity::new(&format!("{}@{}", base_id, i+1), name, tier, enchant, Item::Consumable(Consumable::Food),value, Some(Recipe { output_count, ingredients }),Some(station.clone())));
    }
    variants
}

fn main() -> anyhow::Result<()> {
    let mut items: Vec<ItemEntity> = Vec::new();

    // --- Crops ---
    let farmables = vec![
        ("T1_CARROT", "Carrot", Tier::T1), ("T2_BEAN", "Bean", Tier::T2),
        ("T3_WHEAT", "Wheat", Tier::T3), ("T4_TURNIP", "Turnip", Tier::T4),
        ("T5_CABBAGE", "Cabbage", Tier::T5), ("T6_POTATO", "Potato", Tier::T6),
        ("T7_CORN", "Corn", Tier::T7), ("T8_PUMPKIN", "Pumpkin", Tier::T8),
    ];
    for (id, n, t) in farmables { items.push(ItemEntity::new(id, n, t, Enchantment::Common, Item::Product(Product::Crop), 40, None,None)); }

    let herbs = vec![
        ("T1_CARROT", "Carrot", Tier::T1),
        ("T2_AGARIC", "Arcane Agaric", Tier::T2),
        ("T3_COMFREY", "Brightleaf Comfrey", Tier::T3),
        ("T4_BURDOCK", "Crenellated Burdock", Tier::T4),
        ("T5_TEASEL", "Dragon Teasel", Tier::T5),
        ("T6_FOXGLOVE", "Elusive Foxglove", Tier::T6),
        ("T7_MULLEIN", "Firebrand Mullein", Tier::T7),
        ("T8_YARROW", "Ghastly Yarrow", Tier::T8),
    ];

    for (id, n, t) in herbs {
        items.push(ItemEntity::new(
            id,
            n,
            t,
            Enchantment::Common,
            Item::Product(Product::Herb),
            40,
            None,
            None
        ));
    }


    // --- Fish Freshwater---
    let fw_common = vec![
        ("T1_FISH_FRESHWATER_ALL_COMMON", "Common Rudd", Tier::T1, 1),
        ("T2_FISH_FRESHWATER_ALL_COMMON", "Striped Carp", Tier::T2, 2),
        ("T3_FISH_FRESHWATER_ALL_COMMON", "Albion Perch", Tier::T3, 3),
        ("T4_FISH_FRESHWATER_ALL_COMMON", "Bluescale Pike", Tier::T4, 4),
        ("T5_FISH_FRESHWATER_ALL_COMMON", "Spotted Trout", Tier::T5, 6),
        ("T6_FISH_FRESHWATER_ALL_COMMON", "Brightscale Zander", Tier::T6, 8),
        ("T7_FISH_FRESHWATER_ALL_COMMON", "Danglemouth Catfish", Tier::T7, 10),
        ("T8_FISH_FRESHWATER_ALL_COMMON", "River Sturgeon", Tier::T8, 14),
    ];

    // --- Fish Saltwater ---
    let sw_common = vec![
        ("T1_FISH_SALTWATER_ALL_COMMON", "Common Herring", Tier::T1, 1),
        ("T2_FISH_SALTWATER_ALL_COMMON", "Striped Mackerel", Tier::T2, 2),
        ("T3_FISH_SALTWATER_ALL_COMMON", "Flatshore Plaice", Tier::T3, 3),
        ("T4_FISH_SALTWATER_ALL_COMMON", "Bluescale Cod", Tier::T4, 4),
        ("T5_FISH_SALTWATER_ALL_COMMON", "Spotted Wolffish", Tier::T5, 6),
        ("T6_FISH_SALTWATER_ALL_COMMON", "Strongfin Salmon", Tier::T6, 8),
        ("T7_FISH_SALTWATER_ALL_COMMON", "Bluefin Tuna", Tier::T7, 10),
        ("T8_FISH_SALTWATER_ALL_COMMON", "Steelscale Swordfish", Tier::T8, 14),
    ];

    // --- Fish rare ---
    let rare_fish = vec![
        // Forest
        ("T3_FISH_FRESHWATER_FOREST_RARE", "Greenriver Eel", Tier::T3, 10),
        ("T5_FISH_FRESHWATER_FOREST_RARE", "Redspring Eel", Tier::T5, 20),
        ("T7_FISH_FRESHWATER_FOREST_RARE", "Deadwater Eel", Tier::T7, 30),
        // Mountain
        ("T3_FISH_FRESHWATER_MOUNTAIN_RARE", "Upland Coldeye", Tier::T3, 10),
        ("T5_FISH_FRESHWATER_MOUNTAIN_RARE", "Mountain Blindeye", Tier::T5, 20),
        ("T7_FISH_FRESHWATER_MOUNTAIN_RARE", "Frostpeak Deadeye", Tier::T7, 30),
        // Highlands
        ("T3_FISH_FRESHWATER_HIGHLANDS_RARE", "Stonestream Lurcher", Tier::T3, 10),
        ("T5_FISH_FRESHWATER_HIGHLANDS_RARE", "Rushwater Lurcher", Tier::T5, 20),
        ("T7_FISH_FRESHWATER_HIGHLANDS_RARE", "Thunderfall Lurcher", Tier::T7, 30),
        // Steppe
        ("T3_FISH_FRESHWATER_STEPPE_RARE", "Lowriver Crab", Tier::T3, 10),
        ("T5_FISH_FRESHWATER_STEPPE_RARE", "Drybrook Crab", Tier::T5, 20),
        ("T7_FISH_FRESHWATER_STEPPE_RARE", "Dusthole Crab", Tier::T7, 30),
        // Swamp
        ("T3_FISH_FRESHWATER_SWAMP_RARE", "Greenmoor Clam", Tier::T3, 10),
        ("T5_FISH_FRESHWATER_SWAMP_RARE", "Murkwater Clam", Tier::T5, 20),
        ("T7_FISH_FRESHWATER_SWAMP_RARE", "Blackbog Clam", Tier::T7, 30),
        // Saltwater Rare
        ("T3_FISH_SALTWATER_ALL_RARE", "Shallowshore Squid", Tier::T3, 10),
        ("T5_FISH_SALTWATER_ALL_RARE", "Midwater Octopus", Tier::T5, 20),
        ("T7_FISH_SALTWATER_ALL_RARE", "Deepwater Kraken", Tier::T7, 30),
        // Avalon
        ("T3_FISH_FRESHWATER_AVALON_RARE", "Whitefog Snapper", Tier::T3, 10),
        ("T5_FISH_FRESHWATER_AVALON_RARE", "Clearhaze Snapper", Tier::T5, 20),
        ("T7_FISH_FRESHWATER_AVALON_RARE", "Puremist Snapper", Tier::T7, 30),
    ];

    for (id, n, t, v) in fw_common { items.push(ItemEntity::new(id, n, t, Enchantment::Common,Item::Consumable(Consumable::Food), v, None,None)); }
    for (id, n, t, v) in sw_common { items.push(ItemEntity::new(id, n, t, Enchantment::Common,Item::Consumable(Consumable::Food), v, None,None)); }
    for (id, n, t, v) in rare_fish { items.push(ItemEntity::new(id, n, t, Enchantment::Common,Item::Consumable(Consumable::Food), v, None,None)); }

    // Shark & Seaweed
    items.push(ItemEntity::new("T8_FISH_SALTWATER_ALL_BOSS_SHARK", "Shark", Tier::T8, Enchantment::Common, Item::Consumable(Consumable::Fish), 200, None,None));
    items.push(ItemEntity::new("T1_SEAWEED", "Seaweed", Tier::T1, Enchantment::Common, Item::Material, 1, None,None));
    items.push(ItemEntity::new("T1_FISHCHOPS", "Chopped Fish", Tier::T1, Enchantment::Common, Item::Material, 1, None,None));

    // --- PRODUCTS ---
    items.push(ItemEntity::new("T4_BUTTER", "Goat Butter", Tier::T4, Enchantment::Common,Item::Consumable(Consumable::None),40, Some(Recipe { output_count: 1, ingredients: vec![ing("T4_MILK", 1)] }),Some(Station::Mill)));
    items.push(ItemEntity::new("T6_BUTTER", "Cow Butter", Tier::T6, Enchantment::Common, Item::Consumable(Consumable::None), 40, Option::from(Recipe { output_count: 1, ingredients: vec![ing("T6_MILK", 1)] }), Some(Station::Mill)));
    items.push(ItemEntity::new("T8_BUTTER", "Pork Butter", Tier::T8, Enchantment::Common, Item::Consumable(Consumable::None), 40, Option::from(Recipe { output_count: 1, ingredients: vec![ing("T8_MILK", 1)] }), Some(Station::Mill)));

    // --- FOODS ---
    // Soups
    items.extend(food_enchants("T1_MEAL_SOUP", "Carrot Soup", Tier::T1, 64, 10, vec![ing("T1_CARROT", 16)],Station::Cook, [10, 10, 10]));
    items.extend(food_enchants("T3_MEAL_SOUP", "Wheat Soup", Tier::T3, 128, 10, vec![ing("T3_WHEAT", 48)],Station::Cook, [30, 30, 30]));
    items.extend(food_enchants("T5_MEAL_SOUP", "Cabbage Soup", Tier::T5, 576, 10, vec![ing("T5_CABBAGE", 144)],Station::Cook, [90, 90, 90]));

    // Omelettes
    items.extend(food_enchants("T3_MEAL_OMELETTE", "Chicken Omelette", Tier::T3, 56, 10, vec![ing("T3_EGG", 2), ing("T3_MEAT", 8),ing("T3_WHEAT",4)],Station::Cook, [10, 10, 10]));
    items.extend(food_enchants("T5_MEAL_OMELETTE", "Goose Omelette", Tier::T5, 168, 10, vec![ing("T5_EGG", 6), ing("T5_MEAT", 24), ing("T5_CABBAGE", 12)],Station::Cook,[30, 30, 30]));
    items.extend(food_enchants("T7_MEAL_OMELETTE", "Pork Omelette", Tier::T7, 504, 10, vec![ing("T5_EGG", 18), ing("T7_MEAT", 72), ing("T7_CORN", 36)],Station::Cook,[90, 90, 90]));

    items.extend(food_enchants("T3_MEAL_OMELETTE_FISH", "Lowriver Crab Omelette", Tier::T3, 90, 1, vec![ing("T3_FISH_FRESHWATER_STEPPE_RARE", 1), ing("T3_EGG", 1), ing("T3_COMFREY", 1)], Station::Cook, [3, 3, 3]));
    items.extend(food_enchants("T5_MEAL_OMELETTE_FISH", "Drybrook Crab Omelette", Tier::T5, 260, 1, vec![ing("T5_FISH_FRESHWATER_STEPPE_RARE", 1), ing("T5_EGG", 2), ing("T5_CABBAGE", 2),ing("T5_TEASEL", 2)], Station::Cook, [9, 9, 9]));
    items.extend(food_enchants("T7_MEAL_OMELETTE_FISH", "Dusthole Crab Omelette", Tier::T7, 750, 1, vec![ing("T7_FISH_FRESHWATER_STEPPE_RARE", 1), ing("T7_CORN", 6), ing("T7_MULLEIN", 6),ing("T7_MEAT", 6)], Station::Cook, [27, 27, 27]));

    items.extend(food_enchants("T3_MEAL_OMELETTE_AVALON", "Avalonian Chicken Omelette", Tier::T3, 120, 10, vec![ing("T3_EGG", 2), ing("T3_MEAT", 8),ing("T4_MILK",4),ing("QUESTITEM_TOKEN_AVALON",10)], Station::Cook, [10, 10, 10]));
    items.extend(food_enchants("T5_MEAL_OMELETTE_AVALON", "Avalonian Goose Omelette", Tier::T5, 360, 10, vec![ing("T5_EGG", 6), ing("T5_MEAT", 24), ing("T6_MILK", 12),ing("QUESTITEM_TOKEN_AVALON",30)], Station::Cook, [30, 30, 30]));
    items.extend(food_enchants("T7_MEAL_OMELETTE_AVALON", "Avalonian Pork Omelette", Tier::T7, 1080, 10, vec![ing("T5_EGG", 18), ing("T7_MEAT", 72), ing("T8_MILK", 36),ing("QUESTITEM_TOKEN_AVALON",90)], Station::Cook, [90, 90, 90]));

    // Stews
    items.extend(food_enchants("T4_MEAL_STEW", "Goat Stew", Tier::T4, 64, 10, vec![ing("T4_MEAT", 8), ing("T4_TURNIP", 4), ing("T4_BREAD", 4)], Station::Cook,[10, 10, 10]));
    items.extend(food_enchants("T6_MEAL_STEW", "Mutton Stew", Tier::T6, 192, 10, vec![ing("T6_MEAT", 24), ing("T6_POTATO", 12), ing("T4_BREAD", 12)],Station::Cook, [30, 30, 30]));
    items.extend(food_enchants("T8_MEAL_STEW", "Beef Stew", Tier::T8, 576, 10, vec![ing("T8_MEAT", 72), ing("T8_PUMPKIN", 36), ing("T4_BREAD", 36)], Station::Cook,[90, 90, 90]));

    items.extend(food_enchants("T4_MEAL_STEW_FISH", "Greenriver Eel Stew", Tier::T4, 90, 1, vec![ing("T3_FISH_FRESHWATER_FOREST_RARE", 1), ing("T4_TURNIP", 1), ing("T4_BURDOCK", 1)], Station::Cook, [3, 3, 3]));
    items.extend(food_enchants("T6_MEAL_STEW_FISH", "Redspring Eel Stew", Tier::T6, 192, 1, vec![ing("T5_FISH_FRESHWATER_FOREST_RARE", 1), ing("T6_POTATO", 2), ing("T6_FOXGLOVE", 2), ing("T6_MILK", 2)], Station::Cook, [9, 9, 9]));
    items.extend(food_enchants("T8_MEAL_STEW_FISH", "Deadwater Eel Stew", Tier::T8, 750, 1, vec![ing("T7_FISH_FRESHWATER_FOREST_RARE", 1), ing("T8_PUMPKIN", 6), ing("T8_YARROW", 6), ing("T8_MILK", 6)], Station::Cook, [27, 27, 27]));

    items.extend(food_enchants("T4_MEAL_STEW_AVALON", "Avalonian Beef Stew", Tier::T4, 128, 10, vec![ing("T4_MEAT", 8), ing("T4_TURNIP", 4), ing("T1_CARROT", 4), ing("QUESTITEM_TOKEN_AVALON", 10)], Station::Cook, [10, 10, 10]));
    items.extend(food_enchants("T6_MEAL_STEW_AVALON", "Avalonian Mutton Stew", Tier::T6, 384, 10, vec![ing("T6_MEAT", 24), ing("T6_POTATO", 12), ing("T5_CABBAGE", 12), ing("QUESTITEM_TOKEN_AVALON", 30)], Station::Cook, [30, 30, 30]));
    items.extend(food_enchants("T8_MEAL_STEW_AVALON", "Avalonian Beef Stew", Tier::T8, 1152, 10, vec![ing("T8_MEAT", 72), ing("T8_PUMPKIN", 36), ing("T7_CORN", 36), ing("QUESTITEM_TOKEN_AVALON", 90)], Station::Cook, [90, 90, 90]));

    // Pies
    items.extend(food_enchants("T3_MEAL_PIE", "Chicken Pie", Tier::T3, 56, 10, vec![ing("T3_MEAT", 8), ing("T3_WHEAT", 2), ing("T3_FLOUR", 4)],Station::Cook, [10, 10, 10]));
    items.extend(food_enchants("T5_MEAL_PIE", "Goose Pie", Tier::T5, 192, 10, vec![ing("T5_MEAT", 24), ing("T5_CABBAGE", 6), ing("T3_FLOUR", 12),ing("T4_MILK", 6)],Station::Cook, [30, 30, 30]));
    items.extend(food_enchants("T7_MEAL_PIE", "Pork Pie", Tier::T7, 576, 10, vec![ing("T7_MEAT", 72), ing("T7_CORN", 18), ing("T3_FLOUR", 18),ing("T6_MILK", 18)], Station::Cook,[90, 90, 90]));

    items.extend(food_enchants("T3_MEAL_PIE_FISH", "Upland Coldeye Pie", Tier::T3, 90, 1, vec![ing("T3_FISH_FRESHWATER_MOUNTAIN_RARE", 1), ing("T3_EGG", 1), ing("T3_FLOUR", 1)], Station::Cook, [3, 3, 3]));
    items.extend(food_enchants("T5_MEAL_PIE_FISH", "Mountain Blindeye Pie", Tier::T5, 260, 1, vec![ing("T5_FISH_FRESHWATER_MOUNTAIN_RARE", 1), ing("T5_EGG", 2), ing("T5_CABBAGE", 2),ing("T5_TEASEL", 2)], Station::Cook, [9, 9, 9]));
    items.extend(food_enchants("T7_MEAL_PIE_FISH", "Frostpeak Deadeye Pie", Tier::T7, 750, 1, vec![ing("T7_FISH_FRESHWATER_MOUNTAIN_RARE", 1), ing("T7_MEAT", 6), ing("T7_CORN", 6),ing("T7_MULLEIN", 6)], Station::Cook, [27, 27, 27]));

    // --- ROASTS ---
    items.extend(food_enchants("T3_MEAL_ROAST", "Roast Chicken Roast", Tier::T3, 64, 10, vec![ing("T3_MEAT", 8), ing("T2_BEAN", 4),ing("T4_MILK", 4)], Station::Cook, [10, 10, 10]));
    items.extend(food_enchants("T5_MEAL_ROAST", "Roast Goose", Tier::T5, 192, 10, vec![ing("T5_MEAT", 24), ing("T5_CABBAGE", 12),ing("T6_MILK", 12)], Station::Cook, [30, 30, 30]));
    items.extend(food_enchants("T7_MEAL_ROAST", "Roast Pork", Tier::T7, 576, 10, vec![ing("T7_MEAT", 72), ing("T7_CORN", 36),ing("T8_MILK", 36)], Station::Cook, [90, 90, 90]));

    items.extend(food_enchants("T3_MEAL_ROAST_FISH", "Roasted Whitefog Snapper", Tier::T3, 90, 1, vec![ing("T3_FISH_FRESHWATER_AVALON_RARE", 1), ing("T3_COMFREY", 1),ing("T4_MILK", 1)], Station::Cook, [3, 3, 3]));
    items.extend(food_enchants("T5_MEAL_ROAST_FISH", "Roasted Clearhaze Snapper", Tier::T5, 260, 1, vec![ing("T5_FISH_FRESHWATER_AVALON_RARE", 1), ing("T5_MEAT", 24),ing("T5_CABBAGE", 2),ing("T5_TEASEL", 2),ing("T6_MILK", 2)], Station::Cook, [9, 9, 9]));
    items.extend(food_enchants("T7_MEAL_ROAST_FISH", "Roasted Puremist Snapper", Tier::T7, 750, 1, vec![ing("T7_FISH_FRESHWATER_AVALON_RARE", 1),ing("T7_CORN", 6),ing("T7_MULLEIN", 6),ing("T8_MILK", 6)], Station::Cook, [27, 27, 27]));

    // --- SANDWICHES (Kanapki) ---
    items.extend(food_enchants("T4_MEAL_SANDWICH", "Goat Sandwich", Tier::T4, 56, 10, vec![ing("T4_MEAT", 8), ing("T4_BREAD", 4), ing("T4_BUTTER", 2)], Station::Cook, [10, 10, 10]));
    items.extend(food_enchants("T6_MEAL_SANDWICH", "Mutton Sandwich", Tier::T6, 168, 10, vec![ing("T6_MEAT", 24), ing("T4_BREAD", 12), ing("T6_BUTTER", 6)], Station::Cook, [30, 30, 30]));
    items.extend(food_enchants("T8_MEAL_SANDWICH", "Beef Sandwich", Tier::T8, 504, 10, vec![ing("T8_MEAT", 72), ing("T4_BREAD", 36), ing("T8_BUTTER", 18)], Station::Cook, [90, 90, 90]));

    items.extend(food_enchants("T4_MEAL_SANDWICH_FISH", "Stonestream Lurcher Sandwich", Tier::T4, 90, 1, vec![ing("T3_FISH_FRESHWATER_HIGHLANDS_RARE", 1), ing("T4_TURNIP", 1), ing("T4_BUTTER", 1)], Station::Cook, [3, 3, 3]));
    items.extend(food_enchants("T6_MEAL_SANDWICH_FISH", "Rushwater Lurcher Sandwich", Tier::T6, 260, 1, vec![ing("T5_FISH_FRESHWATER_HIGHLANDS_RARE", 1), ing("T6_POTATO", 2), ing("T6_FOXGLOVE", 2),ing("T6_BUTTER", 2)], Station::Cook, [9, 9, 9]));
    items.extend(food_enchants("T8_MEAL_SANDWICH_FISH", "Thunderfall Lurcher Sandwich", Tier::T8, 750, 1, vec![ing("T7_FISH_FRESHWATER_HIGHLANDS_RARE", 1), ing("T8_PUMPKIN", 6), ing("T8_YARROW", 6),ing("T8_BUTTER", 6)], Station::Cook, [27, 27, 27]));

    items.extend(food_enchants("T4_MEAL_SANDWICH_AVALON", "Avalonian Goat Sandwich", Tier::T4, 120, 10, vec![ing("T4_MEAT", 8), ing("T4_BREAD", 4), ing("T4_BUTTER", 2), ing("QUESTITEM_TOKEN_AVALON", 10)], Station::Cook, [10, 10, 10]));
    items.extend(food_enchants("T6_MEAL_SANDWICH_AVALON", "Avalonian Mutton Sandwich", Tier::T6, 360, 10, vec![ing("T6_MEAT", 24), ing("T4_BREAD", 12), ing("T6_BUTTER", 6), ing("QUESTITEM_TOKEN_AVALON", 30)], Station::Cook, [30, 30, 30]));
    items.extend(food_enchants("T8_MEAL_SANDWICH_AVALON", "Avalonian Beef Sandwich", Tier::T8, 1080, 10, vec![ing("T8_MEAT", 72), ing("T4_BREAD", 36), ing("T8_BUTTER", 18), ing("QUESTITEM_TOKEN_AVALON", 90)], Station::Cook, [90, 90, 90]));

    // --- MISSING BASES (Bread, Flour) ---
    items.push(ItemEntity::new("T3_FLOUR", "Flour", Tier::T3, Enchantment::Common, Item::Material, 1, Some(Recipe { output_count: 1, ingredients: vec![ing("T3_WHEAT", 1)] }), Some(Station::Mill)));
    items.push(ItemEntity::new("T4_BREAD", "Bread", Tier::T4, Enchantment::Common, Item::Material, 1, Some(Recipe { output_count: 1, ingredients: vec![ing("T3_FLOUR", 1)] }), Some(Station::Cook)));


    // --- MILL ---
    let butters = vec![
        ("T4_BUTTER", "Goat Butter", Tier::T4, "T4_MILK"),
        ("T6_BUTTER", "Cow Butter", Tier::T6, "T6_MILK"),
        ("T8_BUTTER", "Sheep Butter", Tier::T8, "T8_MILK"),
    ];

    for (id, n, t, milk_id) in butters {
        items.push(ItemEntity::new(
            id, n, t, Enchantment::Common,
            Item::Consumable(Consumable::None),
            40,
            Some(Recipe { output_count: 1, ingredients: vec![ing(milk_id, 1)] }),
            Some(Station::Mill)
        ));
    }


    // --- AVALONIAN FOODS (Tokens) ---
    items.push(ItemEntity::new(
        "QUESTITEM_TOKEN_AVALON",
        "Avalonian Energy",
        Tier::T6,
        Enchantment::Common,
        Item::Crafting(Crafting::Resource),
        64,
        None,
        None
    ));


    let registry = ItemRegistry {
        items: items.into_iter().map(|i| (i.unique_name.clone(), i)).collect(),
        last_price_update: None,
    };

    let home = std::env::var("HOME").expect("Brak HOME");
    let path = PathBuf::from(home).join(".config/Albion Economy Tools/items.json");
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, serde_json::to_string_pretty(&registry)?)?;

    println!("Zapisano {} przedmiotów do {:?}", registry.items.len(), path);
    Ok(())
}