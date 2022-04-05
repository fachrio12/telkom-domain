use yew::prelude::*;
use yew_router::prelude::*;
use route::{
    Route,
    RouteGetStarted,
};

use home::Home;
use articles_home::ArticlesHome;
use get_started_main::GetStartedMain;


// #[function_component(Secure)]
// fn secure() -> Html {
//     let history = use_history().unwrap();

//     let onclick = Callback::once(move |_| history.push(Route::Home));
//     html! {
//         <div>
//             <h1>{ "Secure" }</h1>
//             <button {onclick}>{ "Go Home" }</button>
//         </div>
//     }
// }


pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home/>
        },
        Route::Articles => html! {
            <ArticlesHome/>
        },
        Route::GetStarted => html! {
            <Switch<RouteGetStarted> render={Switch::render(switch_get_started)} />
        },
        Route::GetStartedHome => html! {
            <GetStartedMain topic={ String::from("Home") } sub_topic={ String::from("Home") } />
        },
        Route::Secure => html! {
            <div>{"SECURE"}</div>
        },
        Route::NotFound => {
            log::info!("not found from main router");
            html! { <h1>{ "404" }</h1> }
        },
    }
}




pub fn switch_get_started(routes: &RouteGetStarted) -> Html {
    match routes {
        RouteGetStarted::Home => html! {
            <GetStartedMain topic={ String::from("Home") } sub_topic={ String::from("Home") } />
        },
        RouteGetStarted::NotFound => {
            log::info!("not found from get started router");
            html! {
                <Redirect<Route> to={Route::NotFound} />
            }
        }
    }
}