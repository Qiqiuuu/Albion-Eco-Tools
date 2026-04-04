use std::collections::HashMap;
use leptos::prelude::{event_target_value, signal, CollectView, ElementChild, OnAttribute, PropAttribute, StyleAttribute, Update};
use leptos::component;
use leptos::prelude::{ClassAttribute, Get, Set};
use crate::data::sidebar::{CategoryOfSpecializations, Specialization};
use crate::IntoView;
use crate::view;
use crate::WriteSignal;
use crate::ReadSignal;

// ... (importy bez zmian)

#[component]
pub fn Sidebar(
    active_category: ReadSignal<String>,
    set_active_category: WriteSignal<String>,
    specializations: ReadSignal<HashMap<String, u32>>,
    set_specializations: WriteSignal<HashMap<String, u32>>,
) -> impl IntoView {
    let (categories, _set_categories) = signal(CategoryOfSpecializations::default_list());

    view! {
        <div class="sidebar">
            <div class="sb-logo">
                <div class="sb-logo-main">"Albion"<br/>"Profit"</div>
                <div class="sb-logo-sub">"COOKING ENGINE"</div>
            </div>

            <div class="sb-scroll">
                {move || {
                    categories.get().into_iter().map(|category| {
                        let label = category.label().to_string();
                        let label_clone = label.clone();

                        let is_open = move || active_category.get() == label_clone;
                        let is_open_for_arrow = is_open.clone();
                        let is_open_for_tree = is_open.clone();

                        let specs = category.get_specs().clone();

                        view! {
                            <div class="sb-sec">
                                <button
                                    class="sb-sec-hd"
                                    on:click={
                                        let label_click = label.clone();
                                        move |_| {
                                            let current = active_category.get();

                                            if current == label_click {
                                                set_active_category.set(String::new());
                                            } else {
                                                set_active_category.set(label_click.clone());
                                            }
                                        }
                                    }
                                >
                                    <span class="sb-sec-label">{label.clone()}</span>
                                    <span class=move || if is_open_for_arrow() { "sb-arrow open" } else { "sb-arrow" }>
                                        "▶"
                                    </span>
                                </button>

                                {move || is_open_for_tree().then(|| view! {
                                    <div class="spec-tree">
                                        {specs.clone().into_iter().map(|spec| {
                                            let name = spec.get_name();
                                            let level = move || specializations.get().get(name).cloned().unwrap_or(0);
                                            let curr_class = move || if level() > 0 { "spec-row active" } else { "spec-row" };

                                            view! {
                                                <div class=curr_class>
                                                    <span class="spec-nm">{name}</span>
                                                    <div class="spec-bar-w">
                                                        <div
                                                            class="spec-bar-f"
                                                            style=move || format!("width: {}%", level().min(100))
                                                        ></div>
                                                    </div>
                                                    <input
                                                        class="spec-inp"
                                                        type="number"
                                                        prop:value=level
                                                        on:input=move |ev| {
                                                            let val = event_target_value(&ev).parse::<u32>().unwrap_or(0);
                                                            set_specializations.update(|map| {
                                                                map.insert(name.to_string(), val);
                                                            });
                                                        }
                                                    />
                                                </div>
                                            }
                                        }).collect_view()}
                                    </div>
                                })}
                            </div>
                        }
                    }).collect_view()
                }}
            </div>

            <div class="sb-city">
                <div class="sb-city-lbl">"Market City"</div>
                <select class="sb-city-sel">
                    <option value="Caerleon">"Caerleon"</option>
                    <option value="Bridgewatch">"Bridgewatch"</option>
                </select>
            </div>
        </div>
    }
}


