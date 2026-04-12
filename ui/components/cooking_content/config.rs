use leptos::{component, view, IntoView};
use leptos::prelude::*;
use strum::IntoEnumIterator;
use aet_shared::models::config::{AveragePrice, Cities};

#[component]
pub fn Config(
    use_premium: ReadSignal<bool>,
    set_premium: WriteSignal<bool>,
    use_focus: ReadSignal<bool>,
    set_focus: WriteSignal<bool>,
    silver_fee: ReadSignal<u32>,
    set_silver_fee: WriteSignal<u32>,
    avg_days: ReadSignal<String>,
    set_avg_days: WriteSignal<String>,
    is_avg_open: ReadSignal<bool>,
    set_is_avg_open: WriteSignal<bool>,
    selected_city: ReadSignal<String>,
    set_selected_city: WriteSignal<String>,
    is_city_open: ReadSignal<bool>,
    set_is_city_open: WriteSignal<bool>,
) -> impl IntoView{

    let(use_premium,set_premium) = signal(false);
    let(use_focus,set_focus) = signal(false);
    view! {
         <div class="settings-bar">
                <div class="setting-group">
                    <span class="setting-label">"Premium"</span>
                    <label class="toggle">
                        <input
                            type="checkbox"
                            prop:checked=use_premium
                            on:change=move |ev| set_premium.set(event_target_checked(&ev))
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
                            prop:checked=use_focus
                            on:change=move |ev| set_focus.set(event_target_checked(&ev))
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
                        prop:value=move || silver_fee.get()
                        min="0" max="1000"
                        on:input=move |ev| {
                            let val = event_target_value(&ev).parse::<u32>().unwrap_or(0);
                            set_silver_fee.set(val);
                        }
                    />
                </div>
                <div class="sep"></div>
                <div class="setting-group">
                    <span class="setting-label">"Average Price"</span>
                    <div class="custom-select">
                        <div class="select-trigger" on:click=move |_| set_is_avg_open.update(|v| *v = !*v)>
                            {move || format!("{}", avg_days.get())}
                            <span class="arrow">"▼"</span>
                        </div>
                        <div class="select-menu" class:hidden=move || !is_avg_open.get()>
                            {AveragePrice::iter().map(|avg| {
                                let a = avg.clone();
                                view! {
                                    <div class="select-option" on:click=move |_| {
                                        set_avg_days.set(a.clone().to_string());
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
                            {move || format!("{}", selected_city.get())}
                            <span class="arrow">"▼"</span>
                        </div>
                        <div class="select-menu" class:hidden=move || !is_city_open.get()>
                            {Cities::iter().map(|city| {
                                let c = city.clone();
                                view! {
                                    <div class="select-option" on:click=move |_| {
                                        set_selected_city.set(c.clone().to_string());
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