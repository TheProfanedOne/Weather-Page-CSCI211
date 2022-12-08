mod app;
mod data;
mod top_line;
mod middle_line;
mod bottom_line;
mod attributions;
mod forecast;
mod global_state;
mod custom_hooks;
mod forecast_callbacks;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
