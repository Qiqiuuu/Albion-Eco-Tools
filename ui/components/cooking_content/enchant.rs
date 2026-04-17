use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos::prelude::ClassAttribute;
use strum::IntoEnumIterator;
use aet_shared::models::calculations::CraftingLocation;
use aet_shared::models::cooking::{FishSauce, CHOPPED_FISH, SEAWEED};
use aet_shared::models::prices::{PriceMap};
use aet_shared::models::user::UserData;
use crate::api::items::calculate_crafting;


#[component]
pub fn Enchant(
) -> impl IntoView {

    let prices = use_context::<ReadSignal<PriceMap>>().expect("No prices context");
    let data = use_context::<ReadSignal<UserData>>().expect("No user data context");


    let price_of = move |name: &str| {
        prices.get().get(name).map(|c| c.current).unwrap_or(0)
    };

    let all_results = LocalResource::new(move || {
        let data = data.get();
        async move {
            let mut results = std::collections::HashMap::new();
            for sauce in FishSauce::iter() {
                let id = sauce.get_unique_name().to_owned();
                let res = calculate_crafting(
                    &id,
                    CraftingLocation::RoyalCity,
                    data.use_focus,
                    data.silver_fee,
                    data.use_premium
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
                <div class="sauce-results-row">::iter()
                    {FishSauce::iter().map(|sauce| {
                        let sauce_id = sauce.get_unique_name().to_owned();
                        let sauce_name = sauce.to_string();
                        let id = sauce_id.clone();

                        view! {
                        <Suspense fallback=move || view! { <div class="sauce-badge">"..."</div> }>
                            {move || {
                                all_results.get().map(|results| {
                                    match results.get(&id) {
                                        Some(Some(res)) => {
                                            let market = price_of(&sauce_id) as f64;
                                            let diff = market - res.actual_cost;
                                            let is_profit = diff > 0.0;
                                            view! {
                                                <div class="sauce-badge" class:is_profit=is_profit>
                                                    // ...
                                                </div>
                                            }.into_any()
                                        }
                                        _ => view! { <div class="sauce-badge loading">"..."</div> }.into_any()
                                    }
                                })
                            }}
                        </Suspense>
                    }
                }).collect_view()}
                </div>
            </div>

            <div class="enchant-sep"></div>


            <div class="enchant-section">
                <span class="enchant-label">"INGREDIENTS"</span>
                <div class="input-with-tag">
                    <label>"Chopped Fish"</label>
                    <div class="price-display">{move || price_of(CHOPPED_FISH)}</div>
                </div>
                <div class="input-with-tag">
                    <label>"Seaweed"</label>
                    <div class="price-display">{move || price_of(SEAWEED)}</div>
                </div>
            </div>

            <div class="enchant-sep"></div>


            <div class="enchant-section">
                <span class="enchant-label">"MARKET PRICE"</span>
                <div class="market-inputs-row">
                    {FishSauce::iter().map(|sauce| {
                        let id = sauce.get_unique_name().to_owned();
                        let label = sauce.to_string();
                        view! {
                            <div class="input-with-tag">
                                <label>{label}</label>
                                <div class="price-display">
                                    {move || price_of(&id)}
                                </div>
                            </div>
                        }
                    }).collect_view()}
                </div>
            </div>
        </div>
    }
}