use leptos::{component, view, IntoView};
use leptos::prelude::*;
use strum::IntoEnumIterator;
use aet_shared::models::config::{AveragePrice, Cities};
use aet_shared::models::user::UserData;

#[component]
pub fn Config(
) -> impl IntoView{

    let data = use_context::<ReadSignal<UserData>>().expect("No user data set");
    let set_data = use_context::<WriteSignal<UserData>>().expect("No user data set");

    let (is_avg_open, set_is_avg_open) = signal(false);
    let (is_city_open, set_is_city_open) = signal(false);

    view! {
         <div class="settings-bar">
                <div class="setting-group">
                    <span class="setting-label">"Premium"</span>
                    <label class="toggle">
                        <input
                            type="checkbox"
                            prop:checked=data.get().use_premium
                            on:change=move |ev| {
                            let val = event_target_checked(&ev);
                            set_data.update(|d| d.use_premium = val);
                        }
                        />
                        <span class="toggle-slider"></span>
                    </label>
                </div>
                <div class="sep"></div>
                <div class="setting-group">
                    <span class="setting-label">Focus</span>
                    <label class="toggle">
                        <input
                            type="checkbox"
                            prop:checked=data.get().use_focus
                            on:change=move |ev| {
                            let _val = event_target_checked(&ev);
                            set_data.update(|d| d.use_focus = _val);
                        }
                        />
                        <span class="toggle-slider"></span>
                    </label>
                </div>
                <div class="sep"></div>
                <div class="setting-group">
                    <span class="setting-label">"Silver Fee %"</span>
                    <input
                        class="setting-input"
                        type="number"
                        prop:value=move || data.get().silver_fee
                        min="0" max="1000"
                        on:input=move |ev| {
                        let _val = event_target_value(&ev).parse::<u32>().unwrap_or(0);
                        set_data.update(|d| d.silver_fee = _val);
                    }
                    />
                </div>
                <div class="sep"></div>
                <div class="setting-group">
                    <span class="setting-label">"Average Price"</span>
                    <div class="custom-select">
                        <div class="select-trigger" on:click=move |_| set_is_avg_open.update(|v| *v = !*v)>
                            {move || format!("{}", data.get().avg)}
                            <span class="arrow">"▼"</span>
                        </div>
                        <div class="select-menu" class:hidden=move || !is_avg_open.get()>
                            {AveragePrice::iter().map(|avg| {
                                let a = avg.clone();
                                view! {
                                    <div class="select-option" on:click=move |_| {
                                        set_data.update(|d| d.avg = a.clone());
                                        set_is_avg_open.set(false);
                                    }>
                                        {format!("{}", avg)}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>

                <div class="sep"></div>

                <div class="setting-group">
                    <span class="setting-label">"City"</span>
                    <div class="custom-select">
                        <div class="select-trigger" on:click=move |_| set_is_city_open.update(|v| *v = !*v)>
                            {move || format!("{}", data.get().city)}
                            <span class="arrow">"▼"</span>
                        </div>
                        <div class="select-menu" class:hidden=move || !is_city_open.get()>
                            {Cities::iter().map(|city| {
                                let c = city.clone();
                                view! {
                                    <div class="select-option" on:click=move |_| {
                                        set_data.update(|d| d.city = c.clone());
                                        set_is_city_open.set(false);
                                    }>
                                        {format!("{}", city)}
                                    </div>
                                }
                            }).collect_view()}
                        </div>
                    </div>
                </div>
            </div>
    }
}