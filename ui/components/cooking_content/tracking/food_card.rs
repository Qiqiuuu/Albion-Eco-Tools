use leptos::prelude::{event_target_value, ElementChild, IntoAny, LocalResource, OnAttribute, PropAttribute, StyleAttribute, Transition};
use leptos::{component, view, IntoView};
use leptos::callback::Callable;
use leptos::context::use_context;
use leptos::prelude::{Callback, ClassAttribute, Get, ReadSignal,WriteSignal};
use aet_shared::models::calculations::CraftingLocation;
use aet_shared::models::items::TrackedFood;
use aet_shared::models::prices::PriceMap;
use aet_shared::models::user::UserData;
use crate::api::items::{calculate_crafting};
use crate::utils::{fmt_silver, price_of, update_price};

#[component]
pub fn FoodCard(
    idx: usize,
    food: TrackedFood,
    on_click: Callback<usize>,
    on_remove: Callback<usize>,
) -> impl IntoView {
    let data = use_context::<ReadSignal<UserData>>().expect("No user data context");
    let set_prices = use_context::<WriteSignal<PriceMap>>().expect("No set_prices context");
    let prices = use_context::<ReadSignal<PriceMap>>().expect("No prices context");

    let (tier_cls, tier_lbl) = food.item.tier.badge();
    let (enc_cls, enc_lbl) = food.item.enchantment.badge();
    let img_url = food.item.get_img();
    let name = food.item.name.clone();
    let show_enchant = !enc_lbl.is_empty();
    let u_name = food.item.unique_name.clone();
    let u_name_for_price = u_name.clone();
    let u_name_debug = u_name.clone();



    let crafting_res = LocalResource::new(move || {
        let _prices = prices.get();
        let u_name = food.item.unique_name.clone();
        let settings = data.get();
        async move {
            calculate_crafting(
                &u_name,
                CraftingLocation::RoyalCity,
                settings.use_focus,
                settings.silver_fee,
                settings.use_premium,
                food.quantity as u32
            ).await
        }
    });

    view! {
        <div class="food-card" on:click=move |_| on_click.run(idx)>
            <div class="food-card-top">
            <img class="ing-card-icon" src=img_url alt=name.clone() />
                <div class="food-info">
                    <div class="food-card-name">{name}</div>
                    <div class="food-card-meta">
                        <span class=tier_cls>{tier_lbl}</span>
                        {if show_enchant {
                            Some(view! { <span class=enc_cls>{enc_lbl}</span> })
                        } else {
                            None
                        }}
                        <span class="qty-sep">"×"</span>
                        <span class="food-qty">{move || food.quantity}</span>
                        <input
                            type="number"
                            class="food-price-input"
                            placeholder="price"
                            prop:value=move || price_of(u_name_for_price.clone(), prices)
                            min="0"
                            on:click=move |ev| ev.stop_propagation()
                            on:input=move |ev| {
                                let val = event_target_value(&ev).parse::<u32>().unwrap_or(0);
                                update_price(u_name.clone(), val, set_prices);
                            }
                        />
                    </div>
                </div>

                <button
                    class="food-card-remove"
                    on:click=move |ev| {
                        ev.stop_propagation();
                        on_remove.run(idx);
                    }
                >"×"</button>
            </div>
            <div class="food-card-bottom" on:click=move |ev| ev.stop_propagation()>
                <Transition fallback=move || view! {
                    <div class="stat-box neutral"><span class="stat-box-label">"PROFIT"</span><span class="stat-box-val">"···"</span></div>
                    <div class="stat-box neutral"><span class="stat-box-label">"FOCUS"</span><span class="stat-box-val">"···"</span></div>
                }>
                {move || crafting_res.get().map(|res|{
                    match res {
                        Some(res) => {
                            let profit = res.profit;
                            let spf = res.silver_per_focus;
                            let focus_cost = res.focus_cost;
                            let margin = res.profit_margin;

                            let profit_cls = if profit >= 0.0 { "fc-stat pos" } else { "fc-stat neg" };
                            let margin_cls = if margin >= 0.0 { "fc-stat pos" } else { "fc-stat neg" };
                            let spf_cls = if spf == 0.0 { "fc-stat neutral" } else if spf >= 100.0 { "fc-stat pos" } else { "fc-stat neg" };
                            let focus_cls  = "fc-stat neutral";
                            view! {
                                <div class="fc-stats-grid">
                                    <div class=profit_cls>
                                        <span class="fc-stat-lbl">"PROFIT"</span>
                                        <span class="fc-stat-val">{fmt_silver(profit)}</span>
                                    </div>
                                    <div class=margin_cls>
                                        <span class="fc-stat-lbl">"MARGIN"</span>
                                        <span class="fc-stat-val">{format!("{:.1}%", margin)}</span>
                                    </div>
                                    <div class=spf_cls>
                                        <span class="fc-stat-lbl">"SPF"</span>
                                        <span class="fc-stat-val">{fmt_silver(spf)}</span>
                                    </div>
                                    <div class=focus_cls>
                                        <span class="fc-stat-lbl">"FOCUS"</span>
                                        <span class="fc-stat-val">{format!("{:.0}", focus_cost)}</span>
                                    </div>
                                </div>
                            }.into_any()
                        },
                        None => view! {
                            <div class="fc-stats-grid">
                                <div class="fc-stat neutral"><span class="fc-stat-lbl">"PROFIT"</span><span class="fc-stat-val">"–"</span></div>
                                <div class="fc-stat neutral"><span class="fc-stat-lbl">"MARGIN"</span><span class="fc-stat-val">"–"</span></div>
                                <div class="fc-stat neutral"><span class="fc-stat-lbl">"SPF"</span><span class="fc-stat-val">"–"</span></div>
                                <div class="fc-stat neutral"><span class="fc-stat-lbl">"FOCUS"</span><span class="fc-stat-val">"–"</span></div>
                            </div>
                        }.into_any()
                        }
                    })
                }
                </Transition>
            </div>
        </div>
    }
}