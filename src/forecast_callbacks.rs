use yew::prelude::*;
use crate::{data::get_data, global_state::*};
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::{FormData, HtmlFormElement};

pub fn generate_top_line_callbacks(
    trigger: UseForceUpdateHandle
) -> [Callback<MouseEvent>; 2] {
    let app_call = {
        let trigger = trigger.clone();

        Callback::from(move |_: MouseEvent| {
            let trigger = trigger.clone();

            let document = gloo::utils::document();
            let city_form = document.forms().named_item("city_form")
                .expect_throw("No element named `city_form`")
                .dyn_into::<HtmlFormElement>()
                .expect_throw("Dynamic casting failed");
            
            let form_data = FormData::new_with_form(&city_form).expect_throw("FormData Failed");
            let new_city = form_data.get("selected_city").as_string().expect_throw("FormData::get Failed")
                .parse().expect_throw("Usize Casting Failed");

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

    [app_call, ref_call]
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
