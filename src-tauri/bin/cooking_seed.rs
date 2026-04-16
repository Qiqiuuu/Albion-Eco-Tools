
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::Utc;
use aet_shared::models::items::{
    Enchantment, Ingredient, Item, ItemEntity, ItemRegistry,
    Quality, Recipe, Station, Tier, Consumable, Crafting, Product
};
use aet_shared::models::prices::CityPrice;
use aet_shared::models::specializations::{SpecId};
use aet_shared::models::user::UserData;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FocusCost {
    pub base: u32,
    pub uncommon: u32,
    pub rare: u32,
    pub exceptional: u32,
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
    spec: SpecId,
    station: Station,
    sauce_counts: [u32; 3],
    focus_costs: [u32; 4],
) -> Vec<ItemEntity> {
    let sauces = [
        ("T1_FISHSAUCE_LEVEL1", sauce_counts[0]),
        ("T1_FISHSAUCE_LEVEL2", sauce_counts[1]),
        ("T1_FISHSAUCE_LEVEL3", sauce_counts[2]),
    ];
    let enchants = [
        Enchantment::Common,
        Enchantment::Uncommon,
        Enchantment::Rare,
        Enchantment::Exceptional,
    ];

    let mut variants = vec![
        ItemEntity::new(base_id, name, tier, value)
            .with_category(Item::Consumable(Consumable::Food))
            .with_station(station.clone())
            .with_recipe(Recipe::new(output_count, base_ingredients.clone()))
            .with_specialization(spec.clone())
            .with_base_focus(focus_costs[0]),
    ];

    for i in 0..3 {
        let mut ingredients = base_ingredients.clone();
        ingredients.push(ing(sauces[i].0, sauces[i].1));

        let id = format!("{}@{}", base_id, i + 1);
        variants.push(
            ItemEntity::new(&id, name, tier, value)
                .with_category(Item::Consumable(Consumable::Food))
                .with_enchantment(enchants[i + 1].clone())
                .with_station(station.clone())
                .with_recipe(Recipe::new(output_count, ingredients))
                .with_specialization(spec.clone())
                .with_base_focus(focus_costs[i + 1]),
        );
    }
    variants
}

