
use leptos::prelude::{CollectView, ElementChild, OnAttribute, ReadSignal, StyleAttribute, Update, WriteSignal};
use leptos::component;
use leptos::context::use_context;
use leptos::prelude::{ClassAttribute, Get};
use leptos::reactive::spawn_local;
use aet_shared::models::user::UserData;
use crate::api::user::{send_specs_update};
use crate::IntoView;
use crate::view;


#[component]
pub fn Sidebar() -> impl IntoView {
    let data = use_context::<ReadSignal<UserData>>().expect("No user data set");
    let set_data = use_context::<WriteSignal<UserData>>().expect("No user data set");

    view! {
        <div class="sidebar">
            {move || {
                data.get().specializations.into_iter().map(|category| {
                    let cat_id = category.id;
                    let label = category.get_label();
                    let specs = category.get_specs();


                    let is_open = move || data.get().active_category == cat_id;

                    view! {
                        <div class="sidebar-section">
                            <div
                                class="sidebar-cat"
                                class:open=is_open
                                on:click=move |_| {
                                    set_data.update(|u| u.active_category = cat_id);
                                }
                            >
                                {label}
                                <span class="sidebar-cat-arrow">
                                    {move || if is_open() { "▼" } else { "►" }}
                                </span>
                            </div>

                            <div class="sidebar-items"
                                 style:display=move || if is_open() { "block" } else { "none" }>
                                {specs.into_iter().map(|spec| {
                                    let spec_id = spec.id;
                                    let spec_name = spec.get_name();
                                    let spec_level = move || {
                                        data.get()
                                            .specializations
                                            .iter()
                                            .find(|cat| cat.id == cat_id)
                                            .and_then(|cat| cat.get_specs().into_iter().find(|s| s.id == spec_id))
                                            .map(|s| s.get_level())
                                            .unwrap_or(0)
                                    };

                                    view! {
                                        <div class="sidebar-item">
                                            <span class="sidebar-item-name">{spec_name}</span>
                                            <div class="spec-level">
                                                <button
                                                    class="spec-lvl-btn"
                                                    on:click=move |e| {
    e.stop_propagation();
    let new_lvl = (spec_level() + 1).min(100);
    leptos::logging::log!("KLIK + spec_id={:?} new_lvl={}", spec_id, new_lvl);
    set_data.update(|d| d.set_spec_level(spec_id, new_lvl));
    spawn_local(async move {
        leptos::logging::log!("spawn_local START");
        let result = send_specs_update(spec_id, new_lvl).await;
        leptos::logging::log!("spawn_local END result={:?}", result);
    });
}
                                                >
                                                    "−"
                                                </button>

                                                <span class="spec-lvl-val">{spec_level}</span>

                                                <button
                                                    class="spec-lvl-btn"
                                                    on:click=move |e| {
                                                        e.stop_propagation();
                                                        let new_lvl = (spec_level() + 1).min(100);
                                                        set_data.update(|d| d.set_spec_level(spec_id, new_lvl));
                                                        spawn_local(async move {
                                                            let _ = send_specs_update(spec_id, new_lvl).await;
                                                        });
                                                    }
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