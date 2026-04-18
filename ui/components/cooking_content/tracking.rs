use leptos::prelude::*;
use leptos::{component, view, IntoView};
use aet_shared::models::items::{Consumable, Enchantment, Item, ItemEntity, ItemRegistry};

#[derive(Clone, Debug, PartialEq)]
pub struct TrackedFood {
    pub item: ItemEntity,
    pub quantity: i32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ModalMode {
    Add,
    Edit(usize),
}

fn tier_info(food: &TrackedFood) -> (&'static str, &'static str) {
    let s = format!("{:?}", food.item.tier);
    if s.contains('1')      { ("tier-badge t1", "T1") }
    else if s.contains('2') { ("tier-badge t2", "T2") }
    else if s.contains('3') { ("tier-badge t3", "T3") }
    else if s.contains('4') { ("tier-badge t4", "T4") }
    else if s.contains('5') { ("tier-badge t5", "T5") }
    else if s.contains('6') { ("tier-badge t6", "T6") }
    else if s.contains('7') { ("tier-badge t7", "T7") }
    else                    { ("tier-badge t8", "T8") }
}

fn enchant_info(food: &TrackedFood) -> (&'static str, &'static str, &'static str) {
    match food.item.enchantment {
        Enchantment::Uncommon    => ("@1", ".1", "enchant-badge e1"),
        Enchantment::Rare        => ("@2", ".2", "enchant-badge e2"),
        Enchantment::Exceptional => ("@3", ".3", "enchant-badge e3"),
        _                        => ("",   "",   ""),
    }
}

#[component]
pub fn Tracking() -> impl IntoView {
    let items = use_context::<ReadSignal<ItemRegistry>>().expect("No items data set");

    let search_query    = RwSignal::new(String::new());
    let tracked_foods   = RwSignal::new(Vec::<TrackedFood>::new());
    let modal_mode      = RwSignal::new(None::<ModalMode>);
    let selected_dish   = RwSignal::new(None::<ItemEntity>);
    let pending_qty     = RwSignal::new(10i32);
    let pending_enchant = RwSignal::new(0u8);

    let all_dishes = Memo::new(move |_| {
        items.get().items.values()
            .filter(|e| matches!(e.category, Item::Consumable(Consumable::Food)))
            .cloned()
            .collect::<Vec<ItemEntity>>()
    });

    let suggestions = Memo::new(move |_| {
        let query = search_query.get().to_lowercase();
        if query.is_empty() { return Vec::new(); }
        all_dishes.get().into_iter()
            .filter(|item| item.name.to_lowercase().contains(&query))
            .take(5)
            .collect::<Vec<_>>()
    });

    let close_modal = move || {
        modal_mode.set(None);
        selected_dish.set(None);
        search_query.set(String::new());
        pending_qty.set(10);
        pending_enchant.set(0);
    };

    let open_add = move || {
        modal_mode.set(Some(ModalMode::Add));
    };

    let open_edit = move |idx: usize| {
        let foods = tracked_foods.get();
        if let Some(food) = foods.get(idx) {
            selected_dish.set(Some(food.item.clone()));
            pending_qty.set(food.quantity);
            pending_enchant.set(match food.item.enchantment {
                Enchantment::Uncommon    => 1,
                Enchantment::Rare        => 2,
                Enchantment::Exceptional => 3,
                _                        => 0,
            });
            modal_mode.set(Some(ModalMode::Edit(idx)));
        }
    };

    let confirm_save = move || {
        if let Some(dish) = selected_dish.get() {
            let enchantment = match pending_enchant.get() {
                1 => Enchantment::Uncommon,
                2 => Enchantment::Rare,
                3 => Enchantment::Exceptional,
                _ => Enchantment::Common,
            };
            let mut final_item = dish.clone();
            final_item.enchantment = enchantment;
            tracked_foods.update(|list| {
                let new_entry = TrackedFood {
                    item:     final_item,
                    quantity: pending_qty.get(),
                };
                match modal_mode.get() {
                    Some(ModalMode::Edit(idx)) => {
                        if let Some(entry) = list.get_mut(idx) {
                            *entry = new_entry;
                        }
                    }
                    _ => list.push(new_entry),
                }
            });
            close_modal();
        }
    };

    let remove_food = move |idx: usize| {
        tracked_foods.update(|list| { list.remove(idx); });
    };

    view! {
        <div class="panel panel-food">
            <div class="panel-header">
                <div class="panel-title">"Tracked Food"</div>
            </div>
            <div class="panel-body">

                <For
                    each={move || tracked_foods.get().into_iter().enumerate().collect::<Vec<_>>()}
                    key={|(_, food)| format!("{}_{:?}", food.item.unique_name, food.item.enchantment)}
                    let:entry
                >
                    {
                        let (idx, food) = entry;
                        let (tier_class, tier_label)                     = tier_info(&food);
                        let (enchant_suffix, enchant_label, enchant_cls) = enchant_info(&food);
                        let name         = food.item.name.clone();
                        let qty          = food.quantity;
                        let show_enchant = !enchant_label.is_empty();
                        let img_url      = format!(
                            "https://render.albiononline.com/v1/item/{}{}.png",
                            food.item.unique_name, enchant_suffix
                        );

                        view! {
                            <div class="food-card" on:click=move |_| open_edit(idx)>
                                <div class="food-icon">
                                    <img src=img_url alt=name.clone() />
                                </div>
                                <div class="food-info">
                                    <div class="food-card-name">{name}</div>
                                    <div class="food-card-meta">
                                        <span class=tier_class>{tier_label}</span>
                                        {if show_enchant {
                                            Some(view! {
                                                <span class=enchant_cls>{enchant_label}</span>
                                            })
                                        } else {
                                            None
                                        }}
                                        <span class="qty-sep">"×"</span>
                                        <span class="food-qty">{qty}</span>
                                    </div>
                                </div>
                                <button
                                    class="food-card-remove"
                                    on:click=move |ev| {
                                        ev.stop_propagation();
                                        remove_food(idx);
                                    }
                                >
                                    "×"
                                </button>
                            </div>
                        }
                    }
                </For>

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
                                    each={move || suggestions.get()}
                                    key={|dish| dish.unique_name.clone()}
                                    let:dish
                                >
                                    {
                                        let d_name     = dish.name.clone();
                                        let dish_clone = dish.clone();
                                        view! {
                                            <div
                                                class="food-sugg-item"
                                                on:click=move |_| selected_dish.set(Some(dish_clone.clone()))
                                            >
                                                {d_name}
                                            </div>
                                        }
                                    }
                                </For>
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
                                {vec![0u8, 1, 2, 3].into_iter().map(|e| {
                                    let label = if e == 0 {
                                        "None".to_string()
                                    } else {
                                        format!(".{e}")
                                    };
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
                                    selected_dish.set(None);
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

                        <Show when=move || selected_dish.get().is_some()>
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