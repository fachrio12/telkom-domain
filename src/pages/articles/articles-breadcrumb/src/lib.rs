use yew::prelude::*;
use get_started_topics::{
    Topic,
    SubTopic,
    SubTopic2,
};
use yew_router::prelude::*;
use route::{
    Route,
};


#[derive(Properties, PartialEq, Debug)]
pub struct ArticlesBreadcrumbProps {
    #[prop_or(Topic::Home)]
    pub topic: Topic,
    #[prop_or(SubTopic::Home)]
    pub sub_topic: SubTopic,
    #[prop_or(SubTopic2::Home)]
    pub sub_topic_2: SubTopic2,
}

pub struct ArticlesBreadcrumb {
    topic: Topic,
    sub_topic: SubTopic,
    sub_topic_2: SubTopic2,
}

pub enum Msg {}

impl Component for ArticlesBreadcrumb {
    type Message = Msg;
    type Properties = ArticlesBreadcrumbProps;

    fn create(ctx: &Context<Self>) -> Self {
        // log::info!("This is log from articles breadcrumb");
        ArticlesBreadcrumb {
            topic: ctx.props().topic.to_owned(),
            sub_topic: ctx.props().sub_topic.to_owned(),
            sub_topic_2: ctx.props().sub_topic_2.to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.topic != ctx.props().topic || self.sub_topic != ctx.props().sub_topic || self.sub_topic_2 != ctx.props().sub_topic_2 {
            self.topic = ctx.props().topic.to_owned();
            self.sub_topic = ctx.props().sub_topic.to_owned();
            self.sub_topic_2 = ctx.props().sub_topic_2.to_owned();
            true
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {

        html! {
            <ul class="uk-breadcrumb">
                <li>
                    <Link<Route>
                        to={Route::Home}
                    >
                        { "Docs" }
                    </Link<Route>>
                </li>
                <li>
                    <Link<Route>
                        to={Route::Home}
                    >
                        { "Get Started" }
                    </Link<Route>>
                </li>
                {
                    match self.topic {
                        Topic::Home => html! {},
                        Topic::IdentityFundamentals => html! {
                            <li>
                                <Link<Route>
                                    to={Route::GetStartedIdentityFundamentals}
                                >
                                    { "Identity Fundamentals" }
                                </Link<Route>>
                            </li>
                        },
                        Topic::DomainOverview => html! {
                            <li>
                                <Link<Route>
                                    to={Route::GetStartedDomainOverview}
                                >
                                    { "Telkom Domain Overview" }
                                </Link<Route>>
                            </li>
                        },
                        Topic::TenantSettings => html! {
                            <li>
                                <Link<Route>
                                    to={Route::GetStartedTenantSettings}
                                >
                                    { "Tenant Settings" }
                                </Link<Route>>
                            </li>
                        },
                    }
                }
                {
                    match self.sub_topic {
                        SubTopic::Home => html! {},
                        SubTopic::IntroductionToIAM => html! {
                            <li>
                                <Link<Route>
                                    to={Route::IntroductionToIAM}
                                >
                                    { "Introduction to Identity and Access Management (IAM)" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic::IntroductionToDomain => html! {
                            <li>
                                <Link<Route>
                                    to={Route::IntroductionToDomain}
                                >
                                    { "Introduction to Domain" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic::AuthenticationVsAuthorization => html! {
                            <li>
                                <Link<Route>
                                    to={Route::AuthenticationVsAuthorization}
                                >
                                    { "Authentication vs. Authorization" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic::DomainDashboard => html! {
                            <li>
                                <Link<Route>
                                    to={Route::DomainDashboard}
                                >
                                    { "Telkom Domain Dashboard" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic::CreateTenants => html! {
                            <li>
                                <Link<Route>
                                    to={Route::CreateTenants}
                                >
                                    { "Create Tenants" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic::RegisterApis => html! {
                            <li>
                                <Link<Route>
                                    to={Route::RegisterApis}
                                >
                                    { "Register APIs" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic::SigningKeys => html! {
                            <li>
                                <Link<Route>
                                    to={Route::SigningKeys}
                                >
                                    { "Signing Keys" }
                                </Link<Route>>
                            </li>
                        },
                    }
                }
                {
                    match self.sub_topic_2 {
                        SubTopic2::Home => html! {},
                        SubTopic2::ActivityAbout => html! {
                            <li>
                                <Link<Route>
                                    to={Route::ActivityAbout}
                                >
                                    { "About The Activity Page" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic2::CreateMultipleTenants => html! {
                            <li>
                                <Link<Route>
                                    to={Route::CreateMultipleTenants}
                                >
                                    { "Create Multiple Tenants" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic2::MultipleTenantsToSingleSubscription => html! {
                            <li>
                                <Link<Route>
                                    to={Route::MultipleTenantsToSingleSubscription}
                                >
                                    { "Link Multiple Tenants to a Single Subscription" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic2::SetUpMultipleEnvironments => html! {
                            <li>
                                <Link<Route>
                                    to={Route::SetUpMultipleEnvironments}
                                >
                                    { "Set Up Multiple Environments" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic2::MultiTenantBestPractices => html! {
                            <li>
                                <Link<Route>
                                    to={Route::MultiTenantBestPractices}
                                >
                                    { "Multi-Tenant Applications Best Practices" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic2::RotateSigningKeys => html! {
                            <li>
                                <Link<Route>
                                    to={Route::RotateSigningKeys}
                                >
                                    { "Rotate Signing Keys" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic2::RevokeSigningKeys => html! {
                            <li>
                                <Link<Route>
                                    to={Route::RevokeSigningKeys}
                                >
                                    { "Revoke Signing Keys" }
                                </Link<Route>>
                            </li>
                        },
                        SubTopic2::ViewSigningCertificates => html! {
                            <li>
                                <Link<Route>
                                    to={Route::ViewSigningCertificates}
                                >
                                    { "Revoke Signing Keys" }
                                </Link<Route>>
                            </li>
                        },
                    }
                }
            </ul>
        }

    }
}
