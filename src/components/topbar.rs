use leptos::prelude::{ElementChild};
use leptos::*;
use leptos::prelude::{ClassAttribute, Get, OnAttribute, ReadSignal, Set, WriteSignal};
use crate::data::topbar::ActiveTab;

#[component]
pub fn Topbar(
    active_tab: ReadSignal<ActiveTab>,
    set_active_tab: WriteSignal<ActiveTab>
) -> impl IntoView {
    view! {
        <div class="topbar">
            <button
                class=move || if active_tab.get() == ActiveTab::Cooking { "tab active" } else { "tab" }
                on:click=move |_| set_active_tab.set(ActiveTab::Cooking)
            >
                "COOKING"
            </button>

            <button
                class=move || if active_tab.get() == ActiveTab::Farming { "tab active" } else { "tab" }
                on:click=move |_| set_active_tab.set(ActiveTab::Farming)
            >
                "FARMING"
            </button>

            <button class="tab dis">"GATHERING"</button>
        </div>
    }
}