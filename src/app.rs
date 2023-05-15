// Contains the App first layer structure. Mainly routing.

use yew_router::prelude::*;
use yew::prelude::*;


use super::components::{
    pages::{
        // Fundamentals
        home::*,
        not_found::*,

    },
    inc::{
        navbar::*,
    }
};


#[derive(Clone, Routable, PartialEq)]
pub enum Route { // Where (Connects path and Route enum)
    #[at("/")]
    Home,

    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(routes: Route) -> Html { // What (Connects Route enum to Component)
    match routes {
        Route::Home => html! { <Home /> },

        Route::NotFound => html! { <NotFound /> },
    }
}

#[function_component]
pub fn App() -> Html {
    html! {
        <BrowserRouter>
            <Navbar />
            <Switch<Route> render={switch} /> // <- must be child of <BrowserRouter>
        </BrowserRouter>
    }
}