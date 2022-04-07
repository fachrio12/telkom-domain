use yew::prelude::*;
use sidebar_get_started::SidebarGetStarted;
use get_started_home::GetStartedHome;
use identity_fundamentals_home::IdentityFundamentalsHome;



#[derive(Properties, PartialEq, Debug)]
pub struct GetStartedContentProps {
    // #[prop_or_else(create_default_state_topic)]
    #[prop_or("Home".to_string())]
    pub topic: String,
    // #[prop_or_else(create_default_state_subtopic)]
    #[prop_or("Home".to_string())]
    pub sub_topic: String,
}


pub struct GetStartedContent {
    topic: String,
    sub_topic: String,
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
        } else {
            false
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        match self.topic.as_str() {
            "Home" => {
                html! {
                    <GetStartedHome/>
                }
            }
            "Identity Fundamentals" => {
                match self.sub_topic.as_str() {
                    "Home" => {
                        html! {
                            <IdentityFundamentalsHome/>
                        }
                    }
                    _ => {
                        // CHANGE TO REDIRECT TO HOME
                        html! {
                            <div>{ "PAGE NOT FOUND" }</div>
                        }
                    }
                }
            }
            _ => {
                html! {
                    <GetStartedHome/>
                }
            }
        }
    }
}

