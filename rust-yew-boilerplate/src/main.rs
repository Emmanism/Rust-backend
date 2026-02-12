use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod containers;
mod screens;

use crate::screens::home::Home;
use crate::screens::not_found::NotFound;
use crate::screens::todo::TodoScreen;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Home,
    #[at("/todo")]
    Todo,
    #[at("/*path")]
    NotFound,
}

#[function_component]
fn App() -> Html {
    fn switch(routes: Route) -> Html {
        match routes {
            Route::Home => html! { <Home /> },
            Route::Todo => html! {<TodoScreen />},
            Route::NotFound => html! { <NotFound /> },
        }
    }

    html! {
        <BrowserRouter>
             <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

// /about
// /contact
// / => home

// <Render id="root" />
