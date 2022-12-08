use std::cell::Cell;
use chrono::prelude::*;

use crate::data::WeatherData;

thread_local! {
    static TIME: Cell<DateTime<Local>> = Cell::new(Local::now());
}

static mut ROUTED: bool = false;
static mut CITY: usize  = 0;
static mut DAYS: u8     = 3;
static mut THEME: bool  = true;
static mut DATA: Option<WeatherData> = None;

pub fn is_routed() -> bool {
    unsafe { ROUTED }
}
pub fn set_routed_true() {
    unsafe { ROUTED = true }
}
pub fn set_routed_false() {
    unsafe { ROUTED = false }
}

pub fn get_global_city() -> usize {
    unsafe { CITY }
}
pub fn set_global_city(new_city: usize) {
    unsafe { CITY = new_city }
}

pub fn get_global_days() -> u8 {
    unsafe { DAYS }
}
pub fn toggle_global_days() {
    unsafe {
        if DAYS == 3 {
            DAYS = 5
        } else {
            DAYS = 3
        }
    }
}

pub fn get_global_theme() -> bool {
    unsafe { THEME }
}
pub fn toggle_global_theme() {
    unsafe { THEME = !THEME }
}

pub fn get_global_data() -> Option<WeatherData> {
    unsafe { DATA.clone() }
}
pub fn set_global_data(new_data: Option<WeatherData>) -> bool {
    unsafe {
        if new_data != DATA {
            drop(std::mem::replace(&mut DATA, new_data));
            true
        } else {
            false
        }
    }
}
pub fn clear_global_data(){
    unsafe { drop(std::mem::take(&mut DATA)) }
}

pub fn get_global_time() -> DateTime<Local> {
    TIME.with(|t| (*t).get())
}
pub fn update_global_time() {
    TIME.with(|t| (*t).set(Local::now()))
}