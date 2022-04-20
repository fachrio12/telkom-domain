use yew::prelude::*;
use route::{
    Route,
};

use home::Home;
use articles_home::ArticlesHome;
use get_started_main::GetStartedMain;
use get_started_topics::{
    Topic,
    SubTopic,
    SubTopic2,
};


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
            <GetStartedMain topic={ Topic::IdentityFundamentals } sub_topic={ SubTopic::IntroductionToIAM } />
        },
        Route::AuthenticationVsAuthorization => html! {
            <GetStartedMain topic={ Topic::IdentityFundamentals } sub_topic={ SubTopic::AuthenticationVsAuthorization } />
        },
        Route::GetStartedIdentityFundamentals => html! {
            <GetStartedMain topic={ Topic::IdentityFundamentals } sub_topic={ SubTopic::Home } />
        },

        Route::ActivityAbout => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::DomainDashboard } sub_topic_2={ SubTopic2::ActivityAbout } />
        },
        Route::DomainDashboard => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::DomainDashboard } />
        },

        Route::CreateTenants => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::CreateTenants } />
        },

        Route::GetStartedDomainOverview => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::Home } />
        },

        Route::GetStartedHome => html! {
            <GetStartedMain topic={ Topic::Home } sub_topic={ SubTopic::Home } />
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
