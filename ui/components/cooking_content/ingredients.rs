pub mod ingredient_card;

use std::collections::HashMap;
use leptos::prelude::{ElementChild, For, Show, StyleAttribute};
use leptos::{component, logging, view, IntoView};
use leptos::prelude::{use_context, ClassAttribute, Get, Memo, ReadSignal};
use aet_shared::models::items::ItemRegistry;
use aet_shared::models::prices::PriceMap;
use aet_shared::models::user::UserData;
use crate::components::cooking_content::ingredients::ingredient_card::IngredientCard;
use crate::utils::fmt_silver;

#[derive(Clone, PartialEq)]
pub struct AggregatedIngredient {
    pub unique_name: String,
    pub display_name: String,
    pub total_count: u32,
}

#[component]
pub fn Ingredients() -> impl IntoView {
    let prices     = use_context::<ReadSignal<PriceMap>>().expect("No prices context");
    let data       = use_context::<ReadSignal<UserData>>().expect("No user data context");
    let items      = use_context::<ReadSignal<ItemRegistry>>().expect("No items data set");

    let tracked_foods = Memo::new(move |_| data.get().tracked_foods);

    let aggregated_ingredients = Memo::new(move |_| {
        let registry = items.get();
        let mut totals: HashMap<String, (String, u32)> = Default::default();

        for food in tracked_foods.get() {
            if let Some(recipe) = food.item.recipes.as_ref().and_then(|r| r.first()) {
                for ing in &recipe.ingredients {
                    let key   = ing.unique_name.clone();
                    let count = ing.count  * food.quantity;
                    let name  = registry.items.get(&key)
                        .map(|it| it.name.clone())
                        .unwrap_or_else(|| {
                            logging::warn!("Missing key in registry: '{}'", key);
                            key.clone()
                        });
                    let entry = totals.entry(key).or_insert((name, 0));
                    entry.1 += count;
                }
            }
        }

        let mut result = totals.into_iter()
            .map(|(id, (name, count))| AggregatedIngredient {
                unique_name: id,
                display_name: name,
                total_count: count,
            })
            .collect::<Vec<_>>();
        result.sort_by(|a, b| a.display_name.cmp(&b.display_name));
        result
    });

    let price_of = move |key: &str| -> u32 {
        prices.get().get(key).map(|p| p.current).unwrap_or(0)
    };

    let total_cost = move || {
        aggregated_ingredients.get().iter().fold(0.0f64, |acc, ing| {
            acc + price_of(&ing.unique_name) as f64 * ing.total_count as f64
        })
    };

    view! {
        <div class="panel panel-ingredients">
            <div class="panel-header">
                <div class="panel-title">"Ingredients"</div>
                <span style="font-size:11px;color:var(--text4);margin-left:auto">
                    {move || {
                        let tracked = tracked_foods.get().len();
                        let types   = aggregated_ingredients.get().len();
                        format!("{tracked} tracked · {types} types")
                    }}
                </span>
            </div>
            <div class="panel-body">
                {move || tracked_foods.get().is_empty().then(|| view! {
                    <div style="padding:24px 0;color:var(--text4);font-size:12px;text-align:center">
                        "Add food to the tracker to see required ingredients."
                    </div>
                })}
                <Show when=move || !tracked_foods.get().is_empty()>
                    <div class="ing-grid">
                        <For
                            each=move || aggregated_ingredients.get()
                            key=|ing| ing.unique_name.clone()
                            let:ing
                        >
                            <IngredientCard
                                unique_name=ing.unique_name.clone()
                                display_name=ing.display_name.clone()
                                total_count=ing.total_count
                            />
                        </For>
                    </div>

                    <div style="margin-top:12px;border-top:1px solid var(--border2);padding-top:10px;display:flex;flex-direction:column;gap:5px">
                        <div class="analysis-row">
                            <span class="analysis-row-label">"Total Ingredient Cost"</span>
                            <span class="analysis-row-val gold">
                                {move || fmt_silver(total_cost())}
                            </span>
                        </div>
                        <div class="analysis-row">
                            <span class="analysis-row-label">"Cost per craft unit"</span>
                            <span class="analysis-row-val">
                                {move || {
                                    let qty = tracked_foods.get().iter()
                                        .map(|f| f.quantity).sum::<u32>();
                                    if qty > 0 { fmt_silver(total_cost() / qty as f64) }
                                    else { "—".into() }
                                }}
                            </span>
                        </div>
                    </div>
                </Show>
            </div>
        </div>
    }
}