use leptos::prelude::*;
use leptos::{component, view, IntoView};
use leptos::reactive::spawn_local;
use aet_shared::models::items::{Consumable, Enchantment, Item, ItemRegistry, TrackedFood, TrackedFoodMap};
use aet_shared::models::user::UserData;
use crate::api::user::{send_add_tracked_food, send_remove_tracked_food, send_update_tracked_food}; // Dodaj update
use food_card::FoodCard;

pub mod food_card;

#[component]
pub fn Tracking() -> impl IntoView {
    let items = use_context::<ReadSignal<ItemRegistry>>().expect("No items data set");
    let data = use_context::<ReadSignal<UserData>>().expect("No UserData set");
    let set_data = use_context::<WriteSignal<UserData>>().expect("No data setter set");

    let search_query  = RwSignal::new(String::new());
    let modal_open    = RwSignal::new(false);

    let tracked_foods = Memo::new(move |_| data.get().tracked_foods);

    let all_dishes = Memo::new(move |_| {
        items.get().items.values()
            .filter(|e| matches!(e.category, Item::Consumable(Consumable::Food)))
            .filter(|e| e.enchantment == Enchantment::Common)
            .map(|e| (e.name.clone(), e.unique_name.clone()))
            .collect::<Vec<(String, String)>>()
    });

    let suggestions = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() { return Vec::new(); }
        all_dishes.get().into_iter()
            .filter(|(name, _)| name.to_lowercase().contains(&query))
            .collect::<Vec<_>>()
    });

    let close_modal = move || {
        modal_open.set(false);
        search_query.set(String::new());
    };

    let add_food = move |dish_name: String| {
        let new_map = TrackedFoodMap::new_food(dish_name);

        set_data.update(|user_state| {
            user_state.add_tracked_food(new_map.clone());
            spawn_local(async move {
                let _ = send_add_tracked_food(new_map).await;
            });
        });
        close_modal();
    };

    let remove_food = move |base_name: String| {
        set_data.update(|user_state| {
            user_state.remove_tracked_food(&base_name);
            spawn_local(async move {
                let _ = send_remove_tracked_food(base_name).await;
            });
        });
    };

    let update_food_map = move |updated_map: TrackedFoodMap| {
        set_data.update(|user_state| {
            user_state.update_tracked_food(updated_map.clone());
            spawn_local(async move {
                let _ = send_update_tracked_food(updated_map).await;
            });
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
                        each=move || tracked_foods.get()
                        key=|f| f.current_tracked.clone()
                        let:food_map
                    >
                        {
                            let map_clone = food_map.clone();
                            let remove_id = food_map.current_tracked.clone();
                            let update_cb = update_food_map;

                            view! {
                                <FoodCard
                                    food_map=map_clone
                                    on_remove=Callback::new(move |_| remove_food(remove_id.clone()))
                                    on_update=Callback::new(move |new_map| update_cb(new_map))
                                />
                            }
                        }
                    </For>
                </div>

                <button class="add-food-btn" on:click=move |_| modal_open.set(true)>
                    "+ Add Food Item"
                </button>
            </div>
        </div>

        <Show when=move || modal_open.get()>
            <div class="modal-overlay" on:click=move |_| close_modal()>
                <div class="modal" on:click=move |ev| ev.stop_propagation()>
                    <div class="modal-title">"Add Food Item"</div>

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
                            key=|(_,unique)| unique.clone()
                            let:dish
                        >
                            {
                                let (display, unique) = dish;
                                view! {
                                    <div
                                        class="food-sugg-item"
                                        on:click=move |_| add_food(unique.clone())
                                    >
                                        {display.clone()}
                                    </div>
                                }
                            }
                        </For>
                    </div>

                    <div class="modal-actions">
                        <button class="btn-cancel" on:click=move |_| close_modal()>"Cancel"</button>
                    </div>
                </div>
            </div>
        </Show>
    }
}