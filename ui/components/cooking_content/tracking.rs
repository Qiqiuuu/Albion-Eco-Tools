use std::collections::HashSet;
use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos::reactive::spawn_local;
use aet_shared::models::items::{Consumable, Enchantment, Item, ItemRegistry, TrackedFood};
use aet_shared::models::user::UserData;
use crate::api::user::{send_add_tracked_food, send_remove_tracked_food};
use food_card::FoodCard;

pub mod food_card;

#[derive(Clone, Copy, Debug, PartialEq)]
enum ModalMode {
    Add,
    Edit(usize),
}

fn build_tracked_food(item_registry: ItemRegistry,dish_name: String, enchant: u8, qty: u32) -> TrackedFood {
    let enchant = Enchantment::from_u8(enchant);
    let item = item_registry.get_item_entity_by_name_and_enchant(dish_name.as_str(), enchant);
    TrackedFood {item: item.clone(), quantity: qty }
}

#[component]
pub fn Tracking() -> impl IntoView {
    let items = use_context::<ReadSignal<ItemRegistry>>().expect("No items data set");
    let data = use_context::<ReadSignal<UserData>>().expect("No UserData set");
    let set_data = use_context::<WriteSignal<UserData>>().expect("No data setter set");

    let tracked_foods = Memo::new(move |_| data.get().tracked_foods);

    let search_query    = RwSignal::new(String::new());
    let modal_mode      = RwSignal::new(None::<ModalMode>);
    let selected_dish   = RwSignal::new(String::new());
    let pending_qty     = RwSignal::new(1u32);
    let pending_enchant = RwSignal::new(0u8);
    let dish_selected = move || !selected_dish.get().is_empty();


    let all_dishes = Memo::new(move |_| {
        items.get().items.values()
            .filter(|e| matches!(e.category, Item::Consumable(Consumable::Food)))
            .cloned()
            .map(|e| e.name)
            .collect::<HashSet<_>>()
    });

    let suggestions = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() { return Vec::new(); }
        all_dishes.get().into_iter()
            .filter(|item| item.to_lowercase().contains(&query))
            .collect::<Vec<_>>()
    });

    let close_modal = move || {
        modal_mode.set(None);
        selected_dish.set(String::new());
        search_query.set(String::new());
        pending_qty.set(1);
        pending_enchant.set(0);
    };

    let open_add = move || modal_mode.set(Some(ModalMode::Add));

    let open_edit = move |idx: usize| {
        let Some(food) = tracked_foods.get().into_iter().nth(idx) else { return };
        selected_dish.set(food.item.name);
        pending_qty.set(food.quantity);
        pending_enchant.set(food.item.enchantment.to_u8());
        modal_mode.set(Some(ModalMode::Edit(idx)));
    };

    let confirm_save = move || {
        let dish = selected_dish.get();

        let qty     = pending_qty.get();
        let enchant = pending_enchant.get();

        let new_entry = build_tracked_food(items.get(),dish, enchant, qty);

        set_data.update(|user_state| {
            match modal_mode.get() {
                Some(ModalMode::Edit(idx)) => {
                    if let Some(entry) = user_state.tracked_foods.get_mut(idx) {
                        let old = entry.clone();
                        *entry = new_entry.clone();

                        spawn_local(async move {
                            send_remove_tracked_food(old).await;
                            send_add_tracked_food(new_entry).await;
                        });
                    }
                }
                _ => {
                    user_state.tracked_foods.push(new_entry.clone());
                    spawn_local(async move {
                        send_add_tracked_food(new_entry).await;
                    });
                }
            }
        });
        close_modal();
    };

    let remove_food = move |idx: usize| {
        set_data.update(|data| {
            if idx < data.tracked_foods.len() {
                let food = data.tracked_foods.remove(idx);
                spawn_local(async move {
                    send_remove_tracked_food(food).await;
                });
            }
        });
    };

    view! {
        <div class="panel panel-food">
            <div class="panel-header">
                <div class="panel-title">"Tracked Food"</div>
            </div>
            <div class="panel-body">
                    <div class="food-grid">
                        <For
                            each={move || tracked_foods.get().into_iter().enumerate().collect::<Vec<_>>()}
                            key={|(_, f)| format!("{}-{}", f.item.unique_name, f.item.enchantment.to_u8())}
                            let:entry
                        >
                            {
                                let (idx, food) = entry;
                                view! {
                                    <FoodCard
                                        idx=idx
                                        food=food
                                        on_click=Callback::new(move |i| open_edit(i))
                                        on_remove=Callback::new(move |i| remove_food(i))
                                    />
                                }
                            }
                        </For>
                </div>

                <button class="add-food-btn" on:click=move |_| open_add()>
                    "+ Add Food Item"
                </button>
            </div>
        </div>

        <Show when=move || modal_mode.get().is_some()>
            <div class="modal-overlay" on:click=move |_| close_modal()>
                <div class="modal" on:click=move |ev| ev.stop_propagation()>

                    <div class="modal-title">
                        {move || if matches!(modal_mode.get(), Some(ModalMode::Edit(_))) {
                            "Edit Food Item"
                        } else {
                            "Add Food Item"
                        }}
                    </div>

                    <Show
                        when=move || dish_selected()
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
                                    key=|d| d.clone()
                                    let:dish
                                >
                                    <div
                                        class="food-sugg-item"
                                        on:click=move |_| selected_dish.set(dish.clone())
                                    >
                                        {dish.clone()}
                                    </div>
                                </For>
                            </div>
                        }
                    >
                        <div class="modal-selected-name">
                            {move || selected_dish.get()}
                        </div>

                        <div class="modal-row">
                            <label>"Quantity"</label>
                            <input
                                type="number"
                                class="modal-small-input"
                                min="1"
                                prop:value=move || pending_qty.get()
                                on:input=move |ev| {
                                    pending_qty.set(event_target_value(&ev).parse().unwrap_or(1));
                                }
                            />
                        </div>

                        <div class="modal-row">
                            <label>"Enchant"</label>
                            <div class="etch-opt-group">
                                {[0, 1, 2, 3].into_iter().map(|e| {
                                    let label = if e == 0 { "None".into() } else { format!(".{e}") };
                                    view! {
                                        <button
                                            class="etch-opt"
                                            class:active=move || pending_enchant.get() == e
                                            on:click=move |_| pending_enchant.set(e)
                                        >
                                            {label}
                                        </button>
                                    }
                                }).collect_view()}
                            </div>
                        </div>

                        <Show when=move || matches!(modal_mode.get(), Some(ModalMode::Add))>
                            <button
                                class="btn-back"
                                on:click=move |_| {
                                    selected_dish.set(String::new());
                                    search_query.set(String::new());
                                }
                            >
                                "← Back to search"
                            </button>
                        </Show>
                    </Show>

                    <div class="modal-actions">
                        <Show when=move || matches!(modal_mode.get(), Some(ModalMode::Edit(_)))>
                            <button
                                class="btn-delete"
                                on:click=move |_| {
                                    if let Some(ModalMode::Edit(idx)) = modal_mode.get() {
                                        remove_food(idx);
                                    }
                                    close_modal();
                                }
                            >
                                "Delete"
                            </button>
                        </Show>

                        <button class="btn-cancel" on:click=move |_| close_modal()>
                            "Cancel"
                        </button>

                        <Show when=move || dish_selected()>
                            <button class="btn-add" on:click=move |_| confirm_save()>
                                {move || if matches!(modal_mode.get(), Some(ModalMode::Edit(_))) {
                                    "Save Changes"
                                } else {
                                    "Add to Tracker"
                                }}
                            </button>
                        </Show>
                    </div>
                </div>
            </div>
        </Show>
    }
}