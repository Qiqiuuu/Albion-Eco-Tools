use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos::prelude::ClassAttribute;
use strum::IntoEnumIterator;
use aet_shared::models::cooking::{FishSauce, CHOPPED_FISH, SEAWEED};
use aet_shared::models::prices::ItemPrices;


fn get_price(prices: &ItemPrices, unique_name: &str, city: &str) -> u32 {
    prices
        .get(unique_name)
        .and_then(|c| c.get(city))
        .map(|p| p.sell_price_min as u32)
        .unwrap_or(0)
}


#[component]
pub fn Enchant(
    selected_city: ReadSignal<String>
) -> impl IntoView {

    let prices = use_context::<ReadSignal<ItemPrices>>().unwrap();

    let city         = move || selected_city.get();
    let chopped_fish = move || get_price(&prices.get(), CHOPPED_FISH, &city());
    let seaweed      = move || get_price(&prices.get(), SEAWEED, &city());
    let sauce_price  = move |sauce: FishSauce| get_price(&prices.get(), sauce.get_unique_name(), &city());


    view! {
        <div class="enchant-bar">


            <div class="enchant-section">
                <span class="enchant-label">"CRAFT VS BUY"</span>
                <div class="sauce-results-row">::iter()
                    {FishSauce::iter().map(|sauce| {
                        let diff = move || {
                            let craft  = sauce.calculate_craft_cost(chopped_fish, seaweed);
                            let market = sauce_price(sauce);
                            market as i32 - craft as i32
                        };
                        view! {
                            <div class="sauce-badge" class:is-profit=move || (diff() > 0)>
                                <span class="sauce-name">{sauce.to_string()}</span>
                                <span class="sauce-diff">
                                    {move || {
                                        let d = diff();
                                              if d >= 0 { format!("+{:.1}k", d as f64 / 1000.0) }
                                              else { format!("{:.1}k",   d as f64 / 1000.0) }
                                    }}
                                </span>
                                <span class="sauce-verdict">
                                    {move || if diff() > 0 { "CRAFT" } else { "BUY" }}
                                </span>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="enchant-sep"></div>


            <div class="enchant-section">
                <span class="enchant-label">"INGREDIENTS"</span>
                <div class="input-with-tag">
                    <label>"Chopped Fish"</label>
                    <input type="number" prop:value=chopped_fish
                        on:input=move |e| set_chopped_fish_price.set(
                            event_target_value(&e).parse().unwrap_or(0)
                        )/>
                </div>
                <div class="input-with-tag">
                    <label>"Seaweed"</label>
                    <input type="number" prop:value=seaweed
                        on:input=move |e| set_seaweed_price.set(
                            event_target_value(&e).parse().unwrap_or(0)
                        )/>
                </div>
            </div>

            <div class="enchant-sep"></div>


            <div class="enchant-section">
                <span class="enchant-label">"MARKET PRICE"</span>
                <div class="market-inputs-row">
                    {FishSauce::iter().map(|sauce| {
                        view! {
                            <div class="input-with-tag">
                                <label>{sauce.to_string()}</label>
                                <input class="market-input" type="number"
                                    prop:value=move || get_price_signal(sauce)
                                    on:input=move |e| set_price_signal(
                                        sauce,
                                        event_target_value(&e).parse().unwrap_or(0)
                                    )/>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>

        </div>
    }
}