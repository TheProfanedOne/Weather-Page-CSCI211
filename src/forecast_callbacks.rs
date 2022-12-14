use yew::prelude::*;
use crate::{data::get_data, global_state::*};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlSelectElement;

pub fn generate_top_line_callbacks(
    trigger: UseForceUpdateHandle
) -> (Callback<Event>, Callback<MouseEvent>) {
    let app_call = {
        let trigger = trigger.clone();

        Callback::from(move |evt: Event| {
            let trigger = trigger.clone();

            let new_city = evt.target()
                .expect_throw("Could not get event target.")
                .dyn_into::<HtmlSelectElement>()
                .expect_throw("Could not cast to HtmlSelectElement.")
                .selected_index() as usize;

            set_global_city(new_city);
            if set_global_data(get_data(new_city)) {
                trigger.force_update();
            }
        })
    };

    let ref_call = Callback::from(move |_: MouseEvent| {
        let trigger = trigger.clone();

        update_global_time();
        clear_global_data();
        trigger.force_update();
    });

    (app_call, ref_call)
}

pub fn generate_bottom_line_callbacks(
    trigger: UseForceUpdateHandle,
) -> [Callback<MouseEvent>; 2] {
    let trigger = trigger.clone();

    let theme_call = {
        let trigger = trigger.clone();

        Callback::from(move |_: MouseEvent| {
            let trigger = trigger.clone();

            toggle_global_theme();
            trigger.force_update();
        })
    };

    let days_call = Callback::from(move |_: MouseEvent| {
        let trigger = trigger.clone();

        toggle_global_days();
        trigger.force_update();
    });

    [theme_call, days_call]
}
