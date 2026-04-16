use leptos::prelude::{event_target_value, use_context, ClassAttribute, CollectView, ElementChild, For, Get, Memo, OnAttribute, PropAttribute, ReadSignal, RwSignal, Set, Show, Update};
use leptos::{component, view, IntoView};
use aet_shared::models::items::{Consumable, Enchantment, Item, ItemEntity, ItemRegistry};

#[derive(Clone, Debug, PartialEq)]
pub struct TrackedFood {
    pub item: ItemEntity,
    pub quantity: i32,
}


#[component]
  pub fn Tracking() -> impl IntoView{
    let items = use_context::<ReadSignal<ItemRegistry>>().expect("No items data set");


    let search_query = RwSignal::new(String::new());
    let tracked_foods = RwSignal::new(Vec::<TrackedFood>::new());
    let show_modal = RwSignal::new(false);

    let selected_dish = RwSignal::new(None::<ItemEntity>);
    let pending_qty    = RwSignal::new(10i32);
    let pending_enchant = RwSignal::new(0u8);

    let all_dishes = Memo::new(move |_| {
        items.get().items.values()
            .filter(|entity| matches!(entity.category, Item::Consumable(Consumable::Food)))
            .cloned()
            .collect::<Vec<ItemEntity>>()
    });


    let suggestions = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() {
            return Vec::new();
        }

        all_dishes.get().into_iter()
            .filter(|item| item.name.to_lowercase().contains(&query))
            .take(5)
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
            let enchantment = match pending_enchant.get() {
                1 => Enchantment::Uncommon,
                2 => Enchantment::Rare,
                3 => Enchantment::Exceptional,
                _ => Enchantment::Common,
            };

            let mut final_item = dish.clone();
            final_item.enchantment = enchantment;

            tracked_foods.update(|list| list.push(TrackedFood {
                item: final_item,
                quantity: pending_qty.get(),
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
                    key=|food| format!("{}_{:?}", food.item.unique_name, food.item.enchantment)
                    children=move |food| {
                        let name = food.item.name.clone();
                        let tier_label = format!("{:?}", food.item.tier);
                        let qty = food.quantity;

                        let enchant_label = match food.item.enchantment {
                            Enchantment::Uncommon => ".1",
                            Enchantment::Rare => ".2",
                            Enchantment::Exceptional => ".3",
                            Enchantment::Common => "",
                            _ => {""}};

                        view! {
                            <div class="food-card">
                                <div class="food-card-name">{name}</div>
                                <div class="food-card-meta">
                                    <span>{tier_label}</span>
                                    <span>" × " {qty}</span>
                                    <span class="enchant-badge">{enchant_label}</span>
                                </div>
                            </div>
                        }
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

                    <Show
                        when=move || selected_dish.get().is_some()
                        fallback=move || view! {
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
                                    key=|dish| dish.unique_name.clone()
                                    children=move |dish| {
                                        let d_name = dish.name.clone();
                                        view! {
                                            <div class="food-sugg-item" on:click=move |_| selected_dish.set(Some(dish.clone()))>
                                                {d_name}
                                            </div>
                                        }
                                    }
                                />
                            </div>
                        }
                    >
                        <div class="modal-selected-name">
                            {move || selected_dish.get().map(|d| d.name).unwrap_or_default()}
                        </div>

                        <div class="modal-row">
                            <label>"Quantity"</label>
                            <input
                                type="number"
                                class="modal-small-input"
                                min="1"
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
                                {vec![0u8, 1, 2, 3, 4].into_iter().map(|e| view! {
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
                            "← Back to search"
                        </button>
                    </Show>

                    <div class="modal-actions">
                        <button class="btn-cancel" on:click=move |_| close_modal()>"Cancel"</button>
                        <Show when=move || selected_dish.get().is_some()>
                            <button class="btn-add" on:click=move |_| confirm_add()>"Add to Tracker"</button>
                        </Show>
                    </div>
                </div>
            </div>
        </Show>
    }
}