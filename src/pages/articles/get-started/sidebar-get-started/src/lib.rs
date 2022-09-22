use yew::prelude::*;
use yew_router::prelude::*;
use sidebar_main_menu::SidebarMainMenu;
use route::{
    Route,
};
use get_started_topics::{
    Topic,
    SubTopic,
    SubTopic2,
};

#[derive(Clone, PartialEq, Debug)]
pub enum SidebarGetStartedState {
    MainMenu,
    GetStarted,
}

fn create_default_state() -> SidebarGetStartedState {
    SidebarGetStartedState::GetStarted
}


#[derive(Properties, PartialEq, Debug)]
pub struct SidebarGetStartedProps {
    #[prop_or_else(create_default_state)]
    state: SidebarGetStartedState,
    #[prop_or(Topic::Home)]
    pub topic: Topic,
    #[prop_or(SubTopic::Home)]
    pub sub_topic: SubTopic,
    #[prop_or(SubTopic2::Home)]
    pub sub_topic_2: SubTopic2,
}

pub struct SidebarGetStarted {
    state: SidebarGetStartedState,
    topic: Topic,
    sub_topic: SubTopic,
    sub_topic_2: SubTopic2,
}

pub enum Msg {
    UpdateState(SidebarGetStartedState),
}

impl Component for SidebarGetStarted {
    type Message = Msg;
    type Properties = SidebarGetStartedProps;

