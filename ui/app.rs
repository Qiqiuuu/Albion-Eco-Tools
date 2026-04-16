use leptos::prelude::*;
use aet_shared::models::config::ActiveTab;
use aet_shared::models::items::ItemRegistry;
use aet_shared::models::prices::{PriceMap};
use aet_shared::models::user::UserData;
use crate::api::items::{fetch_all_items, fetch_all_prices};
use crate::api::user::{fetch_player_data};
use crate::components::cooking_content::CookingContent;
use crate::components::sidebar::Sidebar;
use crate::components::topbar::Topbar;


#[component]
pub fn App() -> impl IntoView {
    let (prices, set_prices) = signal(PriceMap::new());
    let (items, set_items) = signal(ItemRegistry::default());
    let (data,set_data) = signal(UserData::default());

    let prices_res = LocalResource::new(|| async move { fetch_all_prices().await });
    let items_res  = LocalResource::new(|| async move { fetch_all_items().await });
    let data_res  = LocalResource::new(|| async move { fetch_player_data().await });

    // Effect::new(move |_| { if let Some(d) = prices_res.get() { set_prices.set(d); } });
    Effect::new(move |_| { if let Some(d) = items_res.get()  { set_items.set(d); } });
    Effect::new(move |_| { if let Some(d) = data_res.get()  { set_data.set(d); } });


    provide_context(prices);
    provide_context(set_prices);
    provide_context(items);
    provide_context(set_items);
    provide_context(data);
    provide_context(set_data);

    view! {
        <div class="app">
            <Topbar/>
            <div class="main">
                <Sidebar/>
                {move || match data.get().active_tab {
                    ActiveTab::Cooking => view! {<CookingContent/>}.into_view(),
                    ActiveTab::Farming   => todo!(),
                    ActiveTab::Gathering => todo!(),
                }}
            </div>
        </div>
    }
}

