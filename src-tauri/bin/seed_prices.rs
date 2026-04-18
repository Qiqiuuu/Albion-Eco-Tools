use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use chrono::Utc;
use aet_shared::models::items::ItemRegistry;

use aet_shared::models::prices::{CityPrice, ItemPrice, PriceMap};

pub fn run() -> anyhow::Result<()> {
    let home = std::env::var("HOME")?;
    let items_path = PathBuf::from(&home).join(".config/Albion Economy Tools/items.json");

    let data = fs::read_to_string(items_path)?;
    let registry: ItemRegistry = serde_json::from_str(&data)?;

    let cities_list = vec![
        "Caerleon", "Bridgewatch", "Martlock", "Thetford",
        "Fort Sterling", "Lymhurst", "Brecilien"
    ];

    let mut price_map: PriceMap = HashMap::new();

    for unique_name in registry.items.keys() {
        let mut city_data = HashMap::new();

        for city in &cities_list {
            city_data.insert(
                city.to_string(),
                CityPrice {
                    sell_price_min: 0,
                    buy_price_max: 0,
                    updated_at: Utc::now(),
                },
            );
        }

        price_map.insert(
            unique_name.clone(),
            ItemPrice {
                current: 0,
                cities: city_data,
            },
        );
    }

    let prices_path = PathBuf::from(home).join(".config/Albion Economy Tools/prices.json");
    fs::write(
        &prices_path,
        serde_json::to_string_pretty(&price_map)?
    )?;

    println!("💰 Wygenerowano PriceMap dla {} przedmiotów w nowym formacie.", price_map.len());
    Ok(())
}