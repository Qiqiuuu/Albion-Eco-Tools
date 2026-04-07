use leptos::prelude::{event_target_value, CollectView, ElementChild, For, Get, GlobalAttributes, Memo, OnAttribute, PropAttribute, RwSignal, Set, Show, Update};
use leptos::{component, view, IntoView};
use leptos::prelude::ClassAttribute;
use crate::data::cooking::food::{AlbionItem, ApiTier, DishCategory, FoodCategory, FoodName};

#[derive(Clone, Debug, PartialEq)]
pub struct TrackedFood {
    pub item: AlbionItem,
    pub quantity: i32,
    pub daily_demand: u64,
}

#[component]
  pub fn Tracking() -> impl IntoView{
    let search_query = RwSignal::new(String::new());
    let tracked_foods = RwSignal::new(Vec::<TrackedFood>::new());
    let show_modal = RwSignal::new(false);

    let selected_dish = RwSignal::new(Option::<DishCategory>::None);
    let pending_qty    = RwSignal::new(10i32);
    let pending_enchant = RwSignal::new(0u8);

    let all_dishes = DishCategory::get_all_dishes();


    let suggestions = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }

        all_dishes.iter()
            .filter(|item| item.food_name().to_lowercase().contains(&query))
            .take(5)
            .cloned()
            .collect::<Vec<_>>()
    });

    let close_modal = move || {
        show_modal.set(false);
        selected_dish.set(None);
        search_query.set(String::new());
        pending_qty.set(10);
        pending_enchant.set(0);
    };

    let confirm_add = move || {
        if let Some(dish) = selected_dish.get() {
            let tier = match dish {
                DishCategory::Stew(s) => s.api_tier(),
            };
            let new_item = AlbionItem {
                category: FoodCategory::Dish(dish),
                tier,
                enchant: pending_enchant.get(),
            };
            tracked_foods.update(|list| list.push(TrackedFood {
                item: new_item,
                quantity: pending_qty.get(),
                daily_demand: 1500,
            }));
            close_modal();
        }
    };

    view! {
        <div class="panel panel-food">
            <div class="panel-header"><div class="panel-title">"Tracked Food"</div></div>
            <div class="panel-body">
                <For
                    each=move || tracked_foods.get()
                    key=|food| food.item.api_name()
                    children=move |food| view! {
                        <div class="food-card">
                            <div class="food-card-name">{food.item.category.food_name()}</div>
                            <div class="food-card-meta">
                                <span>"T"{food.item.tier}</span>
                                <span>"×"{food.quantity}</span>
                                {if food.item.enchant > 0 { format!(".{}", food.item.enchant) } else { String::new() }}
                                <span class="api-id-hint">{food.item.api_name()}</span>
                            </div>
                        </div>
                    }
                />
                <button class="add-food-btn" on:click=move |_| show_modal.set(true)>
                    "+ Add Food Item"
                </button>
            </div>
        </div>

        <Show when=move || show_modal.get()>
            <div class="modal-overlay" on:click=move |_| close_modal()>
                <div class="modal" on:click=move |ev| ev.stop_propagation()>
                    <div class="modal-title">"Add Food Item"</div>

                    <Show when=move || selected_dish.get().is_none()>
                        <input
                            type="text"
                            class="modal-input"
                            placeholder="Search food..."
                            prop:value=move || search_query.get()
                            on:input=move |ev| search_query.set(event_target_value(&ev))
                        />
                        <div class="food-suggestions open">
                            <For
                                each=move || suggestions.get()
                                key=|dish| dish.food_name()
                                children=move |dish| view! {
                                    <div class="food-sugg-item" on:click=move |_| {
                                        selected_dish.set(Some(dish));
                                    }>
                                        {dish.food_name()}
                                    </div>
                                }
                            />
                        </div>
                    </Show>

                    <Show when=move || selected_dish.get().is_some()>
                        <div class="modal-selected-name">
                            {move || selected_dish.get()
                                .map(|d| d.food_name())
                                .unwrap_or_default()}
                        </div>

                        <div class="modal-row">
                            <label>"Quantity"</label>
                            <input
                                type="number"
                                class="modal-small-input"
                                min="1" max="9999"
                                prop:value=move || pending_qty.get()
                                on:input=move |ev| {
                                    let v = event_target_value(&ev).parse().unwrap_or(1);
                                    pending_qty.set(v);
                                }
                            />
                        </div>

                        <div class="modal-row">
                            <label>"Enchant"</label>
                            <div class="etch-opt-group">
                                {[0u8, 1, 2, 3].iter().map(|&e| view! {
                                    <button
                                        class="etch-opt"
                                        class:active=move || pending_enchant.get() == e
                                        on:click=move |_| pending_enchant.set(e)
                                    >
                                        {if e == 0 { "None".to_string() } else { format!(".{}", e) }}
                                    </button>
                                }).collect_view()}
                            </div>
                        </div>

                        <button
                            class="btn-back"
                            on:click=move |_| {
                                selected_dish.set(None);
                                search_query.set(String::new());
                            }
                        >
                            "← Back"
                        </button>
                    </Show>

                    <div class="modal-actions">
                        <button class="btn-cancel" on:click=move |_| close_modal()>"Cancel"</button>
                        <Show when=move || selected_dish.get().is_some()>
                            <button class="btn-add" on:click=move |_| confirm_add()>
                                "Add to Tracker"
                            </button>
                        </Show>
                    </div>
                </div>
            </div>
        </Show>
    }
}