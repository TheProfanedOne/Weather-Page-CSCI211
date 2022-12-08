use yew_router::prelude::*;
use yew::prelude::*;
use crate::attributions::Attributions;
use crate::forecast::Forecast;
use stylist::yew::styled_component;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Home,
    
    #[at("/attributions")]
    Attributions,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Forecast />
        },
        Route::Attributions => {
            html!(<Attributions />)
        },
    }
}

#[styled_component(App)]
pub fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