fn main() -> anyhow::Result<()> {
    let mut items: Vec<ItemEntity> = Vec::new();
    // --- Crops ---
    for (id, n, t) in [
        ("T1_CARROT", "Carrot", Tier::T1), ("T2_BEAN", "Bean", Tier::T2),
        ("T3_WHEAT", "Wheat", Tier::T3),   ("T4_TURNIP", "Turnip", Tier::T4),
        ("T5_CABBAGE", "Cabbage", Tier::T5),("T6_POTATO", "Potato", Tier::T6),
        ("T7_CORN", "Corn", Tier::T7),     ("T8_PUMPKIN", "Pumpkin", Tier::T8),
    ] {
        items.push(ItemEntity::new(id, n, t, 40).with_category(Item::Product(Product::Crop)));
    }

    for (id, n, t) in [
        ("T2_AGARIC",  "Arcane Agaric",       Tier::T2),
        ("T3_COMFREY", "Brightleaf Comfrey",   Tier::T3),
        ("T4_BURDOCK", "Crenellated Burdock",  Tier::T4),
        ("T5_TEASEL",  "Dragon Teasel",        Tier::T5),
        ("T6_FOXGLOVE","Elusive Foxglove",      Tier::T6),
        ("T7_MULLEIN", "Firebrand Mullein",     Tier::T7),
        ("T8_YARROW",  "Ghastly Yarrow",        Tier::T8),
    ] {
        items.push(ItemEntity::new(id, n, t, 40).with_category(Item::Product(Product::Herb)));
    }


    // --- Fish Freshwater Common ---
    for (id, n, t, v) in [
        ("T1_FISH_FRESHWATER_ALL_COMMON", "Common Rudd",        Tier::T1, 1),
        ("T2_FISH_FRESHWATER_ALL_COMMON", "Striped Carp",       Tier::T2, 2),
        ("T3_FISH_FRESHWATER_ALL_COMMON", "Albion Perch",       Tier::T3, 3),
        ("T4_FISH_FRESHWATER_ALL_COMMON", "Bluescale Pike",     Tier::T4, 4),
        ("T5_FISH_FRESHWATER_ALL_COMMON", "Spotted Trout",      Tier::T5, 6),
        ("T6_FISH_FRESHWATER_ALL_COMMON", "Brightscale Zander", Tier::T6, 8),
        ("T7_FISH_FRESHWATER_ALL_COMMON", "Danglemouth Catfish",Tier::T7, 10),
        ("T8_FISH_FRESHWATER_ALL_COMMON", "River Sturgeon",     Tier::T8, 14),
    ] {
        items.push(ItemEntity::new(id, n, t, v).with_category(Item::Consumable(Consumable::Fish)));
    }

    // --- Fish Saltwater Common ---
    for (id, n, t, v) in [
        ("T1_FISH_SALTWATER_ALL_COMMON", "Common Herring",       Tier::T1, 1),
        ("T2_FISH_SALTWATER_ALL_COMMON", "Striped Mackerel",     Tier::T2, 2),
        ("T3_FISH_SALTWATER_ALL_COMMON", "Flatshore Plaice",     Tier::T3, 3),
        ("T4_FISH_SALTWATER_ALL_COMMON", "Bluescale Cod",        Tier::T4, 4),
        ("T5_FISH_SALTWATER_ALL_COMMON", "Spotted Wolffish",     Tier::T5, 6),
        ("T6_FISH_SALTWATER_ALL_COMMON", "Strongfin Salmon",     Tier::T6, 8),
        ("T7_FISH_SALTWATER_ALL_COMMON", "Bluefin Tuna",         Tier::T7, 10),
        ("T8_FISH_SALTWATER_ALL_COMMON", "Steelscale Swordfish", Tier::T8, 14),
    ] {
        items.push(ItemEntity::new(id, n, t, v).with_category(Item::Consumable(Consumable::Fish)));
    }

    // --- Fish Rare ---
    for (id, n, t, v) in [
        ("T3_FISH_FRESHWATER_FOREST_RARE",    "Greenriver Eel",       Tier::T3, 10),
        ("T5_FISH_FRESHWATER_FOREST_RARE",    "Redspring Eel",        Tier::T5, 20),
        ("T7_FISH_FRESHWATER_FOREST_RARE",    "Deadwater Eel",        Tier::T7, 30),
        ("T3_FISH_FRESHWATER_MOUNTAIN_RARE",  "Upland Coldeye",       Tier::T3, 10),
        ("T5_FISH_FRESHWATER_MOUNTAIN_RARE",  "Mountain Blindeye",    Tier::T5, 20),
        ("T7_FISH_FRESHWATER_MOUNTAIN_RARE",  "Frostpeak Deadeye",    Tier::T7, 30),
        ("T3_FISH_FRESHWATER_HIGHLANDS_RARE", "Stonestream Lurcher",  Tier::T3, 10),
        ("T5_FISH_FRESHWATER_HIGHLANDS_RARE", "Rushwater Lurcher",    Tier::T5, 20),
        ("T7_FISH_FRESHWATER_HIGHLANDS_RARE", "Thunderfall Lurcher",  Tier::T7, 30),
        ("T3_FISH_FRESHWATER_STEPPE_RARE",    "Lowriver Crab",        Tier::T3, 10),
        ("T5_FISH_FRESHWATER_STEPPE_RARE",    "Drybrook Crab",        Tier::T5, 20),
        ("T7_FISH_FRESHWATER_STEPPE_RARE",    "Dusthole Crab",        Tier::T7, 30),
        ("T3_FISH_FRESHWATER_SWAMP_RARE",     "Greenmoor Clam",       Tier::T3, 10),
        ("T5_FISH_FRESHWATER_SWAMP_RARE",     "Murkwater Clam",       Tier::T5, 20),
        ("T7_FISH_FRESHWATER_SWAMP_RARE",     "Blackbog Clam",        Tier::T7, 30),
        ("T3_FISH_SALTWATER_ALL_RARE",        "Shallowshore Squid",   Tier::T3, 10),
        ("T5_FISH_SALTWATER_ALL_RARE",        "Midwater Octopus",     Tier::T5, 20),
        ("T7_FISH_SALTWATER_ALL_RARE",        "Deepwater Kraken",     Tier::T7, 30),
        ("T3_FISH_FRESHWATER_AVALON_RARE",    "Whitefog Snapper",     Tier::T3, 10),
        ("T5_FISH_FRESHWATER_AVALON_RARE",    "Clearhaze Snapper",    Tier::T5, 20),
        ("T7_FISH_FRESHWATER_AVALON_RARE",    "Puremist Snapper",     Tier::T7, 30),
    ] {
        items.push(ItemEntity::new(id, n, t, v).with_category(Item::Consumable(Consumable::Fish)));
    }

    // --- Misc ---
    items.push(ItemEntity::new("T8_FISH_SALTWATER_ALL_BOSS_SHARK", "Shark", Tier::T8, 200)
        .with_category(Item::Consumable(Consumable::Fish)));

    items.push(ItemEntity::new("T1_SEAWEED",   "Seaweed",      Tier::T1, 1)
        .with_category(Item::Material));

    items.push(ItemEntity::new("T1_FISHCHOPS", "Chopped Fish", Tier::T1, 1)
        .with_category(Item::Material));

    items.push(ItemEntity::new("T3_FLOUR",     "Flour",        Tier::T3, 1)
        .with_category(Item::Material)
        .with_base_focus(38)
        .with_recipe(Recipe::new(1, vec![ing("T3_WHEAT", 1)]))
        .with_station(Station::Mill));

    items.push(ItemEntity::new("T4_BREAD", "Bread", Tier::T4, 1)
        .with_category(Item::Material)
        .with_recipe(Recipe::new(1, vec![ing("T3_FLOUR", 1)]))
        .with_station(Station::Cook)
        .with_base_focus(42));

    items.push(ItemEntity::new("QUESTITEM_TOKEN_AVALON", "Avalonian Energy", Tier::T6, 64)
        .with_category(Item::Crafting(Crafting::Resource)));

    // --- Butters ---
    for (id, n, t, milk) in [
        ("T4_BUTTER", "Goat Butter",  Tier::T4, "T4_MILK"),
        ("T6_BUTTER", "Cow Butter",   Tier::T6, "T6_MILK"),
        ("T8_BUTTER", "Sheep Butter", Tier::T8, "T8_MILK"),
    ] {
        items.push(ItemEntity::new(id, n, t, 40)
            .with_category(Item::Material)
            .with_recipe(Recipe::new(1, vec![ing(milk, 1)]))
            .with_station(Station::Mill)
            .with_specialization(SpecId::IngredientChef));
    }

    // --- Fish Sauce ---
    for (id, name, fish, seaweed) in [
        ("T1_FISHSAUCE_LEVEL1", "Basic Fish Sauce",   15, 1),
        ("T1_FISHSAUCE_LEVEL2", "Fancy Fish Sauce",   45, 3),
        ("T1_FISHSAUCE_LEVEL3", "Special Fish Sauce", 135, 9),
    ] {
        items.push(ItemEntity::new(id, name, Tier::T1, 0)
            .with_category(Item::Material)
            .with_recipe(Recipe::new(1, vec![ing("T1_FISHCHOPS", fish), ing("T1_SEAWEED", seaweed)]))
            .with_station(Station::Cook)
            .with_specialization(SpecId::IngredientChef));
    }

    // --- Soups ---
    items.extend(food_enchants("T1_MEAL_SOUP", "Carrot Soup", Tier::T1, 64, 10,
                               vec![ing("T1_CARROT", 16)],
                               SpecId::SoupChef, Station::Cook, [10, 10, 10], [56,78,123,256]));
    items.extend(food_enchants("T3_MEAL_SOUP", "Wheat Soup", Tier::T3, 128, 10,
                               vec![ing("T3_WHEAT", 48)],
                               SpecId::SoupChef, Station::Cook, [30, 30, 30], [168,235,368,769]));
    items.extend(food_enchants("T5_MEAL_SOUP", "Cabbage Soup", Tier::T5, 576, 10,
                               vec![ing("T5_CABBAGE", 144)],
                               SpecId::SoupChef, Station::Cook, [90, 90, 90], [504,704,1105,2306]));

    // Omelettes
    items.extend(food_enchants("T3_MEAL_OMELETTE", "Chicken Omelette", Tier::T3, 56, 10,
                               vec![ing("T3_EGG", 2), ing("T3_MEAT", 8), ing("T3_WHEAT", 4)],
                               SpecId::OmeletteChef, Station::Cook, [10, 10, 10], [52,74,118,252]));
    items.extend(food_enchants("T5_MEAL_OMELETTE", "Goose Omelette", Tier::T5, 168, 10,
                               vec![ing("T5_EGG", 6), ing("T5_MEAT", 24), ing("T5_CABBAGE", 12)],
                               SpecId::OmeletteChef, Station::Cook, [30, 30, 30], [155,222,355,755]));
    items.extend(food_enchants("T7_MEAL_OMELETTE", "Pork Omelette", Tier::T7, 504, 10,
                               vec![ing("T5_EGG", 18), ing("T7_MEAT", 72), ing("T7_CORN", 36)],
                               SpecId::OmeletteChef, Station::Cook, [90, 90, 90], [464,665,1065,2266]));

    items.extend(food_enchants("T3_MEAL_OMELETTE_FISH", "Lowriver Crab Omelette", Tier::T3, 90, 1,
                               vec![ing("T3_FISH_FRESHWATER_STEPPE_RARE", 1), ing("T3_EGG", 1), ing("T3_COMFREY", 1)],
                               SpecId::OmeletteChef, Station::Cook, [3, 3, 3], [77,144,278,678]));
    items.extend(food_enchants("T5_MEAL_OMELETTE_FISH", "Drybrook Crab Omelette", Tier::T5, 260, 1,
                               vec![ing("T5_FISH_FRESHWATER_STEPPE_RARE", 1), ing("T5_EGG", 2), ing("T5_CABBAGE", 2), ing("T5_TEASEL", 2)],
                               SpecId::OmeletteChef, Station::Cook, [9, 9, 9], [225,425,825,2026]));
    items.extend(food_enchants("T7_MEAL_OMELETTE_FISH", "Dusthole Crab Omelette", Tier::T7, 750, 1,
                               vec![ing("T7_FISH_FRESHWATER_STEPPE_RARE", 1), ing("T7_CORN", 6), ing("T7_MULLEIN", 6), ing("T7_MEAT", 6)],
                               SpecId::OmeletteChef, Station::Cook, [27, 27, 27], [672,1272,2473,6076]));

    items.extend(food_enchants("T3_MEAL_OMELETTE_AVALON", "Avalonian Chicken Omelette", Tier::T3, 120, 10,
                               vec![ing("T3_EGG", 2), ing("T3_MEAT", 8), ing("T4_MILK", 4), ing("QUESTITEM_TOKEN_AVALON", 10)],
                               SpecId::OmeletteChef, Station::Cook, [10, 10, 10], [52,74,118,252]));
    items.extend(food_enchants("T5_MEAL_OMELETTE_AVALON", "Avalonian Goose Omelette", Tier::T5, 360, 10,
                               vec![ing("T5_EGG", 6), ing("T5_MEAT", 24), ing("T6_MILK", 12), ing("QUESTITEM_TOKEN_AVALON", 30)],
                               SpecId::OmeletteChef, Station::Cook, [30, 30, 30], [155,222,355,755]));
    items.extend(food_enchants("T7_MEAL_OMELETTE_AVALON", "Avalonian Pork Omelette", Tier::T7, 1080, 10,
                               vec![ing("T5_EGG", 18), ing("T7_MEAT", 72), ing("T8_MILK", 36), ing("QUESTITEM_TOKEN_AVALON", 90)],
                               SpecId::OmeletteChef, Station::Cook, [90, 90, 90], [464,665,1065,2266]));

    // --- Stews ---
    items.extend(food_enchants("T4_MEAL_STEW", "Goat Stew", Tier::T4, 64, 10,
                               vec![ing("T4_MEAT", 8), ing("T4_TURNIP", 4), ing("T4_BREAD", 4)],
                               SpecId::StewChef, Station::Cook, [10, 10, 10], [61,84,128,262]));
    items.extend(food_enchants("T6_MEAL_STEW", "Mutton Stew", Tier::T6, 192, 10,
                               vec![ing("T6_MEAT", 24), ing("T6_POTATO", 12), ing("T4_BREAD", 12)],
                               SpecId::StewChef, Station::Cook, [30, 30, 30], [184,251,384,785]));
    items.extend(food_enchants("T8_MEAL_STEW", "Beef Stew", Tier::T8, 576, 10,
                               vec![ing("T8_MEAT", 72), ing("T8_PUMPKIN", 36), ing("T4_BREAD", 36)],
                               SpecId::StewChef, Station::Cook, [90, 90, 90], [551,752,1152,2353]));

    items.extend(food_enchants("T4_MEAL_STEW_FISH", "Greenriver Eel Stew", Tier::T4, 90, 1,
                               vec![ing("T3_FISH_FRESHWATER_FOREST_RARE", 1), ing("T4_TURNIP", 1), ing("T4_BURDOCK", 1)],
                               SpecId::StewChef, Station::Cook, [3, 3, 3], [77,144,278,678]));
    items.extend(food_enchants("T6_MEAL_STEW_FISH", "Redspring Eel Stew", Tier::T6, 192, 1,
                               vec![ing("T5_FISH_FRESHWATER_FOREST_RARE", 1), ing("T6_POTATO", 2), ing("T6_FOXGLOVE", 2), ing("T6_MILK", 2)],
                               SpecId::StewChef, Station::Cook, [9, 9, 9], [225,425,825,2026]));
    items.extend(food_enchants("T8_MEAL_STEW_FISH", "Deadwater Eel Stew", Tier::T8, 750, 1,
                               vec![ing("T7_FISH_FRESHWATER_FOREST_RARE", 1), ing("T8_PUMPKIN", 6), ing("T8_YARROW", 6), ing("T8_MILK", 6)],
                               SpecId::StewChef, Station::Cook, [27, 27, 27], [652,1253,2454,6056]));

    items.extend(food_enchants("T4_MEAL_STEW_AVALON", "Avalonian Beef Stew", Tier::T4, 128, 10,
                               vec![ing("T4_MEAT", 8), ing("T4_TURNIP", 4), ing("T1_CARROT", 4), ing("QUESTITEM_TOKEN_AVALON", 10)],
                               SpecId::StewChef, Station::Cook, [10, 10, 10], [58,81,125,259]));
    items.extend(food_enchants("T6_MEAL_STEW_AVALON", "Avalonian Mutton Stew", Tier::T6, 384, 10,
                               vec![ing("T6_MEAT", 24), ing("T6_POTATO", 12), ing("T5_CABBAGE", 12), ing("QUESTITEM_TOKEN_AVALON", 30)],
                               SpecId::StewChef, Station::Cook, [30, 30, 30], [176,243,376,777]));
    items.extend(food_enchants("T8_MEAL_STEW_AVALON", "Avalonian Beef Stew", Tier::T8, 1152, 10,
                               vec![ing("T8_MEAT", 72), ing("T8_PUMPKIN", 36), ing("T7_CORN", 36), ing("QUESTITEM_TOKEN_AVALON", 90)],
                               SpecId::StewChef, Station::Cook, [90, 90, 90], [528,728,1128,2329]));

    // --- Pies ---
    items.extend(food_enchants("T3_MEAL_PIE", "Chicken Pie", Tier::T3, 56, 10,
                               vec![ing("T3_MEAT", 8), ing("T3_WHEAT", 2), ing("T3_FLOUR", 4)],
                               SpecId::PieChef, Station::Cook, [10, 10, 10], [53,75,120,253]));
    items.extend(food_enchants("T5_MEAL_PIE", "Goose Pie", Tier::T5, 192, 10,
                               vec![ing("T5_MEAT", 24), ing("T5_CABBAGE", 6), ing("T3_FLOUR", 12), ing("T4_MILK", 6)],
                               SpecId::PieChef, Station::Cook, [30, 30, 30], [180,246,380,780]));
    items.extend(food_enchants("T7_MEAL_PIE", "Pork Pie", Tier::T7, 576, 10,
                               vec![ing("T7_MEAT", 72), ing("T7_CORN", 18), ing("T3_FLOUR", 18), ing("T6_MILK", 18)],
                               SpecId::PieChef, Station::Cook, [90, 90, 90], [540,739,1140,2341]));

    items.extend(food_enchants("T3_MEAL_PIE_FISH", "Upland Coldeye Pie", Tier::T3, 90, 1,
                               vec![ing("T3_FISH_FRESHWATER_MOUNTAIN_RARE", 1), ing("T3_EGG", 1), ing("T3_FLOUR", 1)],
                               SpecId::PieChef, Station::Cook, [3, 3, 3], [81,147,281,681]));
    items.extend(food_enchants("T5_MEAL_PIE_FISH", "Mountain Blindeye Pie", Tier::T5, 260, 1,
                               vec![ing("T5_FISH_FRESHWATER_MOUNTAIN_RARE", 1), ing("T5_EGG", 2), ing("T5_CABBAGE", 2), ing("T5_TEASEL", 2)],
                               SpecId::PieChef, Station::Cook, [9, 9, 9], [225,425,825,2026]));
    items.extend(food_enchants("T7_MEAL_PIE_FISH", "Frostpeak Deadeye Pie", Tier::T7, 750, 1,
                               vec![ing("T7_FISH_FRESHWATER_MOUNTAIN_RARE", 1), ing("T7_MEAT", 6), ing("T7_CORN", 6), ing("T7_MULLEIN", 6)],
                               SpecId::PieChef, Station::Cook, [27, 27, 27], [672,1272,2473,6076]));

    // --- Roasts ---
    items.extend(food_enchants("T3_MEAL_ROAST", "Roast Chicken", Tier::T3, 64, 10,
                               vec![ing("T3_MEAT", 8), ing("T2_BEAN", 4), ing("T4_MILK", 4)],
                               SpecId::RoastChef, Station::Cook, [10, 10, 10], [58,81,125,259]));
    items.extend(food_enchants("T5_MEAL_ROAST", "Roast Goose", Tier::T5, 192, 10,
                               vec![ing("T5_MEAT", 24), ing("T5_CABBAGE", 12), ing("T6_MILK", 12)],
                               SpecId::RoastChef, Station::Cook, [30, 30, 30], [176,243,376,777]));
    items.extend(food_enchants("T7_MEAL_ROAST", "Roast Pork", Tier::T7, 576, 10,
                               vec![ing("T7_MEAT", 72), ing("T7_CORN", 36), ing("T8_MILK", 36)],
                               SpecId::RoastChef, Station::Cook, [90, 90, 90], [528,728,1128,2329]));

    items.extend(food_enchants("T3_MEAL_ROAST_FISH", "Roasted Whitefog Snapper", Tier::T3, 90, 1,
                               vec![ing("T3_FISH_FRESHWATER_AVALON_RARE", 1), ing("T3_COMFREY", 1), ing("T4_MILK", 1)],
                               SpecId::RoastChef, Station::Cook, [3, 3, 3], [77,144,278,678]));
    items.extend(food_enchants("T5_MEAL_ROAST_FISH", "Roasted Clearhaze Snapper", Tier::T5, 260, 1,
                               vec![ing("T5_FISH_FRESHWATER_AVALON_RARE", 1), ing("T5_MEAT", 2), ing("T5_CABBAGE", 2), ing("T5_TEASEL", 2), ing("T6_MILK", 2)],
                               SpecId::RoastChef, Station::Cook, [9, 9, 9], [225,425,825,2026]));
    items.extend(food_enchants("T7_MEAL_ROAST_FISH", "Roasted Puremist Snapper", Tier::T7, 750, 1,
                               vec![ing("T7_FISH_FRESHWATER_AVALON_RARE", 1), ing("T7_CORN", 6), ing("T7_MULLEIN", 6), ing("T8_MILK", 6)],
                               SpecId::RoastChef, Station::Cook, [27, 27, 27], [652,1253,2454,6056]));


    // --- SANDWICHES  ---

    items.extend(food_enchants("T4_MEAL_SANDWICH", "Goat Sandwich", Tier::T4, 56, 10,
                               vec![ing("T4_MEAT", 8), ing("T4_BREAD", 4), ing("T4_BUTTER", 2)],
                               SpecId::SandwichChef, Station::Cook, [10, 10, 10], [55,77,122,255]));
    items.extend(food_enchants("T6_MEAL_SANDWICH", "Mutton Sandwich", Tier::T6, 168, 10,
                               vec![ing("T6_MEAT", 24), ing("T4_BREAD", 12), ing("T6_BUTTER", 6)],
                               SpecId::SandwichChef, Station::Cook, [30, 30, 30], [165,231,365,765]));
    items.extend(food_enchants("T8_MEAL_SANDWICH", "Beef Sandwich", Tier::T8, 504, 10,
                               vec![ing("T8_MEAT", 72), ing("T4_BREAD", 36), ing("T8_BUTTER", 18)],
                               SpecId::SandwichChef, Station::Cook, [90, 90, 90], [494,694,1094,2295]));

    items.extend(food_enchants("T4_MEAL_SANDWICH_FISH", "Stonestream Lurcher Sandwich", Tier::T4, 90, 1,
                               vec![ing("T3_FISH_FRESHWATER_HIGHLANDS_RARE", 1), ing("T4_TURNIP", 1), ing("T4_BUTTER", 1)],
                               SpecId::SandwichChef, Station::Cook, [3, 3, 3], [81,147,281,681]));
    items.extend(food_enchants("T6_MEAL_SANDWICH_FISH", "Rushwater Lurcher Sandwich", Tier::T6, 260, 1,
                               vec![ing("T5_FISH_FRESHWATER_HIGHLANDS_RARE", 1), ing("T6_POTATO", 2), ing("T6_FOXGLOVE", 2), ing("T6_BUTTER", 2)],
                               SpecId::SandwichChef, Station::Cook, [9, 9, 9], [231,432,832,2033]));
    items.extend(food_enchants("T8_MEAL_SANDWICH_FISH", "Thunderfall Lurcher Sandwich", Tier::T8, 750, 1,
                               vec![ing("T7_FISH_FRESHWATER_HIGHLANDS_RARE", 1), ing("T8_PUMPKIN", 6), ing("T8_YARROW", 6), ing("T8_BUTTER", 6)],
                               SpecId::SandwichChef, Station::Cook, [27, 27, 27], [672,1272,2473,6076]));

    items.extend(food_enchants("T4_MEAL_SANDWICH_AVALON", "Avalonian Goat Sandwich", Tier::T4, 120, 10,
                               vec![ing("T4_MEAT", 8), ing("T4_BREAD", 4), ing("T4_BUTTER", 2), ing("QUESTITEM_TOKEN_AVALON", 10)],
                               SpecId::SandwichChef, Station::Cook, [10, 10, 10], [55,77,122,255]));
    items.extend(food_enchants("T6_MEAL_SANDWICH_AVALON", "Avalonian Mutton Sandwich", Tier::T6, 360, 10,
                               vec![ing("T6_MEAT", 24), ing("T4_BREAD", 12), ing("T6_BUTTER", 6), ing("QUESTITEM_TOKEN_AVALON", 30)],
                               SpecId::SandwichChef, Station::Cook, [30, 30, 30], [165,231,365,765]));
    items.extend(food_enchants("T8_MEAL_SANDWICH_AVALON", "Avalonian Beef Sandwich", Tier::T8, 1080, 10,
                               vec![ing("T8_MEAT", 72), ing("T4_BREAD", 36), ing("T8_BUTTER", 18), ing("QUESTITEM_TOKEN_AVALON", 90)],
                               SpecId::SandwichChef, Station::Cook, [90, 90, 90], [494,694,1094,2295]));







    let registry = ItemRegistry {
        items: items.into_iter().map(|i| (i.unique_name.clone(), i)).collect(),
        last_price_update: None,
    };

    let home = std::env::var("HOME").expect("Brak HOME");
    let path = PathBuf::from(home.clone()).join(".config/Albion Economy Tools/items.json");
    fs::create_dir_all(path.parent().unwrap())?;
    fs::write(&path, serde_json::to_string_pretty(&registry)?)?;

    println!("Zapisano {} przedmiotów do {:?}", registry.items.len(), path);


    let cities = vec![
        "Caerleon", "Bridgewatch", "Martlock", "Thetford", "Fort Sterling", "Lymhurst", "Brecilien"
    ];

    let empty_price = CityPrice {
        sell_price_min: 0,
        buy_price_max: 0,
        updated_at: Utc::now(),
    };

    let price_map: HashMap<String, HashMap<String, CityPrice>> = registry
        .items
        .keys()
        .map(|unique_name| {
            let city_prices: HashMap<String, CityPrice> = cities
                .iter()
                .map(|city| (city.to_string(), empty_price.clone()))
                .collect();
            (unique_name.clone(), city_prices)
        })
        .collect();

    let prices_path = PathBuf::from(home).join(".config/Albion Economy Tools/prices.json");
    fs::write(&prices_path, serde_json::to_string_pretty(&price_map)?)?;
    println!("Zapisano ceny dla {} przedmiotów do {:?}", price_map.len(), prices_path);


    let specs = UserData::default_spec();

    let home = std::env::var("HOME")?;
    let path = PathBuf::from(home)
        .join(".config/Albion Economy Tools/specs.json");
    std::fs::create_dir_all(path.parent().unwrap())?;
    std::fs::write(&path, serde_json::to_string_pretty(&specs)?)?;

    println!("Zapisano specs do {:?}", path);


    Ok(())


}