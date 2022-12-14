use yew::prelude::*;
use stylist::yew::styled_component;
use crate::data::WeatherData;

const IMAGES: [&str; 6] = [
    "./images/Great_Falls.jpg",
    "./images/London.jpg",
    "./images/Paris.jpg",
    "./images/Rio.jpg",
    "./images/Sydney.jpg",
    "./images/New_York.jpg",
];

#[derive(Properties, PartialEq)]
struct DayProps {
    date: String,
    summ: String,
    icon: String,
    temp: String,
    desc: String,
}

#[function_component]
fn Day(DayProps { date, summ, icon, temp, desc }: &DayProps) -> Html {
    html! {
        <section class={classes!("forecast")}>
            <p>{date}</p>
            <p>{summ}</p>
            <p style="font-size: 1.1rem;"><i class={classes!(icon)}></i></p>
            <p>{temp}</p>
            <p>{desc}</p>
        </section>
    }
}

fn parse_code_summ(code: usize) -> String {
    match code {
        0  => "Clear Sky",
        1  => "Mainly Clear",
        2  => "Partly Cloudy",
        3  => "Overcast",
        45 => "Foggy",
        48 => "Depositing Rime Fog",
        51 => "Slight Drizzle",
        53 => "Moderate Drizzle",
        55 => "Heavy Drizzle",
        56 => "Slight Freezing Drizzle",
        57 => "Heavy Freezing Drizzle",
        61 => "Slight Rain",
        63 => "Moderate Rain",
        65 => "Heavy Rain",
        66 => "Slight Freezing Rain",
        67 => "Heavy Freezing Rain",
        71 => "Slight Snow Fall",
        73 => "Moderate Snow Fall",
        75 => "Heavy Snow Fall",
        77 => "Snow Grains",
        80 => "Slight Rain Showers",
        81 => "Moderate Rain Showers",
        82 => "Violent Rain Showers",
        85 => "Slight Snow Showers",
        86 => "Heavy Snow Showers",
        95 => "Thunderstorm",
        96 => "Thunderstorm with Slight Hail",
        99 => "Thunderstorm with Heavy Hail",
        _  => "Unrecognized Weathercode",
    }.into()
}

fn parse_code_icon(code: usize) -> &'static str {
    match code {
        0                 => "wi-day-sunny",
        1                 => "wi-day-sunny-overcast",
        2                 => "wi-day-cloudy",
        3                 => "wi-cloudy",
        45 | 48           => "wi-day-fog",
        51 | 53 | 55      => "wi-day-sprinkle",
        56 | 57           => "wi-day-sleet",
        61 | 63 | 65      => "wi-day-rain",
        66 | 67           => "wi-day-rain-mix",
        71 | 73 | 75 | 77 => "wi-day-snow",
        80 | 81 | 82      => "wi-day-showers",
        85 | 86           => "wi-day-snow-wind",
        95                => "wi-day-thunderstorm",
        96 | 99           => "wi-day-sleet-storm",
        _                 => "wi-alien"
    }
}

fn parse_temp(tmax: f64, tmin: f64) -> String {
    let avg = ((tmax + tmin) / 2f64) as i32;
    if      avg >= 140 { "Impossibly Hot" }
    else if avg >= 120 { "Dangerously Hot" }
    else if avg >= 110 { "Extremely Hot" }
    else if avg >= 100 { "Very Hot" }
    else if avg >= 90  { "Hot" }
    else if avg >= 80  { "Very Warm" }
    else if avg >= 70  { "Warm" }
    else if avg >= 60  { "Moderate" }
    else if avg >= 50  { "Cool" }
    else if avg >= 40  { "Very Cool" }
    else if avg >= 30  { "Slightly Cold" }
    else if avg >= 20  { "Moderately Cold" }
    else if avg >= 10  { "Cold" }
    else if avg >= 0   { "Very Cold" }
    else if avg >= -10 { "Extremely Cold" }
    else if avg >= -70 { "Dangerously Cold" }
    else               { "Impossibly Cold" }
    .into()
}

#[derive(Properties, PartialEq)]
pub struct MLProps {
    pub city: usize,
    pub len: u8,
    pub data: Option<WeatherData>,
}

#[styled_component]
pub fn MiddleLine(MLProps { city, len, data }: &MLProps) -> Html {
    let (city, len, data) = (city.clone(), len.clone(), data.clone());

    let img_obj_pos = match len {
        3 => "center".into(),
        _ => format!("center {}%", match city {
            0 => 60, 1 => 45, 2 => 25, 3 => 30, 4 => 65, _ => 50
        }),
    };

    let days = if let Some(data) = data {
        let time = data.daily.time;
        let code = data.daily.weathercode;
        let tmax = data.daily.temperature_2m_max;
        let tmin = data.daily.temperature_2m_min;
        let unit = data.daily_units.temperature_2m_max;
        let zone = data.timezone_abbreviation;

        (0..(len as usize)).map(move |i| {
            let time = time[i].clone();
            let code = code[i];
            let tmax = tmax[i];
            let tmin = tmin[i];

            let date = format!("{} ({})", time, zone.clone());
            let summ = parse_code_summ(code);
            let icon = format!("wi {}", parse_code_icon(code));
            let temp = format!("{}{} / {}{}", tmax, unit.clone(), tmin, unit.clone());
            let desc = parse_temp(tmax, tmin);

            html!(<Day {date} {summ} {icon} {temp} {desc} />)
        }).collect()
    } else {
        html!(<h1 id="fallback">{"Generating Weather Data ..."}</h1>)
    };

    let day_width = if len == 3 { 98 } else { 99 };

    let class = classes!(css!(
        img { object-position: ${img_obj_pos}; }

        div { grid-template-columns: repeat(${len}, 1fr); }

        div > section { width: ${day_width}%; }
    ));

    html! {
        <section {class}>
            <img src={IMAGES[city]} />
            <div>{ days }</div>
        </section>
    }
}
