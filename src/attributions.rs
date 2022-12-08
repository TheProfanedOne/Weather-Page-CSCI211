use yew::prelude::*;
use yew_hooks::use_title;
use yew_router::prelude::*;
use crate::{app::Route, global_state::{get_global_theme, set_routed_true}};
use stylist::yew::styled_component;

#[styled_component(Attributions)]
pub fn attributions() -> Html {
    set_routed_true();

    let theme = get_global_theme();
    let color1 = if theme { "white" } else { "#003131" };
    let color2 = if theme { "black" } else { "white" };
    let color3 = if theme { "blue" }  else { "cyan" };

    let class = classes!(css!(
        background-color: ${color1};
        color: ${color2};

        ul a { color: ${color3}; }
    ));

    let home = Route::Home;

    use_title("Attributions".to_string());

    html! {
        <main id="attr" {class}>
            <Link<Route> to={home}>
                {"Click Here to Return to the Home Page"}
            </Link<Route>>
            <h1>{"Attributions"}</h1>
            <ul>
                <li><a href="https://commons.wikimedia.org/wiki/File:Great_Falls,_MT,_USA_-_panoramio_(25).jpg">{"Eric Friedebach"}</a>{", "}<a href="https://creativecommons.org/licenses/by/3.0">{"CC BY 3.0"}</a>{", via Wikimedia Commons"}</li>
                <li><a href="https://www.flickr.com/photos/189420050@N03/51861269652">{"Irene Steeves"}</a>{", "}<a href="https://creativecommons.org/licenses/by-nd/2.0">{"CC BY-ND 2.0"}</a>{", via Flickr"}</li>
                <li><a href="https://commons.wikimedia.org/wiki/File:Paris_Night.jpg">{"Benh LIEU SONG"}</a>{", "}<a href="https://creativecommons.org/licenses/by-sa/4.0">{"CC BY-SA 4.0"}</a>{", via Wikimedia Commons"}</li>
                <li><a href="https://commons.wikimedia.org/wiki/File:Cidade_Maravilhosa.jpg">{"Rafael Rabello de Barros"}</a>{", "}<a href="https://creativecommons.org/licenses/by-sa/3.0">{"CC BY-SA 3.0"}</a>{", via Wikimedia Commons"}</li>
                <li><a href="https://commons.wikimedia.org/wiki/File:Sydney_Opera_House_and_Harbour_Bridge_Dusk_(2)_2019-06-21.jpg">{"Benh LIEU SONG (Flickr)"}</a>{", "}<a href="https://creativecommons.org/licenses/by-sa/4.0">{"CC BY-SA 4.0"}</a>{", via Wikimedia Commons"}</li>
                <li><a href="https://www.flickr.com/photos/80038275@N00/31769153946">{"Michael Vadon"}</a>{", "}<a href="https://creativecommons.org/licenses/by/2.0">{"CC BY 2.0"}</a>{", via Flickr"}</li>
                <li><a href="https://github.com/erikflowers/weather-icons">{"Weather Icons"}</a>{" ("}<a href="https://scripts.sil.org/cms/scripts/page.php?item_id=OFL_web">{"SIL OFL 1.1"}</a>{", "}<a href="http://opensource.org/licenses/mit-license.html">{"MIT License"}</a>{", "}<a href="http://creativecommons.org/licenses/by/3.0">{"CC BY 3.0"}</a>{") from erikflowers on github.com"}</li>
            </ul>
        </main>
    }
}
