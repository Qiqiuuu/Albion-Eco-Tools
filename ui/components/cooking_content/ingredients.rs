pub mod ingredient_card;

use std::collections::HashMap;
use leptos::prelude::*;
use aet_shared::models::items::ItemRegistry;
use aet_shared::models::user::UserData;
use crate::components::cooking_content::ingredients::ingredient_card::IngredientCard;

#[derive(Clone, PartialEq, Debug)]
pub struct AggregatedIngredient {
    pub unique_name: String,
    pub display_name: String,
    pub total_count: u32,
}

#[component]
pub fn Ingredients() -> impl IntoView {
    let data = use_context::<ReadSignal<UserData>>().expect("No user data context");
    let items = use_context::<ReadSignal<ItemRegistry>>().expect("No items data set");

    let tracked_foods = Memo::new(move |_| data.get().tracked_foods);

    let aggregated_ingredients = Memo::new(move |_| {
        let mut totals: HashMap<String, (String, u32)> = HashMap::new();
        let food_data = tracked_foods.get();
        let registry = items.get();

        food_data.iter().for_each(|food_map| {
            if let Some(item_data) = registry.items.get(&food_map.current_tracked) {

                let recipe = item_data.recipes.as_ref()
                    .and_then(|r| r.first());

                if let Some(recipe) = recipe {
                    recipe.ingredients.iter().for_each(|ingredient| {
                        let total_qty = ingredient.count * 1;

                        totals.entry(ingredient.unique_name.clone())
                            .and_modify(|(_, count)| *count += total_qty)
                            .or_insert_with(|| {
                                let display_name = registry.items.get(&ingredient.unique_name)
                                    .map(|i| i.name.clone())
                                    .unwrap_or_else(|| ingredient.unique_name.clone());

                                (display_name, total_qty)
                            });
                    });
                }
            }
        });

        totals.into_iter()
            .map(|(unique_name, (display_name, total_count))| AggregatedIngredient {
                unique_name,
                display_name,
                total_count,
            })
            .collect::<Vec<_>>()
    });

    view! {
        <div class="panel panel-ingredients">
            <div class="panel-header">
                <div class="panel-title">"Ingredients"</div>
                <span style="font-size:11px;color:var(--text4);margin-left:auto">
                    {move || {
                        let count = tracked_foods.get().len();
                        if count == 0 {
                            "No foods tracked".to_string()
                        } else {
                            format!("Aggregating {} items", count)
                        }
                    }}
                </span>
            </div>
            <div class="panel-body">
                {move || {
                    tracked_foods.get().is_empty().then(|| view! {
                        <div style="padding:24px 0;color:var(--text4);font-size:12px;text-align:center">
                            "Add foods to your tracker to see total ingredients."
                        </div>
                    })
                }}

                <Show when=move || !aggregated_ingredients.get().is_empty()>
                    <div class="ing-grid">
                        <For
                            each=move || aggregated_ingredients.get()
                            key=|ing| ing.unique_name.clone()
                            let:ing
                        >
                            <IngredientCard
                                unique_name=ing.unique_name
                                display_name=ing.display_name
                                total_count=ing.total_count
                            />
                        </For>
                    </div>
                </Show>
            </div>
        </div>
    }
}