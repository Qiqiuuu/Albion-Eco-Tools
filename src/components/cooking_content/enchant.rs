use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos::prelude::ClassAttribute;
use strum::IntoEnumIterator;
use crate::data::cooking::fish_sauce::{FishSauce};



#[component]
pub fn Enchant() -> impl IntoView {


    let (chopped_fish_price,set_chopped_fish_price) = signal(200);
    let (seaweed_price, set_seaweed_price) = signal(400);

    let (basic_fish_sauce_price,set_basic_fish_sauce_price) = signal(200);
    let (fancy_fish_sauce_price,set_fancy_fish_sauce_price) = signal(400);
    let (special_fish_sauce_price,set_special_fish_sauce_price) = signal(400);

    let get_price_signal = move |sauce: FishSauce| match sauce {
        FishSauce::Basic => basic_fish_sauce_price.get(),
        FishSauce::Fancy => fancy_fish_sauce_price.get(),
        FishSauce::Special => special_fish_sauce_price.get(),
    };

    let set_price_signal = move |sauce: FishSauce, price: u32| match sauce {
        FishSauce::Basic => set_basic_fish_sauce_price.set(price),
        FishSauce::Fancy => set_fancy_fish_sauce_price.set(price),
        FishSauce::Special=> set_special_fish_sauce_price.set(price),
    };




    view! {
        <div class="enchant-bar">


            <div class="enchant-section">
                <span class="enchant-label">"CRAFT VS BUY"</span>
                <div class="sauce-results-row">
                    {FishSauce::iter().map(|sauce| {
                        let diff = move || {
                            let craft  = sauce.calculate_craft_cost(chopped_fish_price.get(), seaweed_price.get());
                            let market = get_price_signal(sauce);
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
                    <input type="number" prop:value=chopped_fish_price
                        on:input=move |e| set_chopped_fish_price.set(
                            event_target_value(&e).parse().unwrap_or(0)
                        )/>
                </div>
                <div class="input-with-tag">
                    <label>"Seaweed"</label>
                    <input type="number" prop:value=seaweed_price
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