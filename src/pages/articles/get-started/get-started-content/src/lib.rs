use yew::prelude::*;
use get_started_topics::{
    Topic,
    SubTopic,
};
use get_started_home::GetStartedHome;
use identity_fundamentals_home::IdentityFundamentalsHome;
use introduction_to_iam::IntroductionToIAM;
use authentication_vs_authorization::AuthenticationVsAuthorization;



#[derive(Properties, PartialEq, Debug)]
pub struct GetStartedContentProps {
    // #[prop_or_else(create_default_state_topic)]
    #[prop_or(Topic::Home)]
    pub topic: Topic,
    // #[prop_or_else(create_default_state_subtopic)]
    #[prop_or(SubTopic::Home)]
    pub sub_topic: SubTopic,
}


pub struct GetStartedContent {
    topic: Topic,
    sub_topic: SubTopic,
}

pub enum Msg {}

impl Component for GetStartedContent {
    type Message = Msg;
    type Properties = GetStartedContentProps;

    fn create(ctx: &Context<Self>) -> Self {
        let GetStartedContentProps {
            topic,
            sub_topic,
        } = ctx.props();

        GetStartedContent {
            topic: topic.to_owned(),
            sub_topic: sub_topic.to_owned(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if self.topic != ctx.props().topic {
            self.topic = ctx.props().topic.to_owned();
            true
        } else if self.sub_topic != ctx.props().sub_topic {
            self.sub_topic = ctx.props().sub_topic.to_owned();
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
                }
            }
        }
    }
}

