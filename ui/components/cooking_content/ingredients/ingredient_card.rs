use leptos::prelude::{ElementChild, PropAttribute, event_target_value, OnAttribute, StyleAttribute, use_context, ReadSignal};
use leptos::{component, view, IntoView};
use leptos::prelude::{ClassAttribute, WriteSignal};
use aet_shared::models::prices::PriceMap;
use crate::utils::{fmt_silver, price_of, update_price};

#[component]
pub fn IngredientCard(
    unique_name: String,
    display_name: String,
    total_count: u32,
) -> impl IntoView {
    let set_prices = use_context::<WriteSignal<PriceMap>>().expect("No set_prices context");
    let prices     = use_context::<ReadSignal<PriceMap>>().expect("No prices context");

    let key_input  = unique_name.clone();
    let key_total  = unique_name.clone();
    

    view! {
        <div class="ing-card">
            <div class="ing-card-top">
                <img
                    class="ing-card-icon"
                    src=format!("https://render.albiononline.com/v1/item/{}.png", unique_name.clone())
                    alt=display_name.clone()
                />
                <div class="ing-card-info">
                    <div class="ing-card-name">{display_name}</div>
                    <div class="ing-card-qty">
                        <span class="qty-sep">"×"</span>
                        <span>{total_count}</span>
                        <input
                        type="number"
                        class="food-price-input"
                        placeholder="price"
                        prop:value=move || price_of(key_input.clone(),prices)
                        on:input=move |e| {
                            let val = event_target_value(&e).parse::<u32>().unwrap_or(0);
                            update_price(unique_name.clone(), val, set_prices);
                        }
                    />
                    </div>
                </div>
            </div>
            <div class="ing-card-bottom" on:click=move |ev| ev.stop_propagation()>

                <div class="ing-stat-box">
                    <span class="stat-box-label">"SUBTOTAL"</span>
                    <span class="stat-box-val" style="color:var(--text2)">
                        {move || fmt_silver(price_of(key_total.clone(),prices) as f64 * total_count as f64)}
                    </span>
                </div>
            </div>
        </div>
    }
}