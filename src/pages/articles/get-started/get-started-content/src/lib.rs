use yew::prelude::*;
use get_started_topics::{
    Topic,
    SubTopic,
    SubTopic2,
};
use get_started_home::GetStartedHome;

use identity_fundamentals_home::IdentityFundamentalsHome;
use introduction_to_iam::IntroductionToIAM;
use introduction_to_domain::IntroductionToDomain;
use authentication_vs_authorization::AuthenticationVsAuthorization;

use telkom_domain_overview_home::TelkomDomainOverviewHome;
use telkom_domain_dashboard_home::TelkomDomainDashboardHome;
use activity_about::ActivityAbout;
use create_tenants_home::CreateTenantsHome;
use create_multiple_tenants::CreateMultipleTenants;
use multiple_tenants_to_single_subscription::MultipleTenantsToSingleSubscription;
use set_up_multiple_environments::SetUpMultipleEnvironments;
use multi_tenant_apps_best_practices::MultiTenantBestPractices;
use register_apis::RegisterApis;

use tenant_settings_home::TenantSettingsHome;
use signing_keys_home::SigningKeysHome;
use rotate_signing_keys::RotateSigningKeys;
use revoke_signing_keys::RevokeSigningKeys;
use view_signing_certificates::ViewSigningCertificates;

use applications_in_domain_home::ApplicationsInDomainHome;
use application_settings::ApplicationSettings;


#[derive(Properties, PartialEq, Debug)]
pub struct GetStartedContentProps {
    #[prop_or(Topic::Home)]
    pub topic: Topic,
    #[prop_or(SubTopic::Home)]
    pub sub_topic: SubTopic,
    #[prop_or(SubTopic2::Home)]
    pub sub_topic_2: SubTopic2,
}


pub struct GetStartedContent {
    topic: Topic,
    sub_topic: SubTopic,
    sub_topic_2: SubTopic2,
}

pub enum Msg {}

impl Component for GetStartedContent {
    type Message = Msg;
    type Properties = GetStartedContentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let GetStartedContentProps {
            topic,
            sub_topic,
            sub_topic_2,
        } = ctx.props();

        GetStartedContent {
            topic: topic.to_owned(),
            sub_topic: sub_topic.to_owned(),
            sub_topic_2: sub_topic_2.to_owned(),
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
        match self.topic {
            Topic::Home => {
                html! {
                    <GetStartedHome/>
                }
            }
            Topic::IdentityFundamentals => {
                match self.sub_topic {
                    SubTopic::Home => {
                        html! {
                            <IdentityFundamentalsHome/>
                        }
                    }
                    SubTopic::IntroductionToIAM => {
                        html! {
                            <IntroductionToIAM/>
                        }
                    }
                    SubTopic::IntroductionToDomain => {
                        html! {
                            <IntroductionToDomain/>
                        }
                    }
                    SubTopic::AuthenticationVsAuthorization => {
                        html! {
                            <AuthenticationVsAuthorization/>
                        }
                    }
                    _ => html! {}
                }
            }
            Topic::DomainOverview => {
                log::info!("get started content, sub topic 2 ====== {:?}", self.sub_topic_2);
                match self.sub_topic {
                    SubTopic::Home => {
                        html! {
                            <TelkomDomainOverviewHome/>
                        }
                    }
                    SubTopic::DomainDashboard => {
                        match self.sub_topic_2 {
                            SubTopic2::Home => {
                                html! {
                                    <TelkomDomainDashboardHome/>
                                }
                            }
                            SubTopic2::ActivityAbout => {
                                html! {
                                    <ActivityAbout/>
                                }
                            }
                            _ => html! {}
                        }
                    }
                    SubTopic::CreateTenants => {
                        match self.sub_topic_2 {
                            SubTopic2::Home => html! {
                                <CreateTenantsHome/>
                            },
                            SubTopic2::CreateMultipleTenants => html! {
                                <CreateMultipleTenants/>
                            },
                            SubTopic2::MultipleTenantsToSingleSubscription => html! {
                                <MultipleTenantsToSingleSubscription/>
                            },
                            SubTopic2::SetUpMultipleEnvironments => html! {
                                <SetUpMultipleEnvironments/>
                            },
                            SubTopic2::MultiTenantBestPractices => html! {
                                <MultiTenantBestPractices/>
                            },
                            _ => html! {}
                        }
                    }
                    SubTopic::RegisterApis => {
                        html! {
                            <RegisterApis/>
                        }
                    }
                    _ => html! {}
                }
            }
            Topic::TenantSettings => {
                match self.sub_topic {
                    SubTopic::Home => {
                        html! {
                            <TenantSettingsHome/>
                        }
                    }
                    SubTopic::SigningKeys => {
                        match self.sub_topic_2 {
                            SubTopic2::Home => {
                                html! {
                                    <SigningKeysHome/>
                                }
                            }
                            SubTopic2::RotateSigningKeys => {
                                html! {
                                    <RotateSigningKeys/>
                                }
                            }
                            SubTopic2::RevokeSigningKeys => {
                                html! {
                                    <RevokeSigningKeys/>
                                }
                            }
                            SubTopic2::ViewSigningCertificates => {
                                html! {
                                    <ViewSigningCertificates/>
                                }
                            }
                            _ => html! {}
                        }
                    }
                    _ => html! {}
                }
            }
            Topic::ApplicationsInDomain => {
                match self.sub_topic {
                    SubTopic::Home => {
                        html! {
                            <ApplicationsInDomainHome/>
                        }
                    }
                    SubTopic::ApplicationSettings => {
                        html! {
                            <ApplicationSettings/>
                        }
                    }
                    _ => html! {}
                }
            }
        }
    }
}

