use yew::prelude::*;
use chrono::prelude::*;

pub const CITY_NAMES: [&str; 6] = [
    "Great Falls, MT, US",
    "London, UK",
    "Paris, FR",
    "Rio de Janeiro, BR",
    "Sydney, AU",
    "New York, NY, US",
];

fn f_date_str(rn: DateTime<Local>, zone: &str) -> String {
    rn.format(&format! {
        "Today is: %A %b %d, %Y ({})", zone
    }).to_string()
}

#[derive(Properties, PartialEq)]
pub struct TLProps {
    pub time: DateTime<Local>,
    pub zone: String,
    pub app_oncl: Callback<MouseEvent>,
    pub ref_oncl: Callback<MouseEvent>,
}

#[function_component(TopLine)]
pub fn top_line(props: &TLProps) -> Html {
    let TLProps { time, zone, app_oncl, ref_oncl } = props.clone();
    let date_str = f_date_str(*time, zone);

    let city_options = CITY_NAMES.iter().enumerate().map(|(i, name)| html! {
        <option value={format!("{}", i)} selected={i == 0}>{*name}</option>
    });

    html! {
        <section>
            <p>{date_str}</p>
            <div>
                <form id="city_form" action="" method="post">
                    <fieldset>
                        <label for="city_menu">{"City: "}</label>
                        <select id="city_menu" name="selected_city" size="1">
                            { for city_options }
                        </select>
                    </fieldset>
                </form>
                <button type="button" onclick={app_oncl.clone()}>{"Apply"}</button>
                <button type="button" onclick={ref_oncl.clone()}>{"Refresh"}</button>
            </div>
        </section>
    }
}