    fn create(ctx: &Context<Self>) -> Self {
        // let data_test = ctx.props();
        // log::info!("sidebar state ====== {:?}", data_test);

        SidebarGetStarted {
            state: ctx.props().state.to_owned(),
            topic: ctx.props().topic.to_owned(),
            sub_topic: ctx.props().sub_topic.to_owned(),
            sub_topic_2: ctx.props().sub_topic_2.to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateState(state) => {
                self.state = state;
                true
            }
        }
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

    fn view(&self, ctx: &Context<Self>) -> Html {
        log::info!("component state === {:?}", self.state);
        let is_main_menu = if self.state == SidebarGetStartedState::MainMenu {true} else {false};
        html! {
            <div class="uk-padding-small">
                {
                    if is_main_menu {
                        html! {
                            <div
                                class="uk-text-primary"
                                style="cursor: pointer;"
                                onclick={ ctx.link().callback(|_| Msg::UpdateState(SidebarGetStartedState::GetStarted)) }
                            >
                                <span
                                    class="td-text-weight-bold"
                                >
                                    { "Back" }
                                </span>
                                <span uk-icon="icon: arrow-right"></span>
                            </div>
                        }
                    } else {
                        html! {
                            <div
                                class="uk-text-primary"
                                style="cursor: pointer;"
                                onclick={ ctx.link().callback(|_| Msg::UpdateState(SidebarGetStartedState::MainMenu)) }
                            >
                                <span uk-icon="icon: arrow-left"></span>
                                <span
                                    class="td-text-weight-bold"
                                >
                                    { "Main Menu" }
                                </span>
                            </div>
                        }
                    }
                }

                {
                    if is_main_menu {
                        html! {
                            <SidebarMainMenu/>
                        }
                    } else {
                        self.view_get_started_menu()
                    }
                }

            </div>
        }
    }
}


impl SidebarGetStarted {
    fn view_get_started_menu (&self) -> Html {
        let topic = self.topic.to_owned();
        let sub_topic = self.sub_topic.to_owned();
        html! {
            <>

              <div style="overflow-y:scroll;">
                <ul
                    class="uk-nav uk-nav-default uk-margin-medium-top uk-margin-medium-bottom"
                    style="opacity: .8;"
                   
                >
                    
                    <li>
                        <Link<Route>
                            to={Route::GetStartedHome}
                            // classes="uk-text-muted"
                        >
                            <span
                                class="uk-text-center"
                                style="width: 45px; margin-top: 3px;"
                            >
                                <i
                                    class="fa-solid fa-bolt uk-text-primary uk-text-default"
                                >
                                </i>
                            </span>
                            <span
                                class="uk-text-secondary td-text-size-big td-text-weight-bold"
                            >
                                { "Get Started" }
                            </span>
                        </Link<Route>>
                    </li>
                </ul>

                <div
                    class="uk-margin-medium-bottom uk-text-small td-text-weight-500"
                >
                    <div
                        class="uk-margin-small-bottom"
                        style="text-transform: uppercase;"
                    >
                        { "Start building" }
                    </div>
                    <div
                        class="uk-text-muted"
                    >
                        <div>
                            { "Quickstart" }
                            <span uk-icon="icon: arrow-right"></span>
                        </div>
                    </div>
                </div>

                <div
                    class="uk-margin-medium-bottom uk-text-small td-text-weight-500"
                >
                    <div
                        class="uk-margin-small-bottom"
                        style="text-transform: uppercase;"
                    >
                        { "Learn the basic" }
                    </div>
                    <div
                        class="uk-text-muted uk-margin-small-bottom"
                    >
                        <div>
                            <Link<Route>
                                to={Route::GetStartedIdentityFundamentals}
                                classes="uk-text-muted"
                            >
                                { "Identity Fundamentals" }
                                <span
                                    style="float: right; padding-top: 4px;"
                                    uk-icon="icon: chevron-right;  ratio: 0.85">
                                </span>
                            </Link<Route>>
                            {
                                if topic == Topic::IdentityFundamentals {
                                    html! {
                                        <div>
                                            <div
                                                class="uk-margin-left td-sidebar-subtopic"
                                            >
                                                <Link<Route>
                                                    to={Route::IntroductionToIAM}
                                                    classes="uk-text-muted"
                                                    >
                                                    <span>
                                                        { "Introduction to Identity and Access Management (IAM)" }
                                                    </span>
                                                </Link<Route>>
                                            </div>
                                            <div
                                                class="uk-margin-left td-sidebar-subtopic"
                                            >
                                                <Link<Route>
                                                    to={Route::IntroductionToDomain}
                                                    classes="uk-text-muted"
                                                    >
                                                    <span>
                                                        { "Introduction to Domain" }
                                                    </span>
                                                </Link<Route>>
                                            </div>
                                            <div
                                                class="uk-margin-left td-sidebar-subtopic"
                                            >
                                                <Link<Route>
                                                    to={Route::AuthenticationVsAuthorization}
                                                    classes="uk-text-muted"
                                                    >
                                                    <span>
                                                        { "Authentication vs. Authorization" }
                                                    </span>
                                                </Link<Route>>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    </div>
                    <div
                        class="uk-text-muted uk-margin-small-bottom"
                    >
                        <div>
                            // <div>
                            //     { "Telkom Domain Overview" }
                            //     <span
                            //         style="float: right; padding-top: 4px;"
                            //         uk-icon="icon: chevron-right;  ratio: 0.85"></span>
                            // </div>
                            <Link<Route>
                                to={Route::GetStartedDomainOverview}
                                classes="uk-text-muted"
                            >
                                { "Telkom Domain Overview" }
                                <span
                                    style="float: right; padding-top: 4px;"
                                    uk-icon="icon: chevron-right;  ratio: 0.85">
                                </span>
                            </Link<Route>>
                            {
                                if topic == Topic::DomainOverview {
                                    html! {
                                        <div>
                                            <div
                                                class="uk-margin-left td-sidebar-subtopic"
                                            >
                                                <Link<Route>
                                                    to={Route::DomainDashboard}
                                                    classes="uk-text-muted"
                                                >
                                                    <span>
                                                        { "Telkom Domain Dashboard" }
                                                    </span>
                                                    <span
                                                        style="float: right; padding-top: 4px;"
                                                        uk-icon="icon: chevron-right;  ratio: 0.7">
                                                    </span>
                                                </Link<Route>>
                                                {
                                                    if sub_topic == SubTopic::DomainDashboard {
                                                        html! {
                                                            <div class="uk-margin-left td-sidebar-subtopic">
                                                                <Link<Route>
                                                                    to={Route::ActivityAbout}
                                                                    classes="uk-text-muted"
                                                                >
                                                                    <span>
                                                                        { "About The Activity Page" }
                                                                    </span>
                                                                </Link<Route>>
                                                            </div>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                }
                                            </div>
                                            <div
                                                class="uk-margin-left td-sidebar-subtopic"
                                            >
                                                <Link<Route>
                                                    to={Route::CreateTenants}
                                                    classes="uk-text-muted"
                                                >
                                                    <span>
                                                        { "Create Tenants" }
                                                    </span>
                                                    <span
                                                        style="float: right; padding-top: 4px;"
                                                        uk-icon="icon: chevron-right;  ratio: 0.7">
                                                    </span>
                                                </Link<Route>>
                                                {
                                                    if sub_topic == SubTopic::CreateTenants {
                                                        html! {
                                                            <>
                                                                <div class="uk-margin-left td-sidebar-subtopic">
                                                                    <Link<Route>
                                                                        to={Route::CreateMultipleTenants}
                                                                        classes="uk-text-muted"
                                                                    >
                                                                        <span>
                                                                            { "Create Multiple Tenants" }
                                                                        </span>
                                                                    </Link<Route>>
                                                                </div>
                                                                <div class="uk-margin-left td-sidebar-subtopic">
                                                                    <Link<Route>
                                                                        to={Route::MultipleTenantsToSingleSubscription}
                                                                        classes="uk-text-muted"
                                                                    >
                                                                        <span>
                                                                            { "Link Multiple Tenants to a Single Subscription" }
                                                                        </span>
                                                                    </Link<Route>>
                                                                </div>
                                                                <div class="uk-margin-left td-sidebar-subtopic">
                                                                    <Link<Route>
                                                                        to={Route::SetUpMultipleEnvironments}
                                                                        classes="uk-text-muted"
                                                                    >
                                                                        <span>
                                                                            { "Set Up Multiple Environments" }
                                                                        </span>
                                                                    </Link<Route>>
                                                                </div>
                                                                <div class="uk-margin-left td-sidebar-subtopic">
                                                                    <Link<Route>
                                                                        to={Route::MultiTenantBestPractices}
                                                                        classes="uk-text-muted"
                                                                    >
                                                                        <span>
                                                                            { "Multi-Tenant Applications Best Practices" }
                                                                        </span>
                                                                    </Link<Route>>
                                                                </div>
                                                            </>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                }
                                            </div>
                                            <div
                                                class="uk-margin-left td-sidebar-subtopic"
                                            >
                                                <Link<Route>
                                                    to={Route::RegisterApis}
                                                    classes="uk-text-muted"
                                                >
                                                    <span>
                                                        { "Register APIs" }
                                                    </span>
                                                </Link<Route>>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    </div>
                </div>

                <div
                    class="uk-margin-medium-bottom uk-text-small td-text-weight-500"
                >
                    <div
                        class="uk-margin-small-bottom"
                        style="text-transform: uppercase;"
                    >
                        { "Configure Telkom Domain" }
                    </div>
                    <div
                        class="uk-text-muted uk-margin-small-bottom"
                    >
                        <div>
                            <Link<Route>
                                to={Route::GetStartedTenantSettings}
                                classes="uk-text-muted"
                            >
                                { "Tenant Settings" }
                                <span
                                    style="float: right; padding-top: 4px;"
                                    uk-icon="icon: chevron-right;  ratio: 0.85">
                                </span>
                            </Link<Route>>
                            {
                                if topic == Topic::TenantSettings {
                                    html! {
                                        <div>
                                            <div
                                                class="uk-margin-left td-sidebar-subtopic"
                                            >
                                                <Link<Route>
                                                    to={Route::SigningKeys}
                                                    classes="uk-text-muted"
                                                >
                                                    <span>
                                                        { "Signing Keys" }
                                                    </span>
                                                    <span
                                                        style="float: right; padding-top: 4px;"
                                                        uk-icon="icon: chevron-right;  ratio: 0.7">
                                                    </span>
                                                </Link<Route>>
                                                {
                                                    if sub_topic == SubTopic::SigningKeys {
                                                        html! {
                                                            <>
                                                                <div class="uk-margin-left td-sidebar-subtopic">
                                                                    <Link<Route>
                                                                        to={Route::RotateSigningKeys}
                                                                        classes="uk-text-muted"
                                                                    >
                                                                        <span>
                                                                            { "Rotate Signing Keys" }
                                                                        </span>
                                                                    </Link<Route>>
                                                                </div>
                                                                <div class="uk-margin-left td-sidebar-subtopic">
                                                                    <Link<Route>
                                                                        to={Route::RevokeSigningKeys}
                                                                        classes="uk-text-muted"
                                                                    >
                                                                        <span>
                                                                            { "Revoke Signing Keys" }
                                                                        </span>
                                                                    </Link<Route>>
                                                                </div>
                                                                <div class="uk-margin-left td-sidebar-subtopic">
                                                                    <Link<Route>
                                                                        to={Route::ViewSigningCertificates}
                                                                        classes="uk-text-muted"
                                                                    >
                                                                        <span>
                                                                            { "View Signing Certificates" }
                                                                        </span>
                                                                    </Link<Route>>
                                                                </div>
                                                            </>
                                                        }
                                                    } else {
                                                        html! {}
                                                    }
                                                }
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    </div>

                    <div
                        class="uk-text-muted uk-margin-small-bottom"
                    >
                        <div>
                            <Link<Route>
                                to={Route::GetStartedApplicationsInDomain}
                                classes="uk-text-muted"
                            >
                                { "Applications in Domain" }
                                <span
                                    style="float: right; padding-top: 4px;"
                                    uk-icon="icon: chevron-right;  ratio: 0.85">
                                </span>
                            </Link<Route>>
                            {
                                if topic == Topic::ApplicationsInDomain {
                                    html! {
                                        <div>
                                            <div
                                                class="uk-margin-left td-sidebar-subtopic"
                                            >
                                                <Link<Route>
                                                    to={Route::ApplicationSettings}
                                                    classes="uk-text-muted"
                                                >
                                                    <span>
                                                        { "Application Settings" }
                                                    </span>
                                                </Link<Route>>
                                            </div>
                                            <div
                                                class="uk-margin-left td-sidebar-subtopic"
                                            >
                                                <Link<Route>
                                                    to={Route::RemoveApplications}
                                                    classes="uk-text-muted"
                                                >
                                                    <span>
                                                        { "Remove Applications" }
                                                    </span>
                                                </Link<Route>>
                                            </div>
                                        </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    </div>
                    <div
                        class="uk-text-muted uk-margin-small-bottom"
                        style="overflow-y:scroll;"
                    >
                        <div>
                            <Link<Route>
                                to={Route::GetStartedApis}
                                classes="uk-text-muted"
                            >
                                { "APIs" }
                                <span
                                    style="float: right; padding-top: 4px;"
                                    uk-icon="icon: chevron-right;  ratio: 0.85">
                                </span>
                            </Link<Route>>
                            {
                                if topic == Topic::Apis {
                                    html! {
                                        <div>
                                            <div
                                                class="uk-margin-left td-sidebar-subtopic"
                                            >
                                                <Link<Route>
                                                    to={Route::ApisSettings}
                                                    classes="uk-text-muted"
                                                >
                                                    <span>
                                                        { "API Settings" }
                                                    </span>
                                                </Link<Route>>
                                            </div>
                                            <div
                                            class="uk-margin-left td-sidebar-subtopic"
                                        >
                                            <Link<Route>
                                                to={Route::AddApiPermissions}
                                                classes="uk-text-muted"
                                            >
                                                <span>
                                                    { "Add API Permissions" }
                                                </span>
                                            </Link<Route>>
                                        </div>

                                        <div
                                            class="uk-margin-left td-sidebar-subtopic"
                                        >
                                            <Link<Route>
                                                to={Route::DeleteApiPermissions}
                                                classes="uk-text-muted"
                                            >
                                                <span>
                                                    { " Delete API Permissions" }
                                                </span>
                                            </Link<Route>>
                                        </div>  
                                        
                                        <div
                                        class="uk-margin-left td-sidebar-subtopic"
                                    >
                                        <Link<Route>
                                            to={Route::Scopes}
                                            classes="uk-text-muted"
                                        >
                                            <span>
                                                { "Scopes" }
                                            </span>
                                        </Link<Route>>
                                    </div>   

                              </div>
                                    }
                                } else {
                                    html! {}
                                }
                            }
                        </div>
                    </div>
                </div>

                </div>
            </>
        }
    }
}