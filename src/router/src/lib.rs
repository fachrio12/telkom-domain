use yew::prelude::*;
use yew_router::prelude::*;

#[derive(Clone, Routable, PartialEq)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/secure")]
    Secure,
    #[not_found]
    #[at("/404")]
    NotFound,
}


#[function_component(Secure)]
fn secure() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Home));
    html! {
        <div>
            <h1>{ "Secure" }</h1>
            <button {onclick}>{ "Go Home" }</button>
        </div>
    }
}

#[function_component(Home)]
fn home() -> Html {
    let history = use_history().unwrap();

    let onclick = Callback::once(move |_| history.push(Route::Secure));
    html! {
        <div>
            <h1>{ "Home" }</h1>
            <button {onclick}>{ "Go Secure" }</button>
        </div>
    }
}

pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home />
        },
        Route::Secure => html! {
            <Secure />
        },
        Route::NotFound => html! { <h1>{ "404" }</h1> },
    }
}