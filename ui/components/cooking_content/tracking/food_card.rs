use std::collections::HashMap;
use leptos::prelude::*;
use leptos::callback::Callable;
use aet_shared::models::calculations::CraftingLocation;
use aet_shared::models::items::{Enchantment, ItemEntity, ItemRegistry, TrackedFoodMap};
use aet_shared::models::prices::PriceMap;
use aet_shared::models::user::UserData;
use crate::api::items::calculate_crafting;
use crate::utils::{fmt_silver, price_of, update_demand, update_price};

#[component]
pub fn FoodCard(
    food_map: TrackedFoodMap,
    on_remove: Callback<()>,
    on_update: Callback<TrackedFoodMap>,
) -> impl IntoView {
    let data = use_context::<ReadSignal<UserData>>().expect("No user data context");
    let set_prices = use_context::<WriteSignal<PriceMap>>().expect("No set_prices context");
    let prices = use_context::<ReadSignal<PriceMap>>().expect("No prices context");
    let items = use_context::<ReadSignal<ItemRegistry>>().expect("No items data set");

    let silver_fee = Memo::new(move |_| data.with(|d| d.silver_fee));
    let use_focus = Memo::new(move |_| data.with(|d| d.use_focus));
    let use_premium = Memo::new(move |_| data.with(|d| d.use_premium));


    let food_versions: HashMap<Enchantment, ItemEntity> = items
        .get_untracked()
        .get_item_enchantment_versions(&food_map.base_name)
        .into_iter()
        .map(|(k, v)| (k, v.clone()))
        .collect();

    let demand_of = |id: String, p: ReadSignal<PriceMap>| {
        p.with(|map| map.get(&id).map(|entry| entry.demand).unwrap_or(0))
    };

    let mut available_enchantments: Vec<Enchantment> = food_versions.keys().cloned().collect();
    available_enchantments.sort_by_key(|e| e.to_u8());


    let initial_id = food_map.current_tracked.clone();
    let initial_item = food_versions
        .values()
        .find(|i| i.unique_name == initial_id)
        .cloned()
        .unwrap_or_else(|| food_versions.values().next().expect("No item versions found").clone());

    let active_item = RwSignal::new(initial_item);
    let qty = RwSignal::new(food_map.map.get(&initial_id).map(|f| f.quantity).unwrap_or(1));
    let price_signal = RwSignal::new(price_of(initial_id.clone(), prices));
    let demand_signal = RwSignal::new(demand_of(initial_id, prices));


    let trigger_update = move |new_id: String, new_qty: u32| {
        let mut updated = food_map.clone();
        updated.current_tracked = new_id.clone();
        if let Some(food_entry) = updated.map.get_mut(&new_id) {
            food_entry.quantity = new_qty;
        }
        on_update.run(updated);
    };

    let crafting_res = LocalResource::new(move || {
        let u_name = active_item.get().unique_name;
        let q = qty.get();
        let fee = silver_fee.get();
        let focus = use_focus.get();
        let prem = use_premium.get();
        let _prices = prices.get();

        async move {
            calculate_crafting(
                &u_name,
                CraftingLocation::RoyalCity,
                focus,
                fee,
                prem,
                q,
            ).await
        }
    });

    view! {
        <div class="food-card fc-new">
            <div class="fc-top">
                <div class="fc-left">
                    <div class="fc-icon-wrap">
                        <img
                            src=move || active_item.get().get_img()
                            alt=move || active_item.get().name
                            class="fc-icon-img"
                        />
                    </div>
                    <div class="fc-info">
                        <div class="food-card-name">{move || active_item.get().name}</div>
                        <div class="fc-tier-row">
                            {move || {
                                let (cls, lbl) = active_item.get().tier.badge();
                                view! { <span class=cls>{lbl}</span> }
                            }}
                            {move || {
                                let (cls, lbl) = active_item.get().enchantment.badge();
                                (!lbl.is_empty()).then(|| view! { <span class=cls>{lbl}</span> })
                            }}
                        </div>
                    </div>
                </div>

                <div class="fc-right">
                    <div class="fc-ench-grid">
                        {available_enchantments.into_iter().map(|enc| {
                            let versions = food_versions.clone();
                            let trigger_update = trigger_update.clone();
                            let enc_button = enc.clone();

                            let (_, lbl) = enc.badge();
                            let btn_lbl = if lbl.is_empty() { ".0" } else { lbl };
                            let enc_key = match enc {
                                Enchantment::Common => "e0",
                                Enchantment::Uncommon => "e1",
                                Enchantment::Rare => "e2",
                                Enchantment::Exceptional => "e3",
                                Enchantment::Pristine => "e4",
                            };

                            view! {
                                <button
                                    class=move || {
                                        let active = active_item.get().enchantment == enc;
                                        format!("fc-ench-btn {} {}", enc_key, if active { "active" } else { "" })
                                    }
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        if let Some(item) = versions.get(&enc_button) {
                                            let new_id = item.unique_name.clone();
                                            active_item.set(item.clone());
                                            price_signal.set(price_of(new_id.clone(), prices));
                                            demand_signal.set(demand_of(new_id.clone(), prices));
                                            trigger_update(new_id, qty.get_untracked());
                                        }
                                    }
                                >
                                    {btn_lbl}
                                </button>
                            }
                        }).collect_view()}
                    </div>
                    <button
                        class="food-card-remove"
                        on:click=move |ev| {
                            ev.stop_propagation();
                            on_remove.run(());
                        }
                    >"×"</button>
                </div>
            </div>

            <div class="fc-sep"></div>

            <div class="fc-inputs" on:click=move |ev| ev.stop_propagation()>
                <div class="fc-input-group">
                    <label class="fc-input-lbl">"DAILY"</label>
                    <input
                        type="number" min="0"
                        class="fc-input"
                        prop:value=move || demand_signal.get()
                        on:input=move |ev| {
                            let val = event_target_value(&ev).parse::<u32>().unwrap_or(0);
                            demand_signal.set(val);
                            update_demand(active_item.get_untracked().unique_name, val, set_prices);
                        }
                    />
                </div>
                <div class="fc-input-group">
                    <label class="fc-input-lbl">"QTY"</label>
                    <input
                        type="number" min="1"
                        class="fc-input"
                        prop:value=move || qty.get()
                        on:input=move |ev| {
                            let val = event_target_value(&ev).parse::<u32>().unwrap_or(1);
                            qty.set(val);
                            trigger_update(active_item.get_untracked().unique_name, val);
                        }
                    />
                </div>
                <div class="fc-input-group">
                    <label class="fc-input-lbl">"PRICE"</label>
                    <input
                        type="number" min="0"
                        class="fc-input"
                        prop:value=move || price_signal.get()
                        on:input=move |ev| {
                            let val = event_target_value(&ev).parse::<u32>().unwrap_or(0);
                            price_signal.set(val);
                            update_price(active_item.get_untracked().unique_name, val, set_prices);
                        }
                    />
                </div>
            </div>

            <div class="food-card-bottom" on:click=move |ev| ev.stop_propagation()>
                <Transition fallback=move || view! { <div class="fc-stats-grid">""</div> }>
                    {move || crafting_res.get().map(|res| {
                        match res {
                            Some(r) => {
                                let profit = r.profit;
                                let spf = r.silver_per_focus;
                                let margin = r.profit_margin;

                                let profit_cls = if profit >= 0.0 { "fc-stat pos" } else { "fc-stat neg" };
                                let margin_cls = if margin >= 0.0 { "fc-stat pos" } else { "fc-stat neg" };
                                let spf_cls = if spf == 0.0 { "fc-stat neutral" } else if spf >= 100.0 { "fc-stat pos" } else { "fc-stat neg" };

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
                                        <div class="fc-stat neutral">
                                            <span class="fc-stat-lbl">"FOCUS"</span>
                                            <span class="fc-stat-val">{format!("{:.0}", r.focus_cost)}</span>
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
                    })}
                </Transition>
            </div>
        </div>
    }.into_any()
}