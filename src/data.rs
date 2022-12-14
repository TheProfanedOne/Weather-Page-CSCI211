// #![allow(dead_code)]
use chrono::prelude::*;
use serde::{Deserialize, Serialize};
use reqwest::Client;
use wasm_bindgen::UnwrapThrowExt;

static mut DATASETS: Option<Vec<WeatherData>> = None;

fn gen_url(start: NaiveDate, end: NaiveDate, loc: &str) -> String {
    let api_file  = "https://api.open-meteo.com/v1/forecast?";
    let api_model = "&models=best_match";
    let api_daily = "&daily=weathercode,temperature_2m_max,temperature_2m_min";
    let api_units = "&temperature_unit=fahrenheit";
    let api_time  = "&timezone=auto";
    let api_dates = format!("&start_date={}&end_date={}", start.format("%F"), end.format("%F"));

    format! {
        "{}{}{}{}{}{}{}",
        api_file, loc, api_model, api_daily, api_units, api_time, api_dates
    }
}

async fn call_api(url: String, client: Client) -> WeatherData {
    let resp = client.get(url).send().await.expect_throw("API Call Failed!");
    resp.json().await.expect_throw("JSON Conversion Failed")
}

pub fn get_data(city: usize) -> Option<WeatherData> {
    unsafe {
        if let Some(ds) = DATASETS.clone() {
            Some(ds[city].clone())
        } else {
            None
        }
    }
}

const LOCATIONS: [&str; 6] = [
    "latitude=47.50&longitude=-111.30",
    "latitude=51.51&longitude=-0.13",
    "latitude=48.85&longitude=2.35",
    "latitude=-22.91&longitude=-43.18",
    "latitude=-33.87&longitude=151.21",
    "latitude=40.71&longitude=-74.01",
];

pub async fn generate_data(rn: NaiveDate) {
    let start = rn + chrono::Duration::days(1);
    let end   = rn + chrono::Duration::days(5);

    let client = Client::new();
    let mut sets = vec![];

    for url in LOCATIONS.iter().map(|&loc| gen_url(start, end, loc)) {
        let data = call_api(url, client.clone()).await;
        sets.push(data);
    }

    unsafe { drop(DATASETS.replace(sets)) }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct DailyUnits {
    pub time: String,
    pub weathercode: String,
    pub temperature_2m_max: String,
    pub temperature_2m_min: String,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct Daily {
    pub time: Vec<String>,
    pub weathercode: Vec<usize>,
    pub temperature_2m_max: Vec<f64>,
    pub temperature_2m_min: Vec<f64>,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq)]
pub struct WeatherData {
    pub latitude: f64,
    pub longitude: f64,
    pub generationtime_ms: f64,
    pub utc_offset_seconds: i32,
    pub timezone: String,
    pub timezone_abbreviation: String,
    pub elevation: f64,
    pub daily_units: DailyUnits,
    pub daily: Daily,
}
