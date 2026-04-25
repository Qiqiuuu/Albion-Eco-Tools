use leptos::prelude::{Get, ReadSignal, Set, WriteSignal};
use leptos::task::spawn_local;
use aet_shared::models::prices::PriceMap;
use crate::api::items::{fetch_all_prices, send_item_price_update};

pub fn fmt_silver(val: f64) -> String {
    let abs = val.abs();
    let sign = if val < 0.0 { "-" } else { "" };
    if abs >= 1_000_000.0 {
        format!("{}{:.2}M", sign, abs / 1_000_000.0)
    } else if abs >= 1_000.0 {
        format!("{}{:.3}", sign, abs / 1_000.0)
    } else {
        format!("{}{:.0}", sign, abs)
    }
}

pub fn update_price(key: String, val: u32, set_prices: WriteSignal<PriceMap>) {
    spawn_local(async move {
        send_item_price_update(key, val).await;
        let fresh = fetch_all_prices().await;
        set_prices.set(fresh);
    });
}

pub fn price_of(key: String, prices: ReadSignal<PriceMap>) -> u32{
    prices.get()
        .get(&key)
        .map(|p| p.current)
        .unwrap_or(0)
}



