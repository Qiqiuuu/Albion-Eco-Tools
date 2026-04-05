use leptos::prelude::*;
use strum::IntoEnumIterator;
use crate::data::cooking_content::Cities;

#[component]
pub fn CookingContent(
    is_premium: ReadSignal<bool>,
    set_premium: WriteSignal<bool>,
    use_focus: ReadSignal<bool>,
    set_focus: WriteSignal<bool>,
) -> impl IntoView {

    let (silver_fee, set_silver_fee) = signal(0);
    let (avg_days, set_avg_days) = signal("7".to_string());
    let (selected_city, set_selected_city) = signal("Bracilien".to_string());
    let (is_city_open, set_is_city_open) = signal(false);

    view! {
        <div class="content">
                <div class="settings-bar">
                <div class="setting-group">
                    <span class="setting-label">"Premium"</span>
                    <label class="toggle">
                        <input
                            type="checkbox"
                            prop:checked=is_premium
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
                    <select
                        class="avg-select"
                        on:change=move |ev| set_avg_days.set(event_target_value(&ev))
                        prop:value=move || avg_days.get()
                    >
                        <option value="1">"Avg: Today"</option>
                        <option value="7">"Avg: 7 days"</option>
                        <option value="24">"Avg: 24 days"</option>
                    </select>
                </div>
                <div class="sep"></div>
                <div class="setting-group">
                <span class="setting-label">"City"</span>
                <div class="custom-select-wrapper">
                    <div class="select-trigger" on:click=move |_| set_is_city_open.update(|v| *v = !*v)>
                        {move || format!("{:?}", selected_city.get().replace('"',""))}
                        <span class="arrow">"▼"</span>
                    </div>


                    <div class="custom-options" class:hidden=move || !is_city_open.get()>
                        {Cities::iter().map(|city| {
                            let city_str = city.to_string();
                            view! {

                                <div class="custom-option"
                                    on:click=move |_| {
                                        set_selected_city.set(city_str.clone());
                                        set_is_city_open.set(false);
                                    }>
                                    {format!("{:?}", city)}
                                </div>
                            }
                        }).collect_view()}
                    </div>
                </div>
            </div>
            </div>
        </div>
    }
}
