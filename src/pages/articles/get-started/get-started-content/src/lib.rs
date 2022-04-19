use yew::prelude::*;
use get_started_topics::{
    Topic,
    SubTopic,
    SubTopic2,
};
use get_started_home::GetStartedHome;

use identity_fundamentals_home::IdentityFundamentalsHome;
use introduction_to_iam::IntroductionToIAM;
use authentication_vs_authorization::AuthenticationVsAuthorization;

use telkom_domain_overview_home::TelkomDomainOverviewHome;
use telkom_domain_dashboard::TelkomDomainDashboard;


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
        if self.topic != ctx.props().topic || self.sub_topic != ctx.props().sub_topic {
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
                    SubTopic::AuthenticationVsAuthorization => {
                        html! {
                            <AuthenticationVsAuthorization/>
                        }
                    }
                    _ => html! {}
                }
            }
            Topic::DomainOverview => {
                match self.sub_topic {
                    SubTopic::Home => {
                        html! {
                            <TelkomDomainOverviewHome/>
                        }
                    }
                    SubTopic::DomainDashboard => {
                        html! {
                            <TelkomDomainDashboard/>
                        }
                    }
                    _ => html! {}
                }
            }
        }
    }
}

