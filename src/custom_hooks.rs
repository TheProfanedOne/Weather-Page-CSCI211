use chrono::prelude::*;
use yew::prelude::*;
use wasm_bindgen_futures::spawn_local;
use crate::{data::{WeatherData, generate_data, get_data}, global_state::set_global_data};

#[hook]
pub fn use_data_refresh(
    trigger: UseForceUpdateHandle,
    city: usize,
    time: DateTime<Local>,
    data: &Option<WeatherData>
) {
    let data = data.clone();
    let trigger = trigger.clone();
    use_effect_with_deps(move |&rn| {
        spawn_local(async move {
            if data == None {
                generate_data(rn.date_naive()).await;
                set_global_data(get_data(city));
                trigger.force_update();
            }
        });
        || ()
    }, time);
}
