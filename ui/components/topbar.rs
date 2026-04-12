use leptos::prelude::*;
use aet_shared::models::config::ActiveTab;

#[component]
pub fn Topbar(
    active_tab: ReadSignal<ActiveTab>,
    set_active_tab: WriteSignal<ActiveTab>
) -> impl IntoView {

    let on_fetch = move |_| {
        todo!("SFSDF")
    };

    view! {
        <div class="topbar">
            <div class="topbar-logo">"ALBION " <span>"ECON"</span></div>
            <div class="tabs">
                <div
                    class="tab"
                    class:active=move || active_tab.get() == ActiveTab::Cooking
                    on:click=move |_| set_active_tab.set(ActiveTab::Cooking)
                    >
                    "Cooking"
                </div>
                <div
                    class="tab"
                    class:active=move || active_tab.get() == ActiveTab::Farming
                    on:click=move |_| set_active_tab.set(ActiveTab::Farming)
                >
                    "Farming"
                </div>
                <div class="tab dis">"Gathering"</div>
            </div>

            <div class="topbar-actions">
                <span class="fetch-status" id="fetchStatus">"—"</span>

                <button
                    class="btn-fetch"
                    on:click=on_fetch
                >
                    "↻ Fetch Prices"
                </button>
            </div>
        </div>
    }
}