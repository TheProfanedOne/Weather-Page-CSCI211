use chrono::prelude::*;
use wasm_bindgen::UnwrapThrowExt;
use yew::prelude::*;
use yew_hooks::prelude::*;
use yew_router::prelude::*;
use crate::app::Route;
use crate::top_line::TopLine;
use crate::middle_line::MiddleLine;
use crate::bottom_line::BottomLine;
use crate::global_state::*;
use crate::custom_hooks::*;
use crate::forecast_callbacks::*;
use stylist::yew::styled_component;
use localzone::get_local_zone;
use chrono_tz::{TZ_VARIANTS, OffsetName};

#[styled_component(Forecast)]
pub fn forecast() -> Html {
    let trigger = use_force_update();

    let city = get_global_city();
    let days = get_global_days();
    let time = get_global_time();
    let data = get_global_data();

    let timezone_str = (*use_memo(|_| get_local_zone().expect_throw("Could not locate timezone."), time)).clone();
    let zone = (*use_memo(|tz_str| TZ_VARIANTS.iter()
            .find(|&tz| tz.name() == tz_str)
            .expect_throw("Failed to get timezone")
            .offset_from_utc_datetime(&time.naive_local())
            .abbreviation().to_string()
    , timezone_str)).clone();

    use_data_refresh(trigger.clone(), city, time, &data);

    let [app_oncl, ref_oncl] = generate_top_line_callbacks(trigger.clone());

    let [theme_oncl, days_oncl] = generate_bottom_line_callbacks(trigger.clone());
    
    let class = classes!({
        let main_width = if days == 3 { 702 } else { 1170 };
        let theme = get_global_theme();
        let color1 = if theme { "#FFFFFF" } else { "#003131" };
        let color2 = if theme { "#000000" } else { "#FFFFFF" };

        let fallback_margin = 77.5;
        
        css!(
            background-color: ${color1};
            color: ${color2};
            
            > div { width: ${main_width}px; }

            #fallback {
                width: ${main_width}px ${"!important"};
                margin-top: ${fallback_margin}px;
                margin-bottom: ${fallback_margin}px;
            }

            .forecast { border-color: ${color2}; }
        )
    });

    use_title(format!("{}-Day Forecast", match days {
        3 => "Three",
        _ => "Five",
    }));

    html! {
        <main id="weather" {class}>
            <h1>{"CSCI-211 \u{00A0}\u{2014}\u{00A0} Weather Page"}</h1>
            <div>
                <TopLine {time} {zone} {app_oncl} {ref_oncl} />
                <MiddleLine {city} len={days} {data} />
                <BottomLine {days} {theme_oncl} {days_oncl} />
            </div>
            <Link<Route> to={Route::Attributions}>
                {"Click Here for Attributions"}
            </Link<Route>>
        </main>
    }
}
