use leptos::prelude::*;
use leptos::task::spawn_local;
use strum::IntoEnumIterator;
use aet_shared::models::config::ActiveTab;
use aet_shared::models::user::UserData;
use crate::api::user::send_active_tab_update;

#[component]
pub fn Topbar(
) -> impl IntoView {

    let data = use_context::<ReadSignal<UserData>>().expect("No user data set");
    let set_data = use_context::<WriteSignal<UserData>>().expect("No user data set");


    let change_tab = move |new_tab: ActiveTab| {
        set_data.update(|d| d.active_tab = new_tab);
        spawn_local(async move {send_active_tab_update(new_tab).await});
    };

    view! {
        <div class="topbar">
            <div class="topbar-logo">"ALBION " <span>"ECON"</span></div>
            <div class="tabs">
                {ActiveTab::iter().map(|tab| {
                    view! {
                        <div
                            class="tab"
                            class:active=move || data.get().active_tab == tab
                            on:click=move |_| change_tab(tab)
                        >
                        {tab.to_string()}
                        </div>
                    }
                }).collect_view()}
            </div>

            <div class="topbar-actions">
                <span class="fetch-status" id="fetchStatus">"—"</span>

                <button
                    class="btn-fetch"
                    // need to do api here
                    // on:click=on_fetch
                >
                    "↻ Fetch Prices"
                </button>
            </div>
        </div>
    }
}