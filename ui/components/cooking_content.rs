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
) -> impl IntoView {

    view! {
        <div class="content">
            <Config/>
            // <Enchant/>
            <div class="panels">
                // <Tracking/>
                // <Ingredients/>
            </div>
        </div>
    }
}
