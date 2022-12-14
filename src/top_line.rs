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
    pub city: usize,
    pub app_onch: Callback<Event>,
    pub ref_oncl: Callback<MouseEvent>,
}

#[function_component]
pub fn TopLine(props: &TLProps) -> Html {
    let TLProps { time, zone, city, app_onch: onchange, ref_oncl: onclick } = props.clone();
    let date_str = f_date_str(*time, zone);

    let city_options = CITY_NAMES.iter().enumerate().map(|(i, name)| html! {
        <option value={format!("{}", i)} selected={i == *city}>{*name}</option>
    });

    html! {
        <section>
            <p>{date_str}</p>
            <div>
                <form id="city_form" action="" method="post">
                    <fieldset>
                        <label for="city_menu">{"City: "}</label>
                        <select id="city_menu" name="selected_city" size="1" {onchange}>
                            { for city_options }
                        </select>
                    </fieldset>
                </form>
                <button type="button" {onclick}>{"Refresh"}</button>
            </div>
        </section>
    }
}
