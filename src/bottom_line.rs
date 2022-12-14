use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct BLProps {
    pub days: u8,
    pub theme_oncl: Callback<MouseEvent>,
    pub days_oncl: Callback<MouseEvent>,
}

#[function_component]
pub fn BottomLine(BLProps { days, theme_oncl, days_oncl }: &BLProps) -> Html {
    let is_disabled = *days == 3;

    html! {
        <section>
            <div>
                <button type="button" onclick={theme_oncl.clone()}>{"Toggle Dark Theme"}</button>
            </div>
            <div>
                <button type="button" onclick={days_oncl.clone()} disabled={is_disabled}>{"Three-Day Forecast"}</button>
                <button type="button" onclick={days_oncl.clone()} disabled={!is_disabled}>{"Five-Day Forecast"}</button>
            </div>
        </section>
    }
}
