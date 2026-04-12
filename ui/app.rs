use leptos::prelude::*;
use std::collections::HashMap;
use aet_shared::models::config::ActiveTab;
use aet_shared::models::prices::ItemPrice;
use crate::api::items::fetch_all_prices;
use crate::api::user::{fetch_player_specs};
use crate::components::cooking_content::CookingContent;
use crate::components::sidebar::Sidebar;
use crate::components::topbar::Topbar;


#[component]
pub fn App() -> impl IntoView {
    let (active_category, set_active_category) = signal("stews".to_string());
    let (active_tab, set_active_tab) = signal(ActiveTab::Cooking);
    let (specializations, set_specializations) = signal(HashMap::<String, u32>::new());
    let (prices, set_prices) = signal(ItemPrice::new());
    let(use_premium,set_premium) = signal(false);
    let(use_focus,set_focus) = signal(false);

    let user_specs_res = LocalResource::new(|| async move {
        fetch_player_specs().await.specializations
    });

    let all_prices_res = LocalResource::new(|| async move {
        fetch_all_prices().await
    });

    Effect::new(move |_| {
        if let Some(data) = user_specs_res.get() {
            set_specializations.set(data);
        }
    });

    Effect::new(move |_| {
        if let Some(data) = all_prices_res.get() {
            set_prices.set(data);
        }
    });

    provide_context(prices);
    provide_context(set_prices);

    view! {
        <div class="app">
            <Topbar
                active_tab=active_tab
                set_active_tab=set_active_tab
            />
            <div class="main">
                <Sidebar
                    active_category=active_category
                    set_active_category=set_active_category
                    specializations=specializations
                    set_specializations=set_specializations
                />
                {move || match active_tab.get() {
                    ActiveTab::Cooking => view! {
                        <CookingContent
                            use_premium = use_premium
                            set_premium = set_premium
                            use_focus = use_focus
                            set_focus = set_focus
                        />}.into_view(),
                    ActiveTab::Farming => todo!(),
                    ActiveTab::Gathering => todo!(),
                }}
            </div>
        </div>
    }
}