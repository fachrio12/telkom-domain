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
        Route::IntroductionToDomain => html! {
            <GetStartedMain topic={ Topic::IdentityFundamentals } sub_topic={ SubTopic::IntroductionToDomain } />
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

        Route::CreateMultipleTenants => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::CreateTenants } sub_topic_2={ SubTopic2::CreateMultipleTenants } />
        },
        Route::MultipleTenantsToSingleSubscription => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::CreateTenants } sub_topic_2={ SubTopic2::MultipleTenantsToSingleSubscription } />
        },
        Route::SetUpMultipleEnvironments => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::CreateTenants } sub_topic_2={ SubTopic2::SetUpMultipleEnvironments } />
        },
        Route::MultiTenantBestPractices => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::CreateTenants } sub_topic_2={ SubTopic2::MultiTenantBestPractices } />
        },
        Route::CreateTenants => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::CreateTenants } />
        },

        Route::RegisterApis => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::RegisterApis } />
        },

        Route::GetStartedDomainOverview => html! {
            <GetStartedMain topic={ Topic::DomainOverview } sub_topic={ SubTopic::Home } />
        },


        Route::RotateSigningKeys => html! {
            <GetStartedMain topic={ Topic::TenantSettings } sub_topic={ SubTopic::SigningKeys } sub_topic_2={ SubTopic2::RotateSigningKeys } />
        },
        Route::RevokeSigningKeys => html! {
            <GetStartedMain topic={ Topic::TenantSettings } sub_topic={ SubTopic::SigningKeys } sub_topic_2={ SubTopic2::RevokeSigningKeys } />
        },
        Route::ViewSigningCertificates => html! {
            <GetStartedMain topic={ Topic::TenantSettings } sub_topic={ SubTopic::SigningKeys } sub_topic_2={ SubTopic2::ViewSigningCertificates } />
        },
        Route::SigningKeys => html! {
            <GetStartedMain topic={ Topic::TenantSettings } sub_topic={ SubTopic::SigningKeys } />
        },

        Route::GetStartedTenantSettings => html! {
            <GetStartedMain topic={ Topic::TenantSettings } sub_topic={ SubTopic::Home } />
        },


        Route::GetStartedApplicationsInDomain => html! {
            <GetStartedMain topic={ Topic::ApplicationsInDomain } sub_topic={ SubTopic::Home } />
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
