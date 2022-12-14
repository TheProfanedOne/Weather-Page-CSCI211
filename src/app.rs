use yew_router::prelude::*;
use yew::prelude::*;
use crate::attributions::Attributions;
use crate::forecast::Forecast;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    #[not_found]
    Home,
    #[at("/forecast")]
    Forecast,
    #[at("/attributions")]
    Attributions,
}

fn switch(routes: Route) -> Html {
    match routes {
        Route::Home => html! {
            <Redirect<Route> to={Route::Forecast} />
        },
        Route::Forecast => html! {
            <Forecast />
        },
        Route::Attributions => html! {
            <Attributions />
        },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
