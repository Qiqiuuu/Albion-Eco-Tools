use leptos::prelude::*;
use std::collections::HashMap;
use crate::api::user::fetch_player_data;
use crate::components::sidebar::Sidebar;
use crate::components::topbar::Topbar;
use crate::components::workspace::Workspace;
use crate::data::topbar::ActiveTab;

#[component]
pub fn App() -> impl IntoView {
    let (active_category, set_active_category) = signal("stews".to_string());
    let (active_tab, set_active_tab) = signal(ActiveTab::Cooking);
    let (specializations, set_specializations) = signal(HashMap::<String, u32>::new());
    let (_prices, _set_prices) = signal(HashMap::<String, f64>::new());

    let user_specializations = LocalResource::new(
        || async move {
            fetch_player_data().await.specializations
        }
    );

    Effect::new(move |_| {
        if let Some(data) = user_specializations.get() {
            set_specializations.set(data);
        }
    });

    view! {
        <div class="app">
            <Sidebar
                active_category=active_category
                set_active_category=set_active_category
                specializations=specializations
                set_specializations=set_specializations
            />
            <div class="main">
                <Topbar
                    active_tab=active_tab
                    set_active_tab=set_active_tab
                />
                <Workspace/>
            </div>

        </div>
    }
}