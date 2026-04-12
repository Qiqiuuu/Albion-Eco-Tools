pub mod config;
pub mod enchant;
pub mod tracking;
pub mod ingredients;

use leptos::prelude::*;
use crate::components::cooking_content::config::Config;
use crate::components::cooking_content::enchant::Enchant;
use crate::components::cooking_content::ingredients::Ingredients;
use crate::components::cooking_content::tracking::Tracking;

#[component]
pub fn CookingContent(
    use_premium: ReadSignal<bool>,
    set_premium: WriteSignal<bool>,
    use_focus: ReadSignal<bool>,
    set_focus: WriteSignal<bool>,
) -> impl IntoView {

    let (silver_fee, set_silver_fee) = signal(0);
    let (avg_days, set_avg_days) = signal("Day".to_string());
    let (is_avg_open, set_is_avg_open) = signal(false);
    let (selected_city, set_selected_city) = signal("Bracilien".to_string());
    let (is_city_open, set_is_city_open) = signal(false);

    view! {
        <div class="content">
            <Config
                use_premium = use_premium
                set_premium = set_premium
                use_focus = use_focus
                set_focus = set_focus
                silver_fee=silver_fee
                set_silver_fee=set_silver_fee
                avg_days=avg_days
                set_avg_days=set_avg_days
                is_avg_open=is_avg_open
                set_is_avg_open=set_is_avg_open
                selected_city=selected_city
                set_selected_city=set_selected_city
                is_city_open=is_city_open
                set_is_city_open=set_is_city_open
            />
            <Enchant
                selected_city=selected_city
            />
            <div class="panels">
                <Tracking/>
                <Ingredients/>
            </div>


        </div>
    }
}
