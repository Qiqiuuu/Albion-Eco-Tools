use wasm_bindgen::JsValue;
use std::collections::HashMap;
use leptos::prelude::{signal, CollectView, Effect, ElementChild, OnAttribute,Update};
use leptos::component;
use leptos::prelude::{ClassAttribute, Get, Set};
use leptos::reactive::spawn_local;
use wasm_bindgen::prelude::wasm_bindgen;
use aet_shared::models::specializations::{CategoryOfSpecializations, Specialization};
use crate::IntoView;
use crate::view;
use crate::WriteSignal;
use crate::ReadSignal;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"], js_name = invoke)]
    async fn invoke_rust(cmd: &str, args: JsValue) -> JsValue;
}

#[component]
pub fn Sidebar(
    active_category: ReadSignal<String>,
    set_active_category: WriteSignal<String>,
    specializations: ReadSignal<HashMap<String, u32>>,
    set_specializations: WriteSignal<HashMap<String, u32>>,
) -> impl IntoView {
    let (categories, _set_categories) = signal(CategoryOfSpecializations::default_list());

    Effect::new(move |_| {
        spawn_local(async move {
            let res = invoke_rust("load_data", JsValue::NULL).await;
            if let Some(json) = res.as_string() {
                if let Ok(data) = serde_json::from_str::<HashMap<String, u32>>(&json) {
                    set_specializations.set(data);
                }
            }
        });
    });


    Effect::new(move |_| {
        let current = specializations.get();
        if !current.is_empty() {
            let json = serde_json::to_string(&current).unwrap();
            let args = serde_wasm_bindgen::to_value(&serde_json::json!({ "json": json })).unwrap();

            spawn_local(async move {
                invoke_rust("save_data", args).await;
            });
        }
    });


    view! {
        <div class="sidebar">
            {move || {
                categories.get().into_iter().map(|category| {
                    let label = category.label().to_string();
                    let label_clone = label.clone();

                    let is_open = move || active_category.get() == label_clone;
                    let is_open_section = is_open().clone();
                    let is_open_items = is_open().clone();


                    let specs = category.get_specs().clone();

                    view! {
                        <div class="sidebar-section">
                            <div
                                class="sidebar-cat"
                                class:open=is_open_section
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
                                {label.clone()}
                                <span class="sidebar-cat-arrow">
                                    {move || if is_open() { "▼" } else { "►" }}
                                </span>
                            </div>

                            <div class="sidebar-items" class:open=is_open_items>
                                {specs.into_iter().map(|spec| {
                                    let name = spec.get_name().to_string();
                                    let name_item = name.clone();
                                    let name_dec = name.clone();
                                    let name_inc = name.clone();

                                    let level = move || specializations.get().get(&name).cloned().unwrap_or(0);

                                    view! {
                                        <div class="sidebar-item">
                                            <span class="sidebar-item-name">{name_item}</span>
                                            <div class="spec-level">
                                                <button
                                                    class="spec-lvl-btn"
                                                    on:click={let n = name_dec; move |e| {
                                                        e.stop_propagation();
                                                        set_specializations.update(|map| {
                                                            let curr = map.get(&n).cloned().unwrap_or(0);
                                                            map.insert(n.clone(), curr.saturating_sub(1));
                                                        });
                                                    }}
                                                >
                                                    "−"
                                                </button>
                                                <span class="spec-lvl-val">{move || level()}</span>
                                                <button
                                                    class="spec-lvl-btn"
                                                    on:click={let n = name_inc; move |e| {
                                                        e.stop_propagation();
                                                        set_specializations.update(|map| {
                                                            let curr = map.get(&n).cloned().unwrap_or(0);
                                                            if curr < 100 { map.insert(n.clone(), curr + 1); }
                                                        });
                                                    }}
                                                >
                                                    "+"
                                                </button>
                                            </div>
                                        </div>
                                    }
                                }).collect_view()}
                            </div>
                        </div>
                    }
                }).collect_view()
            }}
        </div>
    }
}


