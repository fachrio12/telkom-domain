use yew::prelude::*;
use yew_router::prelude::*;
use route::{
    Route,
};

use home::Home;
use articles_home::ArticlesHome;
use get_started_main::GetStartedMain;


pub fn switch(routes: &Route) -> Html {
    match routes {
        Route::Home => html! {
            <Home/>
        },
        Route::Articles => html! {
            <ArticlesHome/>
        },
        // Route::GetStarted => html! {
        //     <Switch<RouteGetStarted> render={Switch::render(switch_get_started)} />
        // },
        Route::IntroductionToIAM => html! {
            <GetStartedMain topic={ String::from("Identity Fundamentals") } sub_topic={ String::from("Introduction To IAM") } />
        },
        Route::GetStartedIdentityFundamentals => html! {
            <GetStartedMain topic={ String::from("Identity Fundamentals") } sub_topic={ String::from("Home") } />
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




// pub fn switch_get_started(routes: &RouteGetStarted) -> Html {
//     match routes {
//         RouteGetStarted::Home => html! {
//             <GetStartedMain topic={ String::from("Home") } sub_topic={ String::from("Home") } />
//         },
//         RouteGetStarted::NotFound => {
//             log::info!("not found from get started router");
//             html! {
//                 <Redirect<Route> to={Route::NotFound} />
//             }
//         }
//     }
// }