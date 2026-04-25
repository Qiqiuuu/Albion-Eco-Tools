use std::collections::HashMap;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos::prelude::ClassAttribute;
use strum::IntoEnumIterator;
use aet_shared::models::calculations::CraftingLocation;
use aet_shared::models::cooking::{FishSauce, CHOPPED_FISH, SEAWEED};
use aet_shared::models::prices::{PriceMap};
use aet_shared::models::user::UserData;
use crate::api::items::{calculate_crafting};
use crate::utils::{price_of, update_price};

#[component]
pub fn Enchant(
) -> impl IntoView {

    let prices = use_context::<ReadSignal<PriceMap>>().expect("No prices context");
    let data = use_context::<ReadSignal<UserData>>().expect("No user data context");

    let set_prices = use_context::<WriteSignal<PriceMap>>().expect("No set_prices context");


    let all_results = LocalResource::new(move || {
        let use_focus = data.get().use_focus;
        let silver_fee = data.get().silver_fee;
        let use_premium = data.get().use_premium;
        async move {
            let mut results = HashMap::new();
            for sauce in FishSauce::iter() {
                let id = sauce.get_unique_name().to_owned();
                let res = calculate_crafting(
                    &id,
                    CraftingLocation::RoyalCity,
                    use_focus,
                    silver_fee,
                    use_premium,
                    1
                ).await;
                results.insert(id, res);
            }
            results
        }
    });

    view! {
        <div class="enchant-bar">
            <div class="enchant-section">
                <span class="enchant-label">"CRAFT VS BUY"</span>
                <div class="sauce-results-row">
                    {FishSauce::iter().map(|sauce| {
                        let sauce_id = sauce.get_unique_name().to_owned();
                        let sauce_name = sauce.to_string();
                        let sauce_name_badge = sauce_name.clone();

                        view! {
                            <Transition fallback=move || view! {
                                <div class="sauce-badge loading">
                                    <span class="sauce-name">{sauce_name_badge.clone()}</span>
                                    <span class="sauce-diff">"..."</span>
                                </div>
                            }>
                                {move || {
                                    all_results.get().map(|results| {
                                        let res = results.get(&sauce_id).unwrap().as_ref().unwrap();
                                        let is_profit = res.profit > 0.0;
                                        let verdict = if is_profit { "CRAFT" } else { "BUY" };
                                        view! {
                                            <div class="sauce-badge" class:profit=is_profit class:loss=(!is_profit)>
                                            <span class="sauce-name">{sauce_name.clone()}</span>
                                            <span class="sauce-cost">{format!("Cost: {}", res.raw_item_cost.round())}</span>
                                            <span class="sauce-diff">{format!("{:+.0}", res.profit)}</span>
                                            <span class="sauce-verdict">{verdict}</span>
                                            </div>
                                        }
                                    })
                                }}
                            </Transition>
                        }
                    }).collect_view()}
                </div>
            </div>

            <div class="enchant-sep"></div>

            <div class="enchant-section">
                <span class="enchant-label">"INGREDIENTS"</span>
                {[
                    ("Chopped Fish", CHOPPED_FISH),
                    ("Seaweed", SEAWEED),
                ].into_iter().map(|(label, key)| {
                    let key = key.to_string();
                    let label = label.to_string();
                    let key_read = key.clone();
                    view! {
                        <div class="input-with-tag">
                            <label>{label}</label>
                            <input
                                type="number"
                                class="price-input"
                                prop:value=move || price_of(key_read.clone(),prices)
                                on:input=move |e| {
                                    let val = event_target_value(&e).parse::<u32>().unwrap_or(0);
                                    update_price(key.clone(), val, set_prices);
                                }
                            />
                        </div>
                    }
                }).collect_view()}
            </div>

            <div class="enchant-sep"></div>

            <div class="enchant-section">
                <span class="enchant-label">"MARKET PRICE"</span>
                <div class="market-inputs-row">
                    {FishSauce::iter().map(|sauce| {
                        let key = sauce.get_unique_name().to_owned();
                        let label = sauce.to_string();
                        let key_read = key.clone();
                        view! {
                            <div class="input-with-tag">
                                <label>{label}</label>
                                <input
                                    type="number"
                                    class="price-input"
                                    prop:value=move || price_of(key_read.clone(),prices)
                                    on:input=move |e| {
                                        let val = event_target_value(&e).parse::<u32>().unwrap_or(0);
                                        update_price(key.clone(), val, set_prices);
                                    }
                                />
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}